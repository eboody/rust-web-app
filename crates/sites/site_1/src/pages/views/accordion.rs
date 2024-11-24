//use crate::prelude::*;
//
//#[derive(Debug)]
//pub struct Accordion<'a> {
//	items: Vec<AccordionItem<'a>>,
//}
//
//#[derive(Debug)]
//pub struct AccordionItem<'a> {
//	title: &'a str,
//	content: &'a str,
//}
//
//impl Render for Accordion<'_> {
//	fn render(&self) -> maud::Markup {
//		html! {
//			.accordion {
//				@for item in &self.items {
//					.accordion-item {
//						.accordion-header data-title={ item.title } {
//							(item.title)
//						}
//						.accordion-content {
//							(item.content)
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
//	me(".accordion-header").on("click", (ev) => {
//		let content = me(ev).next(".accordion-content");
//		content.toggle(); // Toggle visibility
//		me(ev).toggleClass("active");
//	});
//}
//
//css! {
//	.accordion {
//		.accordion-item {
//			border: 1px solid var(--surface-2);
//			border-radius: 0.5rem;
//			margin-bottom: 1rem;
//
//			.accordion-header {
//				padding: 1rem;
//				cursor: pointer;
//
//				&.active {
//					background: var(--surface-3);
//				}
//			}
//
//			.accordion-content {
//				display: none; // Initially hide content
//				padding: 1rem;
//				background: var(--surface-1);
//			}
//		}
//	}
//}
