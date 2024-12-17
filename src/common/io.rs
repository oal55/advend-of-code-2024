use std::fs::{self, File};
use std::io::BufReader;    

pub fn file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap_or_else(|_| panic!("Cannot open file at: {}", file_path));
    BufReader::new(file)
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Cannot read file at: {}", file_path))
}
