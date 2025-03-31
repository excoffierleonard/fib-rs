use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fibtest::{fib, fib_beyond_max_primitives, fib_primitives};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Max u128 Output | Direct", |b| {
        b.iter(|| fib_primitives(black_box(186)))
    });
    c.bench_function("Max u128 Output | Indirect", |b| {
        b.iter(|| fib(black_box(186)))
    });
    c.bench_function("Max u8 Input | Direct", |b| {
        b.iter(|| fib_beyond_max_primitives(black_box(255)))
    });
    c.bench_function("Max u8 Input | Indirect", |b| {
        b.iter(|| fib(black_box(255)))
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
