use ahash::{HashMap, HashMapExt};

fn main() {
    let s = include_str!("../../input/2025/day2.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn is_invalid(n: u64) -> bool {
    let num_digits = n.ilog10() + 1;

    if !num_digits.is_multiple_of(2) {
        return false;
    }

    let cut = 10_u64.pow(num_digits / 2);

    n % cut == n / cut
}

fn solve_p1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut cache: HashMap<u64, bool> = HashMap::new();

    let input = input.replace('\n', "");

    for pair in input.split_terminator(',') {
        let (start, end) = pair.split_once('-').unwrap();

        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for n in start..=end {
            if *cache.entry(n).or_insert_with(|| is_invalid(n)) {
                total += n;
            }
        }
    }

    total
}

// u64::MAX.ilog10() == 19
const PRIMES: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

fn is_invalid_p2(n: u64) -> bool {
    let num_digits = (n.ilog10() + 1) as u64;

    for s in PRIMES {
        if is_invalid_s(n, s, num_digits) {
            return true;
        }
    }

    false
}

fn is_invalid_s(n: u64, s: u64, num_digits: u64) -> bool {
    if !num_digits.is_multiple_of(s) {
        return false;
    }

    let cut = 10_u64.pow((num_digits / s) as u32);

    let lower = n % cut;

    let mut n = n / cut;

    while n != 0 {
        if (n % cut) != lower {
            return false;
        }

        n /= cut;
    }

    true
}

fn solve_p2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut cache: HashMap<u64, bool> = HashMap::new();

    let input = input.replace('\n', "");

    for pair in input.split_terminator(',') {
        let (start, end) = pair.split_once('-').unwrap();

        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for n in start..=end {
            if *cache.entry(n).or_insert_with(|| is_invalid_p2(n)) {
                total += n;
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1(SAMPLE), 1227775554)
    }

    #[test]
    fn sample_p2() {
        assert_eq!(solve_p2(SAMPLE), 4174379265)
    }
}
