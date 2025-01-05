use std::iter::{zip, once, repeat_n};

use crate::common::io::read_file_as_2d_chars;
use crate::common::{Grid, Point, Direction};

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_cells(read_file_as_2d_chars(file_path));
    (part1(&grid), part2(&grid))
}

fn part1(grid: &Grid<char>) -> i64 {
    let mut res =0;
    let zero = Point::new(0,0);
    
    let x_locations = grid.find(&'X');
    let word = "XMAS".chars().collect::<Vec<_>>();
    for direction in Direction::ALL_DIRS {
        let diffs :Vec<Point> =  once(zero).chain(repeat_n(direction, 3)).collect();
        res += x_locations.iter()
            .filter(|&p| matches_at(&grid, *p, &word, &diffs))
            .count() as i64;
    }
    res
}

fn part2(grid: &Grid<char>) -> i64 {
    let diffs = vec![Point::new(0, 0), Point::new(-1, -1), Point::new(0, 2), Point::new(2, 0), Point::new(0, -2)];
    let words: Vec<Vec<char>> = vec![
        "AMMSS".chars().collect(),
        "AMSSM".chars().collect(),
        "ASSMM".chars().collect(),
        "ASMMS".chars().collect(),
    ];
    let a_locations = grid.find(&'A');
    let mut res =0;
    for word in &words {
        res += a_locations.iter()
            .filter(|&p| matches_at(&grid, *p, &word, &diffs))
            .count() as i64;
    }
    res
}

fn matches_at(grid: &Grid<char>, start_pos: Point, word: &Vec<char>, diffs: &Vec<Point>) -> bool {
    let mut cur_pos = start_pos;
    for (&letter, d_pos) in zip(word, diffs) {
        cur_pos += *d_pos;
        if !grid.contains(&cur_pos) || letter != *grid.get(&cur_pos) {
            return false;
        }
    }
    true
}
