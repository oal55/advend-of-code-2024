use core::panic;
use std::iter::zip;

use crate::common::io::read_file_as_2d_chars;
use crate::common::{Grid, Point, Direction};

struct WordFinder { grid: Grid<char> }

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_cells(read_file_as_2d_chars(file_path));
    let word_finder = WordFinder{ grid };
    (part1(&word_finder), part2(&word_finder))
}

fn part1(word_finder: &WordFinder) -> i64 {
    let mut res =0;
    let zero = Point::new(0,0);
    for direction in Direction::ALL_DIRS {
        let diffs :Vec<Point> =  std::iter::once(zero).chain(std::iter::repeat_n(direction, 3)).collect();
        res += word_finder.count_pattern("XMAS", &diffs);
    }
    res
}

fn part2(word_finder: &WordFinder) -> i64 {
    let diffs = vec![Point::new(0, 0), Point::new(-1, -1), Point::new(0, 2), Point::new(2, 0), Point::new(0, -2)];
    word_finder.count_pattern("AMMSS", &diffs) + 
        word_finder.count_pattern("AMSSM", &diffs) + 
        word_finder.count_pattern("ASSMM", &diffs) + 
        word_finder.count_pattern("ASMMS", &diffs)
}

impl WordFinder {
    fn count_pattern(&self, word: &str, diffs: &Vec<Point>) -> i64 {
        if word.len() != diffs.len() {
            panic!("Brah")
        }
        let chars = word.chars().collect();
        let mut res = 0;
        for i in 0..self.grid.num_rows {
            for j in 0..self.grid.num_cols {
                if self.matches_at(Point{i, j}, &chars, diffs) {
                    res += 1;
                }
            }
        }
        res
    }

    fn matches_at(&self, start_pos: Point, word: &Vec<char>, diffs: &Vec<Point>) -> bool {
        let mut cur_pos = start_pos;
        for (&letter, d_pos) in zip(word, diffs) {
            cur_pos += *d_pos;
            if !self.grid.contains(&cur_pos) || letter != *self.grid.get(&cur_pos) {
                return false;
            }
        }
        true
    }
}
