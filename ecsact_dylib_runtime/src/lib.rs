#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;

#[allow(unused)]
mod internal {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe fn u8cstr_to_string(c_buf: *const i8) -> String {
	let c_str: &CStr = CStr::from_ptr(c_buf);
	let str_slice: &str = c_str.to_str().unwrap();
	str_slice.to_owned()
}

macro_rules! call_or_panic {
	($fn_name:ident $(, $fn_args:expr)*) => {{
		let expect_message = stringify!($fn_name).to_string() + "is not loaded";
		$fn_name.expect(expect_message.as_str())($($fn_args),*)
	}};
}

pub mod meta {
	use crate::internal::ecsact_meta_component_name;
	use crate::internal::ecsact_meta_count_components;
	use crate::internal::ecsact_meta_count_enum_values;
	use crate::internal::ecsact_meta_count_enums;
	use crate::internal::ecsact_meta_count_fields;
	use crate::internal::ecsact_meta_count_packages;
	use crate::internal::ecsact_meta_count_top_level_systems;
	use crate::internal::ecsact_meta_enum_name;
	use crate::internal::ecsact_meta_enum_storage_type;
	use crate::internal::ecsact_meta_enum_value;
	use crate::internal::ecsact_meta_enum_value_name;
	use crate::internal::ecsact_meta_field_name;
	use crate::internal::ecsact_meta_field_type;
	use crate::internal::ecsact_meta_get_component_ids;
	use crate::internal::ecsact_meta_get_enum_ids;
	use crate::internal::ecsact_meta_get_enum_value_ids;
	use crate::internal::ecsact_meta_get_field_ids;
	use crate::internal::ecsact_meta_get_package_ids;
	use crate::internal::ecsact_meta_get_top_level_systems;
	use crate::internal::ecsact_meta_package_file_path;
	use crate::internal::ecsact_meta_package_name;

	use crate::internal::ecsact_builtin_type as ebt;
	use crate::internal::ecsact_type_kind;

