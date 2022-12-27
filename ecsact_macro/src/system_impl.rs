use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg};

use crate::internal::strip_quotes;

pub fn system_impl(
	attr: proc_macro::TokenStream,
	fn_def_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let fn_def = proc_macro2::TokenStream::from(fn_def_input.clone());
	let ecsact_identifier = parse_macro_input!(attr as syn::LitStr);
	let fn_item = parse_macro_input!(fn_def_input as syn::ItemFn);

	if fn_item.sig.inputs.len() != 1 {
		panic!("System implementation functions must have exactly 1 paramter");
	}

	let context_arg = match fn_item.sig.inputs.first().unwrap() {
		FnArg::Receiver(_) => panic!("invalid context arg type"),
		FnArg::Typed(a) => a,
	};

	let context_type = match *context_arg.ty {
		syn::Type::Reference(ref r) => r,
		_ => panic!(
			"System implementation functions context must be a reference type"
		),
	};

	context_type.mutability.expect(
		"System implementation functions context must be a mutable reference",
	);

	let fn_impl_name = fn_item.sig.ident;
	let c_fn_impl_name = proc_macro2::Ident::new(
		&to_c_fn_impl_name(
			strip_quotes(&ecsact_identifier.to_token_stream().to_string())
				.unwrap(),
		),
		proc_macro2::Span::call_site(),
	);

	let c_fn_impl = quote! {
		#[allow(non_snake_case)]
		pub extern "C" fn #c_fn_impl_name(c_ctx: *mut ::std::ffi::c_void) {
			#fn_def
			let mut ctx = #context_type(c_ctx);
			#fn_impl_name(&mut ctx);
		}
	};

	proc_macro::TokenStream::from(quote! {
		#c_fn_impl
	})
}

fn to_c_fn_impl_name(ecsact_full_qualified_name: &str) -> String {
	ecsact_full_qualified_name.replace('.', "__")
}
