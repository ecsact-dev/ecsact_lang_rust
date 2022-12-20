use std::fmt::Write;

use ecsact_dylib_runtime::meta;
use quote::quote;

#[ecsact_codegen::plugin_entry("rs")]
fn woohoo(ctx: &mut ecsact::CodegenPluginContext) {
	for comp_id in meta::get_component_ids(ctx.package_id()).into_iter() {
		let comp_name = proc_macro2::Ident::new(
			meta::component_name(comp_id).as_str(),
			proc_macro2::Span::call_site(),
		);

		let comp_struct = quote! {
			pub struct #comp_name {

			}
		};

		writeln!(ctx, "{}", comp_struct.to_string().as_str()).unwrap();
	}
}
