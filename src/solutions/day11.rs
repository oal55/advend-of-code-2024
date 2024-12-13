use std::{collections::HashMap, sync::LazyLock};
use crate::common::read_file;

static TENS_POWERS: LazyLock<Vec<u64>> = LazyLock::new(|| {
    let mut res = Vec::new();
    let mut pow = 1u64;
    for _ in 0..19 {
        res.push(pow);
        pow *= 10;
    }
    return res;
});

pub fn run(file_path: &str) -> (u64, u64) {

    let stones = read_file(file_path)
        .split_ascii_whitespace().into_iter()
        .map(|num| num.parse::<u64>().expect(format!("Unable to parse: {num}").as_str()))
        .fold(HashMap::<u64, u64>::new(), |mut acc, c| {
            *acc.entry(c).or_default() += 1;
            return acc
        });

    
    let blink_25 = (0..25).fold(stones.clone(), |cur_stones, _| blink(cur_stones));
    let blink_75 = (0..75).fold(stones.clone(), |cur_stones, _| blink(cur_stones));

    return (
        blink_25.values().sum(),
        blink_75.values().sum()
    );
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut res = HashMap::new();
    for (number, count) in stones {
        let num_digits = match TENS_POWERS.binary_search(&number) {
            Ok(i_found) => i_found + 1, // e.g. 100 would be at index 2, has 3 digits.
            Err(i_would_have_inserted) => i_would_have_inserted
        };
        if number == 0 {
            *res.entry(1).or_default() += count;
        } else if num_digits % 2 == 0 {
            let coeff = TENS_POWERS[num_digits/2];
            *res.entry(number/coeff).or_default() += count;
            *res.entry(number%coeff).or_default() += count;
        } else {
            *res.entry(number*2024).or_default() += count;
        }
    }
    // println!("res: {:?}", res);
    return res;
}