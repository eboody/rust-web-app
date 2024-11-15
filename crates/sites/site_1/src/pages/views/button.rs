use crate::prelude::*;

pub enum Button {
	Primary { href: String, text: String },
	Secondary { href: String, text: String },
}

impl Render for Button {
	fn render(&self) -> Markup {
		html! {
			@match self {
				Button::Primary { href, text } => {
					button.primary href=(href) download=(text) type="submit" { (text) }
				},
				Button::Secondary { href, text } => {
					button.secondary href=(href) download=(text) { (text) }
				}
			}
			(css())
		}
	}
}

css! {
	me {
		button {
			text-shadow: none;
			transition:
				box-shadow 0.3s,
				background-color 0.3s,
				color 0.3s;
			box-shadow:
				rgba(50, 50, 105, 0.15) 0px 2px 5px 0px,
				rgba(0, 0, 0, 0.05) 0px 1px 1px 0px;

			&:hover {
				box-shadow:
					rgb(255, 255, 255) 0px 0px 0px 2px,
					rgb(255, 255, 255) 0px 0px 0px 3px,
					rgb(255, 255, 255) 0px 0px 0px 4px,
					rgba(0, 0, 0, 1) 0px 0px 0px 5px,
					rgba(0, 0, 0, 0.15) 0px 3px 3px 0px;
			}
		}

		button.primary {
			background-color: #0x000;
			color: white;
		}

		button.secondary {
			background-color: surface;
			color: var(--text-2);
			border: 1px solid var(--text-2);
		}
		button:disabled {
			background-color: #ccc;
			color: #666;
			cursor: not-allowed;
			opacity: 0.7;

			&:hover {
				box-shadow: none;
			}
		}
	}
}
