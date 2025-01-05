use std::{env, fs, time::{Duration, Instant}};
use dotenv::dotenv;
use solutions::get_solution_function;

mod solutions;
mod common;
mod expected;

const INPUT_FILES_DIR_NAME: &str = "input-files";

enum AocArgs {
    RunAll,
    RunDay(u32, String, bool) // (day_num, filepath, is_custom_input)
}

fn main() {
    dotenv().ok();

    let args = AocArgs::new_from_args();
    match args {
        AocArgs::RunAll => run_all_days(),
        AocArgs::RunDay(day, filepath, is_custom) => {
            if !is_custom {
                ensure_aoc_input_exists(day);
            }
            let (part1, part2, duration) = run_day(2024, day, &filepath);
            println!("Day {day}:");
            println!("  part1: {:width$}  part2: {}", part1, part2, width=18);
            println!("  took - {:.2?}", duration);
        }
    }
}

fn run_all_days() {
    let color = |is_correct: bool| match is_correct {
        true => "\x1b[0;32m", // green
        false => "\x1b[0;31m" // red
    };
    let expected = |expected_val: &str, was_output_correct: bool| -> Option<(String, bool)> {
        match was_output_correct {
            true => None,
            false => Some((expected_val.to_string(), true))
        }
    };
    let pretty_print = |part1: Option<(String, bool)>, part2: Option<(String, bool)>| {
        match part1 {
            Some((res, is_correct)) => print!("  part1: {}{:width$}\x1b[0m", color(is_correct), res, width=18),
            None => print!("{}", " ".repeat(27))
        }
        match part2 {
            Some((res, is_correct)) => println!("part2: {}{res}\x1b[0m", color(is_correct)),
            None => println!()
        }
    };

    for (day, (expected_part1, expected_part2)) in expected::SOLUTIONS.iter().copied() {
        ensure_aoc_input_exists(day); // maybe extract this into a module that handles load instead of spamming the website 
        println!("Running day {day}:");
        let (part1, part2, duration) = run_day(2024, day, &aoc_file_path(day));
        let part1_correct = part1 == *expected_part1;
        let part2_correct = part2 == *expected_part2;
        pretty_print(Some((part1, part1_correct)), Some((part2, part2_correct)));
        if !part1_correct || !part2_correct {
            pretty_print(
                expected(expected_part1, part1_correct),
                expected(expected_part2, part2_correct)
            );
        }
        println!("  took - {:.2?}", duration);
    }
}

fn run_day(year: u32, day: u32, input_file_path: &str) -> (String, String, Duration) {
    let runnable = get_solution_function(year, day);
    
    let start = Instant::now();
    let (t_part1, t_part2) = runnable(input_file_path);
    let elapsed = start.elapsed();
    (t_part1.to_string(), t_part2.to_string(), elapsed)
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
