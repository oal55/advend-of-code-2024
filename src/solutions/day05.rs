use std::cmp::Ordering;
use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (u32, u32) {
    let (dependencies, sequences) = read_things(file_path);

    let mut part1 = 0;
    let mut part2 = 0;

    let topo_order = |a: &u32, b: &u32| ->  Ordering {
        match (dependencies[*a as usize][*b as usize], dependencies[*b as usize][*a as usize]) {
            (false, false) => Ordering::Equal,
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (true, true) => panic!("Bad dependencies"),
        }
    };

    for mut seq in sequences {
        if seq.is_sorted_by(|a ,b| topo_order(a, b) != Ordering::Greater) {
            part1 += middle(&seq);
        } else {
            seq.sort_by(topo_order);
            part2 += middle(&seq);
        }
    }

    (part1, part2)
}

#[inline]
fn middle(vec: &Vec<u32>) -> u32 { vec[vec.len()/2] }

fn read_things(file_path: &str) -> ([[bool; 100]; 100], Vec<Vec<u32>>) {
    let mut dependencies: [[bool; 100]; 100] = [[false; 100]; 100]; // dep[a][b] == true iff a has to come before b
    let mut it = file_reader(file_path).lines().map(Result::unwrap);

    it.by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (fi, se) = line.split_once("|").unwrap_or_else(|| panic!("Cannot split line {}", line));
            dependencies[fi.parse::<usize>().unwrap()][se.parse::<usize>().unwrap()] = true;
        });

    let sequences = it.map(|line| line.split(",").map(|n| n.parse().unwrap()).collect()).collect();

    (dependencies, sequences)
}
