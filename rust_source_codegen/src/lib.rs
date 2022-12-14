use std::fmt::Write;

use ecsact_dylib_runtime::meta;
use indented::indented;
use proc_macro2::TokenStream;
use quote::quote;

fn rust_array_maybe(length: i32, type_str: &str) -> proc_macro2::TokenStream {
	match length {
		0 => unimplemented!(),
		1 => type_str.parse().unwrap(),
		2.. => format!("[{}; {}]", type_str, length).parse().unwrap(),
		_ => panic!("mistakes were made"),
	}
}

fn to_rust_type(t: ecsact::FieldType) -> proc_macro2::TokenStream {
	match t {
		ecsact::FieldType::Bool { length } => rust_array_maybe(length, "bool"),
		ecsact::FieldType::I8 { length } => rust_array_maybe(length, "i8"),
		ecsact::FieldType::U8 { length } => rust_array_maybe(length, "u8"),
		ecsact::FieldType::I16 { length } => rust_array_maybe(length, "i16"),
		ecsact::FieldType::U16 { length } => rust_array_maybe(length, "u16"),
		ecsact::FieldType::I32 { length } => rust_array_maybe(length, "i32"),
		ecsact::FieldType::U32 { length } => rust_array_maybe(length, "u32"),
		ecsact::FieldType::F32 { length } => rust_array_maybe(length, "f32"),
		ecsact::FieldType::Entity { length } => {
			rust_array_maybe(length, "::ecsact::EntityId")
		}
		ecsact::FieldType::Enum { id: _, length: _ } => todo!(),
	}
}

fn is_action(
	pkg_id: ecsact::PackageId,
	sys_like_id: ecsact::SystemLikeId,
) -> bool {
	// this is ecessive
	meta::get_action_ids(pkg_id)
		.into_iter()
		.map(|id| -> ecsact::SystemLikeId { id.into() })
		.any(|id| id == sys_like_id)
}

