# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.2.1 - 2018-04-10
### Fixed
- Documentation test of `Ulid::from_timestamp_with_rng`.

## 0.2.0 - 2018-04-10
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
  `0xFFFF_FFFF_FFFF`. This means that `Ulid::new()` will start to panic in the
  summer of 10889.
  `#Y10889Bug`

## 0.1.0 - 2018-04-09
### Added
- Everything. This was the initial release.
