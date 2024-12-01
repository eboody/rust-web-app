mod config;
mod error;
mod on_docx_upload;
mod on_file_upload;
mod routes;

pub use on_file_upload::on_file_upload;

pub use config::config;

pub use error::{Error, Result};
pub use routes::routes;
