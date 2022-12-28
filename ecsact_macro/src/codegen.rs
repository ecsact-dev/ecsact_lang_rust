use quote::quote;

use crate::internal::{find_fn_name_ident, strip_quotes};

pub fn plugin_entry(
	attr: proc_macro::TokenStream,
	fn_def: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let fn_def = proc_macro2::TokenStream::from(fn_def);
	let mut attr_itr = attr.into_iter();
	let attr_param = attr_itr.next().expect("plugin_entry expects 1 argument");
	let plugin_name: String = match attr_param {
		proc_macro::TokenTree::Literal(lit) => lit.to_string(),
		_ => "".to_string(),
	};

	if plugin_name.is_empty() {
		panic!("Expected string literal for plugin_entry argument");
	}

	let plugin_name = strip_quotes(&plugin_name)
		.expect("Expected string literal for plugin_entry argument");

	let fn_impl_ident = find_fn_name_ident(fn_def.clone())
		.expect("ecsact_codegen::plugin_entry must be used on a function");

	let plugin_name_fn = quote! {
		static PLUGIN_NAME: &str = concat!(#plugin_name, "\0");

		#[no_mangle]
		pub extern "C" fn ecsact_codegen_plugin_name(
		) -> *const ::std::ffi::c_char {
			return PLUGIN_NAME.as_ptr() as *const ::std::ffi::c_char;
		}
	};

	let plugin_impl_fn = quote! {
		#[no_mangle]
		pub extern "C" fn ecsact_codegen_plugin(
			pkg_id: i32,
			write_fn: extern "C" fn(*const ::std::ffi::c_char, i32),
		) {
			let mut ctx = ::ecsact::CodegenPluginContext::new(pkg_id, write_fn);
			#fn_impl_ident(&mut ctx);
		}
	};

	proc_macro::TokenStream::from(quote! {
		#plugin_name_fn
		#fn_def
		#plugin_impl_fn
	})
}
