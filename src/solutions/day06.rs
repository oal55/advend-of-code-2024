use std::{collections::HashMap, io::BufRead};
use crate::common::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let chars: Vec<Vec<char>> = file_reader(file_path).lines().into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let start = find_starting_point(&chars);
    let grid = Grid::new(&chars);

    let seen_points = walk_out_of_grid(&grid, &start);

    let mut num_loops = 0;
    for (point, _) in &seen_points {
        if has_loop(&grid, &start, point) {
            num_loops += 1;
        }
    }
    
    return (seen_points.len() as i64, num_loops)
}

fn has_loop(grid: &Grid, start: &Point, extra: &Point) -> bool {
    let mut direction = Point{i:-1, j:0};
    let mut cur = start.clone();

    let mut steps = 0;
    loop {
        steps += 1;
        if steps == 67605 {
            return true;
        }
        let maybe_next = cur.step(&direction);
        if !grid.inside(&maybe_next) {
            return false;
        }
        if grid.get(&maybe_next) == '#' || maybe_next == *extra {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}

fn walk_out_of_grid(grid: &Grid, start: &Point) -> HashMap<Point, Point> {
    let mut direction = Point{i:-1, j:0};
    let mut cur = start.clone();

    let mut seen_points: HashMap<Point, Point> = HashMap::new(); // coord -> dir

    loop {
        seen_points.insert(cur.clone(), direction.clone()); // this clone necessary?
        
        let maybe_next = cur.step(&direction);
        if !grid.inside(&maybe_next) {
            return seen_points;
        }
        if grid.get(&maybe_next) == '#' {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}


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
