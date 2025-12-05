fn main() {
    let s = include_str!("../../input/2025/dayx.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> usize {
    todo!()
}

fn solve_p2(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sample_p1() {
        let s = "";

        assert_eq!(solve_p1(s), 0)
    }

    #[test]
    #[ignore]
    fn sample_p2() {
        let s = "";

        assert_eq!(solve_p2(s), 0)
    }
}
