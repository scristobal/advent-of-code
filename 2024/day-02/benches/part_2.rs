use advent_of_code::part_2;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(include_str!("../input.txt",));

    c.bench_function("part 2", |b| b.iter(|| part_2::solve(input)));
}

criterion_group!(part_2_bench, criterion_benchmark);
criterion_main!(part_2_bench);
