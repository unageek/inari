use std::{arch::x86_64::*, mem::transmute};

pub(crate) type F64X2 = __m128d;

pub(crate) fn abs(x: __m128d) -> __m128d {
    // Drop sign bits.
    unsafe { _mm_andnot_pd(constant(-0.0), x) }
}

pub(crate) fn all(x: __m128d) -> bool {
    bitmask(x) == 3
}

pub(crate) fn and(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_and_pd(x, y) }
}

pub(crate) fn bitmask(x: __m128d) -> u32 {
    unsafe { _mm_movemask_pd(x) as u32 }
}

pub(crate) fn ceil(x: __m128d) -> __m128d {
    unsafe { _mm_ceil_pd(x) }
}

pub(crate) fn constant(x: f64) -> __m128d {
    unsafe { transmute([x, x]) }
}

pub(crate) fn constants(x: f64, y: f64) -> __m128d {
    unsafe { transmute([x, y]) }
}

pub(crate) fn eq(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmpeq_pd(x, y) }
}

pub(crate) fn extract0(x: __m128d) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[0] }
}

// This is slower than extract0, which usually will be no-op.
pub(crate) fn extract1(x: __m128d) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[1] }
}

pub(crate) fn floor(x: __m128d) -> __m128d {
    unsafe { _mm_floor_pd(x) }
}

pub(crate) fn ge(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmpge_pd(x, y) }
}

pub(crate) fn gt(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmpgt_pd(x, y) }
}

pub(crate) fn le(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmple_pd(x, y) }
}

pub(crate) fn lt(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmplt_pd(x, y) }
}

pub(crate) fn max(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_max_pd(x, y) }
}

pub(crate) fn min(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_min_pd(x, y) }
}

pub(crate) fn negate0(x: __m128d) -> __m128d {
    unsafe { xor(x, _mm_set_sd(-0.0)) }
}

pub(crate) fn or(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_or_pd(x, y) }
}

pub(crate) fn round_ties_to_even(x: __m128d) -> __m128d {
    unsafe { _mm_round_pd(x, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC) }
}

pub(crate) fn shuffle02(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(y, x, 3) }
}

pub(crate) fn shuffle03(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(y, x, 2) }
}

pub(crate) fn shuffle13(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(y, x, 0) }
}

pub(crate) fn swap(x: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(x, x, 1) }
}

pub(crate) fn trunc(x: __m128d) -> __m128d {
    unsafe { _mm_round_pd(x, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC) }
}

pub(crate) fn unord(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_cmpunord_pd(x, y) }
}

pub(crate) fn xor(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_xor_pd(x, y) }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
mod avx512f;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
pub(crate) use avx512f::*;

#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
mod sse2;
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
pub(crate) use sse2::*;
