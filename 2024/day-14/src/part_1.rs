use anyhow::Result;
use scanf::sscanf;

struct Map {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn update(&mut self, map: &Map) {
        self.p = (
            (self.p.0 + self.v.0).rem_euclid(map.width),
            (self.p.1 + self.v.1).rem_euclid(map.height),
        )
    }
}
fn solve_size(input: &'static str, steps: usize, width: i32, height: i32) -> Result<String> {
    let mut robots = Vec::<Robot>::new();

    for line in input.lines() {
        let mut x = 0_i32;
        let mut y = 0_i32;

        let mut u = 0_i32;
        let mut v = 0_i32;

        sscanf!(line, "p={},{} v={},{}", x, y, u, v).unwrap();

        robots.push(Robot {
            p: (x, y),
            v: (u, v),
        });
    }

    let map = Map { width, height };

    for _ in 0..steps {
        robots.iter_mut().for_each(|robot| robot.update(&map));
    }

    let mut c1 = 0_i32;
    let mut c2 = 0_i32;
    let mut c3 = 0_i32;
    let mut c4 = 0_i32;

    for robot in robots {
        if robot.p.0 < width / 2 && robot.p.1 < height / 2 {
            c1 += 1;
        }

        if robot.p.0 < width / 2 && height / 2 + 1 <= robot.p.1 {
            c2 += 1;
        }

        if width / 2 + 1 <= robot.p.0 && robot.p.1 < height / 2 {
            c3 += 1;
        }

        if width / 2 + 1 <= robot.p.0 && height / 2 + 1 <= robot.p.1 {
            c4 += 1;
        }
    }
    Ok((c1 * c2 * c3 * c4).to_string())
}

pub fn solve(input: &'static str) -> Result<String> {
    solve_size(input, 100, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn solve_sample() {
        let result = solve_size(SAMPLE, 100, 11, 7).unwrap();

        assert_eq!(result, "12");
    }
}
