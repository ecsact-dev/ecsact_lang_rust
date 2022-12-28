mod codegen;
mod system_impl;

mod internal;

/// Mark a function as an Ecsact system implementation. A C function will be
/// created for you that will call the marked function with safe context struct
/// from the Ecsact code generator.
///
/// See: https://ecsact.dev/docs/system-impl
///
/// # Example
///
/// Given the following Ecsact file contents:
///
/// ```ecsact
/// package example;
/// component Position { f32 x; f32 y; }
/// system Gravity { readwrite Position; }
/// ```
///
/// A system implementation may be created with the proc macro like so:
///
/// ```rust
/// #[ecsact_macro::system_impl("example.Gravity")]
/// fn gravity(ctx: &mut example::Gravity::Context) {
///     todo!()
/// }
/// ```
#[proc_macro_attribute]
pub fn system_impl(
	attr: proc_macro::TokenStream,
	fn_def: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	system_impl::system_impl(attr, fn_def)
}

/// Mark a function as an Ecsact codegen plugin entry point. Two C functions
/// will be created for you that are required for an Ecsact plugin. One will
/// contain the function you marked as the entry point and be given a "context"
/// object of type `ecsact::CodegenPluginContext`.
///
/// Using the `ecsact_dylib_runtime` crate your entry point will have access to
/// the Ecsact Meta module for the current ecsact file it is processing.
///
/// # Example
///
/// This rust function can use `ctx` to write to a `.txt` file based on the
/// ecsact file.
///
/// ```rust
/// #[ecsact_codegen::plugin_entry("txt")]
/// fn my_plugin(ctx: &mut ecsact::CodegenPluginContext) {
///     let pkg_name = ecsact::meta::package_name(ctx.package_id());
///     writeln!(ctx, "{pkg_name}").unwrap()
/// }
/// ```
///
/// If the ecsact file had a package statement of hello:
///
/// ```ecsact
/// package hello;
/// ```
///
/// The text output file would contain a single line with the packag name.
///
/// ```txt
/// hello
/// ```
#[proc_macro_attribute]
pub fn plugin_entry(
	attr: proc_macro::TokenStream,
	fn_def: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	codegen::plugin_entry(attr, fn_def)
}
