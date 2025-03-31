use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fibtest::fib;

fn criterion_benchmark_primitives_results(c: &mut Criterion) {
    c.bench_function("Max u8", |b| b.iter(|| fib(black_box(13))));
    c.bench_function("Max u16", |b| b.iter(|| fib(black_box(24))));
    c.bench_function("Max u32", |b| b.iter(|| fib(black_box(47))));
    c.bench_function("Max u64", |b| b.iter(|| fib(black_box(93))));
    c.bench_function("Max u128", |b| b.iter(|| fib(black_box(186))));
}

criterion_group!(benches, criterion_benchmark_primitives_results);
criterion_main!(benches);
