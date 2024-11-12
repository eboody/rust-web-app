use crate::prelude::*;

pub fn app_layout(children: Markup) -> Markup {
	html! {
		div #layout {
			div #menu {
				img.logo src="/js/assets/tos_dark_small.svg"{}
				div.menu-item {
					img src="/js/assets/books.svg" {}
				}
				div.menu-item {}
				div.menu-item {}
			}
			main #content {
				(children)
			}
			div #header {
			}
		}
		(styles())
		(js())
	}
}

fn js() -> Markup {
	js! {
		me("#menu").on("click", () => {
			const expandedMenuSize = "500px 1fr";
			const isExpanded = me("#layout").style.gridTemplateColumns === expandedMenuSize;
			if (isExpanded) {
				me("#layout").style.gridTemplateColumns = "var(--menu-width) 1fr";
				//me("img.logo").style.width = "100px";
				//me("img.logo").src = "/js/assets/tos_dark_small.svg";
			} else {
				me("#layout").style.gridTemplateColumns = expandedMenuSize;
				//me("img.logo").src = "/js/assets/tos_dark_large.svg";
				//me("img.logo").style.width = "400px";
			}
		});
	}
}

fn styles() -> Markup {
	css! {
		me {
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
				background-color: #fff;
				border-bottom: 1px solid #ddd;
				box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
			}
			#content {
				grid-area: content;
				background-color: #fff;
				margin: 0;
				padding: 50px;
				overflow: scroll;
			}
			#menu {
				container-type: inline-size;

				grid-area: menu;

				background-color: #000;
				display: flex;
				flex-direction: column;
				gap: 10px;
				padding: var(--menu-padding-inline);
				align-items: center;
				box-shadow: rgba(0, 0, 0, 0.4) 2px 0px 4px, rgba(0, 0, 0, 0.3) 7px 0px 13px -3px;

				@container (min-width: 120px) {
					.menu-item {
					}
				}
			}
			.menu-item {
				width: 66%;
				height: var(--menu-item-width);
				background-color: #555;
				transition: width 0.2s ease, height 0.2s ease;
				display: flex;
				flex-direction: column;
				justify-content: center;
				align-items: center;
			}
			.logo {
				width: 100px;
				height: 100px;
			}
		}
	}
}
