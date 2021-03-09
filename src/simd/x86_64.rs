use std::{arch::x86_64::*, mem::transmute};

pub(crate) const HAS_MAXIMUM: bool = false;

pub(crate) type F64X2 = __m128d;

pub(crate) fn abs(x: F64X2) -> F64X2 {
    // Drop sign bits.
    unsafe { _mm_andnot_pd(splat(-0.0), x) }
}

pub(crate) fn all(x: F64X2) -> bool {
    bitmask(x) == 3
}

pub(crate) fn any(x: F64X2) -> bool {
    bitmask(x) != 0
}

pub(crate) fn and(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_and_pd(x, y) }
}

pub(crate) fn bitmask(x: F64X2) -> u32 {
    unsafe { _mm_movemask_pd(x) as u32 }
}

pub(crate) fn ceil(x: F64X2) -> F64X2 {
    unsafe { _mm_ceil_pd(x) }
}

pub(crate) fn constant(x: f64, y: f64) -> F64X2 {
    unsafe { transmute([x, y]) }
}

pub(crate) fn eq(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmpeq_pd(x, y) }
}

pub(crate) fn extract0(x: F64X2) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[0] }
}

// This is slower than extract0, which usually turns into a no-op.
pub(crate) fn extract1(x: F64X2) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[1] }
}

pub(crate) fn floor(x: F64X2) -> F64X2 {
    unsafe { _mm_floor_pd(x) }
}

pub(crate) fn ge(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmpge_pd(x, y) }
}

pub(crate) fn gt(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmpgt_pd(x, y) }
}

pub(crate) fn le(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmple_pd(x, y) }
}

pub(crate) fn lt(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmplt_pd(x, y) }
}

// The vector version of the `maximumNumber` operation defined in IEEE 754-2019,
// which does not propagate NaNs.
pub(crate) fn max(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_max_pd(x, y) }
}

// The vector version of the `maximum` operation defined in IEEE 754-2019,
// which propagates NaNs.
pub(crate) fn maximum(_x: F64X2, _y: F64X2) -> F64X2 {
    unimplemented!()
}

pub(crate) fn min(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_min_pd(x, y) }
}

pub(crate) fn minimum(_x: F64X2, _y: F64X2) -> F64X2 {
    unimplemented!()
}

pub(crate) fn neg(x: F64X2) -> F64X2 {
    xor(splat(-0.0), x)
}

pub(crate) fn neg0(x: F64X2) -> F64X2 {
    unsafe { xor(x, _mm_set_sd(-0.0)) }
}

pub(crate) fn or(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_or_pd(x, y) }
}

// This one is hard to implement correctly.
// https://www.cockroachlabs.com/blog/rounding-implementations-in-go/
pub(crate) fn round(x: F64X2) -> F64X2 {
    constant(extract0(x).round(), extract1(x).round())
}

pub(crate) fn round_ties_to_even(x: F64X2) -> F64X2 {
    unsafe { _mm_round_pd(x, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC) }
}

pub(crate) fn shuffle02(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_shuffle_pd(x, y, 0) }
}

pub(crate) fn shuffle03(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_shuffle_pd(x, y, 2) }
}

pub(crate) fn shuffle12(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_shuffle_pd(x, y, 1) }
}

pub(crate) fn shuffle13(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_shuffle_pd(x, y, 3) }
}

pub(crate) fn splat(x: f64) -> F64X2 {
    unsafe { transmute([x, x]) }
}

pub(crate) fn swap(x: F64X2) -> F64X2 {
    shuffle12(x, x)
}

pub(crate) fn trunc(x: F64X2) -> F64X2 {
    unsafe { _mm_round_pd(x, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC) }
}

pub(crate) fn unord(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_cmpunord_pd(x, y) }
}

pub(crate) fn xor(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { _mm_xor_pd(x, y) }
}

cfg_if::cfg_if! {
    if #[cfg(target_feature = "avx512f")] {
        mod avx512f;
        pub(crate) use avx512f::*;
    } else if #[cfg(all(target_feature = "avx", target_feature = "fma"))] {
        mod avx_fma;
        pub(crate) use avx_fma::*;
    } else {
        compile_error!("RUSTFLAGS='-C target-cpu=haswell' or later is required. See https://doc.rust-lang.org/rustc/codegen-options/#target-cpu");
    }
}
