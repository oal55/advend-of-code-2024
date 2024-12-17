use std::{collections::HashMap, sync::LazyLock};
use std::io::BufRead;
use regex::Regex;

use crate::common::Point;
use crate::common::io::file_reader;

static LINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    let int_re = r"(-?\d+)";
    return Regex::new(format!("p={int_re},{int_re} v={int_re},{int_re}").as_str()).unwrap()
});

// const I_MAX: i32 = 11; // 103
// const J_MAX: i32 = 7; // 101
const I_MAX: i32 = 103;
const J_MAX: i32 = 101;

struct Bot {
    pos: Point,
    velocity: Point,
}

fn into_bounds(n: i32, size: i32) -> i32 { (n % size + size) % size } // shenanigans because (-17 % 10 == -7) in rust
pub fn run(file_path: &str) -> (u32, u32) {
    let bots = parse_bots(file_path);

    let mut pos_to_count: HashMap<Point, u32> = HashMap::new();
    for bot in bots {
        let moved = bot.pos + bot.velocity.times(100);
        let final_pos = Point{i: into_bounds(moved.i, I_MAX), j: into_bounds(moved.j, J_MAX)};
        *pos_to_count.entry(final_pos).or_default() += 1
    }

    let mut quadrants: [u32; 4] = [0, 0, 0, 0];
    for (pos, num_bots) in pos_to_count {
        
        if pos.i == I_MAX/2 || pos.j == J_MAX/2 {
            continue;
        }
        let i_quant: usize = ((pos.i > I_MAX/2) as usize) * 2 + (pos.j > J_MAX/2) as usize;
        quadrants[i_quant] += num_bots;
    }

    (quadrants.into_iter().fold(1, |acc, cur| acc * cur), 0) }

fn parse_bots(file_path: &str) -> Vec<Bot> {
    return file_reader(file_path).lines().into_iter()
        .map(|line| parse_line(line.unwrap()))
        .collect();
}

fn parse_line(line: String) -> Bot {
    let (_, [px,py,vx,vy]) = LINE_RE.captures(&line)
        .expect(format!("Re doesn't match line: {line}").as_str())
        .extract();
    return Bot{
        pos: Point{i: py.parse().unwrap(), j: px.parse().unwrap()},
        velocity: Point{i: vy.parse().unwrap(), j: vx.parse().unwrap()}
    }
}

// move the python code here o,o