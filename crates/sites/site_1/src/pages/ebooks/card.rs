use super::image;
use crate::prelude::*;
use lib_directus::Ebook;

pub fn card(ebook: &Ebook) -> Markup {
	html! {
		article.card
		hx_patch=(format!("https://tosapp.eman.network/ebooks/{}", ebook.id))
		hx_trigger="mouseover" {
			(image(ebook))
			h2 { (ebook.name) }
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
				box-shadow: rgba(0, 0, 0, 0.05) 0px 6px 24px 0px, rgba(0, 0, 0, 0.08) 0px 0px 0px 1px;
				font-family: "Capitolina", serif; text-align: center;
				padding: 1rem;
				transition: transform 0.2s, box-shadow 0.2s;

				@container grid-auto-fit (inline-size < calc(40ch * 2 + 1rem)) {
					.book-3d {
							max-width: 200px;
							--book-thickness: 15px;
					}
				}
			}
		article.card:hover {
			transform: scale(1.01);
			box-shadow: rgba(0, 0, 0, 0.05) 0px 6px 24px 0px, rgba(0, 0, 0, 0.08) 0px 0px 0px 1px,
			rgba(0, 0, 0, 0.1) 0px 4px 6px -1px, rgba(0, 0, 0, 0.06) 0px 2px 4px -1px;
		}
		.subtext {
			font-size: 20px;
			line-height: 27px;
			font-weight: 400;
			color: #505050;
			font-family: "MyriadPro", sans-serif;
			padding: 0 1rem 1rem 1rem;
		}
		a.download {
			background-color: #0x4683ec;
			color: white;
			border-radius: 4px;
			text-decoration: none;
			display: inline-block;
			text-align: center;
			max-width: 50%;
			font-family: "MyriadPro", sans-serif;
			font-size: 24px;
			letter-spacing: 1.5px;
			font-weight: 600;
			padding: 0.5rem 1rem 0.25rem 1rem;
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
	}
	}
}
