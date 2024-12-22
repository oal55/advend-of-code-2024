use std::io::BufRead;
use crate::common::io::file_reader;

const MOD: usize = 16777216;
const MASK: usize = MOD - 1;

pub fn run(file_path: &str) -> (usize, usize) {
    let seeds = file_reader(file_path).lines().into_iter()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();


    let mut sum_2k_iter = 0;
    for seed in seeds {
        let ho = number_2000(seed);
        sum_2k_iter += ho;
    }

    (sum_2k_iter, 0)
}

fn number_2000(start: usize) -> usize {
    let mut res = start;
    for _ in 0..2000 {
        res = next(res)
    }
    res
}

fn next(mut secret: usize) -> usize {
    secret = ((secret << 6) ^ secret) & MASK; // equivalent of % MOD in this case.
    secret = (secret >> 5) ^ secret;
    secret = ((secret << 11) ^ secret) & MASK;
    secret
}

#[cfg(test)]
mod tests {
    use super::next;

    #[test]
    fn test_next() {
        let nums: Vec<usize> = vec![
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
