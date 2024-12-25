use crate::common::io::read_file;

enum Schematic {
    Lock(u64),
    Key(u64)
}

pub fn run(file_path: &str) -> (u32, u32) {
    let (mut keys, mut locks) = (Vec::new(), Vec::new());
    read_file(file_path).split("\n\n").map(parse_schematic).for_each(|s| match s {
        Schematic::Key(num) => keys.push(num),
        Schematic::Lock(num) => locks.push(num)
    });

    let mut count = 0;
    for key in keys {
        for lock in &locks {
            count += (key & lock == 0) as u32;
        }
    }
    (count, 0)
}

fn parse_schematic(stanza: &str) -> Schematic {
    let is_lock = stanza.starts_with("#");
    let num = stanza.chars().fold(0u64, |res, c| match c {
        '#' => (res << 1) + 1,
        '.' => res << 1,
        '\n' | '\r' => res, // ignore new lines
        _ => panic!("Unexpected char: {c}")
    });
    if is_lock {
        return  Schematic::Lock(num);
    }
    Schematic::Key(num)
}