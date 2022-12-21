use std::convert::From;
use std::ffi::c_char;
use std::fmt::{Error, Write};

macro_rules! typed_id {
	($type_name:ident) => {
		#[derive(Clone, Copy)]
		pub struct $type_name(i32);

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
typed_id!(SystemGeneratesId);
typed_id!(AsyncRequestId);

typed_id!(DeclId);
typed_id!(CompositeId);
typed_id!(SystemLikeId);
typed_id!(ComponentLikeId);

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
