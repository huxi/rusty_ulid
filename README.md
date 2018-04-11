# rusty_ulid

[![Build Status](https://travis-ci.org/huxi/rusty_ulid.svg?branch=master)](https://travis-ci.org/huxi/rusty_ulid)
[![codecov](https://codecov.io/gh/huxi/rusty_ulid/branch/master/graph/badge.svg)](https://codecov.io/gh/huxi/rusty_ulid)
[![Crates.io](https://img.shields.io/crates/v/rusty_ulid.svg)](https://crates.io/crates/rusty_ulid)
[![docs.rs](https://docs.rs/rusty_ulid/badge.svg)](https://docs.rs/rusty_ulid)
[![dependency status](https://deps.rs/repo/github/huxi/rusty_ulid/status.svg)](https://deps.rs/repo/github/huxi/rusty_ulid)

This is a Rust implementation of the [ULID][ulid] Universally Unique Lexicographically Sortable Identifiers.

Take a look at the [changelog][changelog] for a detailed list of all changes.

## Quickstart

```rust
extern crate rusty_ulid;
use rusty_ulid::new_ulid_string;
use rusty_ulid::new_ulid_bytes;

// Generate a ULID string
let ulid_string: String = new_ulid_string();
assert_eq!(ulid_string.len(), 26);

// Generate ULID bytes
let ulid_bytes: [u8; 16] = new_ulid_bytes();
assert_eq!(ulid_bytes.len(), 16);
```

```rust
extern crate rusty_ulid;
use rusty_ulid::Ulid;

// Generate a ULID
let ulid = Ulid::new();

// Generate a string for a ULID
let ulid_string = ulid.to_string();

// Create ULID from a string
let result = Ulid::from_str(&ulid_string);

assert_eq!(Ok(ulid), result);
```

```rust
extern crate rusty_ulid;
use rusty_ulid::Ulid;

// Alternative way to parse a ULID string
// This example assumes a function returning a Result.
let ulid: Ulid = "01CAT3X5Y5G9A62FH1FA6T9GVR".parse()?;

let datetime = ulid.datetime();
assert_eq!(datetime.to_string(), "2018-04-11 10:27:03.749 UTC");
```

## Benchmark

Run the benchmarks by executing `cargo bench`.

[ulid]: https://github.com/ulid/spec
[changelog]: https://github.com/huxi/rusty_ulid/blob/master/CHANGELOG.md
