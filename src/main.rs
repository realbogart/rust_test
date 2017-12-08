use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count_lines(content: &String) -> u32 {
	let lines = content.split("\n");
	return lines.count() as u32;
}

fn count_lines_in_file(filename: &String) -> u32 {
	let mut contents = String::new();

	let mut file = File::open(filename).expect("file not found");
    file.read_to_string(&mut contents).expect("could not read file");

    let linecount = count_lines(&contents);

    println!("Reading file '{}'", filename);
    println!("Number of lines: {}", linecount);

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let ref filename: String = args[1];
	count_lines_in_file(filename);
}
