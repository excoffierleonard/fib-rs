use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fib_rs::fib;
use num_format::ToFormattedString;
use std::time::{Duration, Instant};

fn basics(c: &mut Criterion) {
    c.bench_function("Max u128 Calculation", |b| b.iter(|| fib(black_box(185))));
    c.bench_function("Max u128 Output", |b| b.iter(|| fib(black_box(186))));
    c.bench_function("Max u8 Input", |b| b.iter(|| fib(black_box(255))));
    {
        let mut group = c.benchmark_group("Big Inputs");
        group.sample_size(10);
        group.bench_function("1K Input", |b| b.iter(|| fib(black_box(1000))));
        group.bench_function("10K Input", |b| b.iter(|| fib(black_box(10000))));
        group.bench_function("100K Input", |b| b.iter(|| fib(black_box(100000))));
        group.bench_function("1M Input", |b| b.iter(|| fib(black_box(1000000))));
        group.bench_function("10M Input", |b| b.iter(|| fib(black_box(10000000))));
    }
}

fn thresholds(_c: &mut Criterion) {
    // One frame at 60 FPS
    calculate_threshold(16, 1000);
    // One frame at 120 FPS
    calculate_threshold(8, 1000);
    // One frame at 144 FPS
    calculate_threshold(6, 1000);
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

criterion_group!(benches, basics, thresholds);
criterion_main!(benches);
