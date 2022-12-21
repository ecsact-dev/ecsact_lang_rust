use std::fmt::Write;

use ecsact_dylib_runtime::meta;
use proc_macro2::TokenStream;
use quote::quote;

fn rust_array_maybe(length: i32, type_str: &str) -> proc_macro2::TokenStream {
	match length {
		0 => unimplemented!(),
		1 => type_str.parse().unwrap(),
		1.. => format!("[{}; {}]", type_str, length).parse().unwrap(),
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
		ecsact::FieldType::Entity { length } => todo!(),
		ecsact::FieldType::Enum { id, length } => todo!(),
	}
}

#[ecsact_codegen::plugin_entry("rs")]
fn woohoo(ctx: &mut ecsact::CodegenPluginContext) {
	writeln!(ctx, "/// Generated file - DO NOT EDIT").unwrap();

	for comp_id in meta::get_component_ids(ctx.package_id()).into_iter() {
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
			pub struct #comp_name {
				#(#fields),*
			}
		};

		writeln!(ctx, "{}", comp_struct.to_string().as_str()).unwrap();
	}
}
