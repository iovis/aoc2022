use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::slow;

fn day3_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("day3");

    let input = std::fs::read_to_string("src/input.txt").unwrap();

    group.bench_function("original", |b| {
        b.iter(|| slow::p1(black_box(&input)));
    });

    group.finish();
}

criterion_group!(benches, day3_benches);
criterion_main!(benches);
