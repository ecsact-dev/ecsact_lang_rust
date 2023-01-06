mod system_impls;

use ecsact_example::{core, example};

fn main() {
	let reg = core::create_registry("Example Registry");
	let entity = core::create_entity(reg);
	core::add_component(reg, entity, &example::Position { x: 0.0, y: 0.0 });

	let events_collector = ecsact::ExecutionEventsCollector::default()
		.on_init(&|_entity, pos: &example::Position| {
			println!("Position initialized!, {}, {}", pos.x, pos.y);
		})
		.on_update(&|_entity, pos: &example::Position| {
			println!("Position updated! {}, {}", pos.x, pos.y);
		})
		.on_remove(&|_entity, pos: &example::Position| {
			println!("Position removed! {}, {}", pos.x, pos.y);
		});

	core::execute_systems(reg, 1, Some(events_collector));
}
