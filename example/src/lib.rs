// include!(concat!(env!("OUT_DIR"), "/example.ecsact.rs"));

pub mod example_ecsact;
use example_ecsact::example;

#[ecsact_macro::system_impl("example.Gravity")]
fn _impl(ctx: &mut example::Gravity::__Context) {
	let mut pos: example::Position = ctx.get();
	pos.y -= 9.81;
	ctx.update(&pos);
}
