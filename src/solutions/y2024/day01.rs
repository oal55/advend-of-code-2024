use std::collections::HashMap;
use std::io::BufRead;
use std::iter::zip;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {    
    let (mut nums_fi, mut nums_se): (Vec<_>, Vec<_>) = file_reader(file_path).lines()
        .map(Result::unwrap)
        .map(parse_line)
        .unzip();

    nums_fi.sort();
    nums_se.sort();

    let freq: HashMap<i64, i64> = nums_se.iter().fold(
        HashMap::new(),
        |mut m, &num| {
                *m.entry(num).or_insert(0) += 1;
                m
        }
    );

    let part1 = zip(nums_fi.iter(), nums_se.iter())
        .map(|(p, q)| (p - q).abs())
        .sum();
    let part2 = nums_fi.iter()
        .map(|p| p * freq.get(p).unwrap_or(&0))
        .sum();
    (part1, part2)
}

fn parse_line(line: String) -> (i64, i64) {
    let (fi, se) = line.split_once("   ").unwrap();
    let num1 = fi.parse::<i64>().unwrap_or_else(|_| panic!("Failed to parse `fi` in line: {line}"));
    let num2 = se.parse::<i64>().unwrap_or_else(|_| panic!("Failed to parse `se` in line: {line}"));
    (num1, num2)
}
