mod y2024;

use y2024::day01 as run_2024_day_01;
use y2024::day02 as run_2024_day_02;
use y2024::day03 as run_2024_day_03;
use y2024::day04 as run_2024_day_04;
use y2024::day05 as run_2024_day_05;
use y2024::day06 as run_2024_day_06;
use y2024::day07 as run_2024_day_07;
use y2024::day08 as run_2024_day_08;
use y2024::day09 as run_2024_day_09;
use y2024::day10 as run_2024_day_10;
use y2024::day11 as run_2024_day_11;
use y2024::day12 as run_2024_day_12;
use y2024::day13 as run_2024_day_13;
use y2024::day14 as run_2024_day_14;
use y2024::day15 as run_2024_day_15;
use y2024::day16 as run_2024_day_16;
use y2024::day17 as run_2024_day_17;
use y2024::day18 as run_2024_day_18;
use y2024::day19 as run_2024_day_19;
use y2024::day20 as run_2024_day_20;
use y2024::day22 as run_2024_day_22;
use y2024::day23 as run_2024_day_23;
use y2024::day24 as run_2024_day_24;
use y2024::day25 as run_2024_day_25;


type SolutionFunction<T, V> = fn(filename: &str) -> (T, V);
type DisplayFunction = Box<dyn Fn(&str) -> (String, String)>;

pub fn get_solution_function(year: u32, day: u32) -> DisplayFunction {
    match year {
        2024 => match day {
            1 => wrap(run_2024_day_01),
            2 => wrap(run_2024_day_02),
            3 => wrap(run_2024_day_03),
            4 => wrap(run_2024_day_04),
            5 => wrap(run_2024_day_05),
            6 => wrap(run_2024_day_06),
            7 => wrap(run_2024_day_07),
            8 => wrap(run_2024_day_08),
            9 => wrap(run_2024_day_09),
            10 => wrap(run_2024_day_10),
            11 => wrap(run_2024_day_11),
            12 => wrap(run_2024_day_12),
            13 => wrap(run_2024_day_13),
            14 => wrap(run_2024_day_14),
            15 => wrap(run_2024_day_15),
            16 => wrap(run_2024_day_16),
            17 => wrap(run_2024_day_17),
            18 => wrap(run_2024_day_18),
            19 => wrap(run_2024_day_19),
            20 => wrap(run_2024_day_20),
            22 => wrap(run_2024_day_22),
            23 => wrap(run_2024_day_23),
            24 => wrap(run_2024_day_24),
            25 => wrap(run_2024_day_25),
           _ => panic!("Bad year")
        },
        _ => panic!("Bad year")
    }
}

fn wrap<T, V>(f: SolutionFunction<T, V>) -> DisplayFunction
where
    T: std::fmt::Display + 'static,
    V: std::fmt::Display + 'static,
{
    Box::new(move | filepath: &str | -> (String, String) {
        let (t, v) = f(filepath);
        (t.to_string(), v.to_string())
    })
}
