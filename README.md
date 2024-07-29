# 🦊 inari

[![crates.io](https://img.shields.io/crates/v/inari.svg)](https://crates.io/crates/inari)
[![docs](https://img.shields.io/docsrs/inari)](https://docs.rs/inari)
[![coverage](https://img.shields.io/coveralls/github/unageek/inari/main)](https://coveralls.io/github/unageek/inari?branch=main)
[![rustc 1.65+](https://img.shields.io/badge/rustc-1.65%2B-lightgrey)](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)

**inari** is a Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

It [conforms](https://docs.rs/inari/latest/inari/_docs/conformance/index.html) to [IEEE Std 1788.1-2017](https://doi.org/10.1109/IEEESTD.2018.8277144). It also implements a subset of [IEEE Std 1788-2015](https://doi.org/10.1109/IEEESTD.2015.7140721).

## Supported Platforms

The following CPUs are supported:

- **x86-64**

  Haswell-based and newer processors are supported.

  You need to specify the target CPU when building a crate that depends on inari. One way to do that is using a [configuration file](https://doc.rust-lang.org/cargo/reference/config.html) in your project (see [example](https://github.com/unageek/graphest/blob/main/.cargo/config.toml); you may want to change `native` to `haswell` for the best compatibility if you are going to distribute binaries).

- **AArch64 (ARM64)**

When using the Cargo feature `gmp` (see below), target platforms are limited to the ones that are supported by the [`gmp-mpfr-sys`](https://crates.io/crates/gmp-mpfr-sys) crate. For example, MSVC is not supported.

## Cargo Features

- `gmp` (enabled by default) - Enables [operations](https://docs.rs/inari/latest/inari/_docs/conformance/) that depend on GMP and MPFR. You can opt out the feature to reduce dependencies. Even in that case, you still have access to all operations required by certain kind of tasks, such as making fast robust predicates for computational geometry.

## [Changelog](CHANGELOG.md)

## Building the Documentation Locally

```bash
RUSTDOCFLAGS="--cfg docsrs --html-in-header /path/to/inari/src/_docs/header.html" cargo doc --open
```

The absolute path to [`header.html`](src/_docs/header.html) must be specified.

## Related Project

- [Graphest](https://github.com/unageek/graphest) - A faithful graphing calculator

## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
