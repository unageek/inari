use crate::{interval::*, simd::*};
use std::arch::x86_64::*;

impl Interval {
    /// Returns the lower bound of `self` if `self` is nonempty; otherwise, $+∞$.
    pub fn inf(self) -> f64 {
        let x = self.inf_raw();
        if x.is_nan() {
            // The empty interval.
            f64::INFINITY
        } else if x == 0.0 {
            -0.0
        } else {
            x
        }
    }

    /// Returns the magnitude of `self` if `self` is nonempty; otherwise, NaN.
    pub fn mag(self) -> f64 {
        let abs = abs(self.rep);
        unsafe { _mm_cvtsd_f64(_mm_max_sd(abs, swap(abs))) }
    }

    /// Returns the midpoint of `self` if `self` is nonempty; otherwise, NaN.
    // See Table VII in https://doi.org/10.1145/2493882 for the definition.
    pub fn mid(self) -> f64 {
        let a = self.inf_raw();
        let b = self.sup_raw();

        match (a == f64::NEG_INFINITY, b == f64::INFINITY) {
            (false, false) => {
                let mid = 0.5 * (a + b);
                if mid.is_infinite() {
                    0.5 * a + 0.5 * b
                } else if mid == 0.0 {
                    0.0
                } else {
                    mid
                }
            }
            (false, true) => f64::MAX,
            (true, false) => f64::MIN,
            (true, true) => 0.0,
        }
    }

    /// Returns the mignitude of `self` if `self` is nonempty; otherwise, NaN.
    pub fn mig(self) -> f64 {
        let zero = unsafe { _mm_setzero_pd() };
        let contains_zero = unsafe { _mm_movemask_pd(_mm_cmpge_pd(self.rep, zero)) == 3 };
        if contains_zero {
            return 0.0;
        }

        let abs = abs(self.rep);
        unsafe { _mm_cvtsd_f64(_mm_min_sd(abs, swap(abs))) }
    }

    /// Returns the radius of `self` if `self` is nonempty; otherwise, NaN.
    pub fn rad(self) -> f64 {
        let m = self.mid();
        f64::max(sub1_ru(m, self.inf_raw()), sub1_ru(self.sup_raw(), m))
    }

    /// Returns the upper bound of `self` if `self` is nonempty; otherwise, $-∞$.
    pub fn sup(self) -> f64 {
        let x = self.sup_raw();
        if x.is_nan() {
            // The empty interval.
            f64::NEG_INFINITY
        } else if x == 0.0 {
            0.0
        } else {
            x
        }
    }

    /// Returns the width of `self` if `self` is nonempty; otherwise, NaN.
    pub fn wid(self) -> f64 {
        let wid = sub1_ru(self.sup_raw(), self.inf_raw());
        if wid == 0.0 {
            0.0
        } else {
            wid
        }
    }
}

macro_rules! impl_dec {
    ($f:ident) => {
        pub fn $f(self) -> f64 {
            if self.is_nai() {
                return f64::NAN;
            }

            self.x.$f()
        }
    };
}

impl DecoratedInterval {
    impl_dec!(inf);
    impl_dec!(mag);
    impl_dec!(mid);
    impl_dec!(mig);
    impl_dec!(rad);
    impl_dec!(sup);
    impl_dec!(wid);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn inf() {
        assert!(const_interval!(0.0, 0.0).inf().is_sign_negative());
        assert!(const_interval!(0.0, -0.0).inf().is_sign_negative());
        assert!(const_interval!(-0.0, 0.0).inf().is_sign_negative());
        assert!(const_interval!(-0.0, -0.0).inf().is_sign_negative());
    }

    #[test]
    fn mag() {
        assert!(const_interval!(0.0, 0.0).mag().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).mag().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).mag().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).mag().is_sign_positive());
    }

    #[test]
    fn mid() {
        assert!(const_interval!(0.0, 0.0).mid().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).mid().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).mid().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).mid().is_sign_positive());
    }

    #[test]
    fn mig() {
        assert!(const_interval!(0.0, 0.0).mig().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).mig().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).mig().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).mig().is_sign_positive());
    }

    #[test]
    fn rad() {
        assert!(const_interval!(0.0, 0.0).rad().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).rad().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).rad().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).rad().is_sign_positive());
    }

    #[test]
    fn sup() {
        assert!(const_interval!(0.0, 0.0).sup().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).sup().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).sup().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).sup().is_sign_positive());
    }

    #[test]
    fn wid() {
        assert!(const_interval!(0.0, 0.0).wid().is_sign_positive());
        assert!(const_interval!(0.0, -0.0).wid().is_sign_positive());
        assert!(const_interval!(-0.0, 0.0).wid().is_sign_positive());
        assert!(const_interval!(-0.0, -0.0).wid().is_sign_positive());

        // Check if the result is rounded up.
        assert_eq!(
            const_interval!(-f64::MIN_POSITIVE, f64::MAX).wid(),
            f64::INFINITY
        );
    }
}
