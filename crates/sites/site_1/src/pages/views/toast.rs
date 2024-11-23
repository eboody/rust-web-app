use crate::prelude::*;

#[derive(Debug, Display)]
pub enum Toast<'a> {
	Success { text: &'a str },
	Info { text: &'a str },
	Warning { text: &'a str },
	Error { text: &'a str },
}

impl Render for Toast<'_> {
	fn render(&self) -> maud::Markup {
		let id = format!(
			"toast-{}",
			to_alphanumeric(match self {
				Toast::Success { text } => text,
				Toast::Info { text } => text,
				Toast::Warning { text } => text,
				Toast::Error { text } => text,
			})
			.to_case(Case::Kebab)
		);

		html! {
			.fade-in id=(id) {
				.success-icon.icon {
					@match self {
						Toast::Success {..} => (icon::Check),
						Toast::Info{..} => (icon::Info),
						Toast::Warning{..} => (icon::Warning),
						Toast::Error{..} => (icon::Error)
					}
				}
				.message {
					@match self {
						Toast::Success { text } => {
							div style="color: var(--success);" { (text) }
						},
						Toast::Info { text } => {
							div style="color: var(--info);" { (text) }
						},
						Toast::Warning { text } => {
							div style="color: var(--warning);" { (text) }
						},
						Toast::Error { text } => {
							div style="color: var(--error);" { (text) }
						},
					}
				}
				.close-toast.icon toast-id=(id) {
					(icon::Close)
				}
			}
			(js())
			(css())
		}
	}
}

js! {
	setTimeout(() => {
		me("[id^='toast-']").fadeOut(null, 30);
	}, 4000);

	me(".close-toast").on("click", (ev) => {
		halt(ev);
		let toastId = "#" + me(ev).attribute("toast-id");
		let el = any(toastId);
		el.fadeOut(null, 30);
	});
}

css! {
	me {
		--main-transition: opacity 0.5s var(--ease-5);

		.fade-in.htmx-added,
		[id^="toast-"] {
			animation:
				var(--animation-fade-in) forwards,
				var(--animation-slide-in-down);
			animation-timing-function: var(--ease-5);
			animation-duration: 0.5s;
			animation-delay: 0.5s;
		}

		.fade-in.htmx-added,
		.fade-in,
		.fade-out,
		[id^="toast-"].fade-in {
			transition: var(--main-transition);
			opacity: 0;
		}

		.fade-in {
			opacity: 1;
		}

		[id^="toast-"] {
			position: fixed;
			overflow: hidden;

			display: grid;
			grid-template-columns: min-content 1fr min-content;
			grid-gap: 1rem;

			right: 2%;

			padding: 1rem;
			background-color: var(--surface-1);
			border-radius: 0.5rem;
			box-shadow:
				rgba(0, 0, 0, 0.05) 0px 0px 0px 1px,
				rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;

			.success-icon {
				cursor: default;
				background-image: url("/assets/check.svg");

				svg path {
					stroke: var(--success);
				}
			}
			.message {
				font-size: var(--font-size-fluid-0);
				color: var(--text-1);
				padding: 1rem;
			}
			#close-toast.icon {
				svg path {
					stroke: var(--text-1);
				}
				&:hover {
					background-color: var(--surface-3);
					box-shadow:
						rgba(0, 0, 0, 0.05) 0px 0px 0px 1px,
						rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
				}
			}
		}
	}
}
