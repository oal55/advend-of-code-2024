use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use crate::common::{Grid, Point};
use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (usize, usize) {
    let cells = file_reader(file_path).lines()
        .map(|line| line.unwrap().chars()
            .map(|c| match c {
                '.' => -5,
                _ => c.to_digit(10).unwrap() as i32
            })
            .collect::<Vec<i32>>()
        )
        .collect();
    let field: Grid<i32> = Grid::new_from_cells(cells);

    let mut visited = Vec2D::new(field.num_rows, field.num_cols, || false);
    let mut accessible_trail_ends = Vec2D::new(field.num_rows, field.num_cols, HashSet::<Point>::new);
    let mut num_trails = Vec2D::new(field.num_rows, field.num_cols, || 0);
    let mut queue: VecDeque<Point> = VecDeque::new();

    for p in field.find(&9) {
        accessible_trail_ends.get_mut(&p).insert(p);
        num_trails.set(&p, 1);
        visited.set(&p, true);
        queue.push_back(p);
    }

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let summits_from_cur = accessible_trail_ends.get(&cur).clone();
        let num_trails_from_cur = *num_trails.get(&cur);
        for n in cur.neighbors() {
            if field.contains(&n) && *field.get(&n) + 1 == *field.get(&cur) {
                // we get to add summits this way because all "trails" are same length
                accessible_trail_ends.get_mut(&n).extend(&summits_from_cur);
                *(num_trails.get_mut(&n)) += num_trails_from_cur;

                if !visited.get(&n) {
                    visited.set(&n, true);
                    queue.push_back(n);
                }
            }
        }
    }

    let (mut total_score, mut total_num_trails) = (0usize, 0usize);
    for p in field.find(&0) {
        total_score += accessible_trail_ends.get(&p).len();
        total_num_trails += num_trails.get(&p);
    }
    (total_score, total_num_trails)
}

struct Vec2D<T>(Vec<Vec<T>>);
impl <T>Vec2D<T> {
    // maybe accept a cloneable T arg instead of a T supplier?
    fn new(num_rows: i32, num_cols: i32, zero_t: fn() -> T) -> Vec2D<T>{
        Vec2D((0..num_rows).map(|_| (0..num_cols).map(|_| zero_t()).collect()).collect())
    }
    
    fn get(&self, p: &Point) -> &T { &self.0[p.i as usize][p.j as usize] }

    fn set(&mut self, p: &Point, val: T) { self.0[p.i as usize][p.j as usize] = val; }

    fn get_mut(&mut self, p: &Point) -> &mut T { &mut self.0[p.i as usize][p.j as usize] }
}
