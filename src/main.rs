use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate walkdir;
extern crate regex;

use regex::Regex;
use walkdir::WalkDir;

fn get_lines(content: &String) -> Vec<&str> {
	return content.split("\n").collect();
}

fn get_lines_copy(content: &String) -> Vec<String> {
	let lines: Vec<&str> = get_lines(content);
	let mut lines_new: Vec<String> = Vec::new();

	for line in lines {
		lines_new.push(line.trim().to_owned());
	}

	return lines_new;
}

fn count_lines(content: &String) -> u32 {
	return get_lines(content).iter().count() as u32;
}

fn file_get_content(file_path: &String) -> String {
	let mut content = String::new();

	let mut file = File::open(file_path).expect("file not found");
    file.read_to_string(&mut content).expect("could not read file");

    return content;
}

fn count_lines_in_file(file_path: &String) -> u32 {
	let content = file_get_content(file_path);
	let linecount = count_lines(&content);
    return linecount;
}

fn unwrap_content(content: &String) -> Vec<&str> {
	return get_lines(&content);
}

fn get_header_path(header_files: &Vec<String>, checked_headers: &Vec<String>, include: &str) -> (String, bool) {
	for header in header_files {
		if header.contains(include) {
			for checked_header in checked_headers {
				if checked_header == header {
					return (String::from("// ALREADY INCLUDED"), false);
				}
			}

			return (header.to_owned(), true);
		}
	}
	return (String::from("// COULD NOT FIND INCLUDE FILE"), false);
}

fn unwrap_file(file_path: &String, header_files: &Vec<String>, mut checked_headers: &mut Vec<String>, mut content_out: &mut String) {
	println!("Unwrapping: {}", file_path);

	let content = file_get_content(file_path);
	let lines = unwrap_content(&content);

	let regex = Regex::new("#include[ \t]*\"([a-zA-Z/_]*\\.h)\"").unwrap();

	for line in lines {
		if regex.is_match(line) {
			let captures = regex.captures(line).unwrap();
			let include = captures.get(1).unwrap().as_str();
			let (include_full_path, found_include) = get_header_path(&header_files, &checked_headers, &include);

			if found_include {
				checked_headers.push(include_full_path.to_owned());
				unwrap_file(&include_full_path, &header_files, &mut checked_headers, &mut content_out);
			}
			else {
			    content_out.push_str(&include_full_path);
			}
		}
		else {
		    content_out.push_str(line);
		}

		content_out.push('\n');
	}

	//println!("Success!");

	content_out.pop(); // TODO: Fix this hack
}

fn is_header(filename: &String) -> bool {
	return filename.ends_with(".h");
}

fn get_header_files(directories: &Vec<String>) -> Vec<String> {
	let mut files: Vec<String> = Vec::new();

	for directory in directories {
		let walker = WalkDir::new(directory.trim());
		for entry in walker {
   	 		let entry = entry.unwrap();
    		let filename = String::from(entry.path().to_string_lossy());

    		if is_header(&filename){
    			files.push(String::from(filename.replace("\\", "/")));
    		}
    	}
	}

	return files;
}

fn get_header_paths(file: &String) -> Vec<String> {
	let content = file_get_content(file);
	let lines = get_lines_copy(&content);
	return lines;
}

fn write_to_file(filename: &String, buf: &[u8]) {
	let mut file = match File::create(filename) {
		Err(err) => panic!("Could not create file {}: {}", filename, err),
		Ok(file) => file,
	};

	file.write_all(buf).expect("Could not write to file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let ref file_path: String = args[1];
	let line_count = count_lines_in_file(file_path);

	println!("Reading file '{}'", file_path);
    println!("Number of lines: {}", line_count);

    let header_paths = get_header_paths(&String::from("C:/rust/countlines_data/look_for_headers_here.txt"));
    println!("Header paths: {:?}", header_paths);

    let header_files = get_header_files(&header_paths);
    println!("Files: {:?}", header_files);

    let mut content = String::new();
    let mut checked_headers: Vec<String> = Vec::new();
    unwrap_file(file_path, &header_files, &mut checked_headers, &mut content);

    write_to_file(&String::from("out.cpp"), content.as_bytes());
}
