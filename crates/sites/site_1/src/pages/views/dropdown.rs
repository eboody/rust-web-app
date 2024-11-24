//use crate::prelude::*;
//
//#[derive(Debug, Display)]
//pub struct Dropdown<'a> {
//	label: &'a str,
//	options: Vec<(&'a str, &'a str)>, // (display text, value)
//}
//
//impl Render for Dropdown<'_> {
//	fn render(&self) -> maud::Markup {
//		let uuid = uuid::Uuid::new_v4();
//
//		html! {
//			.dropdown-container id={ "dropdown-" (uuid) } {
//				.dropdown-label {
//					(self.label)
//				}
//
//				.dropdown-menu {
//					@for (display, value) in &self.options {
//						.dropdown-item data-value=(value) {
//							(display)
//						}
//					}
//				}
//			}
//			(js())
//			(css())
//		}
//	}
//}
//
//js! {
//	me(".dropdown-label").on("click", (ev) => {
//		let dropdownId = "#" + me(ev).closest(".dropdown-container").id;
//		let menu = any(dropdownId).find(".dropdown-menu");
//		menu.toggle(); // Toggle visibility
//	});
//
//	me(".dropdown-item").on("click", (ev) => {
//		halt(ev);
//		let value = me(ev).attribute("data-value");
//		let label = me(ev).text();
//		me(ev).closest(".dropdown-container").find(".dropdown-label").text(label); // Update label
//		me(ev).closest(".dropdown-menu").fadeOut(null, 30); // Close menu
//	});
//}
//
//css! {
//	.dropdown-container {
//		position: relative;
//		display: inline-block;
//
//		.dropdown-label {
//			background: var(--surface-1);
//			padding: 0.5rem 1rem;
//			border-radius: 0.5rem;
//			cursor: pointer;
//		}
//
//		.dropdown-menu {
//			display: none;
//			position: absolute;
//			background: var(--surface-1);
//			box-shadow: rgba(0, 0, 0, 0.1) 0px 2px 8px;
//			border-radius: 0.5rem;
//			z-index: 10;
//			width: 100%;
//
//			.dropdown-item {
//				padding: 0.5rem 1rem;
//				cursor: pointer;
//
//				&:hover {
//					background: var(--surface-2);
//				}
//			}
//		}
//	}
//}
