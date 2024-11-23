use crate::prelude::*;

pub struct Info;

impl Render for Info {
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
			class="lucide lucide-info"{
				circle cx="12" cy="12" r="10"{}
				path d="M12 16v-4"{}
				path d="M12 8h.01"{}
			}
		}
	}
}
