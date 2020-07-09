# 🦊 inari

[![Crate](https://img.shields.io/crates/v/inari.svg)](https://crates.io/crates/inari)
[![Documentation](https://docs.rs/inari/badge.svg)](https://docs.rs/inari)
[![Build Status](https://img.shields.io/github/workflow/status/mizuno-gsinet/inari/build)](https://github.com/mizuno-gsinet/inari/actions?query=workflow%3Abuild)
[![Coverage Status](https://img.shields.io/coveralls/github/mizuno-gsinet/inari/master)](https://coveralls.io/github/mizuno-gsinet/inari?branch=master)

A Rust implementation of set-based [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

## Requirements

The nightly Rust toolchain is required, until the new `asm!` macro is stabilized.

## TODO

- Pass `Interval`s by value or by reference?
  - https://github.com/rust-lang/rust/issues/52274
- AVX512F support (_mm512_add_round_pd, etc.)
  - https://github.com/rust-lang/stdarch/issues/310
- Improve conformance to the standard
- More formatting options
  - https://octave.sourceforge.io/interval/function/intervaltotext.html
  - https://en.cppreference.com/w/cpp/utility/format/formatter

## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
