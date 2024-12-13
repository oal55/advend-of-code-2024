use std::{collections::{HashSet, VecDeque}, io::BufRead};
use crate::common::{Point, file_reader};


pub fn run(file_path: &str) -> (usize, usize) {
    let field = Field::new_from_file(file_path);
    
    let mut visited = Vec2D::new(field.num_rows, field.num_cols, || false);
    let mut accessible_trail_ends = Vec2D::new(field.num_rows, field.num_cols, || HashSet::<Point>::new());
    let mut num_trails = Vec2D::new(field.num_rows, field.num_cols, || 0);
    let mut queue: VecDeque<Point> = VecDeque::new();

    for p in field.get_cells_of_height(9) {
        accessible_trail_ends.get_mut(&p).insert(p);
        num_trails.set(&p, 1);
        visited.set(&p, true);
        queue.push_back(p);
    }

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        let summits_from_cur = accessible_trail_ends.get(&cur).clone();
        let num_trails_from_cur = num_trails.get(&cur).clone();
        for n in cur.neighbors() {
            if field.contains(&n) && field.get(&n) + 1 == field.get(&cur) {
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
    for p in field.get_cells_of_height(0) {
        total_score += accessible_trail_ends.get(&p).len();
        total_num_trails += num_trails.get(&p);
    }
    return (total_score, total_num_trails);
}

struct Field {
    heights: Vec<Vec<i32>>, // we'll map dots to -5 for convenience.
    num_rows: i32,
    num_cols: i32,
}

impl Field {
    fn new_from_file(file_path: &str) -> Field {
        let heights: Vec<Vec<i32>> = file_reader(file_path).lines().into_iter()
        .map(|line| line.unwrap().chars()
            .map(|c| match c {
                '.' => -5,
                _ => c.to_digit(10).unwrap() as i32
            })
            .collect::<Vec<i32>>()
        )
        .collect();
        let (num_rows, num_cols) = (heights.len() as i32, heights[0].len() as i32);
        return Field{heights, num_rows, num_cols}
    }
    
    fn get(&self, p: &Point) -> i32 { self.heights[p.i as usize][p.j as usize] }

    fn contains(&self, p: &Point) -> bool { 0 <= p.i && p.i < self.num_rows && 0 <= p.j && p.j < self.num_cols }

    fn get_cells_of_height(&self, target_height: i32) -> Vec<Point> {
        return self.heights.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                .filter(|(_, &height)| height == target_height)
                .map(move |(j, _)| Point{i: i as i32, j: j as i32})
            )
            .collect();
    }

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
