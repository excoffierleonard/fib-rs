use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fib_rs::{fib, fib_sequence};

fn fib_n_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("F(n) Benchmarks");
    g.sample_size(10);

    g.bench_function("n = 1,000", |b| b.iter(|| fib(black_box(1_000))));
    g.bench_function("n = 10,000", |b| b.iter(|| fib(black_box(10_000))));
    g.bench_function("n = 100,000", |b| b.iter(|| fib(black_box(100_000))));
    g.bench_function("n = 1,000,000", |b| b.iter(|| fib(black_box(1_000_000))));
    g.bench_function("n = 10,000,000", |b| b.iter(|| fib(black_box(10_000_000))));
}

fn fib_sequence_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Fibonacci Sequence Benchmarks (Inclusive)");
    g.sample_size(10);

    g.bench_function("0-1,000", |b| b.iter(|| fib_sequence(black_box(0..=1_000))));
    g.bench_function("0-10,000", |b| {
        b.iter(|| fib_sequence(black_box(0..=10_000)))
    });
    g.bench_function("0-100,000", |b| {
        b.iter(|| fib_sequence(black_box(0..=100_000)))
    });
}

criterion_group!(benches, fib_n_benchmark, fib_sequence_benchmark);
criterion_main!(benches);
