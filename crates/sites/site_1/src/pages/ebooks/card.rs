use super::image;
use crate::prelude::*;
use lib_directus::Ebook;

pub fn card(ebook: &Ebook) -> Markup {
	html! {
		article.card
		hx_patch=(format!("https://tosapp.eman.network/ebooks/{}", ebook.id))
		hx_trigger="mouseover" {
			(image(ebook))
			h2 .book-name { (ebook.name) }
			p.subtext { (ebook.sub_text.clone().unwrap_or("".to_owned())) }
			a.download href=(ebook.get_file_download()) { "Download" }
		}
		style { (styles()) }
	}
}

fn styles() -> PreEscaped<String> {
	css! {
		me {
			article.card {
				box-shadow: rgba(17, 12, 46, 0.05) 0px 48px 100px 0px,
				 rgba(17, 17, 26, 0.1) 0px 1px 0px;
				padding: 1rem;
				transition: transform 0.2s, box-shadow 0.2s;
				text-align: center;
				display: flex;
				flex-direction: column;
				align-items: center;
				justify-content: space-between;

				@container grid-auto-fit (inline-size < calc(40ch * 2 + 1rem)) {
					.book-3d {
							max-width: 200px;
							--book-thickness: 30px;
					}
				}
			}
		article.card:hover {
				box-shadow: rgba(17, 12, 46, 0.10) 0px 48px 100px 0px,
				 rgba(17, 17, 26, 0.15) 0px 1px 0px;
		}
		.subtext {
			font-size: 20px;
			line-height: 27px;
			font-weight: 400;
			color: #505050;
			padding: 0 1rem 1rem 1rem;
		}
		a.download {
			background-color: #0x000;
			color: white;
			text-decoration: none;
			display: inline-block;
			text-align: center;
			max-width: 50%;
			font-size: 24px;
			padding: 0.5rem 1rem;
			box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 1px 3px 1px;
			margin-top: 1rem;
		}
		article.card:has(> img) {

			border: 2px solid var(--clr-primary-300);
			box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;


			@container grid-auto-fit (inline-size > calc(30ch * 2 + 1rem)) {
				grid-column: span 2;

				display: grid;
				grid-template-columns: subgrid;
				gap: 0;

				> img {
					grid-column: 2;
					grid-row: 1 / 4;
				}
			}

			@container grid-auto-fit (inline-size > calc(30ch * 4 + 3rem)) {

				grid-column: span 2;
				grid-row: span 2;

				> :not(img) {
					grid-column: 1 / -1;
				}

				> img {
					grid-column: 1 / -1;
					grid-row: 1;
				}
			}
		}
		h2.book-name {
			font-family: "EB Garamond", sans-serif;
		}
	}
	}
}
