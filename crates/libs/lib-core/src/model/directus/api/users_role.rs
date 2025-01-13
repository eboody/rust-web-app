/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// UsersRole : Unique identifier of the role of this user.
/// Unique identifier of the role of this user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersRole {
  String(String),
  Roles(Box<super::Roles>),
}

impl Default for UsersRole {
  fn default() -> Self {
    Self::String(Default::default())
  }
}
