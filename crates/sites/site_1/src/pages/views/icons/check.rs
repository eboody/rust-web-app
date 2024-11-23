use crate::prelude::*;

pub struct Check;

impl Render for Check {
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
			class="lucide lucide-check"{
				path d="M20 6 9 17l-5-5" {}
			}
		}
	}
}
