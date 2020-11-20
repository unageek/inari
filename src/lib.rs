//! A Rust implementation of set-based [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).
//!
//! [Introduction to Interval Arithmetic][`_docs::intro`]

#![feature(asm)]
#![feature(external_doc)]
#![allow(clippy::float_cmp)]

pub use self::{
    interval::{DecInterval, Decoration, Interval, IntervalError, IntervalErrorKind, Result},
    overlap::OverlappingState,
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
mod simd;
