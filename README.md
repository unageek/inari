# 🦊 inari

[![crates.io](https://img.shields.io/crates/v/inari.svg)](https://crates.io/crates/inari)
[![docs](https://docs.rs/inari/badge.svg)](https://docs.rs/inari)
[![build](https://img.shields.io/github/workflow/status/mizuno-gsinet/inari/build/master)](https://github.com/mizuno-gsinet/inari/actions?query=branch%3Amaster+workflow%3Abuild)
[![coverage](https://img.shields.io/coveralls/github/mizuno-gsinet/inari/master)](https://coveralls.io/github/mizuno-gsinet/inari?branch=master)

A Rust implementation of set-based [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

## Requirements

The nightly Rust toolchain is required, since the [new `asm!` macro](https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html) is used in the crate.

There is one conditional feature:

- `gmp` (enabled by default) - Enables operations that depend on GMP and MPFR, namely transcendental functions and conversion between texts and intervals. You can opt-out the feature to reduce dependency. Even in that case, you can still access to basic arithmetic operations that are required for writing filters for geometric predicates.

## Made with inari

- [inari-graph](https://github.com/mizuno-gsinet/inari-graph)

## [Changelog](CHANGELOG.md)

## TODO

- AVX512F support (_mm512_add_round_pd, etc.)
  - https://github.com/rust-lang/stdarch/issues/310
- Improve conformance to the standard
- More formatting options
  - https://octave.sourceforge.io/interval/function/intervaltotext.html
  - https://en.cppreference.com/w/cpp/utility/format/formatter

## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
