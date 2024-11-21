use crate::{pages::ebooks, prelude::*};
use lib_directus::Ebook;

pub struct Menu {
	pub ebooks: Vec<Ebook>,
}

impl Render for Menu {
	fn render(&self) -> Markup {
		html! {
			.ebooks-container {
				section {
					.grid-auto-fit {
						@for ebook in &self.ebooks {
							(ebooks::Card { ebook })
						}
					}
				}
			}
			(css())
		}
	}
}

pub async fn get_menu() -> Result<Markup> {
	let ebooks = EbookService::find().await;

	let ebooks = ebooks?;

	Ok(Menu { ebooks }.render())
}

css! {
	.grid-auto-fit {
		display: grid;
		gap: var(--size-fluid-3);
		grid-template-columns: repeat(auto-fit, minmax(min(35ch, 100%), 1fr));

		container: grid-auto-fit / inline-size;
	}
	me {
	}
}
