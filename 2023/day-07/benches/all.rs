use advent_of_code::{part_1::solve_part1, part_2::solve_part2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(include_str!("../input.txt",));
    c.bench_function("part 1 with actual input", |b| {
        b.iter(|| solve_part1(input).unwrap())
    });

    c.bench_function("part 2 with actual input", |b| {
        b.iter(|| solve_part2(input).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
