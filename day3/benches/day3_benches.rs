use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::{fast, slow};

fn day3_benches(c: &mut Criterion) {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut group = c.benchmark_group("day3.p1");
    group.bench_function("slow", |b| {
        b.iter(|| slow::p1(black_box(&input)));
    });

    group.bench_function("fast", |b| {
        b.iter(|| fast::p1(black_box(&input)));
    });
    group.finish();

    let mut group = c.benchmark_group("day3.p2");
    group.bench_function("slow", |b| {
        b.iter(|| slow::p2(black_box(&input)));
    });

    group.bench_function("fast", |b| {
        b.iter(|| fast::p2(black_box(&input)));
    });
    group.finish();
}

criterion_group!(benches, day3_benches);
criterion_main!(benches);
