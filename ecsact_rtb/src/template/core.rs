// Generated by the ecsact_rtb crate. Do not edit.

use ecsact::{EntityId, RegistryId};
use std::ffi::{c_void, CStr};

use crate::bindings::{
	// format
	ecsact_add_component,
	ecsact_create_entity,
	ecsact_create_registry,
	ecsact_execute_systems,
	ecsact_execution_events_collector,
};

pub fn create_entity(registry: RegistryId) -> EntityId {
	unsafe { ecsact_create_entity(registry.into()).into() }
}

pub fn add_component<C: ecsact::Component>(
	registry: RegistryId,
	entity: EntityId,
	component: &C,
) {
	unsafe {
		ecsact_add_component(
			registry.into(),
			entity.into(),
			<C as ecsact::Component>::ID.into(),
			component as *const _ as *const std::ffi::c_void,
		);
	}
}

pub fn create_registry(registry_name: &str) -> RegistryId {
	let registry_name_nul = registry_name.to_owned() + "\0";
	let registry_name_cstr =
		CStr::from_bytes_with_nul(registry_name_nul.as_bytes()).unwrap();
	unsafe { ecsact_create_registry(registry_name_cstr.as_ptr()).into() }
}

unsafe extern "C" fn c_entity_created_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	placeholder_entity_id: crate::bindings::ecsact_placeholder_entity_id,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ecsact::ExecutionEventsCollector);
	for cb in &evc.entity_created_callbacks {
		cb(entity_id.into(), placeholder_entity_id.into());
	}
}

unsafe extern "C" fn c_entity_destroyed_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	placeholder_entity_id: crate::bindings::ecsact_placeholder_entity_id,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ecsact::ExecutionEventsCollector);
	for cb in &evc.entity_destroyed_callbacks {
		cb(entity_id.into(), placeholder_entity_id.into());
	}
}

unsafe extern "C" fn c_init_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ecsact::ExecutionEventsCollector);
	let component_id: ecsact::ComponentId = component_id.into();
	if let Some(cbs) = evc.init_callbacks.get(&component_id) {
		for cb in cbs {
			cb(entity_id.into(), component_data);
		}
	}
}

unsafe extern "C" fn c_update_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ecsact::ExecutionEventsCollector);
	let component_id: ecsact::ComponentId = component_id.into();
	if let Some(cbs) = evc.update_callbacks.get(&component_id) {
		for cb in cbs {
			cb(entity_id.into(), component_data);
		}
	}
}

unsafe extern "C" fn c_remove_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ecsact::ExecutionEventsCollector);
	let component_id: ecsact::ComponentId = component_id.into();
	if let Some(cbs) = evc.remove_callbacks.get(&component_id) {
		for cb in cbs {
			cb(entity_id.into(), component_data);
		}
	}
}

pub fn execute_systems(
	registry: RegistryId,
	execution_count: i32,
	events_collector: Option<ecsact::ExecutionEventsCollector>,
) {
	let mut c_ev_collector: Option<ecsact_execution_events_collector> = None;

	unsafe {
		if events_collector.is_some() {
			let user_data =
				&mut events_collector.unwrap() as *mut _ as *mut c_void;
			c_ev_collector = Some(ecsact_execution_events_collector {
				entity_created_callback: Some(
					c_entity_created_callback_handler,
				),
				entity_created_callback_user_data: user_data,
				init_callback: Some(c_init_callback_handler),
				init_callback_user_data: user_data,
				update_callback: Some(c_update_callback_handler),
				update_callback_user_data: user_data,
				remove_callback: Some(c_remove_callback_handler),
				remove_callback_user_data: user_data,
				entity_destroyed_callback: Some(
					c_entity_destroyed_callback_handler,
				),
				entity_destroyed_callback_user_data: user_data,
			});
		}

		ecsact_execute_systems(
			registry.into(),
			execution_count,
			std::ptr::null(),
			c_ev_collector
				.map(|cev| &cev as *const ecsact_execution_events_collector)
				.unwrap_or(std::ptr::null()),
		);
	}
}
