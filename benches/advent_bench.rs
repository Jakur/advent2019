
use advent::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let problem = 7;
    let input = get_input(problem);
    let func = problem_multiplex(problem);
    c.bench_function(&format!("Part {}", problem), |b| {
        b.iter(|| func(black_box(&input)))
    });
    println!("{}", func(&input))
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);