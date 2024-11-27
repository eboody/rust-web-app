use reqwest;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
	pub status: reqwest::StatusCode,
	pub content: String,
	pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
	Reqwest(reqwest::Error),
	Serde(serde_json::Error),
	Io(std::io::Error),
	ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (module, e) = match self {
			Error::Reqwest(e) => ("reqwest", e.to_string()),
			Error::Serde(e) => ("serde", e.to_string()),
			Error::Io(e) => ("IO", e.to_string()),
			Error::ResponseError(e) => {
				("response", format!("status code {}", e.status))
			}
		};
		write!(f, "error in {}: {}", module, e)
	}
}

impl<T: fmt::Debug> error::Error for Error<T> {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(match self {
			Error::Reqwest(e) => e,
			Error::Serde(e) => e,
			Error::Io(e) => e,
			Error::ResponseError(_) => return None,
		})
	}
}

impl<T> From<reqwest::Error> for Error<T> {
	fn from(e: reqwest::Error) -> Self {
		Error::Reqwest(e)
	}
}

impl<T> From<serde_json::Error> for Error<T> {
	fn from(e: serde_json::Error) -> Self {
		Error::Serde(e)
	}
}

impl<T> From<std::io::Error> for Error<T> {
	fn from(e: std::io::Error) -> Self {
		Error::Io(e)
	}
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
	::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(
	prefix: &str,
	value: &serde_json::Value,
) -> Vec<(String, String)> {
	if let serde_json::Value::Object(object) = value {
		let mut params = vec![];

		for (key, value) in object {
			match value {
				serde_json::Value::Object(_) => params.append(
					&mut parse_deep_object(&format!("{}[{}]", prefix, key), value),
				),
				serde_json::Value::Array(array) => {
					for (i, value) in array.iter().enumerate() {
						params.append(&mut parse_deep_object(
							&format!("{}[{}][{}]", prefix, key, i),
							value,
						));
					}
				}
				serde_json::Value::String(s) => {
					params.push((format!("{}[{}]", prefix, key), s.clone()))
				}
				_ => {
					params.push((format!("{}[{}]", prefix, key), value.to_string()))
				}
			}
		}

		return params;
	}

	unimplemented!("Only objects are supported with style=deepObject")
}

pub mod activity_api;
pub mod assets_api;
pub mod authentication_api;
pub mod collections_api;
pub mod comments_api;
pub mod extensions_api;
pub mod fields_api;
pub mod files_api;
pub mod flows_api;
pub mod folders_api;
pub mod items_api;
pub mod items_articles_api;
pub mod items_articles_directus_users_api;
pub mod items_articles_tags_api;
pub mod items_articles_translations_api;
pub mod items_ebooks_api;
pub mod items_ebooks_directus_users_api;
pub mod items_ebooks_tags_api;
pub mod items_ebooks_translations_api;
pub mod items_languages_api;
pub mod items_tags_api;
pub mod items_tags_translations_api;
pub mod operations_api;
pub mod permissions_api;
pub mod presets_api;
pub mod relations_api;
pub mod revisions_api;
pub mod roles_api;
pub mod schema_api;
pub mod server_api;
pub mod settings_api;
pub mod users_api;
pub mod utilities_api;
pub mod versions_api;
pub mod webhooks_api;

pub mod configuration;