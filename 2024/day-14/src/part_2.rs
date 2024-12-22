use anyhow::Result;
use itertools::Itertools;
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

fn solve_size(input: &'static str, width: i32, height: i32) -> Result<String> {
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

    let mut i = 0;
    let secs = loop {
        robots.iter_mut().for_each(|robot| robot.update(&map));

        i += 1;
        if robots.iter().map(|robot| robot.p).all_unique() {
            break i;
        }
    };

    for y in 0..height {
        for x in 0..width {
            let r = robots
                .iter()
                .filter(|robot| robot.p.0 == x && robot.p.1 == y)
                .count();

            if r == 0 {
                print!(".");
            } else {
                print!("{r}")
            }
        }
        println!();
    }

    Ok(secs.to_string())
}

pub fn solve(input: &'static str) -> Result<String> {
    solve_size(input, 101, 103)
}
