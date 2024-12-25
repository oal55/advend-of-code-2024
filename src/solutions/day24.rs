use std::{collections::{HashMap, VecDeque}, io::BufRead};
use crate::common::io::file_reader;

#[derive(Debug)]
enum Operator {And, Or, Xor}

#[derive(Debug)]
struct Assignment { fi: String, se: String, op: Operator, target: String}

struct Computer {
    dict: HashMap<String, u32>
}
impl Computer {
    fn execute(&self, a: &Assignment) -> u32 {
        let (fi_val, se_val) = (self.dict[&a.fi], self.dict[&a.se]);
        match a.op {
            Operator::And => fi_val & se_val,
            Operator::Or => fi_val | se_val,
            Operator::Xor => fi_val ^ se_val
        }
    }
    fn set(&mut self, var: &str, val: u32) {
        self.dict.insert(var.to_string(), val);
    }
}

struct Graph {
    nodes: HashMap<String, Vec<String>>, // upstream -> downstream
    indegrees: HashMap<String, u32>
}
impl Graph {

    fn new_from_assignments(assignments: Vec<&Assignment>) -> Self {
        let mut nodes = HashMap::new();
        let mut indegrees = HashMap::new();
        for a in assignments {
            nodes.entry(a.fi.clone()).or_insert_with(Vec::new).push(a.target.clone());
            nodes.entry(a.se.clone()).or_insert_with(Vec::new).push(a.target.clone());
            nodes.entry(a.target.clone()).or_insert_with(Vec::new);
            *indegrees.entry(a.target.clone()).or_default() += 2;
        }
        Graph{nodes, indegrees}
    }

    fn resolve(&mut self, node: &str) -> Vec<&String> {
        let mut res = Vec::new();
        for downstream_neighbor in self.nodes[node].iter() {
            let indegree =self.indegrees.get_mut(downstream_neighbor).unwrap();
            *indegree -= 1;
            if *indegree == 0 {
                println!("{downstream_neighbor} is freed up upon resolving {node}");
                res.push(downstream_neighbor);
            }
        }
        res
    }

    fn indegree(&self, node: &str) -> u32 { *self.indegrees.get(node).unwrap_or(&0) }
}

pub fn run(file_path: &str) -> (u64, u64) {
    let (variables, assignments) = parse_file(file_path);
    
    let mut computer = Computer{dict: variables};
    let mut graph = Graph::new_from_assignments(assignments.values().collect::<Vec<_>>());

    let mut assignment_q = VecDeque::from_iter(assignments.values().filter(|a| graph.indegree(&a.target) == 0));

    computer.dict.keys().for_each(|v| {
        graph.resolve(v).iter()
            .map(|&t| assignments.get(t).unwrap_or_else(|| panic!("No assignment for target {t}")))
            .for_each(|a| assignment_q.push_back(a));
    });
    while let Some(assignment) = assignment_q.pop_front() {
        println!("Popped {:?}", assignment);
        let res = computer.execute(assignment);
        computer.set(&assignment.target, res);
        for downstream in graph.resolve(&assignment.target) {
            assignment_q.push_back(assignments.get(downstream).unwrap());
        }
    }
    let mut z_vars = computer.dict.iter().filter(|(k, _)| k.starts_with("z")).collect::<Vec<_>>();
    z_vars.sort();
    
    // dbg!("z_vars: {}", z_vars.clone());
    let part1 = z_vars.iter().enumerate().fold(0u64, |res, (i, &(_, &val))| res + (val as u64)*(1 << i));
    (part1, 0)
    // (61886126253040, 0)
}


fn parse_file(file_path: &str) -> (HashMap<String, u32>, HashMap<String, Assignment>) {
    let mut it = file_reader(file_path).lines();

    let mut variables = HashMap::new();
    for line in it.by_ref().map(Result::unwrap).take_while(|l| !l.is_empty()) {
        let (var, value) = line.split_once(": ").unwrap();
        variables.insert(var.to_string(), value.parse::<u32>().unwrap());
    }

    let mut assignments_by_name = HashMap::new();
    it.map(Result::unwrap)
        .for_each(|line| {
            let parts =  line.trim().split(" ").collect::<Vec<_>>();
            let op = match parts[1] {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "XOR" => Operator::Xor,
                _ => panic!("Unexpected operator {}. Line: {line}", parts[1]),
            };

            assignments_by_name.insert(
                parts[4].to_string(),
                Assignment{
                    fi: parts[0].to_string(),
                    se: parts[2].to_string(),
                    op,
                    target: parts[4].to_string()
                }
            );
            
        });

    (variables, assignments_by_name)
}
