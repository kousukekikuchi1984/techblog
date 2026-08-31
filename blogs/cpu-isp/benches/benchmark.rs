use criterion::{criterion_group, criterion_main, Criterion, black_box};

// 絶対値を使った方法
fn min_abs(a: i32, b: i32) -> i32 {
    (a + b - (a - b).abs()) / 2
}

// 条件分岐を伴う方法
fn min_if(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("min_abs", |b| {
        b.iter(|| {
            for a in 0..100 {
                for b in 0..100 {
                    let ba = black_box(a);
                    let bb = black_box(b);
                    min_abs(ba, bb);
                }
            }
        })
    });

    c.bench_function("min_if", |b| {
        b.iter(|| {
            for a in 0..100 {
                for b in 0..100 {
                    let ba = black_box(a);
                    let bb = black_box(b);
                    min_if(ba, bb);
                }
            }
        })
    });
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
