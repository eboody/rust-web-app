mod card;
mod image;
mod menu;

pub use card::card;
pub use image::image;
pub use menu::menu;

use crate::prelude::*;

pub fn router() -> Router {
	Router::new().route("/menu", get(menu))
}
