# `bidimap`

<!-- badges -->
[![version][version badge]][lib.rs]
[![documentation][documentation badge]][docs.rs]
[![license][license badge]](#license)


`bidimap` is a Rust library that provides bijective maps with interfaces which will be familiar to users of Rust's standard data structures whenever
possible. There are no external dependencies by default but [Serde] as well as [`no_std`] compatibility are available through feature flags.

This is a maintained fork of [`bimap`](https://crates.io/crates/bimap).

1. [Quick start](#quick-start)
1. [Feature flags](#feature-flags)
1. [Documentation](#documentation)
1. [Semantic versioning](#semantic-versioning)
1. [Minimum supported Rust version](#minimum-supported-rust-version)
1. [License](#license)

## Quick start

To use the latest version of `bidimap` with the default features, add this to
your project's `Cargo.toml` file:

```toml
[dependencies]
bidimap = "0.7"
```

You can now run the `bidimap` Hello World!

```rust
fn main() {
    // A bijective map between letters of the English alphabet and their positions.
    let mut alphabet = bidimap::BiMap::<char, u8>::new();

    alphabet.insert('A', 1);
    // ...
    alphabet.insert('Z', 26);

    println!("A is at position {}", alphabet.get_by_left(&'A').unwrap());
    println!("{} is at position 26", alphabet.get_by_right(&26).unwrap());
}
```

## Feature flags

| Flag name | Description                        | Enabled by default? |
| ---       | ---                                | ---                 |
| `std`     | Standard library usage (`HashMap`) | yes                 |
| `serde`   | (De)serialization using [Serde]    | no                  |

This `Cargo.toml` example shows how these features can be enabled and disabled:

```toml
[dependencies]
# I just want to use `bidimap`.
bidimap = "0.7"

# I want to use `bidimap` without the Rust standard library.
bidimap = { version = "0.7", default-features = false }

# I want to use `bidimap` with Serde support.
bidimap = { version = "0.7", features = ["serde"] }
```

## Custom hashers

`BiHashMap` supports custom hashers for each side of the bimap independently,
just like the standard library's `HashMap`. This is useful when you want to use
a faster or deterministic hasher such as [fnv](https://crates.io/crates/fnv)
or [ahash](https://crates.io/crates/ahash).

```rust
use bidimap::BiHashMap;
use std::collections::hash_map::RandomState;

let s_left = RandomState::new();
let s_right = RandomState::new();
let mut bimap = BiHashMap::<char, i32>::with_hashers(s_left, s_right);
bimap.insert('a', 1);
```

See [`BiHashMap::with_hashers`](https://docs.rs/bidimap/latest/bidimap/hash/struct.BiHashMap.html#method.with_hashers) and
[`BiHashMap::with_capacity_and_hashers`](https://docs.rs/bidimap/latest/bidimap/hash/struct.BiHashMap.html#method.with_capacity_and_hashers) for details.

## Documentation

Documentation for the latest version of `bidimap` is available on [docs.rs].

## Semantic versioning

`bidimap` adheres to the de-facto Rust variety of Semantic Versioning.

## Minimum supported Rust version

| `bidimap` | MSRV   |
| ---       | ---    |
| v0.7.0    | 1.85.0 |

## License

`bidimap` is dual-licensed under the [Apache License] and the [MIT License].
As a library user, this means that you are free to choose either license when
using `bidimap`. As a library contributor, this means that any work you
contribute to `bidimap` will be similarly dual-licensed.

<!-- external links -->
[docs.rs]: https://docs.rs/bidimap/
[lib.rs]: https://lib.rs/crates/bidimap
[`no_std`]: https://rust-embedded.github.io/book/intro/no-std.html
[Serde]: https://serde.rs/

<!-- local files -->
[Apache License]: LICENSE_APACHE
[MIT License]: LICENSE_MIT

<!-- static badge images (all purple) -->
[documentation badge]: https://img.shields.io/static/v1?label=documentation&message=docs.rs&color=blueviolet
[license badge]: https://img.shields.io/static/v1?label=license&message=Apache-2.0/MIT&color=blueviolet
[version badge]: https://img.shields.io/static/v1?label=latest%20version&message=lib.rs&color=blueviolet
