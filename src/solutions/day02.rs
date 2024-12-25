use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let mut num_safe_reports = 0;
    let mut num_safe_after_tweak = 0;

    for r in read_file(file_path) {
        if is_report_safe(&r) {
            num_safe_reports += 1;
        } else if (0..r.len()).any(|i| is_report_safe(&(r[0..i].iter().copied().chain(r[i+1..].iter().copied()).collect()))) {
            num_safe_after_tweak += 1;
        }
    }
    (num_safe_reports, num_safe_reports + num_safe_after_tweak)
}

fn read_file(file_path: &str) -> Vec<Vec<i64>> {
    file_reader(file_path).lines()
        .map(Result::unwrap)
        .map(|line| line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap_or_else(|_| panic!("Cannot parse {num}")))
            .collect())
        .collect()
}

fn is_report_safe(nums: &Vec<i64>) -> bool {
    if nums[0] > nums[1] {
        return is_increasing(nums.iter().rev(), nums.iter().rev().skip(1));
    }
    is_increasing(nums.iter(), nums.iter().skip(1))
}

fn is_increasing<'a>(fi: impl Iterator<Item=&'a i64>, se: impl Iterator<Item=&'a i64>) -> bool {
    fi.zip(se).all(|(sml, big)| sml < big && (big - sml < 4))
}
