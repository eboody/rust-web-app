mod acs;
mod error;
mod store;
mod structs;

pub use structs::*;

pub use self::error::{Error, Result};

//use crate::model::store::dbx::Dbx;
//use crate::model::store::new_db_pool;

#[derive(Clone)]
#[allow(unused)]
pub struct ModelManager {
	//dbx: Dbx,
	reqwest: reqwest::Client,
	directus: ormlite::postgres::PgPool,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		//let db_pool = new_db_pool()
		//	.await
		//	.map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
		//let dbx = Dbx::new(db_pool, false)?;
		let reqwest_client = reqwest::Client::new();

		let orm = ormlite::postgres::PgPoolOptions::new()
			.max_connections(5)
			.connect(&std::env::var("DIRECTUS_DB").unwrap())
			.await
			.unwrap();

		Ok(ModelManager {
			//dbx,
			reqwest: reqwest_client,
			directus: orm,
		})
	}

	//pub fn new_with_txn(&self) -> Result<ModelManager> {
	//	let dbx = Dbx::new(self.dbx.db().clone(), true)?;
	//	let reqwest_client = reqwest::Client::new();
	//	let orm = ormlite::postgres::PgPoolOptions::new()
	//		.max_connections(5)
	//		.connect(&std::env::var("DATABASE_URL").unwrap());
	//	Ok(ModelManager {
	//		dbx,
	//		reqwest_client,
	//		orm,
	//	})
	//}

	//pub fn dbx(&self) -> &Dbx {
	//	&self.dbx
	//}
	pub fn orm(&self) -> &ormlite::postgres::PgPool {
		&self.directus
	}

	pub fn reqwest(&self) -> &reqwest::Client {
		&self.reqwest
	}
}
