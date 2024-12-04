use std::env;

mod day01;
mod day02;
mod common;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => run(day01::run),
        "2" => run(day02::run),
        _ => panic!("Bad argument")
    }
}

fn run(runnable: fn() -> (i64, i64)) { // Make this generic
    let (part1, part2) = runnable();
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
