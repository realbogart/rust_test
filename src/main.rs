use std::env;
use std::fs::File;
use std::io::prelude::*;

use walkdir::WalkDir;

extern crate walkdir;

fn get_lines(content: &String) -> Vec<&str> {
	return content.split("\n").collect();
}

fn get_lines_copy(content: &String) -> Vec<String> {
	let mut lines_new: Vec<String> = Vec::new();
	let lines = get_lines(content);

	for line in lines {
		lines_new.push(String::from(line.trim()));
	}

	return lines_new;
}

fn count_lines(content: &String) -> u32 {
	return get_lines(content).iter().count() as u32;
}

fn count_lines_in_file(file_path: &String) -> u32 {
	let content = get_content(file_path);
    let linecount = count_lines(&content);
    return linecount;
}

fn get_content(file_path: &String) -> String {
	let mut content = String::new();
	let mut file = File::open(file_path).expect("file not found");
    file.read_to_string(&mut content).expect("could not read file");
    return content;
}

fn get_header_directories(file_path: &String) -> Vec<String> {
	return get_lines_copy(&get_content(file_path));
}

fn is_header(filename: &String) -> bool {
    return filename.ends_with(".h");
}

fn get_header_files(directories: &Vec<String>) -> Vec<String> {
	let mut files: Vec<String> = Vec::new();

	for directory in directories {
		let walker = WalkDir::new(directory);
		for entry in walker {
    		let entry = entry.unwrap();
    		let file_path = String::from(entry.path().to_string_lossy());

    		if is_header(&file_path) {
    			files.push(file_path);
    		}
    	}
	}

	return files;
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let ref filename: String = args[1];
	let linecount = count_lines_in_file(filename);

	println!("Reading file '{}'", filename);
    println!("Number of lines: {}", linecount);

    let header_directories = get_header_directories(&String::from("C:/programmering/rust/countlines_test/look_for_headers_here.txt"));
    println!("Header directories: {:?}", header_directories);

    let header_files = get_header_files(&header_directories);
    println!("Headers: {:?}", header_files);
}
