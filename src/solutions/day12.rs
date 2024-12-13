use std::io::BufRead;
use crate::common::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let chars: Vec<Vec<char>> = file_reader(file_path).lines().into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    
    (part1(file_path), part2(file_path))
}

fn part1(file_path: &str) -> i64 { 0 }

fn part2(file_path: &str) -> i64 { 0 }