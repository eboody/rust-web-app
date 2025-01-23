//! The utils module is designed to export independent sub-modules to the application code.
//!
//! Note: Even if the util sub-modules consist of a single file, they contain their own errors
//!       for improved compartmentalization.
//!
#![feature(iter_advance_by)]

pub mod b64;
pub mod envs;
pub mod retry;
pub mod time;
