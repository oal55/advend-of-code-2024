use std::{collections::HashMap, io::BufRead};
use crate::common::file_reader;

const EMPTY_VEC: Vec<i64> = Vec::new(); // fml.

pub fn run(file_path: &str) -> (i64, i64) { (part1(file_path), part2(file_path)) }

fn part1(file_path: &str) -> i64 {
    let (dependencies, sequences) = read_things(file_path);

    return sequences.iter()
        .filter(|&s| in_order(&dependencies, s))
        .map(middle)
        .sum();
}

fn part2(file_path: &str) -> i64 {
    let (dependencies, sequences) = read_things(file_path);

    return sequences.iter()
        .filter(|&s| !in_order(&dependencies, s))
        .map(|sequence| {
            let indegrees = build_indegrees(&dependencies, sequence);
            let mut res = sequence.clone();
            res.sort_by_key(|num| indegrees.get(num).unwrap_or(&0));
            return middle(&res);
        })
        .sum();
}

fn in_order(dependencies: &HashMap<i64, Vec<i64>>, sequence: &Vec<i64>) -> bool {
    let mut indegrees = build_indegrees(&dependencies, sequence);
    for num in sequence {
        if *indegrees.get(num).unwrap_or(&0) != 0 {
            return false;
        }
        for dst in dependencies.get(num).unwrap_or(&EMPTY_VEC) {
            if indegrees.contains_key(dst) {
                *indegrees.get_mut(dst).unwrap() -= 1;
            }
        }
    }

    return true;
}

fn build_indegrees(dependencies: &HashMap<i64, Vec<i64>>, sequence: &Vec<i64>) -> HashMap<i64, i64> {
    let mut indegrees = HashMap::new();
    for src in sequence {
        for dst in dependencies.get(&src).unwrap_or(&EMPTY_VEC) {
            if sequence.contains(dst) {
                *indegrees.entry(*dst).or_insert(0) += 1
            }
        }
    }
    return indegrees;
}

fn read_things(file_path: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut it = file_reader(file_path).lines();
    
    let mut by_dependency = HashMap::new(); // pre-req -> [downstream, things]
    it.by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (fi, se) = line.split_once("|").expect(format!("Cannot split line {}", line).as_str());
            by_dependency.entry(into_i64(fi)).or_insert_with(Vec::new).push(into_i64(se));
        });
    
    let sequences = it.map(|line| line.unwrap())
        .map(|line| line.split(",").into_iter().map(into_i64).collect())
        .collect();

    return (by_dependency, sequences);
}

#[inline]
fn into_i64(num: &str) -> i64 { num.parse().unwrap() }
#[inline]
fn middle(vec: &Vec<i64>) -> i64 { vec[vec.len()/2] }
