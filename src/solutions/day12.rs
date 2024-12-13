use std::collections::HashSet;
use std::io::BufRead;
use crate::common::file_reader;
use crate::common::Point;

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_file(file_path);
    let mut visited: HashSet<Point> = HashSet::new();

    let mut sum_cost = 0;
    for i in 0..grid.num_rows {
        for j in 0..grid.num_cols {
            let p = Point{i, j};
            if !visited.contains(&p) {
                let (mut size, mut perim) = (0i64, 0i64);
                span_region(&grid, grid.get(&p), &p, &mut visited, &mut size, &mut perim);

                sum_cost += size * perim;
            }
        }
    }
    return (sum_cost, 0);
}

fn span_region(grid: &Grid, cur_region_label: char, p: &Point, visited: &mut HashSet<Point>, size: &mut i64, perim: &mut i64) {
    if visited.contains(p) {
        return;
    }
    visited.insert(*p);
    *size += 1;

    for neigbor in p.neighbors() {
        if !grid.contains(&neigbor) || grid.get(&neigbor) != cur_region_label {
            *perim += 1
        } else {
            span_region(grid, cur_region_label, &neigbor, visited, size, perim);
        }
    }
}


struct Grid {
    grid: Vec<Vec<char>>,
    num_rows: i32,
    num_cols: i32,
}

impl Grid {
    pub fn new_from_file(file_path: &str) -> Grid {
        let grid: Vec<Vec<char>> = file_reader(file_path).lines().into_iter()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        return Grid{grid, num_rows: num_rows as i32, num_cols: num_cols as i32}
    }

    fn contains(&self, p: &Point) -> bool { 0 <= p.i && p.i < self.num_rows && 0 <= p.j && p.j < self.num_cols }
    fn get(&self, p: &Point) -> char { self.grid[p.i as usize][p.j as usize] }
}
