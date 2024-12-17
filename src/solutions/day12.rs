use std::collections::HashSet;
use std::io::BufRead;

use crate::common::io::file_reader;
use crate::common::{Point, UNIT_VECTORS};

#[derive(Debug)]
struct RegionData {
    size: i64,
    perim: i64,
    num_corners: i64
}
impl RegionData {
    fn new() -> RegionData              { RegionData{size:0, perim:0, num_corners: 0} }
    fn cost(&self) -> i64               { self.size * self.perim }
    fn cost_bulk_discount(&self) -> i64 { self.size * self.num_corners }
}

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_file(file_path);
    let mut visited: HashSet<Point> = HashSet::new();

    let (mut sum_cost, mut sum_discount_cost) = (0, 0);
    for i in 0..grid.num_rows {
        for j in 0..grid.num_cols {
            let p = Point{i, j};
            if !visited.contains(&p) {
                let mut data = RegionData::new();
                span_region(&grid, grid.get(&p), &p, &mut visited, &mut data);

                sum_cost += data.cost();
                sum_discount_cost += data.cost_bulk_discount();
                // println!("Spanned region at: {:?} has stats: {:?}", p, data);
            }
        }
    }
    return (sum_cost, sum_discount_cost);
}

fn has_convex_corner(grid: &Grid, region_label: char, coordinate: &Point, dir: &Point) -> bool {
    let neighbor_1 = *coordinate + *dir;
    let neighbor_2 = *coordinate + dir.rotated_clockwise();
    return
        (!grid.contains(&neighbor_1) || grid.get(&neighbor_1) != region_label) &&
        (!grid.contains(&neighbor_2) || grid.get(&neighbor_2) != region_label);
}

/*
Concave corner cells will have 2 friendly neighbors, and a foreign diagonal one
 _____
|x x x|
|x|‾‾‾    
 ‾
If the top-left x is called with dir == (0, 1), we'll pick up the concave corner to its bottom right.
*/
fn has_concave_corner(grid: &Grid, region_label: char, coordinate: &Point, dir: &Point) -> bool {
    let neighbor_1 = *coordinate + *dir;
    let neighbor_2 = *coordinate + dir.rotated_clockwise();
    let neighbor_3 = *coordinate + *dir + dir.rotated_clockwise();
    return
        (grid.contains(&neighbor_1) && grid.get(&neighbor_1) == region_label) &&
        (grid.contains(&neighbor_2) && grid.get(&neighbor_2) == region_label) &&
        (grid.get(&neighbor_3) != region_label);
}

fn span_region(grid: &Grid, region_label: char, p: &Point, visited: &mut HashSet<Point>, stats: &mut RegionData) {
    if visited.contains(p) {
        return;
    }
    visited.insert(*p);
    stats.size += 1;
    for dir in &UNIT_VECTORS {
        // we are not under counting as these two calls can never both be true. A corner cannot be concave and convex at the same time
        if has_convex_corner(grid, region_label, p, dir) || has_concave_corner(grid, region_label, p, dir) {
            stats.num_corners += 1;
        }
    }

    for neigbor in p.neighbors() {
        if !grid.contains(&neigbor) || grid.get(&neigbor) != region_label {
            stats.perim += 1
        } else {
            span_region(grid, region_label, &neigbor, visited, stats);
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
