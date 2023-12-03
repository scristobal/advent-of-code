use advent_of_code::{solve_part1, solve_part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part_1() {
    let input = divan::black_box(include_str!("../input.txt"));
    solve_part1(input).unwrap();
}

#[divan::bench]
fn part_2() {
    let input = divan::black_box(include_str!("../input.txt"));
    solve_part2(input).unwrap();
}
