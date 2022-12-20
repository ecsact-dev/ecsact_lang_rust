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
	use crate::internal::ecsact_meta_count_packages;
	use crate::internal::ecsact_meta_count_top_level_systems;
	use crate::internal::ecsact_meta_get_component_ids;
	use crate::internal::ecsact_meta_get_package_ids;
	use crate::internal::ecsact_meta_get_top_level_systems;
	use crate::internal::ecsact_meta_package_file_path;
	use crate::internal::ecsact_meta_package_name;

	pub fn count_components(id: ecsact::PackageId) -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_components, id.into()) }
	}

	pub fn get_component_ids(
		pkg_id: ecsact::PackageId,
	) -> Vec<ecsact::ComponentId> {
		let mut ids = vec![-1, count_components(pkg_id).try_into().unwrap()];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_component_ids,
				pkg_id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				0 as *mut i32
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
		let mut ids =
			vec![-1, count_top_level_systems(pkg_id).try_into().unwrap()];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_top_level_systems,
				pkg_id.into(),
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				0 as *mut i32
			);
		}

		ids.into_iter()
			.map(|id| -> ecsact::SystemLikeId { id.into() })
			.collect()
	}

	pub fn count_packages() -> i32 {
		unsafe { call_or_panic!(ecsact_meta_count_packages).into() }
	}

	pub fn get_package_ids() -> Vec<ecsact::PackageId> {
		let mut ids = vec![-1; count_packages().try_into().unwrap()];
		unsafe {
			call_or_panic!(
				ecsact_meta_get_package_ids,
				ids.len().try_into().unwrap(),
				ids.as_mut_ptr() as *mut i32,
				0 as *mut i32
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
