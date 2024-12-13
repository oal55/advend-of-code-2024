use std::fs::{self, File};
use std::io::BufReader;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use std::fmt;

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
    pub fn step(&self, unit_vec: &Point) -> Point { Point{i: self.i + unit_vec.i, j: self.j + unit_vec.j} }
    pub fn neighbors(&self) -> Vec<Point> {
        return vec![
            Point{i: self.i-1, j: self.j},
            Point{i: self.i, j: self.j+1},
            Point{i: self.i+1, j: self.j},
            Point{i: self.i, j: self.j-1}
        ];
    }
}


impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self {i: self.i + other.i, j: self.j + other.j} }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.i += other.i;
        self.j += other.j;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self {i: self.i - other.i, j: self.j - other.j} }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.i -= other.i;
        self.j -= other.j;
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output { Self { i: -self.i, j: -self.j} }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({},{})", self.i, self.j) }
}
