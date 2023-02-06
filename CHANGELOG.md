# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased][unreleased]
### Changed
- `cargo update`

## [2.0.0] - 2023-01-28
### Added
- optional support for `schemars` and `rocket`. Thanks to [Rinat Shigapov](https://github.com/DXist) for the contribution!

### Changed
- minimal Rust version is now 1.60.0
- moved `doc-comment` to `[dev-dependencies]` by using `cfg(doctest)` (see [cfg(doctest) is stable and you should use it](https://blog.guillaume-gomez.fr/articles/2020-03-07+cfg%28doctest%29+is+stable+and+you+should+use+it))
- `cargo update`
- switched default dependency from `chrono` to `time`
- prefer `time` over `chrono` to obtain current time if both dependencies are enabled.
- added `#[must_use]` to `ulid.offsetdatetime()`.
- fix executable to work with either `chrono` or `time`.

### Removed
- `ulid-generation` and `ulid-generation-time` dependency sets

## [1.0.0] - 2022-01-22
### Added
- optionally use `time` instead of `chrono`. Thanks to [tyhi](https://github.com/tyhi) for the contribution!
### Changed
- minimum supported Rust version is now 1.56
- `cargo update`
- edition = “2021”

## [0.11.0] - 2021-07-04
### Changed
- minimum supported Rust version is now 1.41.0
- `cargo update`

## [0.10.1] - 2021-01-25
### Changed
- `cargo update`

### Removed
- dependency on `time 0.1` by disabling `oldclock` feature of `chrono` crate

## [0.10.0] - 2020-08-23
### Added
- support postprocessing of monotonic ULID values
    - `next_monotonic_from_timestamp_with_rng_and_postprocessor`
    - `next_strictly_monotonic_from_timestamp_with_rng_and_postrocessor`

### Changed
- `cargo update`
- Requires Rust 1.40 or later

## [0.9.3] - 2020-02-24
### Changed
- `cargo update`

### Fixed
- [`#![deny(warnings)]`](https://github.com/rust-unofficial/patterns/blob/main/anti_patterns/deny-warnings.md) anti-pattern

## [0.9.2] - 2020-01-10
### Changed
- criterion 0.3
- `cargo update`

### Removed
- test of deprecated function result to fix build on 1.42-nightly

## [0.9.1] - 2019-08-17
### Added
- [Rust Safety Dance](https://github.com/rust-secure-code/safety-dance) link
- fuzzing

### Changed
- rand 0.7
- minor `Display` performance improvement

## [0.9.0] - 2019-04-25
### Added
- test code in `README.md` using [doc-comment](https://crates.io/crates/doc-comment)
- `impl TryFrom<&[u8]> for Ulid`
- [Miri](https://github.com/rust-lang/miri/) support

### Changed
- all dependencies are now optional but enabled by default

### Removed
- `Ulid::from_slice(&[u8])` in favor of `impl TryFrom<&[u8]> for Ulid`


## [0.8.0] - 2019-03-18
### Added
- `Ulid::from_slice(&[u8])`
- optional serde support

## [0.7.0] - 2018-12-07
### Changed
- edition = “2018”
- rand 0.6.1

## [0.6.1] - 2018-11-16
### Changed
- rand 0.6.0

## [0.6.0] - 2018-10-24
### Added
- derive `Copy` for `Ulid`.

### Changed
- renamed `Ulid::new()` to `Ulid::generate()`.
- renamed `new_ulid_string()` to `generate_ulid_string()`.
- renamed `new_ulid_bytes()` to `generate_ulid_bytes()`.

### Removed
- derive `Default` for `Ulid`.

## [0.5.0] - 2018-08-09
### Added
- support for monotonic ULID values. See `next_monotonic` and `next_strictly_monotonic`.

### Changed
- updated dependencies.

## [0.4.1] - 2018-07-02
### Changed
- `description()` of `DecodingError` is now returning deprecation message like Rust 1.27.

## [0.4.0] - 2018-05-23
### Added
- `append_crockford_u64_tuple` and `parse_crockford_u64_tuple`.

### Changed
- A `Ulid` is now using a private `(u64, u64)` instead of being a `u128`. This is a non-breaking change.
- Using `rand 0.5.0` dependency.
- `append_crockford_u128` appends exactly 26 characters.
- `parse_crockford_u128` requires exactly 26 characters.

### Removed
- `append_crockford_u64` and `parse_crockford_u64`.

### Fixed
- Performance regression. This version is faster than `0.2.0`, even with `rand 0.4.2`.

## [0.3.0] - 2018-05-12
### Added
- Conversion of `Ulid` to and from `u128`.
- `append_crockford_u128` and `parse_crockford_u128`.
- `parse()` quickstart example.
- Derived `Ord`, `Eq` and `Hash` traits for `Ulid`.
- Proper `rusty_ulid` executable functionality including example usage documentation.
- Apache-2.0 license. This crate is now dual-licensed.

### Changed
- A `Ulid` is now using an `u128` instead of being a `(u64, u64)`. This is a breaking change
  if you previously accessed the tuple elements directly.
- `DecodingError::InvalidChar` now contains a `char` instead of `Option<char>`.
- `append_crockford` was renamed to `append_crockford_u64`.
- `parse_crockford` was renamed to `parse_crockford_u64`.
- Panic message is now using proper ISO 8601 formatting for overflow datetime
  `+10889-08-02T05:31:50.655Z`.

### Removed
- `Copy` trait from `Ulid`.

## [0.2.1] - 2018-04-10
### Fixed
- Documentation test of `Ulid::from_timestamp_with_rng`.

## [0.2.0] - 2018-04-10
### Added
- This changelog.
- Some criterion benchmarks. Run the benchmarks by executing `cargo bench`.
- `DecodingError::DataTypeOverflow` that is used by `parse_crockford` and
  `Ulid::from_str` to indicate an overflow. Overflow means that the given string
  contains more bits than the respective data type could handle.

### Changed
- `parse_crockford` can now parse strings of length 13 if they don't cause an `u64`
  overflow. This means that `FZZZZZZZZZZZZ` can still be parsed but `G000000000000` will
  cause a `DecodingError::DataTypeOverflow`.  
  Strings of length 14 or more still result in `DecodingError::InvalidLength`.
- `Ulid::from_str` is now returning properly detecting timestamp overflows. This means
  that `7ZZZZZZZZZZZZZZZZZZZZZZZZZ` can still be parsed but `80000000000000000000000000`
  will cause a `DecodingError::DataTypeOverflow`.  
  Strings of length different than 26 still result in `DecodingError::InvalidLength`.
- `Ulid::from_timestamp_with_rng` will now panic if `timestamp` is bigger than
  `0xFFFF_FFFF_FFFF`.  
  This means that `Ulid::new()` will start to panic after `+10889-08-02T05:31:50.655Z`.  
  `#Y10889Bug`

## [0.1.0] - 2018-04-09
### Added
- Everything. This was the initial release.

[unreleased]: https://github.com/huxi/rusty_ulid/compare/2.0.0...HEAD
[2.0.0]: https://github.com/huxi/rusty_ulid/compare/1.0.0...2.0.0
[1.0.0]: https://github.com/huxi/rusty_ulid/compare/0.11.0...1.0.0
[0.11.0]: https://github.com/huxi/rusty_ulid/compare/0.10.1...0.11.0
[0.10.1]: https://github.com/huxi/rusty_ulid/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/huxi/rusty_ulid/compare/0.9.3...0.10.0
[0.9.3]: https://github.com/huxi/rusty_ulid/compare/0.9.2...0.9.3
[0.9.2]: https://github.com/huxi/rusty_ulid/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/huxi/rusty_ulid/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/huxi/rusty_ulid/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/huxi/rusty_ulid/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/huxi/rusty_ulid/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/huxi/rusty_ulid/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/huxi/rusty_ulid/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/huxi/rusty_ulid/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/huxi/rusty_ulid/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/huxi/rusty_ulid/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/huxi/rusty_ulid/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/huxi/rusty_ulid/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/huxi/rusty_ulid/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/huxi/rusty_ulid/compare/init...0.1.0
