use std::arch::x86_64::*;

pub(crate) fn abs(x: __m128d) -> __m128d {
    // Drop sign bits.
    unsafe { _mm_andnot_pd(_mm_set1_pd(-0.0), x) }
}

pub(crate) fn add(x: __m128d, y: __m128d) -> __m128d {
    unsafe { secure2(_mm_add_pd(secure2(x), secure2(y))) }
}

pub(crate) fn div(x: __m128d, y: __m128d) -> __m128d {
    unsafe { secure2(_mm_div_pd(secure2(x), secure2(y))) }
}

pub(crate) fn dup_hi(x: __m128d) -> __m128d {
    unsafe { _mm_unpackhi_pd(x, x) }
}

pub(crate) fn dup_lo(x: __m128d) -> __m128d {
    unsafe { _mm_unpacklo_pd(x, x) }
}

pub(crate) fn mul(x: __m128d, y: __m128d) -> __m128d {
    unsafe { secure2(_mm_mul_pd(secure2(x), secure2(y))) }
}

pub(crate) fn mul_add(x: __m128d, y: __m128d, z: __m128d) -> __m128d {
    unsafe { secure2(_mm_fmadd_pd(secure2(x), secure2(y), secure2(z))) }
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

// Rounding mode and constant propagation barrier for f64 values.
#[inline]
pub(crate) fn secure(mut x: f64) -> f64 {
    unsafe {
        // `# {0}` fools the compiler to suppress "argument never used" error.
        // https://github.com/rust-lang/rust/pull/73230
        asm!("# {0}", inout(xmm_reg) x, options(nomem, nostack));
    }
    x
}

#[inline]
pub(crate) fn secure2(mut x: __m128d) -> __m128d {
    unsafe {
        asm!("# {0}", inout(xmm_reg) x, options(nomem, nostack));
    }
    x
}

#[inline]
pub(crate) fn round_down() {
    unsafe {
        _MM_SET_ROUNDING_MODE(_MM_ROUND_DOWN);
    }
}

pub(crate) struct RoundUpContext {
    orig_mode: u32,
}

impl RoundUpContext {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let cs = _mm_getcsr();
            _mm_setcsr((cs & !_MM_ROUND_MASK) | _MM_ROUND_UP);
            RoundUpContext {
                orig_mode: cs & _MM_ROUND_MASK,
            }
        }
    }
}

impl Drop for RoundUpContext {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            _MM_SET_ROUNDING_MODE(self.orig_mode);
        }
    }
}
