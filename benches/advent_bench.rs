
use advent::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = get_input(2);
    c.bench_function(&format!("Part {}", input), |b| {
        b.iter(|| p2(black_box(&input)))
    });
    //println!("{}", p4(&input))
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);