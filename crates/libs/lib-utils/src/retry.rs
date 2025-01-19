use reqwest::{RequestBuilder, Response};
use std::iter::{Map, Take};
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

pub trait RetryableRequest {
    fn retry(self) -> Retryable;
}

pub struct Retryable {
    builder: RequestBuilder,
}

impl Retryable {
    pub async fn send(self) -> reqwest::Result<Response> {
        let retry_strategy = ExponentialBackoff::from_millis(1000)
            .map(jitter as fn(std::time::Duration) -> std::time::Duration)
            .take(10);

        Retry::spawn(retry_strategy, || {
            let builder = self
                .builder
                .try_clone()
                .expect("RequestBuilder not clonable");

            async move {
                let res = builder.send().await;

                if let Ok(response) = &res {
                    if response.status().is_success() {
                        return Ok(response);
                    }
                } else {
                    println!("Request failed: {:?}", response);
                    Ok(())
                }
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

pub type RetryStrategy =
    Take<Map<ExponentialBackoff, fn(std::time::Duration) -> std::time::Duration>>;
