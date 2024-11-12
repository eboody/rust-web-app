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
		--menu-width: 100px;//100px;
		--menu-item-width: 60px;
		--header-height: 0px;//75px;
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
			padding: 25px;
			overflow: scroll;
		}
		#menu {
			container-type: inline-size;

			grid-area: menu;

			background-color: #000;

			display: grid;
			align-content: start;
			justify-items: center;
			grid-template-rows: calc(var(--menu-item-width) + 25px) repeat(4, var(--menu-item-width)) 1fr;

			gap: 10px;

			padding: var(--menu-padding-inline);
			padding-top: 25px;
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
			transition: width 0.2s ease, height 0.2s ease;
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;
		}
		.menu-item.bottom {
			align-self: end;
			margin-bottom: 25px;
			border: 1px solid white;
			border-radius: 50%;
			height: 50px;
			width: 50px;
		}
		.logo {
			width: 100px;
			height: 100px;
			gap: 20px;
		}
	}
}
