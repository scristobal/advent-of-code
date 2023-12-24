/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nalgebra::{Matrix4, Vector3, Vector4};

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

impl Particle {
    fn homogeneous(&self) -> (Vector4<f32>, Vector4<f32>) {
        let r1 = Vector4::<f32>::new(
            -self.v.y,
            -self.v.x,
            0.0,
            self.v.x * self.p.y - self.v.y * self.p.x,
        );

        let r2 = Vector4::<f32>::new(
            0.0,
            self.v.z,
            -self.v.y,
            self.v.y * self.p.z - self.v.z * self.p.y,
        );

        (r1, r2)
    }
}

fn are_intersecting(p: &Particle, q: &Particle) -> Option<Vector3<f32>> {
    let (h1_n, h2_n) = p.homogeneous();
    let (h1_m, h2_m) = q.homogeneous();

    let a_ext = Matrix4::<_>::from_rows(&[
        h1_n.transpose(),
        h2_n.transpose(),
        h1_m.transpose(),
        h2_m.transpose(),
    ]);

    let a = &a_ext.fixed_columns::<3>(0);
    let b = a_ext.column(3);

    let r_ext: usize = a_ext.rank(1e-6);
    let r: usize = a.rank(1e-6);

    (r_ext == 3 && r != 4).then_some((a.transpose() * a).try_inverse().unwrap() * a.transpose() * b)
}

fn min_trajectory_distance(p: &Particle, q: &Particle) -> f64 {
    let n = p.v.cross(&q.v);

    let delta = p.p - q.p;

    ((n.dot(&delta)).abs() / n.norm()) as f64
}

fn count_collisions(particles: &[Particle], projectile: &Particle) -> i64 {
    particles
        .iter()
        .filter_map(|p| are_intersecting(p, projectile))
        .count()
        .try_into()
        .unwrap()
}

pub fn solve(input: &'static str) -> String {
    let particles = parse_input(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Particle, Vector3};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn count_collisions_test() {
        let particles = parse_input(SAMPLE);

        let projectile = Particle {
            p: Vector3::new(24.0, 13.0, 10.0),
            v: Vector3::new(-3.0, 1.0, 2.0),
        };

        let result = super::count_collisions(&particles, &projectile).to_string();

        assert_eq!(result, "5");
    }

    #[test]
    fn min_trajectory_distance() {
        let particles = parse_input(SAMPLE);

        let projectile = Particle {
            p: Vector3::new(24.0, 13.0, 10.0),
            v: Vector3::new(-3.0, 1.0, 2.0),
        };

        particles.iter().for_each(|p| {
            assert_eq!(super::min_trajectory_distance(p, &projectile), 0.0);
        });
    }
}
