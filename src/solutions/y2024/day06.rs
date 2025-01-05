use std::collections::{HashMap, HashSet};

use crate::common::{Direction, Grid, Point};
use crate::common::io::read_file_as_2d_chars;

pub fn run(file_path: &str) -> (i64, i64) {
    let grid = Grid::new_from_cells(read_file_as_2d_chars(&file_path));
    let start = grid.find_single(&'^');

    let seen_points = walk_out_of_grid(&grid, &start);

    let jump_table = JumpTable::new_from_grid(&grid);
    let mut num_loops = 0;
    for point in &seen_points {
        if has_loop(&grid, &start, point, &jump_table) {
            num_loops += 1;
        }
    }

    (seen_points.len() as i64, num_loops)
}

fn walk_out_of_grid(grid: &Grid<char>, start: &Point) -> HashSet<Point> {
    let mut direction = Point{i:-1, j:0};
    let mut cur = *start;

    let mut seen_points = HashSet::new();

    loop {
        seen_points.insert(cur);

        let maybe_next = cur.step(&direction);
        if !grid.contains(&maybe_next) {
            return seen_points;
        }
        if *grid.get(&maybe_next) == '#' {
            direction.rotate_clockwise();
        } else {
            cur = maybe_next;
        }
    }
}

// Manual hash:
//   - Use 8 bits each for xy coordinates of point
//   - Use 2 bits for direction (as 4 possible values)
// Because hashmaps sucks that's why.
#[inline]
fn get_id(p: &Point, dir: &Point) -> usize { // res < 2^18
    // Exactly one of dir.i, dir.j is 0.
    (
        (p.i << 10) +
        (p.j << 2) +
        ((dir.i != 0) as i32)*(dir.i + 2) + // 1 or 3
        ((dir.j != 0) as i32)*(dir.j + 1)   // 0 or 2
    ) as usize
}

fn has_loop(grid: &Grid<char>, start: &Point, extra: &Point, jump_table: &JumpTable) -> bool {
    let mut direction = Point{i:-1, j:0};
    let mut i_cur = *start;

    let safe_to_jump = |p: &Point| p.i != extra.i && p.j != extra.j;
    let mut seen = vec![false; 1 << 18];

    loop {
        if safe_to_jump(&i_cur) {
            i_cur = jump_table.jump(&i_cur, &direction);
        }

        if seen[get_id(&i_cur, &direction)] {
            return true;
        }
        seen[get_id(&i_cur, &direction)] = true;

        let maybe_next = i_cur.step(&direction);
        if !grid.contains(&maybe_next) {
            return false;
        }
        if *grid.get(&maybe_next) == '#' || maybe_next == *extra {
            direction.rotate_clockwise();
        } else {
            i_cur = maybe_next;
        }
    }
}

// experiment with 2d vecs if too slow
struct JumpTable {
    up: HashMap<Point, Point>,
    right: HashMap<Point, Point>,
    down: HashMap<Point, Point>,
    left: HashMap<Point, Point>,
}

impl JumpTable {
    pub fn new_from_grid(grid: &Grid<char>) -> Self {
        let mut up = HashMap::new();
        for j in 0..grid.num_cols {
            Self::_elp_new(grid, Point::new(0, j), &Direction::DOWN, &mut up);
        }

        let mut right = HashMap::new();
        for i in 0..grid.num_rows {
            Self::_elp_new(grid, Point::new(i,grid.num_cols - 1), &Direction::LEFT, &mut right);
        }

        let mut down = HashMap::new();
        for j in (0..grid.num_cols).rev() {
            Self::_elp_new(grid, Point::new(grid.num_rows - 1, j), &Direction::UP, &mut down);
        }

        let mut left = HashMap::new();
        for i in 0..grid.num_rows {
            Self::_elp_new(grid, Point::new(i,0), &Direction::RIGHT, &mut left);
        }

        JumpTable{up, right, down, left}
    }

    fn _elp_new(grid: &Grid<char>, i_start: Point, dir: &Point, jumps: &mut HashMap<Point, Point>) {
        let mut i_cur = i_start;
        let mut jump_point: Option<Point> = None;
        while grid.contains(&i_cur) {
            match grid.get(&i_cur) {
                '#' => jump_point = None,
                '.' | '^' => {
                    if jump_point.is_none() {
                        jump_point = Some(i_cur);
                    }
                    jumps.insert(i_cur, jump_point.unwrap());
                },
                _ => unreachable!("bad character: '{}'", *grid.get(&i_cur))
            }
            i_cur += *dir;
        }
    }

    fn jump(&self, i_start: &Point, dir: &Point) -> Point {
        match *dir {
            Direction::UP => *self.up.get(i_start).unwrap(),
            Direction::RIGHT => *self.right.get(i_start).unwrap(),
            Direction::DOWN => *self.down.get(i_start).unwrap(),
            Direction::LEFT => *self.left.get(i_start).unwrap(),
            _ => panic!("Bad direction"),
        }
    }
}
