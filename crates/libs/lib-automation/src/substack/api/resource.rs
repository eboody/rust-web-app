pub trait Resource {
	type Id: ToString;

	/// Returns the API endpoint for the resource.
	fn endpoint() -> &'static str;
}
