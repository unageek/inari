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

macro_rules! impl_dec {
    // is_not_com(x, y) tests if f is not continuous at some point of x,
    // provided that x and y := f(x) are bounded and p_dac(f, x) holds,
    // in which case y is a singleton.
    // The boundedness of x and y are checked by the last statement.
    // In rounding functions, you can effectively check if an endpoint of x
    // is an integer by x.inf == y.inf or x.sup == y.sup.
    ($f:ident, $x:ident, $y:ident, $is_not_com:expr) => {
        pub fn $f(self) -> Self {
            if self.is_nai() {
                return self;
            }

            let $x = self.x;
            let $y = $x.$f();
            let d = if $y.is_empty() {
                Decoration::Trv
            } else if !$y.is_singleton() {
                Decoration::Def
            } else if $is_not_com {
                Decoration::Dac
            } else {
                Decoration::Com
            };
            Self::set_dec($y, d.min(self.d))
        }
    };
}

// https://www.ocf.berkeley.edu/~horie/rounding.html
impl DecoratedInterval {
    // Discontinuous at x iff x ∊ ℤ.
    impl_dec!(ceil, x, y, x.sup_raw() == y.sup_raw()); // No need to check inf.
    impl_dec!(floor, x, y, x.inf_raw() == y.inf_raw()); // No need to check sup.

    // Discontinuous at x iff x ∊ {x′ + 0.5 | x′ ∊ ℤ}.
    impl_dec!(round_ties_to_away, x, y, {
        let abs_a = x.inf_raw().abs();
        let abs_b = x.sup_raw().abs();
        (abs_a - abs_a.trunc() == 0.5) || (abs_b - abs_b.trunc() == 0.5)
    });
    impl_dec!(round_ties_to_even, x, y, {
        let abs_a = x.inf_raw().abs();
        let abs_b = x.sup_raw().abs();
        (abs_a - abs_a.trunc() == 0.5) || (abs_b - abs_b.trunc() == 0.5)
    });

    // Discontinuous at 0.
    impl_dec!(sign, x, y, x.inf_raw() == 0.0); // No need to check sup.

    // Discontinuous at x iff x ∊ ℤ ∖ {0}.
    impl_dec!(
        trunc,
        x,
        y,
        (x.inf_raw() != 0.0 && x.inf_raw() == y.inf_raw())
            || (x.sup_raw() != 0.0 && x.sup_raw() == y.sup_raw())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn empty() {
        assert!(I::empty().ceil().is_empty());
        assert!(I::empty().floor().is_empty());
        assert!(I::empty().round_ties_to_away().is_empty());
        assert!(I::empty().round_ties_to_even().is_empty());
        assert!(I::empty().sign().is_empty());
        assert!(I::empty().trunc().is_empty());

        assert!(DI::empty().ceil().is_empty());
        assert!(DI::empty().floor().is_empty());
        assert!(DI::empty().round_ties_to_away().is_empty());
        assert!(DI::empty().round_ties_to_even().is_empty());
        assert!(DI::empty().sign().is_empty());
        assert!(DI::empty().trunc().is_empty());
    }

    #[test]
    fn nai() {
        assert!(DI::nai().ceil().is_nai());
        assert!(DI::nai().floor().is_nai());
        assert!(DI::nai().round_ties_to_away().is_nai());
        assert!(DI::nai().round_ties_to_even().is_nai());
        assert!(DI::nai().sign().is_nai());
        assert!(DI::nai().trunc().is_nai());
    }
}
