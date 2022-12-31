use ecsact::{ComponentId, EntityId, RegistryId};
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

pub fn create_registry(registry_name: String) -> RegistryId {
	let registry_name_cstr =
		CStr::from_bytes_with_nul(registry_name.as_bytes()).unwrap();
	unsafe { ecsact_create_registry(registry_name_cstr.as_ptr()).into() }
}

pub struct ExecutionEventsCollector {
	pub init_callback: Box<dyn Fn(EntityId, ComponentId, *const c_void)>,
	pub update_callback: Box<dyn Fn(EntityId, ComponentId, *const c_void)>,
	pub remove_callback: Box<dyn Fn(EntityId, ComponentId, *const c_void)>,
}

unsafe extern "C" fn c_init_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ExecutionEventsCollector);
	(evc.init_callback)(entity_id.into(), component_id.into(), component_data);
}

unsafe extern "C" fn c_update_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ExecutionEventsCollector);
	(evc.update_callback)(
		entity_id.into(),
		component_id.into(),
		component_data,
	);
}

unsafe extern "C" fn c_remove_callback_handler(
	_event: crate::bindings::ecsact_event,
	entity_id: crate::bindings::ecsact_entity_id,
	component_id: crate::bindings::ecsact_component_id,
	component_data: *const c_void,
	callback_user_data: *mut c_void,
) {
	let evc = &*(callback_user_data as *const ExecutionEventsCollector);
	(evc.remove_callback)(
		entity_id.into(),
		component_id.into(),
		component_data,
	);
}

pub fn execute_systems(
	registry: RegistryId,
	execution_count: i32,
	events_collector: Option<ExecutionEventsCollector>,
) {
	let mut c_ev_collector: Option<ecsact_execution_events_collector> = None;

	unsafe {
		if let Some(evc) = events_collector {
			let user_data = &evc as *const _ as *mut c_void;
			c_ev_collector = Some(ecsact_execution_events_collector {
				init_callback: Some(c_init_callback_handler),
				init_callback_user_data: user_data,
				update_callback: Some(c_update_callback_handler),
				update_callback_user_data: user_data,
				remove_callback: Some(c_remove_callback_handler),
				remove_callback_user_data: user_data,
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
