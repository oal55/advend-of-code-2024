use std::io::BufRead;
use crate::common::io::file_reader;

const MOD: i64 = 16777216;
const MASK: i64 = MOD - 1;
const DIFF_MASK: u32 = 0x00FF_FFFF;

pub fn run(file_path: &str) -> (i64, usize) {
    let seeds = file_reader(file_path).lines().into_iter()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    
    let mut all_seen_seqs = vec![0; 130321]; // 19 ^ 4. "Diffs" have to be in range [-9, 9] -> 19 possible values
    let mut sum_2k_iter = 0i64;
    for seed in seeds {
        let mut seen_seqs = vec![0; 130321];
        let mut res = seed;
        let mut diffs = 0u32;
        for _ in 0..3 {
            let next_res = next(res);
            let diff = ((next_res % 10 - res % 10) + 9) as u8; // this is never negative.
            diffs = ((DIFF_MASK & diffs) << 8) | (diff as u32);
            res = next_res;
        }

        for _ in 3..2000 {
            let next_res = next(res);
            let diff = ((next_res % 10 - res % 10) + 9) as u8;
            diffs = ((DIFF_MASK & diffs) << 8) | (diff as u32);
            res = next_res;

            let i_seq = (
                (diffs & 0xFF) +
                19 * ((diffs & 0xFF00) >> 8) +
                19 * 19 * ((diffs & 0xFF0000) >> 16) +
                19 * 19 * 19 * ((diffs & 0xFF000000) >> 24)
            ) as usize;

            if seen_seqs[i_seq] == 0 {
                all_seen_seqs[i_seq] += (res % 10) as i32;
                seen_seqs[i_seq] = 1;
            }
        }

        sum_2k_iter += res; 
    }
    let max_bananas = all_seen_seqs.iter().max().unwrap();

    (sum_2k_iter, *max_bananas as usize)
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
