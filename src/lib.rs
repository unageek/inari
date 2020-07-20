//! A Rust implementation of set-based interval arithmetic.
#![feature(asm)]
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate static_assertions;

pub use self::interval::{
    DecoratedInterval, Decoration, Interval, IntervalError, IntervalErrorKind, _interval_rep,
};
pub use self::overlap::OverlappingState;

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
