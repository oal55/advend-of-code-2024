use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use std::{fmt, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: i32,
    pub j: i32
}

pub struct Direction;
impl Direction {
    pub const UP: Point     = Point{i:-1, j:0};
    pub const RIGHT: Point  = Point{i:0, j:1};
    pub const DOWN: Point   = Point{i:1, j:0};
    pub const LEFT: Point   = Point{i:0, j:-1};
    pub const UP_RIGHT: Point = Point{i: -1, j: 1};
    pub const UP_LEFT: Point = Point{i: -1, j: -1};
    pub const DOWN_RIGHT: Point = Point{i: 1, j: 1};
    pub const DOWN_LEFT: Point = Point{i: 1, j: -1};

    // Orthogonal 4 dirs. Top, right, down, left.
    pub const ORTHOGONAL_DIRS: [Point; 4] = [
        Direction::UP,
        Direction::RIGHT,
        Direction::DOWN,
        Direction::LEFT,
    ];

    pub const ALL_DIRS: [Point; 8] = [
        Direction::UP,
        Direction::UP_RIGHT,
        Direction::RIGHT,
        Direction::DOWN_RIGHT,
        Direction::DOWN,
        Direction::DOWN_LEFT,
        Direction::LEFT,
        Direction::UP_LEFT,
    ];
}


impl Point {
    pub fn new(i: i32, j: i32) -> Self { Self{i, j} }

    pub fn rotate_clockwise(&mut self) { (self.i, self.j) = (self.j, -self.i); }

    pub fn rotated_clockwise(&self) -> Self { Self{i:self.j, j:-self.i} }

    pub fn rotated_widdershins(&self) -> Self { Self{i:-self.j, j:self.i} }

    pub fn step(&self, dir: &Self) -> Self { Self{i: self.i + dir.i, j: self.j + dir.j} }

    pub fn times(&self, scalar: i32) -> Self { Self{i: self.i*scalar, j: self.j*scalar} }

    pub fn d_manhattan(&self, other: &Self) -> i32 { (self.i - other.i).abs() + (self.j - other.j).abs() }

    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point{i: self.i-1, j: self.j},
            Point{i: self.i, j: self.j+1},
            Point{i: self.i+1, j: self.j},
            Point{i: self.i, j: self.j-1}
        ]
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
