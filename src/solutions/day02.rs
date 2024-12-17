use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) { (part1(file_path), part2(file_path)) }

fn part1(file_path: &str) -> i64 {
    read_file(file_path)
        .iter()
        .filter(|&report| is_report_safe(report))
        .count() as i64
}

fn part2(file_path: &str) -> i64 {
    read_file(file_path)
        .iter()
        .filter(|&report| {
            is_report_safe(report) || (0..report.len()).any(|i| {
                let left =report[0..i].iter().cloned();
                let right = report[i+1..].iter().cloned();
                is_report_safe(&left.chain(right).collect())
            })
        })
        .count() as i64
}

fn read_file(file_path: &str) -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = Vec::new();
    let reader = file_reader(file_path);
    for line in reader.lines() {
        let report_numbers: Vec<i64> = line.unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap_or_else(|_| panic!("Cannot parse {num}")))
            .collect();
        lines.push(report_numbers);
    }
    lines
}


fn is_increasing<'a>(fi: impl Iterator<Item=&'a i64>, se: impl Iterator<Item=&'a i64>) -> bool {
    fi.zip(se).all(|(sml, big)| sml < big && (big - sml < 4))
}

fn is_report_safe(nums: &Vec<i64>) -> bool {
    if nums[0] > nums[1] {
        return is_increasing(nums.iter().rev(), nums.iter().rev().skip(1));
    }
    is_increasing(nums.iter(), nums.iter().skip(1))
}
