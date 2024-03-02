use std::io::{Cursor, Read, Write};

use bytes::{Buf, BufMut, BytesMut};
use criterion::{criterion_group, criterion_main, Criterion};

fn cursor_vec_new() {
    let mut vec = Cursor::new(Vec::<u8>::new());
}

fn bytes_new() {
    let mut bytes = BytesMut::new();
}

fn cursor_vec_with_capacity_10_000() {
    let mut vec = Cursor::new(Vec::<u8>::with_capacity(10_000));
}

fn bytes_with_capacity_10_000() {
    let mut bytes = BytesMut::with_capacity(10_000);
}

fn cursor_vec_write_10_000(mut cur: Cursor<Vec<u8>>) {
    let buf = &[0u8; 10_000];
    cur.write_all(buf).unwrap();
}

fn bytes_write_10_000(mut bytes: BytesMut) {
    let buf = &[0u8; 10_000];
    bytes.writer().write_all(buf).unwrap();
}

fn buffers(c: &mut Criterion) {
    c.bench_function("cursor vec new", |b| b.iter(|| cursor_vec_new()));
    c.bench_function("bytes new", |b| b.iter(|| bytes_new()));

    c.bench_function("cursor vec with_capacity 10,000", |b| {
        b.iter(|| cursor_vec_with_capacity_10_000())
    });
    c.bench_function("bytes with_capacity 10,000", |b| {
        b.iter(|| bytes_with_capacity_10_000())
    });

    c.bench_function("cursor vec write 10,000", |b| {
        b.iter_batched(
            || Cursor::new(Vec::new()),
            |i| cursor_vec_write_10_000(i),
            criterion::BatchSize::PerIteration,
        );
    });
    c.bench_function("bytes write 10,000", |b| {
        b.iter_batched(
            || BytesMut::new(),
            |i| bytes_write_10_000(i),
            criterion::BatchSize::PerIteration,
        )
    });
}

criterion_group!(benches, buffers);
criterion_main!(benches);
