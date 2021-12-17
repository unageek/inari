# 🦊 inari

[![crates.io](https://img.shields.io/crates/v/inari.svg)](https://crates.io/crates/inari)
[![docs](https://img.shields.io/docsrs/inari)](https://docs.rs/inari)
[![build](https://img.shields.io/github/workflow/status/unageek/inari/build/master)](https://github.com/unageek/inari/actions?query=branch%3Amaster+workflow%3Abuild)
[![coverage](https://img.shields.io/coveralls/github/unageek/inari/master)](https://coveralls.io/github/unageek/inari?branch=master)

**inari** is a Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

## Supported Rust Versions

A **nightly** toolchain >= `nightly-2021-12-16` is required.

The requirement of a nightly toolchain is due to a few [unstable features](https://doc.rust-lang.org/stable/unstable-book/) used in the crate: [`asm`](https://doc.rust-lang.org/stable/unstable-book/library-features/asm.html), [`stdsimd`](https://doc.rust-lang.org/stable/unstable-book/library-features/stdsimd.html), etc. To use the crate as a dependency, you need to [override the toolchain](https://rust-lang.github.io/rustup/overrides.html) in your project. Here is an [example](https://github.com/unageek/graphest/blob/master/rust-toolchain) that does this with the `rust-toolchain` file.

## Supported Platforms

The following CPUs are supported:

- x86-64

  Haswell-based and newer processors are supported.

  You need to specify the target CPU when building a crate that depends on inari. One way to do that is using a [configuration file](https://doc.rust-lang.org/cargo/reference/config.html) in your project (see [example](https://github.com/unageek/graphest/blob/master/.cargo/config.toml); you may want to change `native` to `haswell` to achieve maximum compatibility if you are going to distribute executables).

- AArch64 (ARM64)

  Experimental, it is not tested continuously.

When using the Cargo feature `gmp` (see below), target platforms are limited to the ones that are supported by the [`gmp-mpfr-sys`](https://crates.io/crates/gmp-mpfr-sys) crate.

## Cargo Features

- `gmp` (enabled by default) - Enables [operations](https://docs.rs/inari/latest/inari/_docs/conformance/) that depend on GMP and MPFR. You can opt out the feature to reduce dependencies. Even in that case, you still have access to all operations required by certain kind of tasks, such as making fast robust predicates for computational geometry.

## [Changelog](CHANGELOG.md)

## Related Projects

- [Graphest](https://github.com/unageek/graphest) - a faithful graphing calculator

## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