#[ecsact_macro::plugin_entry("rs")]
fn rust_source_codegen_entry(ctx: &mut ecsact::CodegenPluginContext) {
	write!(ctx, "/// Generated file - DO NOT EDIT\n\n").unwrap();

	let pkg_name = meta::package_name(ctx.package_id());

	write!(ctx, "pub mod {} {{\n\n", pkg_name).unwrap();

	for enum_id in meta::get_enum_ids(ctx.package_id()).into_iter() {
		let enum_name = proc_macro2::Ident::new(
			&meta::enum_name(enum_id),
			proc_macro2::Span::call_site(),
		);

		let mut enum_values: Vec<TokenStream> = Vec::new();

		for enum_value_id in meta::get_enum_value_ids(enum_id).into_iter() {
			let enum_value_name = proc_macro2::Ident::new(
				&meta::enum_value_name(enum_id, enum_value_id),
				proc_macro2::Span::call_site(),
			);
			let enum_value = proc_macro2::Literal::i32_unsuffixed(
				meta::enum_value(enum_id, enum_value_id),
			);

			enum_values.push(quote! {
				#enum_value_name = #enum_value
			});
		}

		let storage_type =
			&to_rust_type(meta::enum_storage_type(enum_id).into());

		let enum_type = quote! {
			#[repr(#storage_type)]
			pub enum #enum_name {
				#(#enum_values),*
			}
		};

		writeln!(ctx, "{}", indented(enum_type.to_string().as_str())).unwrap();
	}

	writeln!(ctx).unwrap();

	for comp_id in meta::get_component_ids(ctx.package_id()).into_iter() {
		let comp_id_num: i32 = comp_id.into();

		let comp_name = proc_macro2::Ident::new(
			meta::component_name(comp_id).as_str(),
			proc_macro2::Span::call_site(),
		);

		let mut fields: Vec<TokenStream> = Vec::new();

		for field_id in meta::get_field_ids(comp_id.into()).into_iter() {
			let field_name = proc_macro2::Ident::new(
				&meta::field_name(comp_id.into(), field_id),
				proc_macro2::Span::call_site(),
			);
			let field_type =
				&to_rust_type(meta::field_type(comp_id.into(), field_id));

			fields.push(quote! {
				pub #field_name: #field_type
			});
		}

		let comp_struct = quote! {
			#[repr(C)]
			pub struct #comp_name {
				#(#fields),*
			}

			impl ::ecsact::Component for #comp_name {
				const ID: ::ecsact::ComponentId = unsafe { ::ecsact::ComponentId::new(#comp_id_num) };
			}
			impl ::ecsact::ComponentLike for #comp_name {
				const ID: ::ecsact::ComponentLikeId = unsafe { ::ecsact::ComponentLikeId::new(#comp_id_num) };
			}
			impl ::ecsact::Composite for #comp_name {
				const ID: ::ecsact::CompositeId = unsafe { ::ecsact::CompositeId::new(#comp_id_num) };
			}
		};

		writeln!(ctx, "{}", indented(comp_struct.to_string())).unwrap();
	}

	for sys_like_id in meta::get_top_level_systems(ctx.package_id()) {
		let type_id = if is_action(ctx.package_id(), sys_like_id) {
			proc_macro2::Ident::new("ActionId", proc_macro2::Span::call_site())
		} else {
			proc_macro2::Ident::new("SystemId", proc_macro2::Span::call_site())
		};

		let sys_mod = create_system_like_rust_mod(
			sys_like_id,
			type_id,
			&(pkg_name.to_owned() + "."),
		);
		writeln!(ctx, "\t{sys_mod}").unwrap();
	}

	writeln!(ctx, "}}").unwrap();
}

fn create_system_like_rust_mod(
	sys_like_id: ecsact::SystemLikeId,
	type_id: proc_macro2::Ident,
	strip_prefix: &str,
) -> proc_macro2::TokenStream {
	let decl_full_name = meta::decl_full_name(sys_like_id.into());

	let decl_name = decl_full_name.strip_prefix(strip_prefix).unwrap();
	let decl_name_ident =
		proc_macro2::Ident::new(decl_name, proc_macro2::Span::call_site());

	let sys_like_id_lit =
		proc_macro2::Literal::i32_unsuffixed(sys_like_id.into());

	let mut child_mods: Vec<proc_macro2::TokenStream> = Vec::new();

	for child_id in dbg!(meta::get_child_system_ids(sys_like_id)) {
		child_mods.push(create_system_like_rust_mod(
			child_id.into(),
			proc_macro2::Ident::new("SystemId", proc_macro2::Span::call_site()),
			&(decl_full_name.to_owned() + "."),
		));
	}

	let mut allow_get_fn = Vec::new();
	let mut allow_update_fn = Vec::new();
	let mut allow_add_fn = Vec::new();
	let mut allow_remove_fn = Vec::new();
	let mut allow_has_fn = Vec::new();

	for (comp_id, cap) in meta::system_capabilities(sys_like_id) {
		match cap {
			ecsact::SystemCapability::Readonly { optional } => {
				allow_get_fn.push(comp_id);
				if optional {
					allow_has_fn.push(comp_id);
				}
			}
			ecsact::SystemCapability::Writeonly { optional: _ } => {
				unimplemented!();
			}
			ecsact::SystemCapability::Readwrite { optional } => {
				allow_update_fn.push(comp_id);
				allow_get_fn.push(comp_id);

				if optional {
					allow_has_fn.push(comp_id);
				}
			}
			ecsact::SystemCapability::Include => {}
			ecsact::SystemCapability::Exclude => {}
			ecsact::SystemCapability::Adds => {
				allow_add_fn.push(comp_id);
			}
			ecsact::SystemCapability::Removes => {
				allow_remove_fn.push(comp_id);
			}
		}
	}

	let addable_trait = make_context_addable_trait(&allow_add_fn);
	let removable_trait = make_context_removable_trait(&allow_remove_fn);
	let gettable_trait = make_context_gettable_trait(&allow_get_fn);
	let updatable_trait = make_context_updatable_trait(&allow_update_fn);

	let add_fn = make_context_add_fn(&allow_add_fn);
	let remove_fn = make_context_remove_fn(&allow_remove_fn);
	let get_fn = make_context_get_fn(&allow_get_fn);
	let update_fn = make_context_update_fn(&allow_get_fn);

	quote! {
		#[allow(non_snake_case)]
		pub mod #decl_name_ident {
			pub const ID: ::ecsact::#type_id = unsafe {
				::ecsact::#type_id::new(#sys_like_id_lit)
			};

			#addable_trait
			#removable_trait
			#gettable_trait
			#updatable_trait

			#[repr(transparent)]
			pub struct __Context(pub *mut ::std::ffi::c_void);

			impl __Context {
				#get_fn
				#update_fn
				#add_fn
				#remove_fn
			}

			#(#child_mods)*

			pub type Context = __Context;
		}
	}
}

