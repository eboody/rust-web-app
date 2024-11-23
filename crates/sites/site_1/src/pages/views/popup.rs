use crate::prelude::*;

pub struct Popup {
	pub content: Markup,
}

impl Render for Popup {
	fn render(&self) -> Markup {
		html! {
			#popup.fade-in {
				.popup-overlay {}
				.popup-content {
					.close.icon { (icon::Close) }
					(self.content)
				}
			}
			(css())
			(js())
		}
	}
}

js! {
	me("#popup").on("popup-dismissed", async (ev) => {
		let el = me(ev);
		el.classAdd("fade-out");
		await sleep(500);
		el.classAdd("vh");
	});

	me(".popup-overlay").on("click", () => me("#popup").send("popup-dismissed"));

	me(".close").on("click", () => me("#popup").send("popup-dismissed"));
}

css! {
	me {
		--main-ease: var(--ease-5);
		--main-transition: opacity 0.5s var(--main-ease);

		.fade-in.htmx-added,
		.fade-in,
		.fade-out,
		.popup-content {
			opacity: 0;
			transition: var(--main-transition);
		}

		.fade-in {
			opacity: 1;
		}

		#popup {
			position: fixed;
			z-index: 1;
			inset: 0;
			width: 100%;
			height: 100%;
			overflow: auto;
			background-color: rgb(0, 0, 0);
			background-color: rgba(0, 0, 0, 0.4);
		}
		.popup-content {
			position: relative;
			overflow-x: hidden;

			border: 1px solid #888;
			background-color: var(--surface-1);
			margin: 15% auto;
			padding: 50px;
			width: fit-content;
			max-width: 90%;

			transition:
				width 0.3s,
				padding 0.3s;
			animation:
				var(--animation-fade-in) forwards,
				var(--animation-slide-in-down);
			animation-timing-function: var(--main-ease);
			animation-duration: 0.5s;
			animation-delay: 0.5s;

			@media (max-width: 30rem) {
				padding: 10px;
				margin: 5% auto;
			}
		}
		.popup-overlay {
			position: fixed;
			inset:0;
			width: 100%;
			height: 100%;
			z-index: -1;
			background-color: rgba(0, 0, 0, 0.7);

			animation: var(--animation-fade-in) 1s var(--main-ease);
		}
		.close {
			position: absolute;
			top: 20px;
			right: 20px;

			cursor: pointer;
			transition:
				background-color 0.3s,
				filter 0.3s,
				height 0.3s,
				width 0.3s,
				transform 0.3s,
				padding 0.3s,
				box-shadow 0.3s;

			&:hover {
				background-color: var(--surface-3);
				box-shadow:
					rgba(0, 0, 0, 0.05) 0px 0px 0px 1px,
					rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
			}

			@media (max-width: 30rem) {
				transform: scale(0.5);
				top: 5px;
				right: 5px;
			}
		}
	}
}
