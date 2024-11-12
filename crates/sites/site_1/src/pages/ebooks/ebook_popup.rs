use super::image::image;
use crate::views::popup;
use lib_directus::get_ebook;

use crate::prelude::*;

pub async fn ebook_popup(Path(ebook_id): Path<u32>) -> Result<Markup> {
	let ebook = get_ebook(ebook_id).await?;
	Ok(popup(html! {
		.popup-container {
			.left-side {
				h3 { "Free Ebook" }
				h1.title { (ebook.name.as_str()) }
				@if let Some(sub_text) = &ebook.sub_text {
					h2.subtext { (sub_text) }
				}
				form hx-post="/ebooks/signup" hx-swap="none" trigger="submit" {
					label { "First Name" }
					input type="text" placeholder="First Name" {}
					label { "Last Name" }
					input type="email" placeholder="Email" {}
					.button-container{
						(button(&ebook.get_file_download(), "Download"))
						button.no-thanks {
							"No Thanks"
						}
					}
				}
			}
			.right-side {
				(image(&ebook))
			}
		}
		(styles())
		(js())
	}))
}

js! {
	me(".no-thanks").on("click", (ev) => {
		halt(ev);
		me(ev).send("popup-dismissed");
	});
}

css! {
	me {
		.popup-container {
			display: grid;
			grid-template-columns: 2fr 1fr;
			max-height: 700px;
		}
		h3 {
			align-self: center;
			font-size: 24px;
			color: slategray;
		}

		.left-side {
			display: grid;
			text-align: center;
			grid-template-rows: 20px .5fr 1fr 2fr;
			padding-inline: 75px;
		}
		.right-side {
			align-self: center;
		}
		.button-container {
			display:flex;
			flex-direction: column;
			align-items: center;
			gap: 1rem;

			.no-thanks {
				background-color: transparent;
				border: none;
				text-decoration: underline;
				font-size: 16px;
			}
		}
		form {
			display: grid;
			min-width: 300px;
			justify-self: center;
			max-height: 255px;
			align-items: baseline;

			label {
				align-self: center;
				max-height: 30px;
			}

			input {
				padding: 5px;
				max-height: 30px;
			}
		}
	}
}
