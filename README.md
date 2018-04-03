# rusty_ulid

[![Build Status](https://travis-ci.org/huxi/rusty_ulid.svg?branch=master)](https://travis-ci.org/huxi/rusty_ulid)
[![Crates.io](https://img.shields.io/crates/v/rusty_ulid.svg)](https://crates.io/crates/rusty_ulid)
[![docs.rs](https://docs.rs/rusty_ulid/badge.svg)](https://docs.rs/rusty_ulid)

This is a Rust implementation of the [ulid][ulid] project which provides
Universally Unique Lexicographically Sortable Identifiers.

## Quickstart

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

## Benchmark

TODO

[ulid]: https://github.com/ulid/spec
