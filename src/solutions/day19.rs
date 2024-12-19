use std::io::BufRead;
use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (u64, u64) {
    let (patterns, towel_rows) = parse_input(file_path);

    towel_rows.iter()
        .map(|row| num_ways_to_organize(&patterns, row))
        .fold((0, 0), |(part1, part2), n_ways| (part1 + (n_ways > 0) as u64, part2 + n_ways))
}

fn num_ways_to_organize(patterns: &Vec<Vec<char>>, row: &Vec<char>) -> u64 {
    let mut num_ways = vec![0; row.len() + 1];
    num_ways[0] = 1;
    for i in 0..row.len() {
        for pattern in patterns {
            if i + pattern.len() <= row.len() && row[i..(i + pattern.len())] == *pattern {
                num_ways[i + pattern.len()] += num_ways[i];
            }
        }
    }
    num_ways[row.len()]
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut lines = file_reader(file_path).lines();

    let patterns = lines.next().unwrap().unwrap()
        .split(", ")
        .map(|p| p.chars().collect())
        .collect();

    lines.next().unwrap().unwrap(); // second line's empty for some reason

    let towel_rows = lines.map(|l| l.unwrap().chars().collect()).collect();

    (patterns, towel_rows)
}
