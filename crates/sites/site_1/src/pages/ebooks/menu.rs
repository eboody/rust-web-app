use super::card;
use crate::prelude::*;

pub async fn menu() -> Result<Markup> {
	let ebooks = lib_directus::get_ebooks().await;

	let ebooks = ebooks?;

	Ok(html! {
		.ebooks-container {
			section {
				.grid-auto-fit {
					@for ebook in ebooks {
						(card(&ebook))
					}
				}
			}
		}
		(css())
	})
}

css! {
	me {
		.grid-auto-fit {
			display: grid;
			gap: var(--size-fluid-3);
			grid-template-columns: repeat(auto-fit, minmax(min(35ch, 100%), 1fr));

			container: grid-auto-fit / inline-size;
		}
	}
}
