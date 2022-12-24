// include!(concat!(env!("OUT_DIR"), "/example.ecsact.rs"));

pub mod example_ecsact;

use example_ecsact::example;

// TODO(zaucy): This function is what we need to export to a DLL or WebAssembly
//              The plan is to make a macro to simplify this.
// #[allow(non_snake_case)]
// extern "C" fn example__Gravity(ctx: *mut c_void) {}

fn _my_example_impl(ctx: &mut example::Attack::__Context) {
	let comp = example::Attacking { target: -1 };
	ctx.add(&comp);
}
