use super::image::image;
use crate::views::popup;
use lib_directus::get_ebook;

use crate::prelude::*;

pub async fn ebook_popup(Path(ebook_id): Path<u32>) -> Result<Markup> {
	let ebook = get_ebook(ebook_id).await?;
	Ok(popup(html! {
		.popup-container {
			.left-side {
				.hook { "Free Ebook" }
				.title { (ebook.name.as_str()) }
				@if let Some(sub_text) = &ebook.sub_text {
					.subtext { (sub_text) }
				}
				form hx-post="/ebooks/signup" hx-swap="none" trigger="submit" {
					label { "First Name" }
					input type="text" placeholder="First Name" {}
					label { "Last Name" }
					input type="email" placeholder="Email" {}
					.button-container{
						(Button::Primary { href: ebook.get_file_download(), text: "Download".to_owned() })
						(Button::Secondary { href: "#".to_owned(), text: "No Thanks".to_owned() })
					}
				}
			}
			.right-side {
				(image(&ebook))
			}
		}
		(css())
		(js())
	}))
}

js! {
	me(".secondary").on("click", (ev) => {
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
			max-width: 800px;
		}
		.title {
			font-size: var(--font-size-3);
			text-align: center;
		}
		.subtext {
			font-size: var(--font-size-1);
			color: var(--text-2);
			text-align: center;
		}
		.hook {
			font-size: var(--font-size-2);
			color: slategray;
			text-align: center;
		}

		.left-side {
			display: grid;
			grid-template-rows: 20px .5fr 1fr 2fr;
			padding-inline: 75px;
			justify-content: center;
		}
		.right-side {
			align-self: center;
			justify-self: center;
			transform: scale(1.5);
		}
		.button-container {
			display:flex;
			flex-direction: column;
			align-items: center;
			margin-top: var(--size-6);
			gap: var(--size-3);

			.no-thanks {
				background-color: transparent;
				color: var(--text-2);
				border: 1px solid var(--text-2);
			}
		}
		form {
			display: grid;
			min-width: 300px;
			justify-self: center;
			max-height: 255px;
			align-items: baseline;
			gap: var(--size-1);

			label {
				align-self: center;
			}

			input {
				padding: var(--size-2);
			}
		}
	}
}
