use aoc2024::day1::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    let input = parse(input);
    c.bench_function("part1", |b| b.iter(|| part1(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
