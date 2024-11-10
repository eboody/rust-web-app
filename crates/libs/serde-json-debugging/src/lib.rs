//! A debug wrapper for Serde JSON deserialization that provides detailed error information.
//!
//! This crate provides a `JsonDebugDeserialize` wrapper that can be used to get more
//! detailed information about JSON deserialization errors, including the problematic field
//! and surrounding context in the JSON.
//!
//! Note: This crate is specific to JSON deserialization and won't work with other formats.

mod debug_deserialize;

pub use debug_deserialize::DebugDeserialize;
