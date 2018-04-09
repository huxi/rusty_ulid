#[macro_use]
extern crate criterion;
extern crate rusty_ulid;

use criterion::Criterion;

use std::str::FromStr;
use rusty_ulid::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_ulid_string", |b| b.iter(|| new_ulid_string()));
    c.bench_function("new_ulid_bytes", |b| b.iter(|| new_ulid_bytes()));
    c.bench_function("from_str", |b| {
        b.iter(|| Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
