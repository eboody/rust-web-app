use crate::prelude::*;

pub fn app_layout(children: Markup) -> Markup {
	let js = js! {
		me("#menu").addEventListener("click", () => {
			me("#layout").style.gridTemplateColumns = me("#layout").style.gridTemplateColumns === "500px 1fr" ? "var(--menu-width) 1fr" : "500px 1fr";
		});
	};
	html! {
		div #layout {
			div #menu {
				img.logo src="/js/assets/tos_dark_small.svg"{}
				div.menu-item {}
				div.menu-item {}
				div.menu-item {}
			}
			main #content {
				(children)
			}
			div #header {
				"Header"
			}
		}
		(styles())
		script { (js) }
	}
}

fn styles() -> Markup {
	html! { style{(
		css! { me {
			--menu-width: 100px;
			--menu-item-width: 60px;
			--header-height: 75px;
			--menu-padding-inline: calc((var(--menu-width) - var(--menu-item-width)) / 2);

			#layout {
				margin: 0;
				height: 100vh;
				display: grid;
				grid-template-columns: var(--menu-width) 1fr;
				grid-template-rows: var(--header-height) 1fr;

				grid-template-areas:
					"menu header"
					"menu content";

				transition: grid-template-columns 0.2s ease;
			}
			#header {
				grid-area: header;
				background-color: #222;
			}
			#content {
				grid-area: content;
				background-color: #444;
				margin: 0;
				padding: 50px;
			}
			#menu {
				grid-area: menu;

				background-color: #333;
				display: flex;
				flex-direction: column;
				gap: 10px;
				padding: var(--menu-padding-inline);
				align-items: center;
			}
			.menu-item {
				width: var(--menu-item-width);
				height: var(--menu-item-width);
				background-color: #555;
			}
			.logo {
				width: 100px;
				height: 100px;
			}
		}})
	}}
}
