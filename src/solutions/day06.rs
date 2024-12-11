use std::{collections::HashSet, io::BufRead};
use crate::common::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let mut chars: Vec<Vec<char>> = file_reader(file_path).lines().into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let start = find_starting_point(&chars);
    chars[start.i as usize][start.j as usize] = '.';
    let grid = Grid::new(&chars);
    
    (part1(&grid, &start), part2(file_path)) }

fn part1(grid: &Grid, start: &Point) -> i64 {
    let mut direction = Point{i:-1, j:0};
    let mut cur = start.clone();

    let mut seen_points: HashSet<Point> = HashSet::new();
    
    loop {
        seen_points.insert(cur);
        let maybe_next = cur.step(&direction);
        if !grid.inside(&maybe_next) {
            return seen_points.len() as i64;
        }
        if grid.get(&maybe_next) == '#' {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}

fn part2(_ile_path: &str) -> i64 { 0 }

fn find_starting_point(chars: &Vec<Vec<char>>) -> Point {
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == '^' {
                return Point{i: i as i32, j: j as i32};
            }
        }
    }
    panic!("No starting point");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point { i: i32, j: i32 }

impl Point {
    // maybe ensure unit vector?
    fn rotate_clockwise(&mut self) { (self.i, self.j) = (self.j, -self.i); }
    fn step(&self, unit_vec: &Point) ->Point { Point{i: self.i + unit_vec.i, j: self.j + unit_vec.j} }
}

struct Grid {
    grid: Vec<Vec<char>>,
    num_rows: i32,
    num_cols: i32,
}

impl Grid {
    pub fn new(grid: &Vec<Vec<char>>) -> Grid { Grid{ grid: grid.clone(), num_rows: grid.len() as i32, num_cols: grid[0].len() as i32} }

    fn inside(&self, p: &Point) -> bool { 0 <= p.i && p.i < self.num_rows && 0 <= p.j && p.j < self.num_cols }
    fn get(&self, p: &Point) -> char { self.grid[p.i as usize][p.j as usize] }
}
