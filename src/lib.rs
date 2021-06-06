//! A Rust implementation of set-based [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).
//!
//! [Introduction to Interval Arithmetic][`_docs::intro`]

#![feature(asm)]
#![cfg_attr(target_arch = "aarch64", feature(stdsimd))]
#![allow(clippy::float_cmp)]

pub use self::{
    interval::{DecInterval, Decoration, Interval, IntervalError, IntervalErrorKind, Result},
    overlap::Overlap,
};

pub mod _docs;

mod absmax;
mod arith;
mod basic;
mod boolean;
mod bytes;
mod classify;
mod constants;
#[cfg(feature = "gmp")]
mod elementary;
#[cfg(feature = "gmp")]
mod format;
mod integer;
mod interval;
mod numeric;
mod overlap;
#[cfg(feature = "gmp")]
mod parse;
mod set_op;

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", docsrs))] {
        mod simd;
    } else {
        compile_error!("Only x86-64 and AArch64 (experimental) architectures are supported by this crate.");
    }
}
