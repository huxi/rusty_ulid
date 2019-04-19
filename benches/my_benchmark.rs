/*
 * The MIT License (MIT)
 * Copyright (c) 2018-2019 Joern Huxhorn
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/*
 * Copyright 2018-2019 Joern Huxhorn
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use criterion::{criterion_group, criterion_main, Criterion};

use rusty_ulid::*;
use std::str::FromStr;

#[allow(clippy::redundant_closure)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate_ulid_string", |b| {
        b.iter(|| generate_ulid_string())
    });
    c.bench_function("generate_ulid_bytes", |b| b.iter(|| generate_ulid_bytes()));
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
