use advent_of_code::{solve_part1, solve_part2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| solve_part1(black_box(include_str!("../input.txt",))).unwrap())
    });

    c.bench_function("part 2", |b| {
        b.iter(|| solve_part2(black_box(include_str!("../input.txt",))).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
