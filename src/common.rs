use std::fs::File;
use std::io::BufReader;

pub fn file_reader(day_num:  i64) -> BufReader<File> {
    let filepath = format!("input-files/day{:02}.txt", day_num);
    let file = File::open(&filepath).expect(format!("Cannot open file at: {}", &filepath).as_str());
    return BufReader::new(file);
}
