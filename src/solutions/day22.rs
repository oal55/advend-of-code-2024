use std::{collections::HashMap, io::BufRead};
use crate::common::io::file_reader;

const MOD: i64 = 16777216;
const MASK: i64 = MOD - 1;

pub fn run(file_path: &str) -> (i64, usize) {
    let seeds = file_reader(file_path).lines().into_iter()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seen_seqs: HashMap<u32, u32> = HashMap::new();
    let mut sum_2k_iter = 0i64;
    for seed in seeds {
        let (res, seqs) = dumber_2000(seed);
        sum_2k_iter += res;
        seqs.iter().for_each(|(&k, &v)| *seen_seqs.entry(k).or_default() += v);
    }
    let max_bananas = seen_seqs.values().max().unwrap();

    assert_eq!(*max_bananas, 1628);
    assert_eq!(sum_2k_iter, 14392541715);

    (sum_2k_iter, *max_bananas as usize)
}

fn dumber_2000(start: i64) -> (i64, HashMap<u32, u32>) {
    let mask = 0x00_FF_FF_FF; // Fs for fml.

    let mut seen_seqs = HashMap::new();
    let mut res = start;
    let mut diffs = 0u32;
    for _ in 0..3 {
        let next_res = next(res);
        let diff = (next_res % 10 - res % 10) as u8;
        diffs = ((mask & diffs) << 8) | (diff as u32);
        res = next_res;
    }

    for _ in 3..2000 {
        let next_res = next(res);
        let diff = (next_res % 10 - res % 10) as u8;
        diffs = ((mask & diffs) << 8) | (diff as u32);
        res = next_res;
        seen_seqs.entry(diffs).or_insert_with(|| (res % 10) as u32); 
    }
    (res, seen_seqs)
}

fn next(mut secret: i64) -> i64 {
    secret = ((secret << 6) ^ secret) & MASK; // equivalent of % MOD in this case.
    secret = (secret >> 5) ^ secret;
    secret = ((secret << 11) ^ secret) & MASK;
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let nums: Vec<i64> = vec![
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254
        ];
        nums.iter().zip(nums.iter().skip(1))
            .for_each(|(&fi, &se)| assert_eq!(next(fi), se));
    }
}
