use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent2019::{get_input, p4};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = get_input(4);
    c.bench_function("Part 4", |b| b.iter(|| p4(black_box(&input))));
    println!("{}", p4(&input))
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);