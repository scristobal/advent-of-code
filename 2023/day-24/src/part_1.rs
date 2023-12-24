/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use glam::{Vec2, Vec3};
use itertools::Itertools;

#[derive(Debug)]
struct Particle {
    p: Vec2,
    v: Vec2,
}

struct Env {
    min: f32,
    max: f32,
}

fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();

            let p: Vec<_> = p.split(',').collect();
            let p: Vec<_> = p.iter().map(|s| s.trim().parse().unwrap()).collect();

            let p = Vec2::new(p[0], p[1]);

            let v: Vec<_> = v.split(',').collect();

            let v: Vec<_> = v.iter().map(|s| s.trim().parse().unwrap()).collect();

            let v = Vec2::new(v[0], v[1]);

            Particle { p, v }
        })
        .collect()
}

impl Particle {
    fn homogeneous(&self) -> Vec3 {
        Vec3::new(-self.v.y, -self.v.x, self.p.perp_dot(self.v))
    }
}

impl Env {
    fn are_intersecting(&self, n: &Particle, m: &Particle) -> bool {
        let w_n = n.homogeneous();
        let w_m = m.homogeneous();

        let p = w_n.cross(w_m);

        // no intersection in affine space
        if p.z.abs() < 1e-6 {
            return false;
        }

        let p = Vec2::new(p.x, -p.y) / p.z;

        let t_n = (p.x - n.p.x) / n.v.x;
        let t_m = (p.x - m.p.x) / m.v.x;

        0.0 < t_n
            && 0.0 < t_m
            && self.min <= p.x
            && p.x <= self.max
            && self.min <= p.y
            && p.y <= self.max
    }

    fn count_collisions(&self, particles: &[Particle]) -> i64 {
        particles
            .iter()
            .enumerate()
            .cartesian_product(particles.iter().enumerate())
            .filter_map(|((i, n), (j, m))| (i < j && self.are_intersecting(n, m)).then_some((n, m)))
            .count()
            .try_into()
            .unwrap()
    }
}

pub fn solve(input: &'static str) -> String {
    let particles = parse_input(input);

    let env = Env {
        min: 200_000_000_000_000.0,
        max: 400_000_000_000_000.0,
    };

    env.count_collisions(&particles).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let particles = parse_input(SAMPLE);

        let env = Env {
            min: 7.0,
            max: 27.0,
        };

        let result = env.count_collisions(&particles).to_string();

        assert_eq!(result, "2");
    }
}
