use crate::prelude::*;

pub fn app_layout(children: Markup) -> Markup {
	html! {
		div #layout {
			div #menu {
				img.logo src="/js/assets/tos_dark_small.svg"{}
				div.menu-item.articles {
					img src="/js/assets/article.svg" {}
				}
				div.menu-item.bookmark {
					img src="/js/assets/bookmark.svg" {}
				}
				div.menu-item.gift {
					img src="/js/assets/gift.svg" {}
				}
				div.menu-item.books {
					img src="/js/assets/books.svg" {}
				}
				div.menu-item.bottom {
					img src="/js/assets/user.svg" {}
				}
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

css! {
	me {
		--menu-width: var(--size-8);
		--menu-item-width: var(--size-5);
		--header-height: var(--size-7);

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
			background-color: var(--surface-1);
			border-bottom: 1px solid var(--surface-4);
			box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
		}
		#content {
			grid-area: content;
			background-color: var(--surface-1);
			margin: 0;
			padding: var(--size-fluid-4);
			overflow: scroll;
		}
		#menu {
			container-type: inline-size;

			grid-area: menu;

			background-color: black;

			display: grid;
			align-content: start;
			align-items: center;
			justify-items: center;

			grid-template-rows: calc(var(--menu-item-width) + 25px) repeat(4, var(--menu-item-width)) 1fr;

			gap: var(--size-fluid-2);
			padding-block: var(--size-fluid-2);
			box-shadow: rgba(0, 0, 0, 0.4) 2px 0px 4px, rgba(0, 0, 0, 0.3) 7px 0px 13px -3px;
		}
		.menu-item {
			width: var(--menu-item-width);
			height: var(--menu-item-width);
			transition: width 0.2s ease, height 0.2s ease;
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;
		}
		.menu-item.bottom {
			align-self: end;
			border: 1px solid white;
			border-radius: 50%;
			height: var(--menu-item-width);
			width: var(--menu-item-width);
		}
		.logo {
			width: calc(var(--menu-item-width) * 2);
			height: calc(var(--menu-item-width) * 2);
		}
	}
}
