use std::{env, fs};
use dotenv::dotenv;

mod solutions;
mod common;
mod expected;

const INPUT_FILES_DIR_NAME: &str = "input-files";

enum AocArgs {
    RunAll,
    RunDay(u32, String, bool) // (day_num, filepath, is_custom_input)
}

type SolutionFunction<T, V> = fn(filename: &str) -> (T, V);

fn main() {
    dotenv().ok();

    let args = AocArgs::new_from_args();
    match args {
        AocArgs::RunAll => run_all_days(),
        AocArgs::RunDay(day, filepath, is_custom) => {
            if !is_custom {
                ensure_aoc_input_exists(day);
            }
            run_day(day, &filepath);
        }
    }
}


fn run_all_days() {
    for day in 1..19 {
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
        17 => run(input_file_path, solutions::run_day17, day),
        18 => run(input_file_path, solutions::run_day18, day),
        _ => panic!("Having a bad day: {day}")
    }
}

fn run<T: std::fmt::Display, V: std::fmt::Display>(input_file_path: &str, runnable: SolutionFunction<T, V>, day: u32) {
    println!("Running day {day}:");
    let (t_part1, t_part2) = runnable(input_file_path);
    let pretty_print = |part1_and_color: Option<(&str, &str)>, part2_and_color: Option<(&str, &str)>| {
        match part1_and_color {
            Some((res, color)) => print!("  part1: {color}{:width$}\x1b[0m", res, width=18),
            None => print!("{}", " ".repeat(27))
        }
        match part2_and_color {
            Some((res, color)) => println!("part2: {color}{res}\x1b[0m"),
            None => println!()
        }
    };
    let color = |is_correct: Option<bool>| match is_correct {
        Some(true) => "\x1b[0;32m", // green
        Some(false) => "\x1b[0;31m", // red
        None => ""
    };

    let (part1, part2) = (t_part1.to_string(), t_part2.to_string());
    match expected::SOLUTIONS.get(&day) {
        Some((e1, e2)) => {
            let part1_correct = part1 == *e1;
            let part2_correct = part2 == *e2;
            pretty_print(
                Some((&part1, &color(Some(part1_correct)))),
                Some((&part2, &color(Some(part2_correct))))
            );
            if !part1_correct || !part2_correct {
                let expected_part_1 = match part1_correct {
                    true => None,
                    false => Some((*e1, color(Some(true))))
                };
                let expected_part_2 = match part2_correct {
                    true => None,
                    false => Some((*e2, color(Some(true))))
                };
                pretty_print(expected_part_1, expected_part_2);
            }
        },
        None => pretty_print(Some((&part1, &color(None))), Some((&part2, &color(None))))
    }
}

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

fn aoc_file_path(day: u32) -> String { format!("{INPUT_FILES_DIR_NAME}/day{:0>2}.txt", day) }

impl AocArgs {
    fn new_from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        if args[1].as_str() == "all" {
            return Self::RunAll;
        }

        let day = args[1].as_str().parse::<u32>().expect("Malformed day argument.");
        let (filepath, is_custom) = match args.get(2) {
            Some(path) => (path.to_string(), false),
            None => (aoc_file_path(day), true)
        };
        Self::RunDay(day, filepath, is_custom)
    }
}
