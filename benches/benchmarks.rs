use criterion::{Criterion, criterion_group, criterion_main};
use fib_rs::Fib;
use std::hint::black_box;

fn fib_n_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("F(n) Benchmarks");
    g.sample_size(10);

    g.bench_function("n = 1,000", |b| b.iter(|| Fib::single(black_box(1_000))));
    g.bench_function("n = 10,000", |b| b.iter(|| Fib::single(black_box(10_000))));
    g.bench_function("n = 100,000", |b| {
        b.iter(|| Fib::single(black_box(100_000)))
    });
    g.bench_function("n = 1,000,000", |b| {
        b.iter(|| Fib::single(black_box(1_000_000)))
    });
    g.bench_function("n = 10,000,000", |b| {
        b.iter(|| Fib::single(black_box(10_000_000)))
    });
}

fn fib_sequence_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Fibonacci Sequence Benchmarks (Inclusive)");
    g.sample_size(10);

    g.bench_function("0-1,000", |b| {
        b.iter(|| Fib::range(black_box(0), black_box(1_000)))
    });
    g.bench_function("0-10,000", |b| {
        b.iter(|| Fib::range(black_box(0), black_box(10_000)))
    });
    g.bench_function("0-100,000", |b| {
        b.iter(|| Fib::range(black_box(0), black_box(100_000)))
    });
    g.bench_function("1,000-2,000", |b| {
        b.iter(|| Fib::range(black_box(1_000), black_box(2_000)))
    });
    g.bench_function("10,000-20,000", |b| {
        b.iter(|| Fib::range(black_box(10_000), black_box(20_000)))
    });
    g.bench_function("100,000-200,000", |b| {
        b.iter(|| Fib::range(black_box(100_000), black_box(200_000)))
    });
}

criterion_group!(benches, fib_n_benchmark, fib_sequence_benchmark);
criterion_main!(benches);
