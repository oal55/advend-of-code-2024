use std::iter::Peekable;

use crate::common::io::read_file;

pub fn run(file_path: &str) -> (i64, i64) {
    let mut num_safe_reports = 0;
    let mut num_safe_after_tweak = 0;

    let reports = read_file(file_path).lines().map(parse_report).collect::<Vec<_>>();
    for r in reports {
        if is_report_safe(r.iter().copied().peekable()) {
            num_safe_reports += 1;
        } else if (0..r.len()).any(|i| is_report_safe(r[0..i].iter().copied().chain(r[i+1..].iter().copied()).peekable())) {
            num_safe_after_tweak += 1;
        }
    }
    (num_safe_reports, num_safe_reports + num_safe_after_tweak)
}

fn parse_report(l: &str) -> Vec<i32> { l.split(" ").map(|n| n.parse().unwrap()).collect() }

fn is_report_safe(mut nums: Peekable<impl Iterator<Item=i32>>) -> bool {
    let mut cur = nums.next().unwrap();
    let order = cur.cmp(nums.peek().unwrap());

    for next_elem in nums {
        if cur.cmp(&next_elem) != order || cur.abs_diff(next_elem) > 3 {
            return false
        }
        cur = next_elem;
    }
    return true;
}
