use axum::extract::{Json, State};

use super::trigger;
use crate::prelude::*;
pub async fn on_item_update(
	State(_mm): State<ModelManager>,
	Json(trigger): Json<trigger::Body>,
) -> Result<String> {
	dbg!("trigger: {}", &trigger);
	todo!()
}
