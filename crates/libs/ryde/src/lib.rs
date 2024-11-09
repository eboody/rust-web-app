extern crate self as ryde;

//mod db;
mod html;
mod router;

pub use axum;
pub use axum::middleware;
pub use axum::{
	extract::*,
	http::{self, header::*, Uri},
	middleware::Next,
	response::*,
	routing::{any, delete, get, head, patch, post, put, trace},
	*,
};
pub use axum_extra::{self, extract::*, headers};
pub use cookie::Cookie;
//pub use db::{db, rusqlite, tokio_rusqlite, Connection};
pub use html::{component, escape, html, Component, Elements, Render};
pub use router::{router, routes, url};
pub use ryde_macros::{RequestParts, StaticFiles};
pub use serde;
pub use serde::*;
pub use std::fmt::Display;
pub use std::sync::Arc;
pub use tokio;
use tokio::task::JoinError;
pub use tokio::*;
pub use tower;

pub type Result<T> = std::result::Result<T, Error>;

pub fn server(ip: &str, router: Router) -> Result<()> {
	let rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(async { serve(ip, router).await });
	Ok(())
}

pub async fn serve(ip: &str, router: Router) {
	let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
	println!("Listening on {}", ip);
	axum::serve(listener, router).await.unwrap();
}

#[macro_export]
macro_rules! render_static_files {
	() => {{
		Assets::render()
	}};
}

pub type Html = Component;

impl IntoResponse for Html {
	fn into_response(self) -> Response {
		axum::response::Html(self.html).into_response()
	}
}

pub fn redirect(s: String) -> Response {
	let headers = [(SET_COOKIE, format!("flash={}", "")), (LOCATION, s.into())];

	(http::StatusCode::SEE_OTHER, headers).into_response()
}

#[macro_export]
macro_rules! redirect_to {
    ($ident:ident $(,$expr:expr)*) => {{
        let _ = $ident;
        redirect(url!($ident, $($expr),*))
    }}
}

#[macro_export]
macro_rules! id {
    ($($idents:ident),*) => {{
        $(let _ = &$idents;)*
        vec![$((stringify!($idents)),)*].join(" ")
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	DatabaseConnectionClosed,
	DatabaseClose,
	Database(String),
	UniqueConstraintFailed(String),
	Io(String),
	NotFound,
	InternalServer,
	Multipart(String),
	Join(String),
}

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Error::Io(value.to_string())
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::DatabaseConnectionClosed => {
				f.write_str("Error: Database connection closed")
			}
			Error::DatabaseClose => {
				f.write_str("Error: Database was already closed")
			}
			Error::Database(e) => {
				f.write_fmt(format_args!("Error: Generic database error {}", e))
			}
			Error::UniqueConstraintFailed(e) => {
				f.write_fmt(format_args!("Error: Unique constraint failed {}", e))
			}
			Error::Io(e) => f.write_fmt(format_args!("Error: Io error {}", e)),
			Error::NotFound => f.write_str("Error: Not found"),
			Error::InternalServer => f.write_str("Error: Internal server error"),
			Error::Multipart(e) => f.write_fmt(format_args!("Error: {}", e)),
			Error::Join(x) => f.write_fmt(format_args!("{x}")),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn ser::StdError + 'static)> {
		None
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn ser::StdError> {
		self.source()
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let (status, body) = match self {
			Error::DatabaseConnectionClosed => (500, "db connection closed".into()),
			Error::DatabaseClose => (500, "db closed".into()),
			Error::Database(err) => (500, err),
			Error::UniqueConstraintFailed(columns) => (500, columns),
			Error::Io(s) => (500, s),
			Error::NotFound => (404, "not found".into()),
			Error::InternalServer => (500, "internal server error".into()),
			Error::Multipart(_s) => (
				422,
				"Unprocessable entity from multipart form request".into(),
			),
			Error::Join(_) => (500, "internal server error".into()),
		};
		Response::builder()
			.status(status)
			.body(body.into())
			.unwrap()
	}
}

//impl From<tokio_rusqlite::Error> for Error {
//    fn from(value: tokio_rusqlite::Error) -> Self {
//        match value {
//            tokio_rusqlite::Error::ConnectionClosed => Error::DatabaseConnectionClosed,
//            tokio_rusqlite::Error::Close(_) => Error::DatabaseClose,
//            tokio_rusqlite::Error::Rusqlite(err) => err.into(),
//            tokio_rusqlite::Error::Other(err) => Error::Database(err.to_string()),
//            _ => unreachable!(),
//        }
//    }
//}

//impl From<rusqlite::Error> for Error {
//    fn from(value: rusqlite::Error) -> Self {
//        match value {
//            rusqlite::Error::QueryReturnedNoRows => Error::NotFound,
//            err => {
//                // TODO: follow the white rabbit to the actual error for unique constraints
//                let s = err.to_string();
//                if s.starts_with("UNIQUE constraint failed: ") {
//                    Error::UniqueConstraintFailed(
//                        s.split(":").map(|s| s.trim()).last().unwrap_or("").into(),
//                    )
//                } else {
//                    Error::Database(s)
//                }
//            }
//        }
//    }
//}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Error::Io(value.to_string())
	}
}

