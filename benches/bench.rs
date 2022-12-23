use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add_range", |b| {
        b.iter(|| {
            let mut ranges = ranges::Ranges::default();

            for v in 0..500 {
                ranges.add_range(black_box(v..(v * 2 + 1500)));
            }

            for v in 100000000..100000500 {
                ranges.add_range(black_box(v..(v * 2 + 1500)));
            }

            assert_eq!(ranges.to_vec().len(), 2);
        });
    });

    c.bench_function("add_range with no overlap", |b| {
        b.iter(|| {
            let mut ranges = ranges::Ranges::default();

            for v in 0..500 {
                ranges.add_range(black_box((v * v)..(v * v + 1)));
            }

            assert_eq!(ranges.to_vec().len(), 499);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
