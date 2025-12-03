fn main() {
    let s = include_str!("../../input/2025/day3.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> u32 {
    let mut result: u32 = 0;

    for line in s.lines() {
        let mut m = 48;
        let mut j = 48;

        for (i, b) in line.bytes().enumerate() {
            if i == line.len() - 1 {
                continue;
            }
            if b > m {
                m = b;
                j = i;
            }
        }

        let n = line.bytes().skip(j + 1).max().unwrap();
        result += (m as u32 - 48) * 10 + n as u32 - 48;
    }

    result
}

fn next_masked(bytes: &[u8], bytemask: &[bool], mut j: usize) -> Option<u8> {
    while j < bytemask.len() {
        if bytemask[j] {
            return Some(bytes[j]);
        }
        j += 1;
    }

    None
}

fn discard_one(bytes: &[u8], bytemask: &mut [bool]) {
    for i in 0..(bytemask.len() - 1) {
        if !bytemask[i] {
            continue;
        }
        let Some(next) = next_masked(bytes, bytemask, i + 1) else {
            continue;
        };
        if bytes[i] < next {
            bytemask[i] = false;
            return;
        }
    }

    *bytemask.last_mut().unwrap() = false;
}

fn discard_many(bytes: &[u8], bytemask: &mut [bool], how_many: usize) {
    for _ in 0..how_many {
        discard_one(bytes, bytemask);
    }
}

fn solve_p2b(s: &str) -> u64 {
    let mut result: u64 = 0;

    for line in s.lines() {
        let bytes = line.as_bytes();

        let num_discard = line.len() - 12;
        let mut bytemask: Box<[_]> = (0..line.len()).map(|_| true).collect();

        discard_many(bytes, &mut bytemask, num_discard);

        let mut p: u64 = 0;
        let mut f: u64 = 10_u64.pow(11);

        for (_, byte) in bytes.iter().enumerate().filter(|(i, _)| bytemask[*i]) {
            p += (*byte as u64 - 48) * f;
            f /= 10;
        }

        result += p;
    }

    result
}

fn solve_p2(s: &str) -> u64 {
    let mut result: u64 = 0;

    for line in s.lines() {
        let bytes = line.as_bytes();

        let mut j = 0;
        let mut current = 0;

        for remaining in (0..12).rev() {
            let (i, n) = bytes[j..(line.len() - remaining)].iter().enumerate().fold(
                (0, 0),
                |(i, n), (k, v)| if *v > n { (k, *v) } else { (i, n) },
            );

            j += i + 1;
            current *= 10;
            current += n as u64 - 48;
        }

        result += current;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sample_p1() {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(solve_p1(s), 357)
    }

    #[test]
    fn sample_p2() {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(solve_p2(s), 3121910778619)
    }
}
