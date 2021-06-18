# 🦊 inari

[![crates.io](https://img.shields.io/crates/v/inari.svg)](https://crates.io/crates/inari)
[![docs](https://docs.rs/inari/badge.svg)](https://docs.rs/inari)
[![build](https://img.shields.io/github/workflow/status/unageek/inari/build/master)](https://github.com/unageek/inari/actions?query=branch%3Amaster+workflow%3Abuild)
[![coverage](https://img.shields.io/coveralls/github/unageek/inari/master)](https://coveralls.io/github/unageek/inari?branch=master)

A Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

## Requirements

### Rust version

A recent version of the nightly toolchain is required since some unstable features ([asm](https://github.com/rust-lang/rust/issues/72016), [stdsimd](https://github.com/rust-lang/rust/issues/48556)) are used in the crate. You need to specify the toolchain by, for example, the [toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) in your crate (see [example](https://github.com/unageek/graphest/blob/master/rust/rust-toolchain).

### Target CPU

- x86-64

  Haswell-based and newer processors are supported.

  You need to specify the target CPU when building a crate that depends on inari. One way to do that is to add a [configuration file](https://doc.rust-lang.org/cargo/reference/config.html) to the consuming crate (see [example](https://github.com/unageek/graphest/blob/master/rust/.cargo/config.toml); you may want to change `native` to `haswell` for maximum compatibility if you are going to distribute your binary).

- AArch64 (a.k.a. ARM64)

  Experimental, it is not tested continuously.

## Conditional features

- `gmp` (enabled by default) - Enables operations that depend on GMP and MPFR, namely, transcendental functions and conversion between texts and intervals. You can opt-out the feature to reduce dependencies. Even in that case, you still have access to all arithmetic operations that are required for writing filters for robust geometric predicates.

## Related projects

- [Graphest](https://github.com/unageek/graphest) - a faithful graphing calculator

## [Changelog](CHANGELOG.md)

## TODO

- Improve conformance to the standard
- More formatting options
  - https://octave.sourceforge.io/interval/function/intervaltotext.html
  - https://docs.python.org/3/library/string.html#formatspec

## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
