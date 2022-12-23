include!(concat!(env!("OUT_DIR"), "/example.ecsact.rs"));

// TODO(zaucy): This function is what we need to export to a DLL or WebAssembly
//              The plan is to make a macro to simplify this.
// #[allow(non_snake_case)]
// extern "C" fn example__Gravity(ctx: *mut c_void) {}

trait AddableComponent {}

struct ExampleComponent {
	something: i32,
}

impl AddableComponent for ExampleComponent {}

fn do_thing(comp: impl AddableComponent) {}

fn _test() {
	let comp = ExampleComponent { something: 42 };
	do_thing(comp);
}
