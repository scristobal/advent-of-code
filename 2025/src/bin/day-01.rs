fn main() {
    let s = include_str!("../../input/2025/day1.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> u32 {
    let mut num_zeros: u32 = 0;
    let mut current: i32 = 50;

    for line in s.lines() {
        let delta = line[1..].parse::<i32>().unwrap();

        match line.chars().next().unwrap() {
            'R' => current += delta,
            'L' => current -= delta,
            _ => unreachable!(),
        }

        current = current.rem_euclid(100);

        num_zeros += (current == 0) as u32;
    }

    num_zeros
}

fn solve_p2(s: &str) -> i32 {
    let mut num_zeros: i32 = 0;
    let mut current: i32 = 50;

    for line in s.lines() {
        let delta = line[1..].parse::<i32>().unwrap();

        match line.chars().next().unwrap() {
            'R' => {
                num_zeros += (current + delta).div_euclid(100);
                current = (current + delta).rem_euclid(100);
            }
            'L' => {
                num_zeros += (delta + 100 - current).div_euclid(100) - (current == 0) as i32;
                current = (current - delta).rem_euclid(100);
            }
            _ => unreachable!(),
        };
    }

    num_zeros
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sample_p1() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solve_p1(s), 3)
    }

    #[test]
    fn sample_p2() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solve_p2(s), 6)
    }
}
