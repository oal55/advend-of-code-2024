use std::io::{BufRead, BufReader};    
use std::fs::{self, File};

pub fn file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap_or_else(|_| panic!("Cannot open file at: {}", file_path));
    BufReader::new(file)
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Cannot read file at: {}", file_path))
}

pub fn read_chars_grid(file_path: &str) -> Vec<Vec<char>> {
    file_reader(file_path).lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}
