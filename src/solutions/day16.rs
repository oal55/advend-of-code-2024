use core::panic;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::io::BufRead;
use std::ops::Range;

use crate::common::{Point, UNIT_VECTORS};
use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (u64, u64) {
    let chars: Vec<Vec<char>> = file_reader(file_path).lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let p_start = find(&chars, 'S');
    let p_end = find(&chars, 'E');

    let graph: DirectedGraph = DirectedGraph::new_from_grid(&chars);

    calc(&graph, &p_start, &p_end)
}

fn calc(graph: &DirectedGraph, p_start: &Point, p_end: &Point) -> (u64, u64) {

    let mut distances: HashMap<Node, u64> = HashMap::new();
    let mut parents: HashMap<Node, HashSet<Node>> = HashMap::new();
    
    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    let node_start = Node{coords: *p_start, dir: Point{i:0, j:1}};

    
    heap.push(HeapNode{n: node_start, cost: 0, parent: node_start});

    let mut max_allowed_cost = u64::MAX;

    while !heap.is_empty() {
        let HeapNode{n: cur_node, cost: cur_cost, parent: cur_parent} = heap.pop().unwrap();
        if cur_cost > max_allowed_cost {
            continue;
        }

        if let Some(&seen_cost) = distances.get(&cur_node) {
            if seen_cost != cur_cost {
                continue;
            }
        }
        
        distances.insert(cur_node, cur_cost);
        parents.entry(cur_node).or_default().insert(cur_parent);
        if cur_node.coords == *p_end {
            max_allowed_cost = cur_cost;
            continue;
        }
        for (neighbor, edge_cost) in graph.neighbors(&cur_node) {
            heap.push(HeapNode{
                n: neighbor,
                cost: cur_cost + edge_cost,
                parent: cur_node
            });
        }
    }

    let empty_set = HashSet::new();
    let mut seen_points: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::from_iter(UNIT_VECTORS.iter().map(|dir| Node{dir: *dir, coords: *p_end}));
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        seen_points.insert(node.coords);
        if node.coords == *p_start {
            break
        }
        for parent in parents.get(&node).unwrap_or(&empty_set) {
            queue.push_back(*parent);
        }
    }
    (max_allowed_cost, seen_points.len() as u64)

}

#[derive(Clone, Copy, Hash, Eq)]
struct HeapNode{
    n: Node,
    parent: Node,
    cost: u64
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool { self.cost == other.cost }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node { coords: Point, dir: Point }

struct DirectedGraph {
    i_range: Range<i32>,
    j_range: Range<i32>,

    walls: HashSet<Point>
}

impl DirectedGraph {
    fn new_from_grid(chars: &Vec<Vec<char>>) -> DirectedGraph {
        let walls: HashSet<Point> = chars.iter().enumerate()
            .flat_map(|(i, l)| l.iter().enumerate()
                .filter(|(_, &cell)| cell == '#')
                .map(move |(j, _)| Point{i: i as i32, j: j as i32}) // why do I move!?@#$%^&*( elp.)
            ).collect();

        DirectedGraph{
            i_range: 0..chars.len() as i32,
            j_range: 0..chars[0].len() as i32,
            walls
        }
    }

    fn neighbors(&self, Node{coords, dir}: &Node) -> Vec<(Node, u64)> {
        let mut neighbors = vec![
            (Node{coords: *coords, dir: dir.rotated_clockwise()}, 1000),
            (Node{coords: *coords, dir: dir.rotated_widdershins()}, 1000)
        ];
        let next_coord = coords.step(dir);
        if self.contains(&next_coord) && !self.walls.contains(&next_coord) {
            neighbors.push((Node{coords: next_coord, dir: *dir}, 1));
        }
        neighbors
    }

    fn contains(&self, p: &Point) -> bool { self.i_range.contains(&p.i) && self.j_range.contains(&p.j) }
}

fn find(chars: &Vec<Vec<char>>, val: char) -> Point {
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == val {
                return Point{i: i as i32, j: j as i32};
            }
        }
    }
    panic!("Send.elp.immediately");
}
