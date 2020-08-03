//! A Rust implementation of set-based interval arithmetic.
#![feature(asm)]
#![allow(clippy::float_cmp)]

pub use self::{
    interval::{DecoratedInterval, Decoration, Interval, IntervalError, IntervalErrorKind, Result},
    overlap::OverlappingState,
};

mod absmax;
mod arith;
mod basic;
mod boolean;
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
