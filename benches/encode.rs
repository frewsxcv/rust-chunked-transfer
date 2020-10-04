#![feature(test)]

use criterion::{criterion_group, criterion_main, Criterion};
use chunked_transfer;
use std::io::Write;

fn encode_benchmark(c: &mut Criterion) {
    c.bench_function("encode", |b| {
        let writer = vec![];
        let mut encoder = chunked_transfer::Encoder::new(writer);
        let mut to_write = vec![b'a'; 1000];

        b.iter(|| {
            encoder.write_all(&mut to_write);
        });
    });
}

criterion_group!(benches, encode_benchmark);
criterion_main!(benches);
