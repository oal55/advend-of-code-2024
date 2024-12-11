use std::io::BufRead;

use crate::common::file_reader;

pub fn run(file_path: &str) -> (i64, i64) { (part1(file_path), part2(file_path)) }

fn part1(file_path: &str) -> i64 {
    let mut res = 0;
    for line in file_reader(file_path).lines() {   
        let (target, nums) = parse_line(line.unwrap());
        if check(target, &nums) {
            res += target;
        }
    }
    return res;
}

fn part2(_file_path: &str) -> i64 { 0 }

fn parse_line(line: String) -> (i64, Vec<i64>) {
    let (fi, se) = line.split_once(": ").unwrap();
    let target = fi.parse::<i64>().unwrap();
    let nums = se.split(" ").into_iter()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    return (target, nums);
}

// ENUM OP INSTEAD OF BINARY BS
fn check(target: i64, nums: &Vec<i64>) -> bool {
    return (0..1<<(nums.len() - 1) as i64)
        .into_iter()
        .any(|encoded_ops| evaluate(&nums, encoded_ops) == target)
}

fn evaluate(nums: &Vec<i64>, operators: i64) -> i64 {
    let mut res = nums[0];
    nums.iter().skip(1).enumerate().for_each(|(i, num)| {
        res = if operators & (1 << i) == 0 {
            res + num 
        } else {
            res * num
        };
    });
    return res;
}
