pub mod base_service;
pub mod error;
mod macros;
mod resource;
mod resource_service;

pub use crate::impl_service;
pub use base_service::BaseService;
pub use error::*;
pub use resource::*;
pub use resource_service::*;
