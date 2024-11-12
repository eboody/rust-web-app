use crate::prelude::*;

pub fn popup(modal_content: Markup) -> Markup {
	html! {
		.popup {
			.popup-overlay {}
			.popup-content {
				span.close {
					"×"
				}
				(modal_content)
			}
		}
		(styles())
		(js())
	}
}

fn js() -> Markup {
	js! {
		me(".popup")
		.on("popup-dismissed", (ev)=>{
			me(ev).classAdd("vh");
		});

		me(".popup-overlay")
		.on("click", () => me(".popup").send("popup-dismissed"));

		me(".close")
		.on("click", ()=>me(".popup").send("popup-dismissed"));
	}
}

fn styles() -> Markup {
	css! {
		me {
		.popup {
			position: fixed;
			z-index: 1;
			left: 0;
			top: 0;
			width: 100%;
			height: 100%;
			overflow: auto;
			background-color: rgb(0,0,0);
			background-color: rgba(0,0,0,0.4);
		}
		.popup-content {
			background-color: #fefefe;
			margin: 15% auto;
			padding: 50px;
			border: 1px solid #888;
			width: 80%;
			overflow-x: hidden;
			max-width: 1300px;
		}
		.popup-overlay {
			position: fixed;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			z-index: -1;
			background-color: rgba(0,0,0,0.7);
		}
		.close {
			color: #aaaaaa;
			float: right;
			font-size: 28px;
			font-weight: bold;
		}
		.close:hover,
		.close:focus {
			color: #000;
			text-decoration: none;
			cursor: pointer;
		}}
	}
}
