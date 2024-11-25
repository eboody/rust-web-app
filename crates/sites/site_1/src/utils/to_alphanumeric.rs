#[allow(dead_code)]
pub fn to_alphanumeric(input: &str) -> String {
	input.chars().filter(|c| c.is_alphanumeric()).collect()
}
