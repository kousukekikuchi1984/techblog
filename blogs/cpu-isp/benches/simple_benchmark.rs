
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| {
            let a = black_box(10);
            let b = black_box(20);
            add(a, b);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
