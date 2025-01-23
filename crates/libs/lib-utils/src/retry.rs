use derive_more::derive::From;
use reqwest::RequestBuilder;
use serde::{Serialize, de::DeserializeOwned};
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use serde_with::{DisplayFromStr, serde_as};

pub trait RetryableRequest {
    fn retry(self) -> Retryable;
}

pub struct Retryable {
    builder: RequestBuilder,
}

impl Retryable {
    // Retry and deserialize the response into the desired type
    pub async fn send<T: DeserializeOwned + 'static>(self) -> Result<T, Error> {
        let mut attempt = 0;

        let retry_strategy = ExponentialBackoff::from_millis(10)
            .map(jitter as fn(std::time::Duration) -> std::time::Duration)
            .take(10);

        let mut retry_times = retry_strategy.clone().peekable();

        Retry::spawn(retry_strategy, || {
            let builder = self
                .builder
                .try_clone()
                .expect("RequestBuilder not clonable");

            attempt += 1;

            if attempt > 1 {
                if let Some(next_wait) = retry_times.peek() {
                    println!(
                        "Retry attempt: {}. Next retry in: {}ms",
                        attempt,
                        next_wait.as_millis()
                    );
                } else {
                    println!("Retry attempt: {}. No more retries.", attempt);
                }
            }

            let _ = retry_times.advance_by(1);

            async move {
                let res = builder.send().await.map_err(Error::Reqwest)?;

                let body = res.text().await.map_err(Error::Reqwest)?;

                if body.len() < 500 {
                    println!("body: {:#?}", body);
                } else {
                    println!("body: {:#?}", &body[..500]);
                }

                //eprintln!("Retrying request due to status code: {}", status);
                //return Err(Error::Blah);

                // Handle `String` specifically
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() {
                    return Ok(serde_plain::from_str::<T>(&body).expect("Failed to deserialize"));
                }

                // Deserialize JSON into the generic type T
                let result: T = json::from_str(&body).map_err(Error::Serde)?;
                Ok(result)
            }
        })
        .await
    }
}

impl RetryableRequest for RequestBuilder {
    fn retry(self) -> Retryable {
        Retryable { builder: self }
    }
}

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    Blah,
    #[from]
    Reqwest(#[serde_as(as = "DisplayFromStr")] reqwest::Error),
    #[from]
    Serde(#[serde_as(as = "DisplayFromStr")] json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Blah => write!(f, "Blah"),
            Error::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            Error::Serde(e) => write!(f, "Serde Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
