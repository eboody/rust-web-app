use crate::prelude::*;

pub fn popup(modal_content: Markup) -> Markup {
	html! {
		.popup.fade-in {
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

css! {
	me {
	.fade-in.htmx-added,
	.popup-content{
		opacity: 0;
	}
	.fade-in {
		opacity: 1;
		transition: opacity 0.5s var(--ease-5);
	}
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
		overflow-x: hidden;
		width: fit-content;

		animation:
		var(--animation-fade-in) forwards,
		var(--animation-slide-in-down);
		animation-timing-function: var(--ease-5);
		animation-duration: 0.5s;
		animation-delay: 0.5s;
	}
	.popup-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: -1;
		background-color: rgba(0,0,0,0.7);

		animation: var(--animation-fade-in) 1s var(--ease-5);
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
