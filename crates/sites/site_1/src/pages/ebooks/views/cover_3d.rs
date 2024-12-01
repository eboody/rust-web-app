use crate::prelude::*;
use lib_core::model::{CoverImage, EbooksTranslations};

pub struct Cover3D<'a> {
	pub ebook: &'a EbooksTranslations,
}

impl Render for Cover3D<'_> {
	fn render(&self) -> Markup {
		let ebook = &self.ebook;
		let title = &ebook.title.as_deref().unwrap_or("Ebook");

		html! {
			.book {
				.inner {
					img.cover src=(ebook.thumbnail_url(100)) alt=(title);
				}
				.shadow {}
			}
		(css())
		}
	}
}

css! {
	@keyframes book-shadow-3d {
		from {
			transform: skewx(-45deg) skewy(2deg);
		}
		to {
			transform: skewX(-63deg) skewY(4.5deg);
		}
	}
	@keyframes book-shadow-3d-back {
		from {
			transform: skewX(-63deg) skewY(4.5deg);
		}
		to {
			transform: skewX(-45deg) skewY(2deg);
		}
	}
	@keyframes book-3d {
		from {
			transform: rotateY(-15deg);
		}
		to {
			transform: rotateY(-25deg);
		}
	}

	@keyframes book-3d-back {
		from {
			transform: rotateY(-25deg);
		}
		to {
			transform: rotateY(-15deg);
		}
	}

	@keyframes book-spine {
		from {
			transform: translateX(0);
		}
		to {
			transform: translateX(-3px);
		}
	}
	@keyframes book-spine-back {
		from {
			transform: translateX(-3px);
		}
		to {
			transform: translateX(0);
		}
	}

	me {
		.book {
			--book-thickness: min(1dvw, 10px);
			--cover-color: slategray;
			--max-width: min(15dvw, 150px);

			perspective: 1000px;
			max-width: var(--max-width);
			transition:
				max-width 0.5s,
				--book-thickness 0.5s;
			@media (max-width: 30rem) {
				--max-width: 75px;
				--book-thickness: 5px;
			}
		}

		.book:hover .inner {
			animation: book-3d-back 0.5s ease forwards;
		}
		.book:hover .shadow {
			animation: book-shadow-3d-back 0.5s ease forwards;
		}
		.book:hover::after {
			animation: book-spine 0.5s ease forwards;
		}

		.shadow {
			content: "";
			position: absolute;
			left: 3%;
			top: 73%;
			z-index: -1;
			background-color: rgba(0, 0, 0, 0.3);
			width: var(--max-width);
			height: calc(var(--max-width) / 2.5);
			transform-origin: bottom center;
			transform-style: preserve-3d;
			transform: skewX(-45deg) skewY(2deg);
			filter: blur(5px);
			transform-style: preserve-3d;
			animation: book-shadow-3d 1s ease forwards;
		}

		.inner {
			position: relative;
			transform-style: preserve-3d;
			transform: rotateY(-25deg);
			animation: book-3d 1s ease forwards;
		}

		.book img.cover {
			display: block;
			width: 100%;
			height: auto;
			border-radius: 0px 2px 2px 0px;
			transform: translateZ(var(--book-thickness));
			z-index: 2;
			background: no-repeat center / contain url("/assets/tos_dark_small.svg")
				#000;
		}
		.book::after {
			content: "";
			position: absolute;
			inset: 3px;
			height: 97%;
			border-radius: 3px;
			pointer-events: none;
			animation: book-spine-back 1s ease forwards;
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
			box-shadow: -10px 0 50px 10px rgba(0, 0, 0, 0.25);
		}
		.inner::before {
			position: absolute;
			content: " ";
			left: 100%;
			top: 1%;
			width: calc(var(--book-thickness) * 2);
			height: 98%;
			z-index: 1;
			transform: translate(-55%, 0) rotateY(90deg);
			background: linear-gradient(
				90deg,
				#fff 0%,
				hsl(0, 0%, 80%) 5%,
				#fff 10%,
				hsl(0, 0%, 80%) 15%,
				#fff 20%,
				hsl(0, 0%, 80%) 25%,
				#fff 30%,
				hsl(0, 0%, 80%) 35%,
				#fff 40%,
				hsl(0, 0%, 80%) 45%,
				#fff 50%,
				hsl(0, 0%, 80%) 55%,
				#fff 60%,
				hsl(0, 0%, 80%) 65%,
				#fff 70%,
				hsl(0, 0%, 80%) 75%,
				#fff 80%,
				hsl(0, 0%, 80%) 85%,
				#fff 90%,
				hsl(0, 0%, 80%) 95%,
				#fff 100%
			);
		}
	}
}
