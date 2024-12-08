use std::env;

mod solutions;
mod common;

type SolutionFunction<T> = fn(filename: &str) -> (T, T);

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = get_input_file_path(&args);
    match args[1].as_str() {
        "1" => run(&input_file_path, solutions::run_day01),
        "2" => run(&input_file_path, solutions::run_day02),
        "3" => run(&input_file_path, solutions::run_day03),
        "4" => run(&input_file_path, solutions::run_day04),
        _ => panic!("Bad argument")
    }
}

fn run<T: std::fmt::Display>(input_file_path: &String, runnable: SolutionFunction<T>) {
    let (part1, part2) = runnable(input_file_path);
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn get_input_file_path(args: &Vec<String>) -> String {
    return match  args.get(2) {
        Some(filepath) => filepath.to_string(),
        None => format!("input-files/day{:0>2}.txt", args[1]),
    }
}
