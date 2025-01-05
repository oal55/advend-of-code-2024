use core::panic;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use std::iter;

use crate::common::{io::file_reader, Point};

const SIZE_N: i32 = 71;
// const SIZE_N: i32 = 7;

pub fn run(file_path: &str) -> (String, String) {
    let coordinates = file_reader(file_path).lines()
        .map(|l| l.unwrap())
        .map(parse_point)
        .collect::<Vec<_>>();

    (
        part1(&coordinates).to_string(),
        part2(coordinates)
    )
}

fn parse_point(line: String) -> Point {
    let (fi, se) = line.split_once(",").unwrap();
    Point{
        i:se.parse::<i32>().unwrap(),
        j:fi.parse::<i32>().unwrap()
    }
}

struct UnweightedGraph {
    size_n: i32, // we are represingting an NxN grid
    blocked_cells: HashSet<Point>
}

impl UnweightedGraph {
    fn new(size_n: i32, blocked_cells: &[Point]) -> Self {
        UnweightedGraph{size_n, blocked_cells: blocked_cells.iter().copied().collect() }
    }

    fn neighbors(&self, node: &Point) -> Vec<Point> {
        node.neighbors().into_iter()
            .filter(|p| (self.contains(p) && !self.blocked_cells.contains(p)))
            .collect()
    }

    fn contains(&self, p: &Point) -> bool {
        0 <= p.i && p.i< self.size_n &&
        0 <= p.j && p.j< self.size_n
    }
}

fn part1(coordinates: &[Point]) -> u32 {
    let graph = UnweightedGraph::new(SIZE_N, &coordinates[0..1024]);
    let src = Point{i:0, j:0};
    let dst = Point{i:SIZE_N-1, j:SIZE_N-1};

    let mut queue: VecDeque<(Point, u32)> =  VecDeque::from_iter(iter::once((src, 0)));
    let mut visited: HashSet<Point> = HashSet::from_iter(iter::once(src));

    while let Some((cur_node, cur_dist)) = queue.pop_front() {
        if cur_node == dst {
            return cur_dist
        }
        for neighbor in graph.neighbors(&cur_node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, cur_dist + 1));
            }
        }
    }
    panic!("No way Jose?!");
}

struct UnionFind {
    size: Vec<usize>,
    group: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self{
        UnionFind{
            size:vec![1; n],
            group: (0..n).collect::<Vec<_>>(),
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let (p_root, q_root) = (self.root(p), self.root(q));
        if p_root == q_root {
            return;
        }

        let (sml, big) = match self.size[p_root] < self.size[q_root] {
            true => (p_root, q_root),
            false => (q_root, p_root)
        };
        self.group[sml] = big;
        self.size[big] += self.size[sml];
    }

    fn is_connected(&mut self, p: usize, q: usize) -> bool { self.root(p) == self.root(q) }

    fn root(&mut self, mut p: usize) -> usize {
        while p != self.group[p] {
            let parents_group = self.group[self.group[p]];
            self.group[p] = parents_group;
            p = parents_group;
        }
        p
    }
}

fn part2(mut coordinates: Vec<Point>) -> String {
    let mut blocked_cells: HashSet<Point> = coordinates.iter().copied().collect();
    let mut union_find = {
        let mut union_find = UnionFind::new((SIZE_N * SIZE_N) as usize);
        for i in 0..SIZE_N {
            for j in 0..SIZE_N {
                let p = Point{i, j};
                if !blocked_cells.contains(&p) {
                    p.neighbors().into_iter()
                        .filter(|n| inside(n) && !blocked_cells.contains(n))
                        .for_each(|neighbor| union_find.union(uf_id(&p), uf_id(&neighbor)));
                }
            }
        }
        union_find
    };

    let (fi_id, last_id) = (uf_id(&Point{i:0, j:0}), uf_id(&Point{i:SIZE_N-1, j:SIZE_N-1}));

    while let Some(p) = coordinates.pop() {
        blocked_cells.remove(&p);
        p.neighbors().into_iter()
            .filter(|n| inside(n) && !blocked_cells.contains(n))
            .for_each(|neighbor| union_find.union(uf_id(&p), uf_id(&neighbor)));
        if union_find.is_connected(fi_id, last_id) {
            return format!("{},{}", p.j, p.i);
        }
    }
    panic!("We shouldn't have got here");
}

fn uf_id(p: &Point) -> usize { (p.i * SIZE_N + p.j) as usize }

fn inside(p: &Point) -> bool {
    0 <= p.i && p.i < SIZE_N &&
    0 <= p.j && p.j < SIZE_N
}