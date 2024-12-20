use std::{collections::{HashMap, VecDeque}, iter};
use crate::common::{io::read_file_as_2d_chars, Grid, Point};

pub fn run(file_path: &str) -> (u32, u32) {
    let grid = Grid::new_from_cells(read_file_as_2d_chars(file_path));
    let end_pos = grid.find_single(&'E');

    let honest_distances = distances_to_end(&grid, &end_pos);
    

    let part1 = count_good_cheats(&grid, &honest_distances, 2, 100);
    let part2 = count_good_cheats(&grid, &honest_distances, 20, 100);
    (part1, part2)
}

fn count_good_cheats(grid: &Grid<char>, distance_to_goal: &HashMap<Point, i32>, cheat_duration: i32, target_gain: i32) -> u32 {
    let mut num_good_cheats = 0;
    // TODO: Steal Chris' iterator, or use some sort of find
    for i in 0..grid.num_rows {
        for j in 0..grid.num_cols {
            let p = Point{i, j};
            if *grid.get(&p) == '#' {
                continue;
            }
            let d_point = *distance_to_goal.get(&p).unwrap_or_else(|| panic!("ohh my goadsdfghsfkd |/{:?}", p));
            for neighbor in manhattan_range(&p, cheat_duration).iter().filter(|n| grid.contains(n) && *grid.get(n) != '#') {
                let d_neighbor = *distance_to_goal.get(&neighbor).unwrap();
                if d_point >= p.d_manhattan(&neighbor) + d_neighbor + target_gain {
                    num_good_cheats += 1
                }
            }
        }
    }
    num_good_cheats
}

fn distances_to_end(grid: &Grid<char>, end_pos: &Point) -> HashMap<Point, i32> {
    let mut distances = HashMap::new();
    let mut search_q = VecDeque::from_iter(iter::once((*end_pos, 0)));

    while let Some((cur_pos, cur_dist)) = search_q.pop_front() {
        if distances.contains_key(&cur_pos) {
            continue;
        }
        distances.insert(cur_pos, cur_dist);
        cur_pos.neighbors().iter()
            .filter(|n| grid.contains(n) && *grid.get(n) != '#')
            .for_each(|n| { search_q.push_back((*n,  cur_dist + 1)); });
    }
    println!("Num nodes: {}", distances.len());
    return distances;
}

fn manhattan_range(p: &Point, distance: i32) -> Vec<Point> {
    let mut res = Vec::new();

    for di in -distance..=distance {
        for dj in -distance..=distance {
            if di.abs() + dj.abs() <= distance {
                res.push(Point {i: p.i + di, j: p.j + dj});
            }
        }
    }
    res
}
