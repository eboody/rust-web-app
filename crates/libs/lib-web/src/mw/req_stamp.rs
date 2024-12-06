use crate::error::{Error, Result};
use axum::{
	async_trait,
	body::Body,
	extract::FromRequestParts,
	http::{request::Parts, Request},
	middleware::Next,
	response::Response,
};
use lib_utils::time::now_utc;
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReqStamp {
	pub uuid: Uuid,
	pub time_in: OffsetDateTime,
}

pub async fn mw_req_stamp_resolver(
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	debug!("{:<12} - mw_req_stamp_resolver", "MIDDLEWARE");

	let time_in = now_utc();
	let uuid = Uuid::new_v4();

	req.extensions_mut().insert(ReqStamp { uuid, time_in });

	Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ReqStamp {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - ReqStamp", "EXTRACTOR");

		parts
			.extensions
			.get::<ReqStamp>()
			.cloned()
			.ok_or(Error::ReqStampNotInReqExt)
	}
}
