use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

struct JsBlock {
	content: String,
}

impl Parse for JsBlock {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		//let content;
		//braced!(content in input);
		let js_code = input.parse::<proc_macro2::TokenStream>()?.to_string();
		Ok(JsBlock { content: js_code })
	}
}

struct CssBlock {
	content: String,
}

impl Parse for CssBlock {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// parse the input as a string literal
		let css_code = input.parse::<proc_macro2::TokenStream>()?.to_string();

		let css_code = css_code
			.replace("#0x", "#")
			.replace(" - ", "-")
			.replace("}", "}\n")
			.replace(">\n", "> ")
			.replace("@\n", "@")
			.replace("book:hover.", "book:hover .")
			.replace("@ ", "@");

		Ok(CssBlock { content: css_code })
	}
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
	let CssBlock { content } = parse_macro_input!(input as CssBlock);

	let output = quote! {
		maud::PreEscaped(#content.to_string())
	};

	TokenStream::from(output)
}

#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
	let JsBlock { content } = parse_macro_input!(input as JsBlock);

	let output = quote! {
			maud::PreEscaped(#content.to_string())
	};

	TokenStream::from(output)
}
