use std::fs::File;
use std::io::{self, Write};

fn main() {
	//// Parse command-line arguments using clap
	//let matches = App::new("Split Structs")
	//	.version("1.0")
	//	.author("Your Name <your.email@example.com>")
	//	.about(
	//		"Splits a Rust file containing multiple structs into individual files",
	//	)
	//	.arg(
	//		Arg::new("input")
	//			.short('i')
	//			.long("input")
	//			.value_name("FILE")
	//			.about("Specifies the input file")
	//			.required(true)
	//			.takes_value(true),
	//	)
	//	.arg(
	//		Arg::new("output")
	//			.short('o')
	//			.long("output")
	//			.value_name("DIRECTORY")
	//			.about("Specifies the output directory")
	//			.required(true)
	//			.takes_value(true),
	//	)
	//	.get_matches();
	//
	//// Get the input file and output directory from the arguments
	//let input_file = matches.value_of("input").unwrap();
	//let output_dir = matches.value_of("output").unwrap();
	//
	//// Ensure the output directory exists
	//if !Path::new(output_dir).exists() {
	//	fs::create_dir_all(output_dir)?;
	//}
	//
	//// Open the input file
	//let file = File::open(input_file)?;
	//let reader = BufReader::new(file);
	//
	//// Variables to hold state
	//let mut current_struct: Option<String> = None;
	//let mut current_lines: Vec<String> = Vec::new();
	//
	//// Process each line
	//for line in reader.lines() {
	//	let line = line?;
	//
	//	// Check if the line defines a new struct
	//	if line.starts_with("#[derive(") || line.starts_with("pub struct ") {
	//		// If we're in the middle of a struct, save it to a file
	//		if let Some(struct_name) = &current_struct {
	//			save_to_file(output_dir, struct_name, &current_lines)?;
	//			current_struct = None;
	//			current_lines.clear();
	//		}
	//
	//		// Extract the struct name
	//		if line.starts_with("pub struct ") {
	//			if let Some(struct_name) = line.split_whitespace().nth(2) {
	//				current_struct = Some(struct_name.trim().to_string());
	//			}
	//		}
	//	}
	//
	//	// Append the line to the current struct's lines
	//	if current_struct.is_some() {
	//		current_lines.push(line);
	//	}
	//}
	//
	//// Save the last struct if any
	//if let Some(struct_name) = current_struct {
	//	save_to_file(output_dir, &struct_name, &current_lines)?;
	//}
}

// Helper function to save the current struct to a file
#[allow(dead_code)]
fn save_to_file(
	output_dir: &str,
	struct_name: &str,
	lines: &[String],
) -> io::Result<()> {
	let file_path = format!("{}/{}.rs", output_dir, struct_name);
	let mut file = File::create(file_path)?;
	for line in lines {
		writeln!(file, "{}", line)?;
	}
	Ok(())
}
