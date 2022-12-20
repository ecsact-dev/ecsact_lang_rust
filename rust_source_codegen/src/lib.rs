use std::fmt::Write;

use ecsact_dylib_runtime::meta;

#[ecsact_codegen::plugin_entry("rs")]
fn woohoo(ctx: &mut ecsact::CodegenPluginContext) {
	for comp_id in meta::get_component_ids(ctx.package_id()).into_iter() {
		let comp_name = meta::component_name(comp_id);
		write!(ctx, "pub struct {} {{}}\n", comp_name).unwrap();
	}
}
