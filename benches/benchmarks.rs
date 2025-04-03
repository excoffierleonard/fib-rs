use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fib_rs::fib;
use num_format::ToFormattedString;
use std::time::{Duration, Instant};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("F(n) Benchmarks");
    g.sample_size(10);
    g.bench_function("n = 1,000", |b| b.iter(|| fib(black_box(1_000))));
    g.bench_function("n = 10,000", |b| b.iter(|| fib(black_box(10_000))));
    g.bench_function("n = 100,000", |b| b.iter(|| fib(black_box(100_000))));
    g.bench_function("n = 1,000,000", |b| b.iter(|| fib(black_box(1_000_000))));
    g.bench_function("n = 10,000,000", |b| b.iter(|| fib(black_box(10_000_000))));
}

fn thresholds(_c: &mut Criterion) {
    // One frame at 60 FPS
    calculate_threshold(16, 1000);
    // One frame at 120 FPS
    calculate_threshold(8, 1000);
    // One frame at 240 FPS
    calculate_threshold(4, 1000);
    // One millisecond
    calculate_threshold(1, 1000);
}

fn calculate_threshold(target_duration_ms: u64, step: u128) {
    let target_duration = Duration::from_millis(target_duration_ms);
    let mut n = 0;
    let mut last_n_below_threshold = n;

    loop {
        let start = Instant::now();
        let _result = fib(n);
        let duration = start.elapsed();

        if duration > target_duration {
            break;
        }

        last_n_below_threshold = n;
        n += step;
    }

    println!(
        "Last input 'n' before exceeding {} ms (step={}): {}",
        target_duration_ms,
        step,
        last_n_below_threshold.to_formatted_string(&num_format::Locale::en)
    );
}

criterion_group!(benches, criterion_benchmark, thresholds);
criterion_main!(benches);
