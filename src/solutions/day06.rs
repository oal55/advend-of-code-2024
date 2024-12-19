use std::collections::HashMap;

use crate::common::{Grid, Point};
use crate::common::io::read_file_as_2d_chars;

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_cells(read_file_as_2d_chars(&file_path));
    let start = grid.find_single(&'^');

    let seen_points = walk_out_of_grid(&grid, &start);

    let mut num_loops = 0;
    for point in seen_points.keys() {
        if has_loop(&grid, &start, point) {
            num_loops += 1;
        }
    }
    
    (seen_points.len() as i64, num_loops)
}

fn has_loop(grid: &Grid<char>, start: &Point, extra: &Point) -> bool {
    let mut direction = Point{i:-1, j:0};
    let mut cur = *start;

    let mut steps = 0;
    loop {
        steps += 1;
        if steps == 67605 {
            return true;
        }
        let maybe_next = cur.step(&direction);
        if !grid.contains(&maybe_next) {
            return false;
        }
        if *grid.get(&maybe_next) == '#' || maybe_next == *extra {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}

fn walk_out_of_grid(grid: &Grid<char>, start: &Point) -> HashMap<Point, Point> {
    let mut direction = Point{i:-1, j:0};
    let mut cur = *start;

    let mut seen_points: HashMap<Point, Point> = HashMap::new(); // coord -> dir

    loop {
        seen_points.insert(cur, direction);
        
        let maybe_next = cur.step(&direction);
        if !grid.contains(&maybe_next) {
            return seen_points;
        }
        if *grid.get(&maybe_next) == '#' {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}
