// Generated by the ecsact_rtb crate. Do not edit.

use std::ffi::c_void;

use crate::bindings::{
	// format line by line
	ecsact_set_system_execution_impl,
	ecsact_system_execution_impl,
};

pub fn set_system_execution_impl(
	sys_like_id: ecsact::SystemLikeId,
	system_impl_fn: extern "C" fn(*mut c_void),
) {
	unsafe {
		ecsact_set_system_execution_impl(
			sys_like_id.into(),
			*(&system_impl_fn as *const _
				as *const ecsact_system_execution_impl),
		);
	}
}
