use super::card;
use crate::prelude::*;

pub async fn menu() -> Result<Markup> {
	let ebooks = lib_directus::get_ebooks().await;
	dbg!("{}", &ebooks);

	let ebooks = ebooks?;

	Ok(html! {
		div.ebook-container {
			section {
				div.grid-auto-fit {
					@for ebook in ebooks {
						(card(&ebook))
					}
				}
			}
		}
		style { (css! {{
			me { margin: 30px; }

			.grid-auto-fit {
				display: grid;
				gap: 3rem;
				grid-template-columns: repeat(auto-fit, minmax(min(45ch, 100%), 1fr));

				container: grid-auto-fit / inline-size;
			}

			@layer card-styling {
				.card {
					--padding: 1rem;

					display: grid;
					overflow: hidden;
					background-color: white;
					border-radius: 4px;

					h2,
					h3 {
						color: black;
					}

					> img {
						object-fit: cover;
						width: 100%;
						height: 100%;
					}

					> :not(img) {
						margin-block-start: 0;
						margin-inline: 1rem;
					}

					> :not(img):first-child {
						margin-block-start: 1rem;
					}

					> :not(img):last-child {
						margin-block-end: 1rem;
					}
				}
			}

			@layer layout {
				.primary-layout {
					display: grid;
					grid-template-columns:
						[full-width] minmax(2rem, 1fr) [content-start] min(1600px, 100% - 4rem)
						[content-end] minmax(2rem, 1fr);

					& > * {
						grid-column: 2 / -2;
					}
				}
			}
	}}) }
	})
}
