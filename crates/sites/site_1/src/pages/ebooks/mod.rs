mod card;
mod ebook_popup;
mod image;
mod menu;

pub use card::card;
pub use ebook_popup::ebook_popup;
pub use image::*;
pub use menu::menu;

use crate::prelude::*;

pub fn router() -> Router {
	Router::new()
		.route("/menu", get(menu))
		.route("/popup/:id", get(ebook_popup))
}
