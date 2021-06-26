//! A Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).
//!
//! [Introduction to interval arithmetic][`_docs::intro`]
//!
//! inari implements a subset of the following standards for interval arithmetic:
//!
//! - [IEEE 1788-2015](https://doi.org/10.1109/IEEESTD.2015.7140721)
//! - [IEEE 1788.1-2017](https://doi.org/10.1109/IEEESTD.2018.8277144) - a simplified version of IEEE 1788-2015
//!
//! See [Conformance to the standard][`_docs::conformance`] for details.
//!
//! # Examples
//!
//! ## Cumulative effect of cancellation
//!
//! Consider the following sequence:
//!
//! $$
//! \begin{align}
//!  x_0 &= 0.1, \\\\
//!  x_k &= 16 ⋅ x_{k-1} - 1.5,\quad \text{for } k = 1, 2, …
//! \end{align}
//! $$
//!
//! It is obvious that $x_0 = x_1 = x_2 = … = 0.1$, but let's pretend not to know that
//! and compute it numerically:
//!
//! ```
//! let mut x = 0.1;
//! println!("x0 = {}", x);
//!
//! for i in 0..20 {
//!     x = 16.0 * x - 1.5;
//!     println!("x{} = {}", i + 1, x);
//! }
//! ```
//!
//! Output:
//!
//! ```text
//! x0 = 0.1
//! x1 = 0.10000000000000009
//! x2 = 0.10000000000000142
//! ...
//! x13 = 0.125
//! x14 = 0.5
//! x15 = 6.5
//! ...
//! x20 = 6710886.5
//! ```
//!
//! Apparently, something went wrong. Let’s see what happened.
//!
//! `x` is a [`f64`] number, which can represent a binary number with up to 53 binary digits (=53 bits).
//! On the other hand, the binary representation of $x_0$ is $0.1 = 0.00011001100… \bin$
//! (1100 repeats forever), which requires infinite bits to write out.
//! Therefore, it cannot be represented exactly as a [`f64`] number.
//! Actually, $\tilde x_0$, the initial value of `x`, is not 0.1 but the closest [`f64`] number to 0.1:
//!
//! $$
//! \begin{align}
//!  \tilde x_0 &= 0.000\overbrace{11001100…110011010}^\text{53 bits} \bin \\\\
//!   &= 0.1000000000000000055511151231257827021181583404541015625.
//! \end{align}
//! $$
//!
//! The inexactness of $\tilde x_0$, which is attributed to the finite nature of the number system,
//! is called a [_round-off error_](https://en.wikipedia.org/wiki/Round-off_error).
//!
//! In the first iteration, $\tilde x_1$ is computed as:
//!
//! $$
//! \begin{array}{rrcl}
//!     & 16 ⋅ \tilde x_0 & = & \overbrace{1.1001100…110011010}^\text{53 bits} \bin \\\\
//!  \- &            1.5 & = & 1.1000000…000000000 \bin \\\\
//!  \hline
//!     &     \tilde x_1 & = & 0.0001100…110011010 \bin.
//! \end{array}
//! $$
//!
//! So $\tilde x_1$ is an approximation of 0.1 again, but is less precise than $\tilde x_0$ by 4 bits.
//! The loss of precision is caused by subtracting two numbers that are close to each other.
//! This is called [_catastrophic cancellation_](https://en.wikipedia.org/wiki/Catastrophic_cancellation).
//!
//! By repeating this process, the precision is lost by 4 bits per iteration. After 14 iterations,
//! the magnitude of the error gets larger than that of the exact value.
//!
//! This example was easy to analyze. But in practical computation, there can be many sources
//! of numerical errors. If that is the case, how far can we trust the result of our computation
//! in general? [Interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic)
//! gives an answer to that problem. Instead of an approximation without concrete information
//! about its error, interval arithmetic computes a solid enclosure of the exact value.
//!
//! Let's replace the [`f64`] type in the previous code with the [`Interval`] type.
//!
//! ```
//! use inari::*;
//!
//! let mut x = interval!("[0.1]").unwrap();
//! println!("x0 ∈ {}", x);
//!
//! for i in 0..20 {
//!     x = const_interval!(16.0, 16.0) * x - const_interval!(1.5, 1.5);
//!     println!("x{} ∈ {}", i + 1, x);
//! }
//! ```
//!
//! Output:
//!
//! ```text
//! x0 ∈ [0.099999,0.100001]
//! x1 ∈ [0.099999,0.100001]
//! x2 ∈ [0.099999,0.100001]
//! ...
//! x13 ∈ [0.062500,0.125000]
//! x14 ∈ [-0.500000,0.500000]
//! x15 ∈ [-9.500000,6.500000]
//! ...
//! x20 ∈ [-10066329.500000,6710886.500000]
//! ```
//!
//! While it does not magically eliminate numerical errors, it gives you a result that you can rely on.
//!
//! ## Solving a quadratic equation
//!
//! In this example, we solve the quadratic equation: $ax^2 + bx + c = 0$.
//! The solutions are given by:
//!
//! $$
//! x_1 = \frac{-b - \sqrt{b^2 - 4ac}}{2a},\quad
//!   x_2 = \frac{-b + \sqrt{b^2 - 4ac}}{2a}.
//! $$
//!
//! There are two possible causes of catastrophic cancellation, $b^2 - 4ac$ and $-b ± \sqrt{\cdots}$.
//! The latter one will be significant when $b^2 ≫ 4ac$, thus $\sqrt{b^2 - 4ac} ≈ |b|$.
//! We can avoid it by modifying the formulae as follows:
//!
//! For $b > 0$:
//!
//! $$
//! x_1 = \frac{-b - \sqrt{b^2 - 4ac}}{2a},\quad
//!   x_2 = \frac{2c}{-b - \sqrt{b^2 - 4ac}},
//! $$
//!
//! for $b < 0$:
//!
//! $$
//! x_1 = \frac{2c}{-b + \sqrt{b^2 - 4ac}},\quad
//!   x_2 = \frac{-b + \sqrt{b^2 - 4ac}}{2a},
//! $$
//!
//! and for $b = 0$, we use the original formulae to avoid division by zero.
//!
//! Let's implement the formulae, but in this time, we use the [`DecInterval`] type
//! instead of the [`Interval`] type. The reason will be made clear soon.
//!
//! ```
//! use inari::*;
//!
//! /// Returns the solutions of the quadratic equation `a x^2 + b x + c = 0`.
//! ///
//! /// Panics if `a` is zero.
//! fn solve(a: DecInterval, b: DecInterval, c: DecInterval) -> (DecInterval, DecInterval) {
//!     if a == const_dec_interval!(0.0, 0.0) {
//!         panic!("`a` must not be zero");
//!     }
//!
//!     const TWO: DecInterval = const_dec_interval!(2.0, 2.0);
//!     const FOUR: DecInterval = const_dec_interval!(4.0, 4.0);
//!
//!     let d = (b.sqr() - FOUR * a * c).sqrt();
//!     if b.contains(0.0) {
//!         ((-b - d) / (TWO * a), (-b + d) / (TWO * a))
//!     } else if b.inf() > 0.0 {
//!         ((-b - d) / (TWO * a), (TWO * c) / (-b - d))
//!     } else {
//!         ((TWO * c) / (-b + d), (-b + d) / (TWO * a))
//!     }
//! }
//!
//! let a = dec_interval!("[1]").unwrap();
//! let b = dec_interval!("[-1e15]").unwrap();
//! let c = dec_interval!("[1e14]").unwrap();
//! let (x1, x2) = solve(a, b, c);
//!
//! println!("x1 ∈ {}", x1);
//! println!("x2 ∈ {}", x2);
//! ```
//!
//! Output:
//!
//! ```text
//! x1 ∈ [0.099999,0.100001]_com
//! x2 ∈ [999999999999999.750000,1000000000000000.000000]_com
//! ```
//!
//! What if the equation does not have solutions in real numbers?
//!
//! ```ignore
//! let a = dec_interval!("[1]").unwrap();
//! let b = dec_interval!("[0]").unwrap();
//! let c = dec_interval!("[1]").unwrap();
//! ```
//!
//! Output:
//!
//! ```text
//! x1 ∈ [empty]_trv
//! x2 ∈ [empty]_trv
//! ```
//!
//! ```ignore
//! let a = dec_interval!("[1]").unwrap();
//! let b = dec_interval!("[-2.82842712474619]").unwrap();
//! let c = dec_interval!("[2]").unwrap();
//! ```
//!
//! Output:
//!
//! ```text
//! x1 ∈ [1.414213,1.414214]_trv
//! x2 ∈ [1.414213,1.414214]_trv
//! ```
//!
//! The `_trv` suffix means that the value of the expression is possibly undefined for a subset
//! of the input. In this case, division by zero and the square root of a negative number are
//! undefined. So they return intervals decorated with `_trv` for such inputs.
//!
//! `x1 ∈ [1.414213,1.414214]_trv` indicates that $x_1 ∈ [1.414213,1.414214]$ or
//! it does not have a solution in real numbers.

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
