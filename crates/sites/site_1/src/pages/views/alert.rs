use crate::prelude::*;

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum Alert<'a> {
	Information { text: &'a str },
	Warning { text: &'a str },
	Error { text: &'a str },
}

impl Render for Alert<'_> {
	fn render(&self) -> maud::Markup {
		html! {
			.alert {
				@match self {
					Alert::Information { text } => {
						.alert-info {
							(text)
						}
					},
					Alert::Warning { text } => {
						.alert-warning {
							(text)
						}
					},
					Alert::Error { text } => {
						.alert-error {
							(text)
						}
					},
				}
			}
			(css())
		}
	}
}

css! {
	.alert {
		padding: 1rem;
		border-radius: 0.5rem;
		margin: 1rem 0;
		box-shadow: rgba(0, 0, 0, 0.1) 0px 2px 8px;
	}
	.alert-info {
		background-color: var(--info-light);
		color: var(--info);
	}
	.alert-warning {
		background-color: var(--warning-light);
		color: var(--warning);
	}
	.alert-error {
		background-color: var(--error-light);
		color: var(--error);
	}
}
