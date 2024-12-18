use std::io::BufRead;
use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let reader = file_reader(file_path);
    (0, 0)
}
