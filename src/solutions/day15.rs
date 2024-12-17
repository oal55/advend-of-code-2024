use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use std::iter;

use crate::common::io::file_reader;
use crate::common::{Grid, Point, Direction};


pub fn run(file_path: &str) -> (i32, i32) {
    let (chars, moves) = parse_file(file_path);
    
    let larger_chars = chars.iter()
        .map(|line| line.iter()
            .flat_map(|c| match c {
                '#' => "##".chars(),
                'O' => "[]".chars(),
                '.' => "..".chars(),
                '@' => "@.".chars(),
                _ => panic!("Can't derive {c}")
            })
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    // let _small_gps = part1(chars, &moves);
    let small_gps = move_and_get_gps(chars, &moves, 'O');
    let big_gps = move_and_get_gps(larger_chars, &moves, '[');
    
    (small_gps, big_gps)
}

fn move_and_get_gps(chars: Vec<Vec<char>>, moves: &Vec<Point>, gps_target: char) -> i32{
    let mut grid = Grid::new_from_cells(chars);
    let mut cur_pos = grid.find_single(&'@');

    for &dir in moves {
        let has_moved = execute_move(&mut grid, cur_pos, dir);
        if has_moved {
            cur_pos += dir;
        }
    }
    return calc_gps(&grid, gps_target);
}

// true iff the bot is able to move
fn execute_move(grid: &mut Grid<char>, robot_pos: Point, dir: Point) -> bool {
    match dir {
        // If I _really_ hated my life I'd make these fucntions statically ensure that direction valuesa re legit
        Direction::LEFT | Direction::RIGHT => try_move_left_right(grid, robot_pos, dir),
        Direction::UP | Direction::DOWN => try_move_up_down(grid, robot_pos, dir),
        _ => panic!("Bad direction: {dir}")
    }
}

fn try_move_left_right(grid: &mut Grid<char>, initial_pos: Point, dir: Point) -> bool {
    let maybe_next_pos = initial_pos + dir;
    let reverse_dir = -dir;
    if let Some(empty_slot) = find_empty_slot_from(maybe_next_pos, &grid, dir) {
        let mut cur_dst = empty_slot;
        while cur_dst != initial_pos {
            let next_dst = cur_dst + reverse_dir;
            move_point(grid, next_dst, dir);
            cur_dst = next_dst;
        }
        return true;
    }
    return false;
}

fn try_move_up_down(grid: &mut Grid<char>, initial_pos: Point, dir: Point) -> bool {
    let mut cur_row: HashSet<Point> = HashSet::from_iter(vec![initial_pos]);
    let mut rows_to_move: VecDeque<HashSet<Point>> = VecDeque::from_iter(iter::once(cur_row.clone()));
    while !cur_row.is_empty() {
        let next_space_to_occupy = rows_to_move.back().unwrap().iter()
            .map(|pos| *pos + dir)
            .collect::<HashSet<_>>();

        let moveable = next_space_to_occupy.iter().all(|pos| grid.contains(pos) && *grid.get(pos) != '#');
        if !moveable {
            return false;
        }

        let mut next_boxes_to_move: HashSet<Point> = HashSet::new();
        next_space_to_occupy.iter()
            .filter(|p| is_moveable(*grid.get(p)))
            .for_each(|p| {
                let val = *grid.get(p);
                if val == ']' {
                    next_boxes_to_move.insert(*p + Direction::LEFT);
                }
                if val == '[' {
                    next_boxes_to_move.insert(*p + Direction::RIGHT);
                }
                next_boxes_to_move.insert(*p);
            });
        
        rows_to_move.push_back(next_boxes_to_move.clone());
        cur_row = next_boxes_to_move;
    }

    while !rows_to_move.is_empty() {
        rows_to_move.pop_back().unwrap().iter().for_each(|pos| move_point(grid, *pos, dir));
    }

    true
}

fn move_point(grid: &mut Grid<char>, pos: Point, dir: Point) {
    grid.set(&(pos + dir), *grid.get(&pos));
    grid.set(&pos, '.');
}
fn is_moveable(c: char)                  -> bool { c == '[' || c == ']' || c == 'O'}
fn calc_gps(grid: &Grid<char>, val: char) -> i32 { grid.find(&val).iter().map(|p| 100*p.i + p.j).sum() }

fn find_empty_slot_from(mut p: Point, grid: &Grid<char>, dir: Point) -> Option<Point>{
    while grid.contains(&p) && *grid.get(&p) != '#' {
        if *grid.get(&p) == '.' {
            return Some(p);
        }
        p += dir;
    }
    return None;
}

fn parse_file(file_path: &str) -> (Vec<Vec<char>>, Vec<Point>) {
    let mut it = file_reader(file_path).lines();
    
    let chars: Vec<Vec<char>> = it.by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    
    let moves: Vec<Point> = it
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .map(|c| match c {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => panic!("Brah: {c}")
        })
        .collect();

    (chars, moves)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn make_grid(rows: Vec<&str>) -> Grid<char> {
        Grid::new_from_cells(rows.iter().map(|r| r.chars().collect::<Vec<_>>()).collect())
    }

    #[test]
    fn move_easy() {
        let mut grid = make_grid(vec![
            "....@....",
            "...[]....",
            "..[][]...",
            ".[][][]..",
            "#........",
        ]);
        let agent_pos = grid.find_single(&'@');
        execute_move(&mut grid, agent_pos, Direction::DOWN);
        assert_eq!(
            grid.get_cells(),
            make_grid(vec![
                ".........",
                "....@....",
                "...[]....",
                "..[][]...",
                "#[][][]..",
            ]).get_cells()
        );
    }

    #[test]
    fn cannot_move_easy() {
        let mut grid = make_grid(vec![
            "....@....",
            "...[]....",
            "..[][]...",
            ".[][][]..",
            "#.....#..",
        ]);
        let agent_pos = grid.find_single(&'@');

        execute_move(&mut grid, agent_pos, Direction::DOWN);

        assert_eq!(
            grid.get_cells(),
            make_grid(vec![
                "....@....",
                "...[]....",
                "..[][]...",
                ".[][][]..",
                "#.....#..",
            ]).get_cells()
        );
    }

    #[test]
    fn move_less_easy() {
        let mut grid = make_grid(vec![
            "....@....",
            "...[]....",
            "..[][]...",
            "...[]....",
            "..#......",
        ]);
        let agent_pos = grid.find_single(&'@');

        execute_move(&mut grid, agent_pos, Direction::DOWN);

        assert_eq!(
            grid.get_cells(),
            make_grid(vec![
                ".........",
                "....@....",
                "...[]....",
                "..[][]...",
                "..#[]....",
            ]).get_cells()
        );
    }

    #[test]
    fn cannot_move_less_easy() {
        let mut grid = make_grid(vec![
            "....@....",
            "...[]....",
            "..[][]...",
            "...[]....",
            "..##.....",
        ]);
        let agent_pos = grid.find_single(&'@');
        execute_move(&mut grid, agent_pos, Direction::DOWN);
        assert_eq!(
            grid.get_cells(),
            make_grid(vec![
                "....@....",
                "...[]....",
                "..[][]...",
                "...[]....",
                "..##.....",
            ]).get_cells()
        );
    }

    #[test]
    fn move_hard() {
        let mut grid = make_grid(vec![
            "....@.....",
            "....[]....",
            "...[][]...",
            "..[][][]..",
            "#[]..[]...",
            "[].##.[]..",
            ".[][]#....",
            "...###....",
        ]);
        let agent_pos = grid.find_single(&'@');
        execute_move(&mut grid, agent_pos, Direction::DOWN);
        assert_eq!(
            grid.get_cells(),
            make_grid(vec![
                "..........",
                "....@.....",
                "....[]....",
                "...[][]...",
                "#.[][][]..",
                ".[]##[]...",
                "[].[]#[]..",
                ".[]###....",
            ]).get_cells()
        );
    }

}
