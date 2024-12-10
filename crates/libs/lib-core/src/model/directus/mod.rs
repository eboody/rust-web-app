mod admin;
pub mod api;
pub mod articles;
mod asset;
mod collections;
mod ebooks;
mod events;
mod language;
mod macros;
mod post;
mod sections;
mod status;
mod tags;

#[allow(unused)]
pub use {
	admin::*, articles::*, asset::*, collections::*, ebooks::*, language::*,
	macros::*, post::Posts, sections::*, status::*, tags::*,
};