fn make_context_add_fn(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	Some(quote! {
		pub fn add<T: __AddableComponent + ::ecsact::ComponentLike>(&mut self, comp: &T) {
			unsafe {
				::ecsact_system_execution_context::add(
					::ecsact_system_execution_context::Context::new(self.0),
					comp,
				);
			}
		}
	})
}

fn make_context_remove_fn(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	Some(quote! {
		pub fn remove<T: __RemovableComponent + ::ecsact::ComponentLike>(
			&mut self,
		) {
			unsafe {
				::ecsact_system_execution_context::remove(
					::ecsact_system_execution_context::Context::new(self.0),
					T::ID,
				);
			}
		}
	})
}

fn make_context_get_fn(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	Some(quote! {
		pub fn get<T: __GettableComponent + ::ecsact::ComponentLike>(
			&mut self,
		) -> T {
			unsafe {
				let mut component: T =
					::std::mem::MaybeUninit::uninit().assume_init();
				::ecsact_system_execution_context::get(
					::ecsact_system_execution_context::Context::new(self.0),
					&mut component,
				);
				component
			}

		}
	})
}

fn make_context_update_fn(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	Some(quote! {
		pub fn update<T: __UpdatableComponent + ::ecsact::ComponentLike>(
			&mut self,
			component: &T,
		) {
			unsafe {
				::ecsact_system_execution_context::update(
					::ecsact_system_execution_context::Context::new(self.0),
					component,
				);
			}
		}
	})
}

fn make_context_addable_trait(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	let comp_trait_impls: Vec<proc_macro2::TokenStream> = comps
		.iter()
		.cloned()
		.map(|comp_id| {
			let comp_full_name = meta::decl_full_name(comp_id.into());
			let comp_ident =
				ecsact::support::ecsact_ident_to_rust_ident(&comp_full_name);

			quote! {
				impl __AddableComponent for crate::#comp_ident {}
			}
		})
		.collect();

	Some(quote! {
		pub trait __AddableComponent: ecsact::ComponentLike {}
		#(#comp_trait_impls)*
	})
}

fn make_context_removable_trait(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	let comp_trait_impls: Vec<proc_macro2::TokenStream> = comps
		.iter()
		.cloned()
		.map(|comp_id| {
			let comp_full_name = meta::decl_full_name(comp_id.into());
			let comp_ident =
				ecsact::support::ecsact_ident_to_rust_ident(&comp_full_name);

			quote! {
				impl __RemovableComponent for crate::#comp_ident {}
			}
		})
		.collect();

	Some(quote! {
		pub trait __RemovableComponent: ecsact::ComponentLike {}
		#(#comp_trait_impls)*
	})
}

fn make_context_gettable_trait(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	let comp_trait_impls: Vec<proc_macro2::TokenStream> = comps
		.iter()
		.cloned()
		.map(|comp_id| {
			let comp_full_name = meta::decl_full_name(comp_id.into());
			let comp_ident =
				ecsact::support::ecsact_ident_to_rust_ident(&comp_full_name);

			quote! {
				impl __GettableComponent for crate::#comp_ident {}
			}
		})
		.collect();

	Some(quote! {
		pub trait __GettableComponent: ecsact::ComponentLike {}
		#(#comp_trait_impls)*
	})
}

fn make_context_updatable_trait(
	comps: &Vec<ecsact::ComponentLikeId>,
) -> Option<proc_macro2::TokenStream> {
	if comps.is_empty() {
		return None;
	}

	let comp_trait_impls: Vec<proc_macro2::TokenStream> = comps
		.iter()
		.cloned()
		.map(|comp_id| {
			let comp_full_name = meta::decl_full_name(comp_id.into());
			let comp_ident =
				ecsact::support::ecsact_ident_to_rust_ident(&comp_full_name);

			quote! {
				impl __UpdatableComponent for crate::#comp_ident {}
			}
		})
		.collect();

	Some(quote! {
		pub trait __UpdatableComponent: ecsact::ComponentLike {}
		#(#comp_trait_impls)*
	})
}
