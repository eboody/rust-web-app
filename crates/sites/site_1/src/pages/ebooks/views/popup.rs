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
	me("button.primary").disabled = true;

	me(".secondary").on("click", (ev) => {
		halt(ev);
		me(ev).send("popup-dismissed");
	});

	me("button.primary").on("valid-email", (ev) => {
		me(ev).validEmail = true;
		if (me(ev).validFirstName) {
			me(ev).disabled = false;
		}
	});

	me("button.primary").on("valid-first-name", (ev) => {
		me(ev).validFirstName = true;
		if (me(ev).validEmail) {
			me(ev).disabled = false;
		}
	});

	me("form input[name='email']").on("input", (ev) => {
		let emailPattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$";
		let emailRegex = new RegExp(emailPattern);

		let isValidEmail = emailRegex.test(ev.target.value);

		if (isValidEmail) {
			me("button.primary").send("valid-email");
		} else {
			me("button.primary").disabled = true;
		}
	});

	me("form input[name='first_name']").on("input", (ev) => {
		let namePattern = r"^[a-zA-Z]{2,}$";
		let nameRegex = new RegExp(namePattern);
		let isValidFirstName = nameRegex.test(ev.target.value);

		if (isValidFirstName) {
			me("button.primary").send("valid-first-name");
		} else {
			me("button.primary").disabled = true;
		}
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
				grid-template-areas:
					"hook"
					"title"
					"subtext"
					"book"
					"form";
				grid-template-columns: 1fr;
				grid-template-rows: 1fr;
				padding-bottom: var(--size-3);
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
			font-family: Capitolina;
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
			@media (max-width: 30rem) {
				padding: var(--size-4) var(--size-2);
			}
		}
		.book {
			grid-area: book;
			align-self: center;
			justify-self: center;
			transform: scale(1.3);
			@media (max-width: 30rem) {
				transform: scale(1);
				margin-bottom: var(--size-5);
				margin-top: var(--size-1);
			}
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
			gap: var(--size-3);

			label {
				align-self: center;
			}

			input {
				padding: var(--size-2);
				box-shadow:
					rgba(204, 219, 232, 0.6) 1px 1px 1px 0px inset,
					rgba(255, 255, 255, 0.3) -3px -3px 6px 1px inset;
			}
		}
	}
}
