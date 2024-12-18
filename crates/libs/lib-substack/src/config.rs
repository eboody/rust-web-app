use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::OnceLock;

pub fn config() -> &'static SubstackConfig {
  static INSTANCE: OnceLock<SubstackConfig> = OnceLock::new();

  INSTANCE.get_or_init(|| {
    SubstackConfig::load_from_env()
      .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
  })
}

#[allow(non_snake_case)]
pub struct SubstackConfig {
  pub HEADERS: HeaderMap,
  pub API_URL: String,
  pub PUBLICATION_ID: i64,
}

impl SubstackConfig {
  fn load_from_env() -> lib_utils::envs::Result<SubstackConfig> {
    let mut substack_headers = reqwest::header::HeaderMap::new();
    substack_headers.insert("accept", "application/json".parse().unwrap());
    substack_headers.insert("content-type", "application/json".parse().unwrap());
    substack_headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".parse().unwrap());
    substack_headers.insert(
      "cookie",
      HeaderValue::from_str(
        std::env::var("SUBSTACK_COOKIE")
          .expect("SUBSTACK_COOKIE not found")
          .as_str(),
      )
      .map_err(|_| lib_utils::envs::Error::WrongFormat("SUBSTACK_COOKIE"))?,
    );

    Ok(SubstackConfig {
      HEADERS: substack_headers,
      API_URL: std::env::var("SUBSTACK_API_URL")
        .expect("SUBSTACK_API_URL not found"),
      PUBLICATION_ID: std::env::var("SUBSTACK_PUBLICATION_ID")
        .unwrap()
        .parse::<i64>()
        .unwrap(),
    })
  }
}
