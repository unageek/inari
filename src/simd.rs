use std::arch::x86_64::*;

pub(crate) fn abs(x: __m128d) -> __m128d {
    // Drop sign bits.
    unsafe { _mm_andnot_pd(_mm_set1_pd(-0.0), x) }
}

pub(crate) fn dup_hi(x: __m128d) -> __m128d {
    unsafe { _mm_unpackhi_pd(x, x) }
}

pub(crate) fn dup_lo(x: __m128d) -> __m128d {
    unsafe { _mm_unpacklo_pd(x, x) }
}

pub(crate) fn max(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_max_pd(x, y) }
}

pub(crate) fn min(x: __m128d, y: __m128d) -> __m128d {
    unsafe { _mm_min_pd(x, y) }
}

pub(crate) fn negate_lo(x: __m128d) -> __m128d {
    unsafe { _mm_xor_pd(x, _mm_set_sd(-0.0)) }
}

pub(crate) fn set_hi_inf(x: __m128d) -> __m128d {
    unsafe { _mm_move_sd(_mm_set1_pd(f64::INFINITY), x) }
}

pub(crate) fn set_lo_inf(x: __m128d) -> __m128d {
    unsafe { _mm_move_sd(x, _mm_set1_pd(f64::INFINITY)) }
}

pub(crate) fn swap(x: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(x, x, 1) }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
mod avx512f;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
pub(crate) use avx512f::*;

#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
mod sse2;
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
pub(crate) use sse2::*;
