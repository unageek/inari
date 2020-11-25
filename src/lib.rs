//! A Rust implementation of set-based [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).
//!
//! [Introduction to Interval Arithmetic][`_docs::intro`]

#![feature(asm)]
#![feature(external_doc)]
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
    if #[cfg(target_arch = "x86_64")] {
        mod simd;
    } else {
        compile_error!("Only x86-64 architecture is supported by this crate.");
    }
}
