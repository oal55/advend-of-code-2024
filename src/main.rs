use std::{env, fs};
use dotenv::dotenv;


mod solutions;
mod common;
mod expected;

const INPUT_FILES_DIR_NAME: &str = "input-files";


type SolutionFunction<T> = fn(filename: &str) -> (T, T);

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args[1].as_str() == "all" {
        return run_all_days();
    }

    let day = args[1].as_str().parse::<u32>().expect("Malformed day argument.");
    let custom_file_arg = args.get(2);
    if custom_file_arg.is_none() {
        ensure_aoc_input_exists(day);
    }
    
    let input_file_path = match custom_file_arg {
        Some(filepath) => filepath.to_string(),
        None => aoc_file_path(day)
    };
    run_day(day, &input_file_path);
}

fn run_all_days() {
    for day in 1..15 {
        ensure_aoc_input_exists(day);
        run_day(day, &aoc_file_path(day));
    }
}

fn run_day(day: u32, input_file_path: &str) {
    match day {
        1 => run(input_file_path, solutions::run_day01, day),
        2 => run(input_file_path, solutions::run_day02, day),
        3 => run(input_file_path, solutions::run_day03, day),
        4 => run(input_file_path, solutions::run_day04, day),
        5 => run(input_file_path, solutions::run_day05, day),
        6 => run(input_file_path, solutions::run_day06, day),
        7 => run(input_file_path, solutions::run_day07, day),
        8 => run(input_file_path, solutions::run_day08, day),
        9 => run(input_file_path, solutions::run_day09, day),
        10 => run(input_file_path, solutions::run_day10, day),
        11 => run(input_file_path, solutions::run_day11, day),
        12 => run(input_file_path, solutions::run_day12, day),
        13 => run(input_file_path, solutions::run_day13, day),
        14 => run(input_file_path, solutions::run_day14, day),
        15 => run(input_file_path, solutions::run_day15, day),
        16 => run(input_file_path, solutions::run_day16, day),
        _ => panic!("Having a bad day: {day}")
    }
}

fn run<T: std::fmt::Display>(input_file_path: &str, runnable: SolutionFunction<T>, day: u32) {
    println!("Running day {day}");
    let (part1, part2) = runnable(input_file_path);
    println!("  part1: {}", part1);
    println!("  Part2: {}", part2);
    if let Some((expected_1, expected_2)) = expected::SOLUTIONS.get(&day) {
        assert_eq!(part1.to_string(), expected_1.to_string());
        assert_eq!(part2.to_string(), expected_2.to_string());
    }
}

fn aoc_file_path(day: u32) -> String { format!("{INPUT_FILES_DIR_NAME}/day{:0>2}.txt", day) }

fn ensure_aoc_input_exists(day: u32) {
    let relative_filepath = aoc_file_path(day);
    let file_exists = fs::exists(&relative_filepath).unwrap_or_else(|_| panic!("Cannot confirm whether file exists at {}", relative_filepath));
    if file_exists {
        return
    }
    println!("Fetching input file");

    let session_id = env::var("SESSION_ID").expect("No session id in env");
    let url = format!("https://adventofcode.com/2024/day/{day}/input");
    let response = reqwest::blocking::Client::new().get(url)
        .header("cookie", format!("session={session_id}"))
        .send()
        .expect("Expected better things from reqwest");
    let contents = response.text().expect("Cannot read response text");
    fs::write(relative_filepath, &contents).expect("Cannot write file.");
}
