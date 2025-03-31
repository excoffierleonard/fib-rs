use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fibtest::fib;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Max u128 Output", |b| b.iter(|| fib(black_box(186))));
    c.bench_function("Max u8 Input", |b| b.iter(|| fib(black_box(255))));
    c.bench_function("1K Input", |b| b.iter(|| fib(black_box(1000))));
    c.bench_function("10K Input", |b| b.iter(|| fib(black_box(10000))));
    c.bench_function("100K Input", |b| b.iter(|| fib(black_box(100000))));
    {
        let mut group = c.benchmark_group("1M Input");
        group.sample_size(10);
        group.bench_function("1M Input", |b| b.iter(|| fib(black_box(1000000))));
    }
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
