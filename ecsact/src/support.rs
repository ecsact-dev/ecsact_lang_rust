use std::str::FromStr;

pub fn ecsact_ident_to_rust_ident(
	ecsact_ident: &str,
) -> proc_macro2::TokenStream {
	proc_macro2::TokenStream::from_str(&ecsact_ident.replace('.', "::"))
		.unwrap()
}
