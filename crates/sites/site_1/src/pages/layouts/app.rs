use crate::prelude::*;

pub struct App(pub Markup);

impl Render for App {
	fn render(&self) -> Markup {
		app(self.0.clone())
	}
}

fn app(children: Markup) -> Markup {
	html! {
		div #layout {
			div #menu {
				div.logo-container {
					.img-bg {}
					img.logo src="/assets/tos_dark_small.svg"{}
				}
				div.menu-item.articles {
					img src="/assets/article.svg" {}
				}
				div.menu-item.bookmark {
					img src="/assets/bookmark.svg" {}
				}
				div.menu-item.gift {
					img src="/assets/gift.svg" {}
				}
				div.menu-item.books {
					img src="/assets/books.svg" {}
				}
				div.menu-item.bottom {
					img src="/assets/user.svg" {}
				}
			}
			main #content {
				(children)
			}
			div #header {
			}
		}
		(css())
		(js())
	}
}

js! {
	me("#menu").on("click", () => {
		const expandedMenuSize = "300px 1fr";
		const isExpanded = me("#layout").style.gridTemplateColumns === expandedMenuSize;
		if (isExpanded) {
			me("#layout").style.gridTemplateColumns = "var(--menu-width) 1fr";
			me("img.logo").src = "assets/tos_dark_small.svg";
			me("img.logo").style.padding = "0";
			me(".img-bg").style.transform= "scaleX(1)";
		} else {
			me("#layout").style.gridTemplateColumns = expandedMenuSize;
			me("img.logo").src = "assets/tos_dark_large.svg";
			me("img.logo").style.padding = "var(--size-2)";
			me(".img-bg").style.transform= "scaleX(3)";
		}
	});
}

css! {
	me {
		--menu-width: 0px;//var(--size-8);
		--menu-item-width: var(--size-5);
		--header-height: 0px; //var(--size-7);

		.logo-container {

			position: relative;
			height: var(--menu-width);
			display: flex;
			justify-content: center;
			align-items: center;
			z-index: 1;

			.logo {
				height: calc(var(--menu-item-width) * 2);
				display: flex;
				justify-content: center;
				align-items: center;
			}

			.img-bg {
				background-image: linear-gradient(
					-45deg,
					var(--brand) 50%,
					var(--link) 50%
				);
				filter: blur(30px);
				width: 50px;
				height: 50px;
				position: absolute;
				z-index: 0;
				top: -4px;
				transition: transform 0.2s ease;
			}
		}



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
			@media (max-width: 30rem) {
				padding: var(--size-0);
			}
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
			overflow: hidden;
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

	}
}
