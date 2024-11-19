use lib_utils::envs::get_env;
use std::sync::OnceLock;

/// Retrieves a static reference to the `ElationConfig` instance.
///
/// This function initializes the configuration by loading environment variables
/// and returns a reference to a singleton `ElationConfig` instance. Subsequent calls
/// to this function will return the same instance without reinitializing it.
///
/// # Panics
///
/// This function will panic if the configuration fails to load from the environment variables.
/// The panic includes the cause of the failure.
///
/// # Example
///
/// ```rust
/// let config = client::elation_config();
/// println!("API URL: {}", config.ELATION_API_URL);
/// ```
pub fn elation_config() -> &'static ElationConfig {
	static INSTANCE: OnceLock<ElationConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		ElationConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

/// Configuration for the Elation EMR API.
///
/// The `ElationConfig` struct holds the configuration parameters required
/// to interact with the Elation EMR API. It includes the API base URL and
/// the URL of the token service used for authentication.
///
/// # Fields
///
/// * `ELATION_API_URL` - The base URL of the Elation EMR API.
/// * `TOKEN_SERVICE_URL` - The URL of the token service to obtain access tokens.
///
/// # Example
///
/// ```rust
/// let config = client::elation_config();
/// println!("API URL: {}", config.ELATION_API_URL);
/// println!("Token Service URL: {}", config.TOKEN_SERVICE_URL);
/// ```
#[allow(non_snake_case)]
pub struct ElationConfig {
	/// The base URL of the Elation EMR API.
	pub ELATION_API_URL: String,
	/// The URL of the token service used to obtain access tokens.
	pub TOKEN_SERVICE_URL: String,
}

/// Configuration for the Directus API.
///
///	The `DirectusConfig` struct holds the configuration parameters required
///	to interact with the Directus API. It includes the API base URL, email, and password.
///
///	# Fields
///
///	* `DIRECUTS_API_URL` - The base URL of the Directus API.
///	* `DIRECUTS_PASSWORD` - Password for the Directus API.
///	* `DIRECUTS_EMAIL` - Email for the Directus API.
///
///	# Example
///
///	```rust
///	let config = client::directus_config();
///	println!("API URL: {}", config.DIRECUTS_API_URL);
///	println!("Password: {}", config.DIRECUTS_PASSWORD);
///	println!("Email: {}", config.DIRECUTS_EMAIL);
///	```
#[allow(non_snake_case)]
pub struct DirectusConfig {
	/// The base URL of the Elation EMR API.
	pub DIRECUTS_API_URL: String,

	/// Password for the Directus API.
	pub DIRECUTS_PASSWORD: String,

	/// Email for the Directus API.
	pub DIRECUTS_EMAIL: String,
}

pub fn directus_config() -> &'static ElationConfig {
	static INSTANCE: OnceLock<DirectusConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		DirectusConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

impl DirectusConfig {
	/// Loads the `DirectusConfig` from environment variables.
	/// This function reads the required configuration parameters from the environment
	/// using the `get_env` function. It returns an `DirectusConfig` instance populated
	/// with the values from the environment.
	/// # Errors
	/// Returns an error if any of the required environment variables are not set.
	/// # Environment Variables
	/// * `DIRECUTS_API_URL` - Must be set to the base URL of the Directus API.
	/// * `DIRECUTS_PASSWORD` - Must be set to the password for the Directus API.
	/// * `DIRECUTS_EMAIL` - Must be set to the email for the Directus API.
	/// # Example
	/// ```rust
	/// // Ensure that the environment variables are set:
	/// // export DIRECUTS_API_URL="https://api.directus.com"
	/// // export DIRECUTS_PASSWORD="password"
	/// // export DIRECUTS_EMAIL="email"
	/// let config = client::DirectusConfig::load_from_env().unwrap();
	/// println!("API URL: {}", config.DIRECUTS_API_URL);
	/// println!("Password: {}", config.DIRECUTS_PASSWORD);
	/// println!("Email: {}", config.DIRECUTS_EMAIL);
	/// ```
	pub fn load_from_env() -> lib_utils::envs::Result<DirectusConfig> {
		Ok(DirectusConfig {
			DIRECUTS_API_URL: get_env("DIRECUTS_API_URL")?,
			DIRECUTS_PASSWORD: get_env("DIRECUTS_PASSWORD")?,
			DIRECUTS_EMAIL: get_env("DIRECUTS_EMAIL")?,
		})
	}
}

impl ElationConfig {
	/// Loads the `ElationConfig` from environment variables.
	///
	/// This function reads the required configuration parameters from the environment
	/// using the `get_env` function. It returns an `ElationConfig` instance populated
	/// with the values from the environment.
	///
	/// # Errors
	///
	/// Returns an error if any of the required environment variables are not set.
	///
	/// # Environment Variables
	///
	/// * `ELATION_API_URL` - Must be set to the base URL of the Elation EMR API.
	/// * `TOKEN_SERVICE_URL` - Must be set to the URL of the token service.
	///
	/// # Example
	///
	/// ```rust
	/// // Ensure that the environment variables are set:
	/// // export ELATION_API_URL="https://api.elationemr.com"
	/// // export TOKEN_SERVICE_URL="https://token.elationemr.com"
	///
	/// let config = client::ElationConfig::load_from_env().unwrap();
	/// println!("API URL: {}", config.ELATION_API_URL);
	/// ```
	pub fn load_from_env() -> lib_utils::envs::Result<ElationConfig> {
		Ok(ElationConfig {
			ELATION_API_URL: get_env("ELATION_API_URL")?,
			TOKEN_SERVICE_URL: get_env("TOKEN_SERVICE_URL")?,
		})
	}
}
