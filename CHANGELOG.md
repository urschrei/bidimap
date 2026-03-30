# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
Added, Changed, Deprecated, Removed, Fixed, Security
-->

## [Unreleased]

### Added
- `Send` and `Sync` implementations for all bimap types and their iterators.
- `BiBTreeMap::new` is now `const`.
- Clippy lint job in CI.

### Changed
- Relaxed trait bounds: `new`, `len`, `is_empty`, `clear`, `iter`, `left_values`,
  `right_values`, `Default`, and `IntoIterator` no longer require `Ord` (for
  `BiBTreeMap`) or `Eq + Hash` (for `BiHashMap`).
- Relaxed `BiHashMap::with_hashers` and `with_capacity_and_hashers` to only
  require `BuildHasher`, not `Eq + Hash`.
- Relaxed `BiHashMap` `Default` to only require `Default` on the hashers.
- Updated CI to use `dtolnay/rust-toolchain`, `Swatinem/rust-cache`, and
  `actions/checkout@v6`, replacing deprecated `actions-rs` actions.
- Tightened internal `Ref` field visibility to `pub(crate)`.

### Fixed
- Clippy `non_canonical_partial_ord_impl` warning on `BiBTreeMap`.
- Clippy `default_constructed_unit_structs` warnings in serde module.
- `hashbrown::DefaultHashBuilder` import path for newer hashbrown versions.

### Removed
- `rustfmt.toml` (all options were nightly-only and produced warnings on stable).

## [0.6.3]

### Added
- `Debug` and `Clone` on iterators (#37).

## [0.6.2]

### Added
- `BiBTreeMap::retain` (#30).
- `BiHashMap::reserve`, `BiHashMap::shrink_to`, and `BiHashMap::shrink_to_fit` (#32).

## [0.6.1]

### Added
- `serde` trait implementations for `BiHashMap` are now generic over the left
and right hashers (#27, #28). Before, they were only implemented for the
default hasher.

## [0.6.0]

### Changed
- Generalize query interfaces using `std::borrow::Borrow` for `BiMap` methods
like `get`, `contains`, and `remove`. This more closely aligns to the API
provided by the Rust standard library.

## [0.5.3]

### Added
- Implement `Hash` for `BiBTreeMap` (#23).

### Changed
- Minor edits to the README.

### Removed
- Unnecessary trait bounds on the `fmt::Debug` impl for `BiMap<L, R>` (#22).

## [0.5.2]

### Added
- Documentation link to docs.rs in Cargo.toml.

## [0.5.1]

### Fixed
- Outdated docs.rs link in README.

## [0.5.0]

### Added
- This changelog.
- `Extend` implementations.
- Pretty `Debug` formatting.
- `left_range` and `right_range` methods for `BiBTreeMap`.

### Changed
- Documentation and useful public documents were created and/or updated.

### Fixed
- Tests for `BiBTreeMap` run correctly with `no_std`.

## [0.4.0]

[Unreleased]: https://github.com/urschrei/bimap-rs/compare/v0.6.3...HEAD
[0.6.3]: https://github.com/billyrieger/bimap-rs/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/billyrieger/bimap-rs/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/billyrieger/bimap-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/billyrieger/bimap-rs/compare/v0.5.3...v0.6.0
[0.5.3]: https://github.com/billyrieger/bimap-rs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/billyrieger/bimap-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/billyrieger/bimap-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/billyrieger/bimap-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/billyrieger/bimap-rs/releases/tag/v0.4.0
