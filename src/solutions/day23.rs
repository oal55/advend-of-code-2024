use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (usize, String) {
    let (adj_list, node_mapping) = build_adj_list(file_path);

    println!("Num nodes: {}", adj_list.len());
    println!("Num edges: {}", adj_list.values().into_iter().map(|neighs| neighs.len()).sum::<usize>() / 2);

    let cliques = cliques_3(&adj_list);
    

    let t_nodes = node_mapping.iter()
        .filter(|(_, v)| v.starts_with("t"))
        .map(|(k, _)| k)
        .collect::<HashSet<_>>();
    let part1 = cliques.iter().filter(|c| c.iter().any(|node| t_nodes.contains(node))).count();

    let mut cur_set = cliques.clone();
    loop {
        let mut next_set = HashSet::new();

        for clique in cur_set.iter() {
            let pivot = clique.first().unwrap();
            for candidate in &adj_list[pivot] {
                if clique.iter().all(|c_elem| adj_list[c_elem].contains(candidate)) {
                    let mut clique_with_node = clique.clone();    
                    clique_with_node.push(*candidate);
                    clique_with_node.sort();
                    next_set.insert(clique_with_node);    
                }
            }
        }
        if next_set.is_empty() {
            break;
        }
        cur_set = next_set;
    }
    if cur_set.len() != 1 {
        dbg!("Found multiple biggest cliques: {cur_set}");
    }

    let mut biggest_clique = cur_set.iter().next().unwrap().iter().map(|num| node_mapping[num].clone()).collect::<Vec<_>>();
    biggest_clique.sort();
    (part1, biggest_clique.join(","))
}

fn cliques_3(adj_list: &HashMap<u32, HashSet<u32>>) -> HashSet<Vec<u32>> {
    let mut res = HashSet::new();
    for (fi, fi_neighbors) in adj_list {
        for se in fi_neighbors {
            for thi in adj_list[se].iter() {
                if fi < se && se < thi && // in order
                    adj_list[thi].contains(fi) { // thi connects back to fi
                        res.insert(vec![*fi, *se, *thi]);
                }
            }
        }
    }
    res
}

fn build_adj_list(file_path: &str) -> (HashMap<u32, HashSet<u32>>, HashMap<u32, String>) {
    let mut adj_list = HashMap::new();
    let mut int_label_to_strings = HashMap::new();

    file_reader(file_path).lines()
    .map(|l| l.unwrap())
    .for_each(|l| {
        let (fi, se) = l.split_once("-").unwrap();
        let (fi_hash, se_hash) = (hashz(fi), hashz(se));

        int_label_to_strings.insert(fi_hash, fi.to_string());
        int_label_to_strings.insert(se_hash, se.to_string());

        adj_list.entry(fi_hash).or_insert_with(HashSet::new).insert(se_hash);
        adj_list.entry(se_hash).or_insert_with(HashSet::new).insert(fi_hash);
    });

    (adj_list, int_label_to_strings)
}

fn hashz(node: &str) -> u32 {
    let mut res = 0u32;
    let mut coeff = 1;
    for c in node.chars() {
        res += ((c as u32) - ('a' as u32)) * coeff;
        coeff *= 30;
    }
    res
}
