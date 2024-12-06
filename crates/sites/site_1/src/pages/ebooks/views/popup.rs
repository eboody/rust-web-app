use crate::{pages::ebooks, prelude::*};
use lib_core::model::directus::{EbooksTranslations, Language};

pub struct Popup<'a> {
	pub ebook: &'a EbooksTranslations,
}

impl Render for Popup<'_> {
	fn render(&self) -> Markup {
		view::Popup {
			content: content_markup(self.ebook),
		}
		.render()
	}
}

fn content_markup(ebook: &EbooksTranslations) -> Markup {
	html! {
		.popup-container ebook_id=(ebook.id) {
				.hook { "Free Ebook" }
					.title { (&ebook.title.as_deref().unwrap_or("")) }
					.descriptor { (&ebook.descriptor.as_deref().unwrap_or("")) }
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
	let ebook = EbooksTranslations::select()
		.where_("languages_code = ?")
		.bind(Language::English.to_string())
		.where_("status = ?")
		.bind("published")
		.join(EbooksTranslations::ebook())
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
			transform: scale(1.3);
			transition:
				margin 0.3s,
				transform 0.3s;

			@media (max-width: 35rem) {
				margin-bottom: var(--size-6);
				margin-top: var(--size-6);
			}
		}
	}
}
