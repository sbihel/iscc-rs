#[macro_use]
extern crate criterion;

use criterion::{black_box, Benchmark, Criterion};
use lipsum::lipsum;

use iscc::{content_id_image, content_id_text, data_id, instance_id};

fn criterion_benchmark(c: &mut Criterion) {
    let test_text = lipsum(1000);
    c.bench(
        "content-id-text",
        Benchmark::new("content-id-text", move |b| {
            b.iter(|| content_id_text(black_box(&test_text), black_box(false)))
        })
        .sample_size(200),
    );
    c.bench_function("content-id-image", |b| {
        b.iter(|| content_id_image(black_box("tests/test_data/lenna.jpg"), black_box(false)))
    });
    c.bench_function("data-id", |b| {
        b.iter(|| data_id(black_box("tests/test_data/lenna.jpg")))
    });
    c.bench_function("instance-id", |b| {
        b.iter(|| instance_id(black_box("tests/test_data/lenna.jpg")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
