use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use std::{fmt, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: i32,
    pub j: i32
}

pub const UNIT_VECTORS: [Point; 4] = [Point{i:1, j:0},Point{i:0, j:1},Point{i:-1, j:0},Point{i:0, j:-1}];

impl Point {
    pub fn rotate_clockwise(&mut self) { (self.i, self.j) = (self.j, -self.i); }

    pub fn rotated_clockwise(&self)   -> Point { Point{i:self.j, j:-self.i} }
    pub fn rotated_widdershins(&self) -> Point { Point{i:-self.j, j:self.i} }

    pub fn step(&self, dir: &Point) -> Point { Point{i: self.i + dir.i, j: self.j + dir.j} }
    
    pub fn times(&self, scalar: i32) -> Point { Point{i: self.i*scalar, j: self.j*scalar} }

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
