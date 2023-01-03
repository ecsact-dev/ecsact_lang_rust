mod system_impls;

fn main() {
	let reg = ecsact_example::core::create_registry("Example Registry");
	let entity = ecsact_example::core::create_entity(reg);
}
