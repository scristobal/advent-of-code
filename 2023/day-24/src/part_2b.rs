/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nalgebra::Vector3;

use optimization::{Func, GradientDescent, Minimizer, NumericalDifferentiation};

#[derive(Debug)]
struct Particle {
    p: Vector3<f32>,
    v: Vector3<f32>,
}

fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();

            let p: Vec<_> = p.split(',').collect();
            let p: Vec<_> = p.iter().map(|s| s.trim().parse().unwrap()).collect();

            let p = Vector3::<f32>::new(p[0], p[1], p[2]);

            let v: Vec<_> = v.split(',').collect();

            let v: Vec<_> = v.iter().map(|s| s.trim().parse().unwrap()).collect();

            let v = Vector3::<f32>::new(v[0], v[1], v[2]);

            Particle { p, v }
        })
        .collect()
}

fn min_trajectory_distance(p: &Particle, q: &Particle) -> f64 {
    let n = p.v.cross(&q.v);

    let delta = p.p - q.p;

    ((n.dot(&delta)).abs() / n.norm()) as f64
}

pub fn solve(input: &'static str) -> String {
    let particles = parse_input(input);

    let initial = Particle {
        p: Vector3::new(189484959431670.0, 401088290781515.0, 262795384692232.0),
        v: Vector3::new(95.0, -36.0, 39.0),
    };

    find_min(particles, &initial).to_string()
}

fn find_min(particles: Vec<Particle>, initial: &Particle) -> f64 {
    let function = NumericalDifferentiation::new(Func(|x: &[f64]| {
        let q = Particle {
            p: Vector3::new(x[0] as f32, x[1] as f32, x[2] as f32),
            v: Vector3::new(x[3] as f32, x[4] as f32, x[5] as f32),
        };

        particles
            .iter()
            .take(1)
            .map(|p| min_trajectory_distance(p, &q))
            .sum()
    }));

    let minimizer = GradientDescent::new();

    let minimizer = minimizer.gradient_tolerance(1e-24);

    let solution = minimizer.minimize(
        &function,
        vec![
            initial.p.x as f64,
            initial.p.y as f64,
            initial.p.z as f64,
            initial.v.x as f64,
            initial.v.y as f64,
            initial.v.z as f64,
        ],
    );

    solution.position.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Particle, Vector3};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn find_min_close() {
        let particles = parse_input(SAMPLE);

        let initial = Particle {
            p: Vector3::new(18.0, 14.0, 8.0),
            v: Vector3::new(-1.0, 1.0, -1.0),
        };

        let result = super::find_min(particles, &initial);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn find_min_exact() {
        let particles = parse_input(SAMPLE);

        let initial = Particle {
            p: Vector3::new(24.0, 13.0, 10.0),
            v: Vector3::new(-3.0, 1.0, 2.0),
        };

        let result = super::find_min(particles, &initial);

        assert_eq!(result, 0.0);
    }
}
