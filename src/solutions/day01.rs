use std::collections::HashMap;
use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let mut nums_fi: Vec<i64> = Vec::new();
    let mut nums_se: Vec<i64> = Vec::new();
    let mut freq: HashMap<i64, i64> = HashMap::new();

    for line in file_reader(file_path).lines().map(Result::unwrap) {
        let (fi, se) = parse_line(line);
        nums_fi.push(fi);
        nums_se.push(se);
        *freq.entry(se).or_default() += 1;
    }

    nums_fi.sort();
    nums_se.sort();

    let part1 = nums_fi.iter().zip(nums_se.iter())
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
