use crate::prelude::*;

pub struct FormField<'a> {
	pub label: &'a str,
	pub input_type: &'a str,
	pub placeholder: &'a str,
	pub name: &'a str,
}

impl<'a> Render for FormField<'a> {
	fn render(&self) -> Markup {
		html! {
			//label { (self.label) }
			input type=(self.input_type) name=(self.name) placeholder=(self.placeholder) {}
		}
	}
}
