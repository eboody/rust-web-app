pub fn to_alphanumeric(input: &str) -> String {
	input.chars().filter(|c| c.is_alphanumeric()).collect()
}
