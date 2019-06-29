# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased][unreleased]
### Changed
- rand 0.7

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

[unreleased]: https://github.com/huxi/rusty_ulid/compare/0.9.0...HEAD
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
