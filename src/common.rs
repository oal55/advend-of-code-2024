use std::fs::File;
use std::io::BufReader;

pub fn file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect(format!("Cannot open file at: {}", file_path).as_str());
    return BufReader::new(file);
}
