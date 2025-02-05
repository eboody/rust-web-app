use crate::{Error, Result};
use axum::http::{Method, Uri};
use json::{Value, json};
use lib_automation::prelude::Uuid;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

pub async fn log_request(
  uuid: Uuid,
  req_method: Method,
  uri: Uri,
  service_error: Option<&Error>,
) -> Result<()> {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

  let error_type = service_error.map(|se| se.to_string());
  let error_data = json::to_value(service_error)
    .ok()
    .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

  // Create the RequestLogLine
  let log_line = RequestLogLine {
    uuid: uuid.to_string(),
    timestamp: timestamp.to_string(),

    req_path: uri.to_string(),
    req_method: req_method.to_string(),

    error_type,
    error_data,
  };

  info!("   ->> log_request: \n{}", json!(log_line));

  // TODO - Send to cloud-watch.

  Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
  uuid: String,      // uuid string formatted
  timestamp: String, // (should be iso8601)

  // -- http request attributes.
  req_path: String,
  req_method: String,

  error_type: Option<String>,
  error_data: Option<Value>,
}
