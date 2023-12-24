/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::ops::{Add, Mul};

use nalgebra::Vector3;

use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

#[derive(Debug)]
struct Particle {
    p: Vector3<i64>,
    v: Vector3<i64>,
}

fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();

            let p: Vec<_> = p.split(',').collect();
            let p: Vec<_> = p.iter().map(|s| s.trim().parse().unwrap()).collect();

            let p = Vector3::<i64>::new(p[0], p[1], p[2]);

            let v: Vec<_> = v.split(',').collect();

            let v: Vec<_> = v.iter().map(|s| s.trim().parse().unwrap()).collect();

            let v = Vector3::<i64>::new(v[0], v[1], v[2]);

            Particle { p, v }
        })
        .collect()
}

pub fn solve(input: &'static str) -> String {
    let particles = parse_input(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    let v_x = Int::new_const(&ctx, "v_x");
    let v_y = Int::new_const(&ctx, "v_y");
    let v_z = Int::new_const(&ctx, "v_z");

    let times: Vec<_> = particles
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(&ctx, format!("t_{i}")))
        .collect();

    particles.into_iter().zip(times).for_each(|(particle, t)| {
        let particle_px = Int::from_i64(&ctx, particle.p.x);
        let particle_py = Int::from_i64(&ctx, particle.p.y);
        let particle_pz = Int::from_i64(&ctx, particle.p.z);

        let particle_vx = Int::from_i64(&ctx, particle.v.x);
        let particle_vy = Int::from_i64(&ctx, particle.v.y);
        let particle_vz = Int::from_i64(&ctx, particle.v.z);

        let b_x = ((&x).add((&v_x).mul(&t)))._eq(&((&particle_px).add((&particle_vx).mul(&t))));
        let b_y = ((&y).add((&v_y).mul(&t)))._eq(&((&particle_py).add((&particle_vy).mul(&t))));
        let b_z = ((&z).add((&v_z).mul(&t)))._eq(&((&particle_pz).add((&particle_vz).mul(&t))));

        solver.assert(&b_x);
        solver.assert(&b_y);
        solver.assert(&b_z);
    });

    assert!(matches!(solver.check(), z3::SatResult::Sat));

    let model = solver.get_model().unwrap();

    let sum = ((&x).add(&y).add(&z)).simplify();

    let sum = model.eval(&sum, true).unwrap();

    sum.as_u64().unwrap().to_string()
}
