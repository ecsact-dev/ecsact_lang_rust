use quote::quote;

fn find_fn_name_ident<'a, I>(tokens: I) -> Option<proc_macro2::Ident>
where
	I: IntoIterator<Item = proc_macro2::TokenTree>,
{
	let mut is_fn = false;
	let mut is_pub = false;

	for token in tokens.into_iter() {
		let ident: Option<proc_macro2::Ident> = match token {
			proc_macro2::TokenTree::Ident(id) => Some(id.clone()),
			_ => None,
		};

		if !ident.is_some() {
			break;
		}

		let ident = ident.unwrap();
		let ident_str = ident.to_string();

		if !is_fn {
			match ident_str.as_str() {
				"pub" => is_pub = true,
				"fn" => is_fn = true,
				_ => {}
			}
		} else {
			if is_pub {
				panic!("Ecsact codegen plugin entry fn must not be public");
			}

			return Some(ident);
		}
	}

	return None;
}

fn strip_quotes<'a>(s: &'a str) -> &'a str {
	let s = match s.strip_suffix('"') {
		Some(s) => s,
		None => s,
	};
	let s = match s.strip_prefix('"') {
		Some(s) => s,
		None => s,
	};

	s
}

#[proc_macro_attribute]
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

	if plugin_name.len() == 0 {
		panic!("Expected string literal for plugin_entry argument");
	}

	let plugin_name = strip_quotes(&plugin_name);

	let fn_impl_ident = find_fn_name_ident(fn_def.clone())
		.expect("ecsact_codegen::plugin_entry must be used on a function");

	let fn_impl_expr = fn_impl_ident.to_string() + "(&mut ctx)";

	println!("FN Impl Ident: {}", fn_impl_expr);

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
