use std::collections::HashMap;
use std::convert::From;
use std::ffi::{c_char, c_void};
use std::fmt::{Error, Write};

pub mod support;

/// Creates a "typed ID". A typed ID is an opaque `i32`. The purpose is for
/// type safety when passing ids to the Ecsact API.
macro_rules! typed_id {
	($type_name:ident) => {
		#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
		#[repr(transparent)]
		pub struct $type_name(i32);

		impl $type_name {
			/// # Safety
			///
			/// Ecsact IDs may only be set by the code generator
			pub const unsafe fn new(id: i32) -> Self {
				Self(id)
			}
		}

		impl From<i32> for $type_name {
			fn from(id: i32) -> Self {
				Self(id)
			}
		}

		impl From<$type_name> for i32 {
			fn from(id: $type_name) -> i32 {
				id.0
			}
		}
	};
}

typed_id!(PackageId);
typed_id!(SystemId);
typed_id!(ActionId);
typed_id!(ComponentId);
typed_id!(TransientId);
typed_id!(EnumId);
typed_id!(EnumValueId);
typed_id!(FieldId);
typed_id!(VariantId);
typed_id!(RegistryId);
typed_id!(EntityId);
typed_id!(PlaceholderEntityId);
typed_id!(SystemGeneratesId);
typed_id!(AsyncRequestId);

typed_id!(DeclId);
typed_id!(CompositeId);
typed_id!(SystemLikeId);
typed_id!(ComponentLikeId);

/// Implements From trait for typed ID structs from the `typed_id!` macro. Some
/// Ecsact IDs are supposed to be easily converted from one to the other.
macro_rules! typed_id_convert {
	($type1:ident, $type2:ident) => {
		impl From<$type1> for $type2 {
			fn from(id: $type1) -> Self {
				Self(id.into())
			}
		}
	};
}

typed_id_convert!(SystemId, SystemLikeId);
typed_id_convert!(ActionId, SystemLikeId);
typed_id_convert!(ActionId, CompositeId);
typed_id_convert!(ComponentId, CompositeId);
typed_id_convert!(TransientId, CompositeId);
typed_id_convert!(ComponentLikeId, CompositeId);

typed_id_convert!(ComponentId, DeclId);
typed_id_convert!(TransientId, DeclId);
typed_id_convert!(SystemId, DeclId);
typed_id_convert!(ActionId, DeclId);
typed_id_convert!(VariantId, DeclId);
typed_id_convert!(SystemLikeId, DeclId);
typed_id_convert!(CompositeId, DeclId);
typed_id_convert!(ComponentLikeId, DeclId);

typed_id_convert!(ComponentId, ComponentLikeId);
typed_id_convert!(TransientId, ComponentLikeId);

pub enum IntType {
	I8,
	U8,
	I16,
	U16,
	I32,
	U32,
}

pub enum FieldType {
	Bool { length: i32 },
	I8 { length: i32 },
	U8 { length: i32 },
	I16 { length: i32 },
	U16 { length: i32 },
	I32 { length: i32 },
	U32 { length: i32 },
	F32 { length: i32 },
	Entity { length: i32 },
	Enum { id: EnumId, length: i32 },
}

impl From<IntType> for FieldType {
	fn from(int_type: IntType) -> FieldType {
		match int_type {
			IntType::I8 => FieldType::I8 { length: 1 },
			IntType::U8 => FieldType::U8 { length: 1 },
			IntType::I16 => FieldType::I16 { length: 1 },
			IntType::U16 => FieldType::U16 { length: 1 },
			IntType::I32 => FieldType::I32 { length: 1 },
			IntType::U32 => FieldType::U32 { length: 1 },
		}
	}
}

pub struct CodegenPluginContext {
	package_id: i32,
	write_fn: extern "C" fn(*const ::std::ffi::c_char, i32),
}

impl CodegenPluginContext {
	pub fn new(
		package_id: i32,
		write_fn: extern "C" fn(*const ::std::ffi::c_char, i32),
	) -> Self {
		Self {
			package_id,
			write_fn,
		}
	}

