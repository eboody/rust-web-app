mod admin;
pub mod api;
pub mod articles;
mod asset;
mod collections;
mod ebooks;
mod events;
mod language;
mod macros;
mod sections;
mod status;
mod tags;
mod wp_post;

#[allow(unused)]
pub use {
	admin::*, articles::*, asset::*, collections::*, ebooks::*, language::*,
	macros::*, sections::*, status::*, tags::*, wp_post::WpPosts,
};