impl From<JoinError> for Error {
	fn from(value: JoinError) -> Self {
		Error::Join(value.to_string())
	}
}

impl From<axum_extra::extract::multipart::MultipartError> for Error {
	fn from(value: axum_extra::extract::multipart::MultipartError) -> Self {
		Error::Multipart(value.body_text())
	}
}

#[macro_export]
macro_rules! embed_static_files {
	($folder:expr) => {
		embed_static_files!($folder, "/", get_files);
	};
	($folder:expr, $prefix:expr) => {
		embed_static_files!($folder, $prefix, get_files);
	};

	($folder:expr, $prefix:expr, $ident:ident) => {
		#[derive(ryde::StaticFiles)]
		#[folder($folder)]
		#[prefix($prefix)]
		pub struct Assets;

		pub async fn $ident(
			uri: axum::http::Uri,
		) -> impl axum::response::IntoResponse {
			match Assets::get(uri.path()) {
				Some((content_type, bytes)) => (
					axum::http::StatusCode::OK,
					[(axum::http::header::CONTENT_TYPE, content_type)],
					bytes,
				),
				None => {
					(
						axum::http::StatusCode::NOT_FOUND,
						[(
							axum::http::header::CONTENT_TYPE,
							"text/html; charset=utf-8",
						)],
						"not found".as_bytes(),
					)
				}
			}
		}
	};
}

pub fn dotenv(s: &str) -> Result<String> {
	use std::collections::HashMap;
	use std::env;
	use std::sync::OnceLock;

	let contents: OnceLock<HashMap<String, String>> = OnceLock::new();
	let map = contents.get_or_init(|| {
		let path = env::current_dir().unwrap().join(".env");
		let contents = std::fs::read_to_string(path);
		match contents {
			Ok(contents) => contents
				.lines()
				.map(|line| line.split("=").map(|s| s.trim().trim_matches('"')))
				.filter_map(|mut s| match (s.next(), s.next()) {
					(Some(key), Some(value)) => {
						Some((key.to_string(), value.to_string()))
					}
					_ => None,
				})
				.collect::<std::collections::HashMap<String, String>>(),
			Err(_) => HashMap::default(),
		}
	});

	map.get(s)
		.cloned()
		.ok_or(Error::Io("dotenv get failed".into()))
}

#[cfg(test)]
mod tests {
	use super::dotenv;

	#[test]
	fn env_works() {
		assert_eq!(dotenv("HELLO"), Ok("WORLD".into()));
		assert!(dotenv("GOODBYE").is_err(),);
		assert_eq!(dotenv("ABC"), Ok("XYZ".into()));
	}
}
