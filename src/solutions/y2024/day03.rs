use regex::Regex;

use crate::common::io::read_file;

const MUL_RE: &str = r"mul\((-?\d{1,3}),(-?\d{1,3})\)";
const TOGGLE_RE: &str = r"(do\(\)|don't\(\))";

pub fn run(file_path: &str) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let re = Regex::new(format!("{MUL_RE}|{TOGGLE_RE}").as_str()).unwrap();
    let mut enabled = true;
    re.captures_iter(&read_file(file_path)).for_each(|c| {
        match c.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let (fi, se) = (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str());
                let p = fi.parse::<i64>().unwrap();
                let q = se.parse::<i64>().unwrap();
                part1 += p*q;
                enabled.then(|| part2 += p*q);
            }
        }
    });

    (part1, part2)
}
