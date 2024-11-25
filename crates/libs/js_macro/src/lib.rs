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
		let js_code = format!("{:#}", input.parse::<proc_macro2::TokenStream>()?);

		let js_code = js_code
			.replace(" r\"", " \"")
			.replace("(r\"", "(\"")
			.replace("[r\"", "[\"")
			.replace(" \"`", " `")
			.replace("`\";", "`;")
			.replace("(\"`", "(`")
			.replace("`\" ", "` ")
			.replace("[\"`", "[`")
			.replace("`\")", "`)")
			.replace("`\"]", "`]");
		Ok(JsBlock { content: js_code })
	}
}

struct CssBlock {
	content: String,
}

impl Parse for CssBlock {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// parse the input as a string literal
		let css_code = format!("{:#}", input.parse::<proc_macro2::TokenStream>()?);

		let css_code = css_code
			.replace(" : ", ":")
			.replace("#0x", "#")
			.replace(" - ", "-")
			.replace("}", "}\n")
			.replace(">\n", "> ")
			.replace("@\n", "@")
			.replace("Em", "em")
			.replace("book:hover.", "book:hover .")
			.replace("@ ", "@");

		Ok(CssBlock { content: css_code })
	}
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
	let CssBlock { content } = parse_macro_input!(input as CssBlock);

	let output = quote! {
		fn css() -> Markup {
			html! { style { (maud::PreEscaped(#content.to_string())) } }
		}
	};

	TokenStream::from(output)
}

#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
	let JsBlock { content } = parse_macro_input!(input as JsBlock);

	let output = quote! {
		fn js() -> Markup {
			html! { script { (PreEscaped(#content.to_string())) } }
		}
	};

	TokenStream::from(output)
}
