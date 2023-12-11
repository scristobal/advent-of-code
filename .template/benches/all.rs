use advent_of_code::{part_1, part_2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(include_str!("../input.txt",));
    c.bench_function("part 1 w/ real input", |b| {
        b.iter(|| part_1::solve(input).unwrap())
    });

    c.bench_function("part 2 w/ real input", |b| {
        b.iter(|| part_2::solve(input).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
