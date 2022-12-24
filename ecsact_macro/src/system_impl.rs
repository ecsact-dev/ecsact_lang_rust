use ecsact::support::ecsact_ident_to_rust_ident;
use quote::quote;

use crate::internal::find_fn_name_ident;

fn validate_attr(attr: &str) {
	if attr.is_empty() {
		panic!("Must supply Ecact system fully qualified identifier");
	}

	// TODO if actually valid ecsact ident
}

pub fn system_impl(
	attr: proc_macro::TokenStream,
	fn_def: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let fn_def = proc_macro2::TokenStream::from(fn_def);

	let ecsact_full_qualified_name: String = attr.to_string();
	validate_attr(&ecsact_full_qualified_name);

	let rust_qualified_name =
		ecsact_ident_to_rust_ident(&ecsact_full_qualified_name);

	let fn_impl_name = find_fn_name_ident(fn_def.clone()).unwrap();
	let c_fn_impl_name = proc_macro2::Ident::new(
		&to_c_fn_impl_name(ecsact_full_qualified_name),
		proc_macro2::Span::call_site(),
	);

	let c_fn_impl = quote! {
		pub extern "C" fn #c_fn_impl_name(c_ctx: *mut ::std::ffi::c_void) {
			let mut ctx = #rust_qualified_name::__Context(c_ctx);
			#fn_impl_name(&mut ctx);
		}
	};

	proc_macro::TokenStream::from(quote! {
		#fn_def
		#c_fn_impl
	})
}

fn to_c_fn_impl_name(ecsact_full_qualified_name: String) -> String {
	ecsact_full_qualified_name.replace('.', "__")
}
