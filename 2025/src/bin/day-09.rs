use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

fn main() {
    let s = include_str!("../../input/2025/day9.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> usize {
    let mut points = vec![];

    for line in s.lines() {
        let coords = line.split_once(',').unwrap();
        let x: usize = coords.0.parse().unwrap();
        let y: usize = coords.1.parse().unwrap();

        points.push((x, y));
    }

    let mut res = 0;

    for i in 0..points.len() {
        for j in 0..i {
            let a = compute_area(&points[i], &points[j]);
            if a > res {
                res = a;
            }
        }
    }

    res
}

fn compute_area(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let p = (a.0.min(b.0), a.1.min(b.1));
    let q = (a.0.max(b.0), a.1.max(b.1));

    (q.0 - p.0 + 1) * (q.1 - p.1 + 1)
}

fn solve_p2(s: &str) -> u64 {
    let mut points = vec![];

    for line in s.lines() {
        let coords = line.split_once(',').unwrap();

        let x: u64 = coords.0.parse().unwrap();
        let y: u64 = coords.1.parse().unwrap();

        points.push((x, y));
    }

    let mut rectangles = Vec::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in 0..i {
            let (a, b) = normalize(&points[i], &points[j]);
            let area = (b.0 - a.0 + 1) * (b.1 - a.1 + 1);

            rectangles.push((a, b, area));
        }
    }

    rectangles.sort_by_key(|(_, _, a)| *a);

    for (a, b, area) in rectangles.iter().rev() {
        let mut intersects = false;

        let n = points.len();

        points.push(*points.first().unwrap());

        for i in 0..n {
            let (p, q) = normalize(&points[i], &points[i + 1]);

            if !(b.0 <= p.0 || a.0 >= q.0 || b.1 <= p.1 || a.1 >= p.1) {
                intersects = true;
                break;
            }
        }

        if !intersects {
            // && is_interior(&points, &(a.0 + 1, a.1 + 1)) {
            return *area;
        }
    }

    unreachable!()
}

fn normalize(p: &(u64, u64), q: &(u64, u64)) -> ((u64, u64), (u64, u64)) {
    ((p.0.min(q.0), p.1.min(q.1)), (p.0.max(q.0), p.1.max(q.1)))
}

fn is_interior(points: &[(u64, u64)], p: &(u64, u64)) -> bool {
    // assumes 0,0 is outside

    let mut crossings = 0;

    let d = p.0.min(p.1);

    let mut q = (p.0 - d, p.1 - d);

    for _ in 1..=d {
        q = (q.0 + 1, q.1 + 1);
        if is_border(points, &q) {
            crossings += 1;
        }
    }

    if (crossings % 2) == 1 {
        return true;
    }

    false
}

fn is_border(points: &[(u64, u64)], p: &(u64, u64)) -> bool {
    if points.contains(p) {
        return true;
    }

    for i in 0..points.len() {
        for j in 0..points.len() {
            let a = points[i];
            let b = points[j];

            if a.0 == b.0 && p.0 == a.0 && a.1.min(b.1) < p.1 && p.1 < a.1.max(b.1) {
                return true;
            }

            if a.1 == b.1 && p.1 == a.1 && a.0.min(b.0) < p.0 && p.0 < a.0.max(b.0) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1(SAMPLE), 50)
    }

    #[test]
    fn sample_p2() {
        assert_eq!(solve_p2(SAMPLE), 24)
    }
}
