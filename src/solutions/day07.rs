use std::io::BufRead;

use crate::common::file_reader;

enum Operator {Add, Mul, Concat}

pub fn run(file_path: &str) -> (i64, i64) {
    let mut res_two_op = 0;
    let mut res_three_op = 0;

    for line in file_reader(file_path).lines() {   
        let (target, nums) = parse_line(line.unwrap());
        if check(target, nums[0], &nums, 1, &vec![Operator::Add, Operator::Mul]) {
            res_two_op += target;
        }
        if check(target, nums[0], &nums, 1, &vec![Operator::Add, Operator::Mul, Operator::Concat]) {
            res_three_op += target;
        }
    }
    return (res_two_op, res_three_op);
}

fn parse_line(line: String) -> (i64, Vec<i64>) {
    let (fi, se) = line.split_once(": ").unwrap();
    let target = fi.parse::<i64>().unwrap();
    let nums = se.split(" ").into_iter()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    return (target, nums);
}

fn check(target: i64, res: i64, nums: &Vec<i64>, i_num: usize, allowed_operators: &Vec<Operator>) -> bool {
    if i_num == nums.len() {
        return res == target;
    }
    if res > target { // not >= because there might be a bunch of trailing 1s.
        return false;
    }
    for op in allowed_operators {
        let new_res = calc(res, nums[i_num], op);
        if new_res.is_some() && check(target, new_res.unwrap(), nums, i_num + 1, allowed_operators) {
            return true;
        }
    }
    return false;
}

fn calc(fi: i64, se: i64, op: &Operator) -> Option<i64> {
    return match op {
        Operator::Add => i64::checked_add(fi, se),
        Operator::Mul => i64::checked_mul(fi, se),
        Operator::Concat => concat(fi, se),
    }
}

fn concat(fi: i64, se: i64) -> Option<i64> {
    let mut coeff = 10i64;
    while se >= coeff {
        coeff *= 10;
    }
    return fi.checked_mul(coeff).and_then(|x| x.checked_add(se));
}
