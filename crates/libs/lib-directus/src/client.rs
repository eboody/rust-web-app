use lib_sdk::prelude::*;

pub struct DirectusClient(Client);

impl AsRef<Client> for DirectusClient {
	fn as_ref(&self) -> &Client {
		&self.0
	}
}

impl DirectusClient {
	pub fn new(client: Client) -> Self {
		Self(client)
	}
}
