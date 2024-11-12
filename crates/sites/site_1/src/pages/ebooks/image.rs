use crate::prelude::*;
use lib_directus::Ebook;

pub fn image(ebook: &Ebook) -> Markup {
	let js = PreEscaped(
		js! {
			console.log("This book with id " + "ebook.id " + "is: " + "ebook.name" );
		}
		.into_string()
		.replace("ebook.name", &ebook.name)
		.replace("ebook.id", &ebook.id.to_string()),
	);

	html! {
		.book {
			.inner {
				img.cover
					src=(ebook.get_cover_image())
					alt=(ebook.name);
			}
		}
		style { (styles()) }
		script { (js) }
	}
}

fn styles() -> PreEscaped<String> {
	css! {
			@keyframes book-3d {
					from { transform: rotateY(-15deg); }
					to   { transform: rotateY(-25deg); }
			}

			@keyframes book-3d-back {
					from { transform: rotateY(-25deg); }
					to   { transform: rotateY(-15deg); }
			}

			me {
				.book {
						--book-thickness: 30px;
						--cover-color: slategray;
						perspective: 1000px;
						max-width: 250px;
						margin: 55px auto;
						transition: max-width 0.3s, --book-thickness 0.3s;
				}

				.book img {
						display: block;
						width: 100%;
						height: auto;
						border-radius: 0px 2px 2px 0px;
						transform: translateZ(var(--book-thickness));
						box-shadow: 5px 5px 20px rgba(0, 0, 0, 0.1);
				}
				.book::after {
						content: "";
						position: absolute;
						inset: 1px;
						height: 99%;
						border-radius: 3px;
						pointer-events: none;
						background: linear-gradient(
								90deg,
								rgba(0, 0, 0, 0.118) 0.65%,
								rgba(255, 255, 255, 0.2) 1.53%,
								rgba(255, 255, 255, 0.1) 2.38%,
								rgba(0, 0, 0, 0.05) 3.26%,
								rgba(255, 255, 255, 0.14) 5.68%,
								rgba(244, 244, 244, 0) 6.96%
						);
				}
				.inner::after {
					content: "";
					position: absolute;
					top: 0;
					left: 1%;
					width: 100%;
					height: 100%;
					transform: translateZ(calc(var(--book-thickness) * -1));
					background-color: var(--cover-color);
					border-radius: 0 2px 2px 0;
					box-shadow: -10px 0 50px 10px rgba(0,0,0, 0.3);
				}
				.inner::before {
					position: absolute;
					content: ' ';
					left: 100%;
					top: 1%;
					width: calc(var(--book-thickness) * 2);
					height: 98%;
					transform: translate(-55%,0) rotateY(90deg);
					background: linear-gradient(90deg, #fff 0%, hsl(0, 0%, 94%) 5%, #fff 10%, hsl(0, 0%, 94%) 15%, #fff 20%, hsl(0, 0%, 94%) 25%, #fff 30%, hsl(0, 0%, 94%) 35%, #fff 40%, hsl(0, 0%, 94%) 45%, #fff 50%, hsl(0, 0%, 94%) 55%, #fff 60%, hsl(0, 0%, 94%) 65%, #fff 70%, hsl(0, 0%, 94%) 75%, #fff 80%, hsl(0, 0%, 94%) 85%, #fff 90%, hsl(0, 0%, 94%) 95%, #fff 100%);
				}
				.inner {
					position: relative;
					transform-style: preserve-3d;
					transform: rotateY(-25deg);
					animation: book-3d 1s ease forwards;
				}
		}
	}
}
