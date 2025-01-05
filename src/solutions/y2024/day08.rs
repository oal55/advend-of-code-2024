use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::common::Point;
use crate::common::io::read_file;

pub fn run(file_path: &str) -> (i64, i64) {
    let field = read_file(file_path).parse::<Field>().unwrap();
    (part1(&field), part2(&field))
}

fn part1(field: &Field) -> i64 {
    let mut unique_points = HashSet::new();
    for (_, points) in field.antennas.clone() {
        Combinations::new(points.len())
            .map(|(i, j)| (&points[i], &points[j]))
            .flat_map(|(&p, &q)| {
                let diff = q - p;
                vec!(q + diff, p - diff)
            })
            .filter(|p| field.contains(p))
            .for_each(|p| { unique_points.insert(p); });
    }
    unique_points.len() as i64
}

fn part2(field: &Field) -> i64 {
    let mut unique_points = HashSet::new();
    for (_, points) in field.antennas.clone() {
        Combinations::new(points.len())
            .map(|(i, j)| (&points[i], &points[j]))
            .for_each(|(&p, &q)| {
                let diff = q - p;
                
                let mut cur = p;
                while field.contains(&cur) {
                    unique_points.insert(cur);
                    cur += diff;
                }

                cur = p;
                while field.contains(&cur) {
                    unique_points.insert(cur);
                    cur -= diff;
                }
            });
    }
    unique_points.len() as i64
}

struct Combinations {
    maxn: usize,
    i: usize,
    j: usize,
}

impl Combinations {
    fn new(maxn: usize) -> Self { Combinations { maxn, i: 0, j: 0 } }
}

impl Iterator for Combinations {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.j += 1;
        if self.i == self.maxn - 2 && self.j == self.maxn {
            return None;
        }        
        if self.j == self.maxn {
            self.i += 1;
            self.j = self.i + 1;
        }
        Some((self.i, self.j))
    }
}

struct Field {
    antennas: HashMap<char, Vec<Point>>,
    num_rows: i32,
    num_cols: i32,
}

impl Field {
    fn contains(&self, p: &Point) -> bool { 0 <= p.i && p.i < self.num_rows && 0 <= p.j && p.j < self.num_cols }
}

#[derive(Debug)] // what exactly is this. Required for unwrap?
struct ParseFieldErr;

impl FromStr for Field {
    type Err = ParseFieldErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect(); // normally we'd sanity check

        let mut antennas = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character != '.' {
                    antennas.entry(character).or_insert_with(Vec::new).push(Point{i: i as i32, j: j as i32}); // NORBAY?! easier/nicer way to cast?
                }
            }
        }
 
        Ok(Field {antennas, num_rows: lines.len() as i32, num_cols: lines[0].len() as i32 })
    }
}
