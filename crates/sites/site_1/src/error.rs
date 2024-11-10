pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	//Ryde(ryde::Error),
	Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
	fn from(e: reqwest::Error) -> Self {
		Error::Reqwest(e)
	}
}

//impl From<ryde::Error> for Error {
//	fn from(e: ryde::Error) -> Self {
//		Error::Ryde(e)
//	}
//}

impl From<Error> for axum::Error {
	fn from(e: Error) -> Self {
		axum::Error::new(e)
	}
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
