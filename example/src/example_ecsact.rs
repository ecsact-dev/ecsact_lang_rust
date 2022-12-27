/// Generated file - DO NOT EDIT

pub mod example {

	#[repr(u8)]
	pub enum DamageType {
		Normal = 0,
		Piercing = 1,
	}

	#[repr(C)]
	pub struct Player {}
	impl ::ecsact::Component for Player {
		const ID: ::ecsact::ComponentId =
			unsafe { ::ecsact::ComponentId::new(2i32) };
	}
	impl ::ecsact::ComponentLike for Player {
		const ID: ::ecsact::ComponentLikeId =
			unsafe { ::ecsact::ComponentLikeId::new(2i32) };
	}
	impl ::ecsact::Composite for Player {
		const ID: ::ecsact::CompositeId =
			unsafe { ::ecsact::CompositeId::new(2i32) };
	}
	#[repr(C)]
	pub struct Health {
		pub value: f32,
	}
	impl ::ecsact::Component for Health {
		const ID: ::ecsact::ComponentId =
			unsafe { ::ecsact::ComponentId::new(3i32) };
	}
	impl ::ecsact::ComponentLike for Health {
		const ID: ::ecsact::ComponentLikeId =
			unsafe { ::ecsact::ComponentLikeId::new(3i32) };
	}
	impl ::ecsact::Composite for Health {
		const ID: ::ecsact::CompositeId =
			unsafe { ::ecsact::CompositeId::new(3i32) };
	}
	#[repr(C)]
	pub struct Position {
		pub x: f32,
		pub y: f32,
	}
	impl ::ecsact::Component for Position {
		const ID: ::ecsact::ComponentId =
			unsafe { ::ecsact::ComponentId::new(4i32) };
	}
	impl ::ecsact::ComponentLike for Position {
		const ID: ::ecsact::ComponentLikeId =
			unsafe { ::ecsact::ComponentLikeId::new(4i32) };
	}
	impl ::ecsact::Composite for Position {
		const ID: ::ecsact::CompositeId =
			unsafe { ::ecsact::CompositeId::new(4i32) };
	}
	#[repr(C)]
	pub struct Attacking {
		pub target: i32,
	}
	impl ::ecsact::Component for Attacking {
		const ID: ::ecsact::ComponentId =
			unsafe { ::ecsact::ComponentId::new(5i32) };
	}
	impl ::ecsact::ComponentLike for Attacking {
		const ID: ::ecsact::ComponentLikeId =
			unsafe { ::ecsact::ComponentLikeId::new(5i32) };
	}
	impl ::ecsact::Composite for Attacking {
		const ID: ::ecsact::CompositeId =
			unsafe { ::ecsact::CompositeId::new(5i32) };
	}
	#[allow(non_snake_case)]
	pub mod Attack {
		pub const ID: i32 = 6;
		pub trait __AddableComponent: ecsact::ComponentLike {}
		impl __AddableComponent for crate::example::Attacking {}
		#[repr(transparent)]
		pub struct __Context(pub *mut ::std::ffi::c_void);
		impl __Context {
			pub fn add<T: __AddableComponent + ::ecsact::ComponentLike>(
				&mut self,
				comp: &T,
			) {
				unsafe {
					::ecsact_system_execution_context::add(
						::ecsact_system_execution_context::Context::new(self.0),
						comp,
					);
				}
			}
		}
	}
	#[allow(non_snake_case)]
	pub mod Gravity {
		pub const ID: i32 = 7;
		#[repr(transparent)]
		pub struct __Context(pub *mut ::std::ffi::c_void);
		impl __Context {}
		#[allow(non_snake_case)]
		pub mod OtherGravity {
			pub const ID: i32 = 8;
			#[repr(transparent)]
			pub struct __Context(pub *mut ::std::ffi::c_void);
			impl __Context {}
		}
	}
}
