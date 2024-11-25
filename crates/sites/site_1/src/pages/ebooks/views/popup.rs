use crate::pages::ebooks;
use crate::prelude::*;
use lib_core::model::Ebook;

pub struct Popup<'a> {
	pub ebook: &'a Ebook,
}

impl Render for Popup<'_> {
	fn render(&self) -> Markup {
		view::Popup {
			content: content_markup(self.ebook),
		}
		.render()
	}
}

fn content_markup(ebook: &Ebook) -> Markup {
	html! {
		.popup-container ebook_id=(ebook.id) {
				.hook { "Free Ebook" }
				@if let Some(title) = &ebook.title {
					.title { (title) }
				}
				@if let Some(descriptor) = &ebook.descriptor {
					.descriptor { (descriptor) }
				}
				(ebooks::PopupSignupForm { ebook })
				.book{
					(ebooks::Cover3D { ebook })
				}
			}
		(css())
		(js())
	}
}

pub async fn get_popup(State(mm): State<ModelManager>) -> Result<Markup> {
	let ebook = Ebook::select()
		.where_("languages_code = ?")
		.bind("en-US")
		.where_("published = true")
		.join(Ebook::ebook())
		.fetch_one(mm.orm())
		.await?;

	Ok(self::Popup { ebook: &ebook }.render())
}

js! {
	me("button.primary").disabled = true;

	me(".secondary").on("click", (ev) => {
		halt(ev);
		me(ev).send("popup-dismissed");
	});

	localStorage.setItem("ebook_popup", {
		ebook_id: me(".popup-container").attribute("ebook_id"),
		date_viewed: new Date().getTime(),
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
				"descriptor book"
				"form book";
			grid-template-columns: 2fr 1fr;
			grid-template-rows: 1fr;

			transition:
				grid-template-columns 0.3s,
				padding-bottom 0.3s,
				grid-template-columns 0.3s,
				grid-template-rows 0.3s;

			@media (max-width: 35rem) {
				grid-template-areas:
					"hook"
					"title"
					"descriptor"
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
			color: var(--brand);
			text-align: center;
		}

		.title {
			grid-area: title;
			color: var(--text-1);

			font-size: var(--font-size-fluid-2);
			text-align: center;
			font-family: Capitolina;

			transition:
				font-size 0.3s,
				line-height 0.3s;

			@media (max-width: 30rem) {
				font-size: var(--font-size-fluid-2);
				line-height: normal;
			}
		}

		.descriptor {
			grid-area: descriptor;

			font-size: var(--font-size-fluid-0);
			padding: var(--size-5) var(--size-10);
			color: var(--text-2);
			text-align: center;

			transition: padding 0.3s;

			@media (max-width: 30rem) {
				padding: var(--size-4) var(--size-2);
			}
		}

		.book {
			grid-area: book;
			align-self: center;
			justify-self: center;
			z-index: -1;
			transform: scale(1.3);
			transition:
				margin 0.3s,
				transform 0.3s;

			@media (max-width: 35rem) {
				margin-bottom: var(--size-6);
				margin-top: var(--size-3);
			}
		}
	}
}
