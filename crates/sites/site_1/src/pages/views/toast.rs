use crate::prelude::*;

#[derive(Debug, Display)]
pub enum Toast {
	Success(String),
	Info(String),
	Warning(String),
	Error(String),
}

impl Render for Toast {
	fn render(&self) -> maud::Markup {
		html! {
			#toast.fade-in {
				.success-icon.icon {
					@match self {
						Toast::Success(_) => (icon::Check),
						Toast::Info(_) => (icon::Info),
						Toast::Warning(_) => (icon::Warning),
						Toast::Error(_) => (icon::Error)
					}
				}
				.message{
					@match self {
						Toast::Success(message) => {
								div style="color: var(--success);" { (message) }
						}
						Toast::Info(message) => {
								div style="color: var(--info);" { (message) }
						}
						Toast::Warning(message) => {
								div style="color: var(--warning);" { (message) }
						}
						Toast::Error(message) => {
								div style="color: var(--error);" { (message) }
						}
					}
				}
				#close-toast.icon {
					(icon::Close)
				}
			}
			(css())
		}
	}
}

css! {
	me {
		.fade-in.htmx-added,
		#toast {
			opacity: 0;
			transition: opacity 0.5s var(--ease-5);
			animation:
				var(--animation-fade-in) forwards,
				var(--animation-slide-in-down);
			animation-timing-function: var(--ease-5);
			animation-duration: 0.5s;
			animation-delay: 0.5s;
		}
		.fade-in {
			opacity: 1;
			transition: opacity 0.5s var(--ease-5);
		}
		.fade-out {
			opacity: 0;
			transition: opacity 0.5s var(--ease-5);
		}
		#toast {
			position: fixed;
			overflow: hidden;

			display: grid;
			grid-template-columns: min-content 1fr min-content;
			grid-gap: 1rem;
			align-self: start;
			justify-self: center;

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