	pub fn package_id(&self) -> PackageId {
		self.package_id.into()
	}
}

impl Write for CodegenPluginContext {
	fn write_str(&mut self, s: &str) -> Result<(), Error> {
		(self.write_fn)(
			s.as_ptr() as *const c_char,
			s.len().try_into().unwrap(),
		);
		Ok(())
	}
}

pub enum SystemCapability {
	Readonly { optional: bool },
	Writeonly { optional: bool },
	Readwrite { optional: bool },
	Include,
	Exclude,
	Adds,
	Removes,
}

// TODO(zaucy): Mark these traits as unsafe because of the repr(C) and ID
// consider the ID constructor to be unsafe
// consider a IsReprC unsafe trait

pub trait Component: ComponentLike + Composite {
	const ID: ComponentId;
}

pub trait Transient: ComponentLike + Composite {
	const ID: TransientId;
}

pub trait Composite {
	const ID: CompositeId;
}

pub trait ComponentLike: Composite {
	const ID: ComponentLikeId;
}

pub trait Action: Composite + SystemLike {
	const ID: ActionId;
}

pub trait System: SystemLike {
	const ID: SystemId;
}

pub trait SystemLike {
	const ID: SystemLikeId;
}

type ComponentVoidDataCallback<'a> = Box<dyn Fn(EntityId, *const c_void) + 'a>;
type EntityEventCallback<'a> = Box<dyn Fn(EntityId, PlaceholderEntityId) + 'a>;

#[derive(Default)]
pub struct ExecutionEventsCollector<'a> {
	pub entity_created_callbacks: Vec<EntityEventCallback<'a>>,
	pub init_callbacks:
		HashMap<ComponentId, Vec<ComponentVoidDataCallback<'a>>>,
	pub update_callbacks:
		HashMap<ComponentId, Vec<ComponentVoidDataCallback<'a>>>,
	pub remove_callbacks:
		HashMap<ComponentId, Vec<ComponentVoidDataCallback<'a>>>,
	pub entity_destroyed_callbacks: Vec<EntityEventCallback<'a>>,
}

impl<'a> ExecutionEventsCollector<'a> {
	pub fn on_entity_created(
		mut self,
		callback: &'a dyn Fn(EntityId, PlaceholderEntityId),
	) -> Self {
		self.entity_created_callbacks.push(Box::new(callback));
		self
	}

	pub fn on_init<C: Component>(
		mut self,
		callback: &'a dyn Fn(EntityId, &C),
	) -> Self {
		self.init_callbacks.entry(<C as Component>::ID).or_default();
		self.init_callbacks
			.get_mut(&<C as Component>::ID)
			.unwrap()
			.push(Box::new(|e, comp_data| unsafe {
				let comp: &C = &*(comp_data as *const _ as *const C);
				callback(e, comp);
			}));
		self
	}

	pub fn on_update<C: Component>(
		mut self,
		callback: &'a dyn Fn(EntityId, &C),
	) -> Self {
		self.update_callbacks
			.entry(<C as Component>::ID)
			.or_default();
		self.update_callbacks
			.get_mut(&<C as Component>::ID)
			.unwrap()
			.push(Box::new(|e, comp_data| unsafe {
				let comp: &C = &*(comp_data as *const _ as *const C);
				callback(e, comp);
			}));
		self
	}

	pub fn on_remove<C: Component>(
		mut self,
		callback: &'a dyn Fn(EntityId, &C),
	) -> Self {
		self.remove_callbacks
			.entry(<C as Component>::ID)
			.or_default();
		self.remove_callbacks
			.get_mut(&<C as Component>::ID)
			.unwrap()
			.push(Box::new(|e, comp_data| unsafe {
				let comp: &C = &*(comp_data as *const _ as *const C);
				callback(e, comp);
			}));
		self
	}

	pub fn on_entity_destroyed(
		mut self,
		callback: &'a dyn Fn(EntityId, PlaceholderEntityId),
	) -> Self {
		self.entity_destroyed_callbacks.push(Box::new(callback));
		self
	}
}
