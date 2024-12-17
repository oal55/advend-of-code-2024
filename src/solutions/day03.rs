use regex::Regex;

use crate::common::io::read_file;

pub fn run(file_path: &str) -> (i64, i64) { (part1(file_path), part2(file_path)) }

const MUL_RE: &str = r"mul\((-?\d{1,3}),(-?\d{1,3})\)";
const TOGGLE_RE: &str = r"(do\(\)|don't\(\))";

fn part1(file_path: &str) -> i64 {
    let re = Regex::new(MUL_RE).unwrap();
    let file_content = read_file(file_path);

    let mut res: i64 = 0;
    for (_, [fi, se]) in re.captures_iter(&file_content).map(|c| c.extract()) {
        let p = fi.parse::<i64>().unwrap();
        let q = se.parse::<i64>().unwrap();
        res += p*q;
    }
    res
}

fn part2(file_path: &str) -> i64 {
    let re = Regex::new(format!("{MUL_RE}|{TOGGLE_RE}").as_str()).unwrap();
    let file_content = read_file(file_path);

    let mut enabled = true;
    let mut res: i64 = 0;
    for c in re.captures_iter(&file_content) {
        match c.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let (fi, se) = (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str());
                    let p = fi.parse::<i64>().unwrap();
                    let q = se.parse::<i64>().unwrap();
                    res += p*q;
                }
            }
        }
    }
    res
}
