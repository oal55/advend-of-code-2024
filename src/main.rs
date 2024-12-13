use reqwest;
use std::{env, fs};
use dotenv::dotenv;


mod solutions;
mod common;

const INPUT_FILES_DIR_NAME: &str = "input-files";


type SolutionFunction<T> = fn(filename: &str) -> (T, T);

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str().parse::<u32>().expect("Malformed day argument.");
    let custom_file_arg = args.get(2);
    if custom_file_arg.is_none() {
        ensure_aoc_input_exists(day);
    }
    
    let input_file_path = match custom_file_arg {
        Some(filepath) => filepath.to_string(),
        None => aoc_file_path(day)
    };
    match day {
        1 => run(&input_file_path, solutions::run_day01),
        2 => run(&input_file_path, solutions::run_day02),
        3 => run(&input_file_path, solutions::run_day03),
        4 => run(&input_file_path, solutions::run_day04),
        5 => run(&input_file_path, solutions::run_day05),
        6 => run(&input_file_path, solutions::run_day06),
        7 => run(&input_file_path, solutions::run_day07),
        8 => run(&input_file_path, solutions::run_day08),
        9 => run(&input_file_path, solutions::run_day09),
        10 => run(&input_file_path, solutions::run_day10),
        _ => panic!("Having a bad day")
    }
}

fn run<T: std::fmt::Display>(input_file_path: &String, runnable: SolutionFunction<T>) {
    let (part1, part2) = runnable(input_file_path);
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn aoc_file_path(day_arg: u32) -> String { format!("{INPUT_FILES_DIR_NAME}/day{:0>2}.txt", day_arg) }

fn ensure_aoc_input_exists(day: u32) {
    let relative_filepath = aoc_file_path(day);
    let file_exists = fs::exists(&relative_filepath).expect(format!("Cannot confirm whether file exists at {}", relative_filepath).as_str());
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
