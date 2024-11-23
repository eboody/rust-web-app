use crate::prelude::*;

pub enum Button<'a> {
	Primary { href: &'a str, text: &'a str },
	Secondary { href: &'a str, text: &'a str },
}

impl Render for Button<'_> {
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
				color 0.3s opacity 0.3s;
			box-shadow:
				rgba(50, 50, 105, 0.15) 0px 2px 5px 0px,
				rgba(0, 0, 0, 0.05) 0px 1px 1px 0px;

			&:hover {
				box-shadow:
					var(--surface-1) 0px 0px 0px 2px,
					var(--surface-1) 0px 0px 0px 3px,
					var(--surface-1) 0px 0px 0px 4px,
					var(--brand) 0px 0px 0px 5px,
					var(--brand) 0px 3px 3px 0px;
			}
		}

		button.primary {
			background-color: var(--brand);
			color: var(--surface-1);
		}

		button.secondary {
			background-color: var(--surface-1);
			color: var(--text-2) !important;
			border: 1px solid var(--text-2) !important;
			&:hover {
				box-shadow:
					var(--surface-1) 0px 0px 0px 2px,
					var(--surface-1) 0px 0px 0px 3px,
					var(--surface-1) 0px 0px 0px 4px,
					var(--text-2) 0px 0px 0px 5px,
					var(--text-2) 0px 3px 3px 0px;
			}
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
