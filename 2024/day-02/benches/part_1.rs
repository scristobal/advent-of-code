use advent_of_code::part_1;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(include_str!("../input.txt",));
    c.bench_function("part 1", |b| b.iter(|| part_1::solve(input)));
}

criterion_group!(part_1_bench, criterion_benchmark);
criterion_main!(part_1_bench);
