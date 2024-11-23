use crate::prelude::*;

pub struct Error;

impl Render for Error {
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
			class="lucide lucide-circle-x"{
				circle cx="12" cy="12" r="10"{}
				path d="m15 9-6 6"{}
				path d="m9 9 6 6"{}
			}
		}
	}
}
