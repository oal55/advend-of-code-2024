use std::fs::{self, File};
use std::io::BufReader;

pub fn file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect(format!("Cannot open file at: {}", file_path).as_str());
    return BufReader::new(file);
}

pub fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect(format!("Cannot read file at: {}", file_path).as_str());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: i32,
    pub j: i32
}

impl Point {
    // maybe ensure unit vector?
    pub fn rotate_clockwise(&mut self) { (self.i, self.j) = (self.j, -self.i); }
    pub fn step(&self, unit_vec: &Point) ->Point { Point{i: self.i + unit_vec.i, j: self.j + unit_vec.j} }
}
