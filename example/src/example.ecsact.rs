/// Generated file - DO NOT EDIT

pub mod example {

	#[repr(u8)]
	pub enum DamageType {
		Normal = 0,
		Piercing = 1,
	}

	#[repr(C)]
	pub struct Player {}
	#[repr(C)]
	pub struct Health {
		pub value: f32,
	}
	#[repr(C)]
	pub struct Position {
		pub x: f32,
		pub y: f32,
	}
	#[repr(C)]
	pub struct Attacking {
		pub target: i32,
	}
	#[allow(non_snake_case)]
	pub mod Attack {
		pub const ID: i32 = 6;
		pub trait __AddableComponent {}
		impl __AddableComponent for crate::example::Attacking {}
		#[repr(transparent)]
		pub struct __Context(*mut ::std::ffi::c_void);
		impl __Context {
			pub fn add(comp: impl __AddableComponent) {
				todo!()
			}
		}
	}
	#[allow(non_snake_case)]
	pub mod Gravity {
		pub const ID: i32 = 7;
		#[repr(transparent)]
		pub struct __Context(*mut ::std::ffi::c_void);
		impl __Context {}
	}
}
