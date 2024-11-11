use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input};

// Struct to parse and store the embedded JavaScript code
struct JsBlock {
	content: String,
}

impl Parse for JsBlock {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let content;
		braced!(content in input);
		let js_code = content.parse::<proc_macro2::TokenStream>()?.to_string();
		Ok(JsBlock { content: js_code })
	}
}

#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
	let JsBlock { content } = parse_macro_input!(input as JsBlock);

	let formatted_js = format!(
		"\"{}\"",
		content
			.replace("\\", "\\\\") // Escape backslashes
			.replace("\"", "\\\"") // Escape double quotes
			.replace("\n", "\\n") // Preserve line breaks
	);

	let output = quote! {
		#formatted_js
	};

	TokenStream::from(output)
}
