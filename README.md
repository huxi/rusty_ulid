# rusty_ulid

[![Build Status](https://travis-ci.org/huxi/rusty_ulid.svg?branch=master)](https://travis-ci.org/huxi/rusty_ulid)
[![codecov](https://codecov.io/gh/huxi/rusty_ulid/branch/master/graph/badge.svg)](https://codecov.io/gh/huxi/rusty_ulid)
[![Crates.io](https://img.shields.io/crates/v/rusty_ulid.svg)](https://crates.io/crates/rusty_ulid)
[![docs.rs](https://docs.rs/rusty_ulid/badge.svg)](https://docs.rs/rusty_ulid)
[![dependency status](https://deps.rs/repo/github/huxi/rusty_ulid/status.svg)](https://deps.rs/repo/github/huxi/rusty_ulid)

This is a Rust implementation of the [ULID][ulid] Universally Unique Lexicographically Sortable Identifiers.

This crate requires **Rust 1.31 or later**.

Take a look at the [changelog][changelog] for a detailed list of all changes.

## Features
- lenient parsing of ULID strings as specified in [Crockford Base32 Encoding][crockford].
- straight-forward creation of string and binary ULIDs.
- optional support for monotonic ULIDs.
- conversion to and from `[u8; 16]`.
- conversion to and from `(u64, u64)`.
- conversion to and from `u128`.

## Quickstart

```rust
use rusty_ulid::generate_ulid_string;
use rusty_ulid::generate_ulid_bytes;

// Generate a ULID string
let ulid_string: String = generate_ulid_string();
assert_eq!(ulid_string.len(), 26);

// Generate ULID bytes
let ulid_bytes: [u8; 16] = generate_ulid_bytes();
assert_eq!(ulid_bytes.len(), 16);
```

```rust
use rusty_ulid::Ulid;

// Generate a ULID
let ulid = Ulid::generate();

// Generate a string for a ULID
let ulid_string = ulid.to_string();

// Create ULID from a string
let result = Ulid::from_str(&ulid_string);

assert_eq!(Ok(ulid), result);
```

```rust
use rusty_ulid::Ulid;

// Alternative way to parse a ULID string
// This example assumes a function returning a Result.
let ulid: Ulid = "01CAT3X5Y5G9A62FH1FA6T9GVR".parse()?;

let datetime = ulid.datetime();
assert_eq!(datetime.to_string(), "2018-04-11 10:27:03.749 UTC");
```

Monotonic ULIDs are supported via `Ulid::next_monotonic(previous_ulid) -> Ulid` and
`Ulid::next_strictly_monotonic(previous_ulid) -> Option<Ulid>`.

`next_monotonic` allows overflow of the random part to zero while `next_strictly_monotonic`
returns `None` instead.

## Benchmark

Run the benchmarks by executing `cargo bench`.

## Executable

Install the executable by executing `cargo install` or `cargo install --force` if a prior version was already installed.

### `rusty_ulid` usage examples

Just calling the executable generates a ULID.

```console
$ rusty_ulid
01CB2EM1J4EMBWRBJK877TM17S
```

Calling the executable with `-v` or `--verbose` generates a ULID and prints its timestamp.

```console
$ rusty_ulid -v
01CB2EMMMV8P51SCR9ZH8K64CX
2018-04-14 16:08:33.691 UTC
```

Calling the executable with any number of ULIDs checks them for validity and returns `0` if they are all fine...

```console
$ rusty_ulid 01CB2EM1J4EMBWRBJK877TM17S 01CB2EMMMV8P51SCR9ZH8K64CX
$ echo $?
0
```

... or `1` if any given value is invalid, printing the invalid values to `err`.

```console
$ rusty_ulid 01CB2EM1J4EMBWRBJK877TM17S foo 01CB2EMMMV8P51SCR9ZH8K64CX
Invalid ULID strings: ["foo"]
$ echo $?
1
```

In addition to that, `-v` or `--verbose` will print the ULIDs with their respective timestamp.

```console
$ rusty_ulid -v 01CB2EM1J4EMBWRBJK877TM17S foo 01CB2EMMMV8P51SCR9ZH8K64CX
01CB2EM1J4EMBWRBJK877TM17S
2018-04-14 16:08:14.148 UTC

01CB2EMMMV8P51SCR9ZH8K64CX
2018-04-14 16:08:33.691 UTC

Invalid ULID strings: ["foo"]
$ echo $?
1
```

Executing `rusty_ulid -h` will print the help.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[ulid]: https://github.com/ulid/spec
[crockford]: https://crockford.com/wrmg/base32.html
[changelog]: https://github.com/huxi/rusty_ulid/blob/master/CHANGELOG.md
