use crate::prelude::*;

pub fn popup(modal_content: Markup) -> Markup {
	let js = js! {document.getElementById("content").classList.toggle("blurred");};
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
		script { (js) }
	}
}

fn styles() -> Markup {
	html! {
		style {
			r#"
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
			"#
		}
	}
}
