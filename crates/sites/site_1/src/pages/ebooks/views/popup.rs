use crate::pages::ebooks;
use crate::views;
use lib_directus::{get_ebook, Ebook};

use crate::prelude::*;

pub struct Popup<'a> {
	pub ebook: &'a Ebook,
}

impl<'a> Render for Popup<'a> {
	fn render(&self) -> Markup {
		views::Popup {
			content: content_markup(self.ebook),
		}
		.render()
	}
}

fn content_markup(ebook: &Ebook) -> Markup {
	html! {
		.popup-container {
				.hook { "Free Ebook" }
				.title { (ebook.name.as_str()) }
				@if let Some(sub_text) = &ebook.sub_text {
					.subtext { (sub_text) }
				}
				form hx-post="/ebooks/signup" hx-swap="none" trigger="submit" {
					(views::FormField { name: "first_name", label: "First Name", input_type: "text", placeholder: "First Name" })
					(views::FormField { name: "email", label: "Email", input_type: "email", placeholder: "Email" })
					.button-container{
						(Button::Primary { href: ebook.get_file_download(), text: "Download".to_owned() })
						(Button::Secondary { href: "#".to_owned(), text: "No Thanks".to_owned() })
					}
				}
				.book{
					(ebooks::Cover3D { ebook })
				}
			}
		(css())
		(js())
	}
}

pub async fn get_popup(Path(ebook_id): Path<u32>) -> Result<Markup> {
	let ebook = get_ebook(ebook_id).await?;
	Ok(self::Popup { ebook: &ebook }.render())
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
			max-width: 800px;
			grid-template-areas:
				"hook book"
				"title book"
				"subtext book"
				"form book";
			grid-template-columns: 2fr 1fr;
			grid-template-rows: 1fr;

			@media (max-width: 30rem) {
				overflow: hidden;
				grid-template-areas:
					"hook"
					"title"
					"subtext"
					"book"
					"form";
				grid-template-columns: 1fr;
				grid-template-rows: 1fr;
			}
		}
		.hook {
			grid-area: hook;
			font-size: var(--font-size-fluid-1);
			color: slategray;
			text-align: center;
		}
		.title {
			grid-area: title;
			font-size: var(--font-size-fluid-2);
			text-align: center;
			@media (max-width: 30rem) {
				font-size: var(--font-size-fluid-3);
			}
		}
		.subtext {
			grid-area: subtext;
			font-size: var(--font-size-fluid-0);
			color: var(--text-2);
			text-align: center;
			padding: var(--size-5) var(--size-10);
		}
		.book {
			grid-area: book;
			align-self: center;
			justify-self: center;
			transform: scale(1.3);
		}
		.button-container {
			display: flex;
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
			grid-area: form;
			display: grid;
			min-width: 300px;
			justify-self: center;
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
