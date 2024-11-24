use crate::prelude::*;

#[derive(Debug)]
pub struct Tooltip<'a> {
	text: &'a str,
	position: TooltipPosition,
}

#[derive(Debug, Display)]
pub enum TooltipPosition {
	Top,
	Bottom,
	Left,
	Right,
}

impl Render for Tooltip<'_> {
	fn render(&self) -> maud::Markup {
		let uuid = uuid::Uuid::new_v4();
		html! {
			.tooltip-container id={ "tooltip-" (uuid) } {
				.tooltip-content position=(self.position) {
					(self.text)
				}
			}
			(css())
		}
	}
}

css! {
	.tooltip-container {
		position: relative;
		display: inline-block;

		.tooltip-content {
			visibility: hidden;
			background-color: var(--surface-2);
			color: var(--text-1);
			text-align: center;
			border-radius: 0.25rem;
			padding: 0.5rem;
			transition: visibility 0.2s ease-in;

			&::after {
				content: "";
				position: absolute;
				border-style: solid;
			}

			&[position="Top"] {
				bottom: 125%;
				left: 50%;
				margin-left: -60px;

				&::after {
					border-width: 6px 6px 0;
					border-color: var(--surface-2) transparent transparent transparent;
					left: 50%;
					margin-left: -6px;
				}
			}

			&[position="Bottom"] {
				top: 125%;
				left: 50%;
				margin-left: -60px;

				&::after {
					border-width: 0 6px 6px;
					border-color: transparent transparent var(--surface-2) transparent;
					left: 50%;
					margin-left: -6px;
				}
			}

			&[position="Left"] {
				top: 50%;
				right: 125%;
				margin-top: -30px;

				&::after {
					border-width: 6px 6px 6px 0;
					border-color: transparent var(--surface-2) transparent transparent;
					top: 50%;
					margin-top: -6px;
				}
			}

			&[position="Right"] {
				top: 50%;
				left: 125%;
				margin-top: -30px;

				&::after {
					border-width: 6px 0 6px 6px;
					border-color: transparent transparent transparent var(--surface-2);
					top: 50%;
					margin-top: -6px;
				}
			}
		}

		&:hover .tooltip-content {
			visibility: visible;
		}
	}
}
