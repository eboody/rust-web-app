use crate::prelude::*;

pub struct Close;

impl Render for Close {
	fn render(&self) -> Markup {
		html! {
			svg xmlns="http://www.w3.org/2000/svg"
			width="48"
			height="48"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="lucide lucide-check" {
				path d="M18 6 6 18" {}
				path d="m6 6 12 12" {}
			}
		}
	}
}