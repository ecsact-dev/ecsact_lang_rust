pub(crate) fn strip_quotes(input: &str) -> Option<&str> {
	if input.chars().filter(|&c| c == '"').count() != 2 {
		panic!("Quotes are not allowed in attribute string literal");
	}
	input.strip_prefix('"')?.strip_suffix('"')
}

pub(crate) fn find_fn_name_ident<I>(tokens: I) -> Option<proc_macro2::Ident>
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

		if ident.is_none() {
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
				panic!("must not be public");
			}

			return Some(ident);
		}
	}

	None
}
