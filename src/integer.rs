use crate::interval::*;
use crate::simd::*;
use core::arch::x86_64::*;

impl Interval {
    pub fn ceil(self) -> Self {
        // _mm_ceil_pd/_mm_floor_pd are slow, better to avoid shuffling them.
        // ceil([a, b]) = [ceil(b); -ceil(a)]
        let r0 = negate_lo(self.rep); // [b; a]
        let r1 = unsafe { _mm_ceil_pd(r0) }; // [ceil(b); ceil(a)]
        let r = negate_lo(r1);
        Self { rep: r }
    }

    pub fn floor(self) -> Self {
        // floor([a, b]) = [floor(b); -floor(a)]
        let r0 = negate_lo(self.rep); // [b; a]
        let r1 = unsafe { _mm_floor_pd(r0) }; // [floor(b); floor(a)]
        let r = negate_lo(r1);
        Self { rep: r }
    }

    // This one is hard to implement correctly.
    // https://www.cockroachlabs.com/blog/rounding-implementations-in-go/
    pub fn round_ties_to_away(self) -> Self {
        Self::with_infsup_raw(self.inf_raw().round(), self.sup_raw().round())
    }

    pub fn round_ties_to_even(self) -> Self {
        Self {
            rep: unsafe { _mm_round_pd(self.rep, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC) },
        }
    }

    // NOTE: Returns 0.0 for 0.0, which is a different definition from `f64::signum`.
    pub fn sign(self) -> Self {
        if self.is_empty() {
            return Self::empty();
        }

        unsafe {
            let zero = _mm_setzero_pd();
            let gt_zero_mask = _mm_cmpgt_pd(self.rep, zero);
            let lt_zero_mask = _mm_cmplt_pd(self.rep, zero);
            // [-(a <= 0), b >= 0] = [b >= 0; -a >= 0]
            let one_or_zero = _mm_and_pd(_mm_set1_pd(1.0), gt_zero_mask);
            // [a >= 0, -(b <= 0)] = [-(b <= 0); -(-a <= 0)]
            let m_one_or_zero = _mm_and_pd(_mm_set1_pd(-1.0), lt_zero_mask);
            // Gives the same result as addition, but faster.
            let r = _mm_or_pd(one_or_zero, m_one_or_zero);
            Self { rep: r }
        }
    }

    pub fn trunc(self) -> Self {
        Self {
            rep: unsafe { _mm_round_pd(self.rep, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type I = Interval;

    #[test]
    fn empty() {
        assert!(I::empty().ceil().is_empty());
        assert!(I::empty().floor().is_empty());
        assert!(I::empty().round_ties_to_away().is_empty());
        assert!(I::empty().round_ties_to_even().is_empty());
        assert!(I::empty().sign().is_empty());
        assert!(I::empty().trunc().is_empty());
    }
}
