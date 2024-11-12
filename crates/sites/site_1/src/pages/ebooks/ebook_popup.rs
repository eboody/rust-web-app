use crate::views::popup;
use lib_directus::get_ebook;

use crate::prelude::*;

pub async fn ebook_popup(Path(ebook_id): Path<u32>) -> Result<Markup> {
	let ebook = get_ebook(ebook_id).await?;
	Ok(popup(html! {
		.popup-container {
			.left-side {
				h1.title { (ebook.name.as_str()) }
				@if let Some(sub_text) = &ebook.sub_text {
					h2.subtext { (sub_text) }
				}
				form {
					label { "First Name" }
					input type="text" placeholder="first name" {}
					label { "Last Name" }
					input type="email" placeholder="Email" {}
				}
				.button-container{
					(button(&ebook.get_file_download(), "Download"))
					button {
						"No Thanks"
					}
				}
			}
			.right-side {
				.ebook-image hx-get=(format!("/ebooks/image/{}", ebook.id)) hx-trigger="load" hx-swap="outerHTML" {}
			}
		}
		style { (styles()) }
	}))
}

fn styles() -> PreEscaped<String> {
	css! {
		me {
			.popup-container {
				display: grid;
				grid-template-columns: 2fr 1fr;
				gap: 5rem;
			}

			.left-side {
				display: grid;
				grid-template-rows: 1fr 1fr 1fr 1fr;
				text-align: center;
			}
			.right-side {
				align-self: center;
			}
			.button-container {
				display:flex;
				flex-direction: column;
				align-items: center;
				gap: 2rem;
			}
			form {
				display: grid;
			}
		}
	}
}
