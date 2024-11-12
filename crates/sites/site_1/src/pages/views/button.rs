use crate::prelude::*;

pub fn button(href: impl Into<String>, text: impl Into<String>) -> Markup {
	html! {
		a.download href=(href.into()) { (text.into()) }
		(styles())
	}
}

css! {
	me {
		a.download:hover {
			box-shadow: rgb(255,255,255) 0px 0px 0px 2px, rgb(255,255,255) 0px 0px 0px 3px, rgb(255,255,255) 0px 0px 0px 4px, rgba(0,0,0, 1) 0px 0px 0px 5px;
		}
		a.download {
			background-color: #0x000;
			color: white;
			text-decoration: none;
			display: inline-block;
			text-align: center;
			max-width: 50%;
			font-size: 24px;
			padding: 0.5rem 1rem;
			box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 1px 3px 1px;
			transition: box-shadow 0.3s;
			margin-top: 1rem;
		}
	}
}
