#[proc_macro_attribute]
pub fn system_impl(
	_attr: proc_macro::TokenStream,
	fn_def: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	fn_def
}