	pub fn enum_name(enum_id: ecsact::EnumId) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_enum_name,
				enum_id.into()
			))
		}
	}

	pub fn enum_storage_type(enum_id: ecsact::EnumId) -> ecsact::IntType {
		unsafe {
			let result =
				call_or_panic!(ecsact_meta_enum_storage_type, enum_id.into());

			match result {
				ebt::ECSACT_I8 => ecsact::IntType::I8,
				ebt::ECSACT_U8 => ecsact::IntType::U8,
				ebt::ECSACT_I16 => ecsact::IntType::I16,
				ebt::ECSACT_U16 => ecsact::IntType::U16,
				ebt::ECSACT_U32 => ecsact::IntType::U32,
				ebt::ECSACT_I32 => ecsact::IntType::I32,
				_ => unimplemented!(),
			}
		}
	}

	pub fn enum_value(
		enum_id: ecsact::EnumId,
		value_id: ecsact::EnumValueId,
	) -> i32 {
		unsafe {
			call_or_panic!(
				ecsact_meta_enum_value,
				enum_id.into(),
				value_id.into()
			)
		}
	}

	pub fn enum_value_name(
		enum_id: ecsact::EnumId,
		value_id: ecsact::EnumValueId,
	) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_enum_value_name,
				enum_id.into(),
				value_id.into()
			))
		}
	}

	pub fn count_enum_values(id: ecsact::EnumId) -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_enum_values, id.into()) }
	}

	pub fn get_enum_value_ids(id: ecsact::EnumId) -> Vec<ecsact::EnumValueId> {
		let mut ids = vec![-1; count_enum_values(id) as usize];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_enum_value_ids,
				id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::EnumValueId { id.into() })
			.collect()
	}

	pub fn count_enums(id: ecsact::PackageId) -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_enums, id.into()) }
	}

	pub fn get_enum_ids(id: ecsact::PackageId) -> Vec<ecsact::EnumId> {
		let mut ids = vec![-1; count_enums(id) as usize];

		unsafe {
			call_or_panic!(
				ecsact_meta_get_enum_ids,
				id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::EnumId { id.into() })
			.collect()
	}

	pub fn count_fields(id: ecsact::CompositeId) -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_fields, id.into()) }
	}

	pub fn field_name(
		compo_id: ecsact::CompositeId,
		field_id: ecsact::FieldId,
	) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_field_name,
				compo_id.into(),
				field_id.into()
			))
		}
	}

	pub fn field_type(
		compo_id: ecsact::CompositeId,
		field_id: ecsact::FieldId,
	) -> ecsact::FieldType {
		unsafe {
			let cfield_type = call_or_panic!(
				ecsact_meta_field_type,
				compo_id.into(),
				field_id.into()
			);

			match cfield_type.kind {
				ecsact_type_kind::ECSACT_TYPE_KIND_BUILTIN => {
					let length = cfield_type.length;
					match cfield_type.type_.builtin {
						ebt::ECSACT_BOOL => ecsact::FieldType::Bool { length },
						ebt::ECSACT_I8 => ecsact::FieldType::I8 { length },
						ebt::ECSACT_U8 => ecsact::FieldType::U8 { length },
						ebt::ECSACT_I16 => ecsact::FieldType::I16 { length },
						ebt::ECSACT_U16 => ecsact::FieldType::U16 { length },
						ebt::ECSACT_I32 => ecsact::FieldType::I32 { length },
						ebt::ECSACT_U32 => ecsact::FieldType::U32 { length },
						ebt::ECSACT_F32 => ecsact::FieldType::F32 { length },
						ebt::ECSACT_ENTITY_TYPE => {
							ecsact::FieldType::Entity { length }
						}
						_ => unimplemented!(),
					}
				}
				ecsact_type_kind::ECSACT_TYPE_KIND_ENUM => {
					ecsact::FieldType::Enum {
						id: cfield_type.type_.enum_id.into(),
						length: cfield_type.length,
					}
				}
				_ => unimplemented!(),
			}
		}
	}

	pub fn get_field_ids(id: ecsact::CompositeId) -> Vec<ecsact::FieldId> {
		let mut ids = vec![-1; count_fields(id) as usize];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_field_ids,
				id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::FieldId { id.into() })
			.collect()
	}

	pub fn count_components(id: ecsact::PackageId) -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_components, id.into()) }
	}

	pub fn get_component_ids(
		pkg_id: ecsact::PackageId,
	) -> Vec<ecsact::ComponentId> {
		let mut ids = vec![-1; count_components(pkg_id) as usize];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_component_ids,
				pkg_id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::ComponentId { id.into() })
			.collect()
	}

	pub fn component_name(component_id: ecsact::ComponentId) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_component_name,
				component_id.into()
			))
		}
	}

	pub fn count_top_level_systems(id: ecsact::PackageId) -> i32 {
		unsafe {
			call_or_panic!(ecsact_meta_count_top_level_systems, id.into())
		}
	}

	pub fn get_top_level_systems(
		pkg_id: ecsact::PackageId,
	) -> Vec<ecsact::SystemLikeId> {
		let mut ids = vec![-1; count_top_level_systems(pkg_id) as usize];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_top_level_systems,
				pkg_id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::SystemLikeId { id.into() })
			.collect()
	}

	pub fn count_packages() -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_packages) }
	}

	pub fn get_package_ids() -> Vec<ecsact::PackageId> {
		let mut ids = vec![-1; count_packages() as usize];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_package_ids,
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				std::ptr::null_mut::<i32>()
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::PackageId { id.into() })
			.collect()
	}

	pub fn package_name(package_id: ecsact::PackageId) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_package_name,
				package_id.into()
			))
		}
	}

	pub fn package_file_path(package_id: i32) -> String {
		unsafe {
			crate::u8cstr_to_string(call_or_panic!(
				ecsact_meta_package_file_path,
				package_id
			))
		}
	}
}
