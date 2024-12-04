use std::io::BufRead;

use crate::common::file_reader;

pub fn run() -> (i64, i64) { (part1(), part2()) }

fn part1() -> i64 {
    return read_file()
        .iter()
        .filter(|&report| is_report_safe(&report))
        .count() as i64;
}

fn part2() -> i64 {
    return read_file()
        .iter()
        .filter(|&report| {
            return is_report_safe(report) || (0..report.len()).any(|i| {
                let left =report[0..i].iter().cloned();
                let right = report[i+1..].iter().cloned();
                return is_report_safe(&left.chain(right).collect());
            });
        })
        .count() as i64;
}

fn read_file() -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = Vec::new();
    let reader = file_reader(2);
    for line in reader.lines() {
        let report_numbers: Vec<i64> = line.unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect(format!("Cannot parse {num}").as_str()))
            .collect();
        lines.push(report_numbers);
    }
    return lines
}


fn is_increasing<'a>(fi: impl IntoIterator<Item=&'a i64>, se: impl IntoIterator<Item=&'a i64>) -> bool {
    return fi.into_iter().zip(se.into_iter()).all(|(sml, big)| sml < big && (big - sml < 4));
}

fn is_report_safe(nums: &Vec<i64>) -> bool {
    if nums[0] > nums[1] {
        return is_increasing(nums.iter().rev(), nums.iter().rev().skip(1));
    }
    return is_increasing(nums.iter(), nums.iter().skip(1));
}
