#[macro_use]
extern crate criterion;
extern crate rusty_ulid;

use criterion::Criterion;

use rusty_ulid::*;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_ulid_string", |b| b.iter(|| new_ulid_string()));
    c.bench_function("new_ulid_bytes", |b| b.iter(|| new_ulid_bytes()));
    c.bench_function("from_str", |b| {
        b.iter(|| Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7"))
    });
    c.bench_function("parse_crockford_u128", |b| {
        b.iter(|| crockford::parse_crockford_u128("01CAH7NXGRDJNE9B1NY7PQGYV7"))
    });
    c.bench_function("parse_crockford_u64_tuple", |b| {
        b.iter(|| crockford::parse_crockford_u64_tuple("01CAH7NXGRDJNE9B1NY7PQGYV7"))
    });
    c.bench_function("append_crockford_u128", |b| {
        b.iter(|| {
            let mut string = String::with_capacity(26);
            crockford::append_crockford_u128(
                0x0162_A27A_F618_6CAA_E4AC_35F1_ED78_7B67,
                &mut string,
            );
            string
        })
    });
    c.bench_function("append_crockford_u64_tuple", |b| {
        b.iter(|| {
            let mut string = String::with_capacity(26);
            crockford::append_crockford_u64_tuple(
                (0x0162_A27A_F618_6CAA, 0xE4AC_35F1_ED78_7B67),
                &mut string,
            );
            string
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
