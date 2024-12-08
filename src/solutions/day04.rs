use core::panic;
use std::{io::BufRead, iter::zip};
use crate::common::file_reader;

struct WordFinder<'a> {
    grid: &'a Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

pub fn run(file_path: &str) -> (i64, i64) {
    let grid: Vec<Vec<char>> = file_reader(file_path).lines().into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let word_finder = WordFinder::new(&grid);
    return (part1(&word_finder), part2(&word_finder));
}

fn part1(word_finder: &WordFinder) -> i64 {
    let mut res =0;
    for direction in massage_into_usize(vec![(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]) {
        let diffs :Vec<(usize, usize)> =  std::iter::once((0, 0)).chain(std::iter::repeat_n(direction, 3)).collect();
        res += word_finder.count_pattern("XMAS", &diffs);
    }
    return res;
}

fn part2(word_finder: &WordFinder) -> i64 {
    let diffs = massage_into_usize(vec![(0, 0), (-1, -1), (0, 2), (2, 0), (0, -2)]);
    return
        word_finder.count_pattern("AMMSS", &diffs) + 
        word_finder.count_pattern("AMSSM", &diffs) + 
        word_finder.count_pattern("ASSMM", &diffs) + 
        word_finder.count_pattern("ASMMS", &diffs);
}

fn massage_into_usize(tuples: Vec<(i32, i32)>) -> Vec<(usize, usize)> { tuples.iter().map(|(i,j)| (*i as usize, *j as usize)).collect() }

impl<'a> WordFinder<'a> {
    pub fn new(grid: &'a Vec<Vec<char>>) -> WordFinder<'a> { WordFinder{ grid, num_rows: grid.len(), num_cols: grid[0].len() } }

    fn count_pattern(&self, word: &str, diffs: &Vec<(usize, usize)>) -> i64 {
        if word.len() != diffs.len() {
            panic!("Brah")
        }

        let chars = word.chars().collect();
        let mut res = 0;
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if self.matches_at(i, j, &chars, diffs) {
                    res += 1;
                }
            }
        }
        return res;
    }

    fn matches_at(&self, i: usize, j: usize, word: &Vec<char>, diffs: &Vec<(usize, usize)>) -> bool {
        let  (mut i_cur, mut j_cur) = (i, j);
        for (&letter, (di, dj)) in zip(word, diffs) {
            i_cur = i_cur.wrapping_add(*di);
            j_cur = j_cur.wrapping_add(*dj);
            if !self.inside(i_cur, j_cur) || letter != self.grid[i_cur][j_cur] {
                return false;
            }
        }
        return true;
    }

    fn inside(&self, i: usize, j: usize) -> bool { i < self.num_rows && j < self.num_cols }
}

