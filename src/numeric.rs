use crate::{interval::*, simd::*};
use std::arch::x86_64::*;

impl Interval {
    /// Returns the lower bound of `self`. If `self` is empty, `f64::INFINITY` is returned.
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

    pub fn mag(self) -> f64 {
        let abs = abs(self.rep);
        unsafe { _mm_cvtsd_f64(_mm_max_sd(abs, swap(abs))) }
    }

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

    pub fn mig(self) -> f64 {
        let zero = unsafe { _mm_setzero_pd() };
        let contains_zero = unsafe { _mm_movemask_pd(_mm_cmpge_pd(self.rep, zero)) == 3 };
        if contains_zero {
            return 0.0;
        }

        let abs = abs(self.rep);
        unsafe { _mm_cvtsd_f64(_mm_min_sd(abs, swap(abs))) }
    }

    pub fn rad(self) -> f64 {
        let m = self.mid();
        f64::max(sub1_ru(m, self.inf_raw()), sub1_ru(self.sup_raw(), m))
    }

    /// Returns the upper bound of `self`. If `self` is empty, `f64::NEG_INFINITY` is returned.
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
    use crate::interval;

    #[test]
    fn inf() {
        assert!(interval!(0.0, 0.0).unwrap().inf().is_sign_negative());
        assert!(interval!(0.0, -0.0).unwrap().inf().is_sign_negative());
        assert!(interval!(-0.0, 0.0).unwrap().inf().is_sign_negative());
        assert!(interval!(-0.0, -0.0).unwrap().inf().is_sign_negative());
    }

    #[test]
    fn mag() {
        assert!(interval!(0.0, 0.0).unwrap().mag().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().mag().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().mag().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().mag().is_sign_positive());
    }

    #[test]
    fn mid() {
        assert!(interval!(0.0, 0.0).unwrap().mid().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().mid().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().mid().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().mid().is_sign_positive());
    }

    #[test]
    fn mig() {
        assert!(interval!(0.0, 0.0).unwrap().mig().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().mig().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().mig().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().mig().is_sign_positive());
    }

    #[test]
    fn rad() {
        assert!(interval!(0.0, 0.0).unwrap().rad().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().rad().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().rad().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().rad().is_sign_positive());
    }

    #[test]
    fn sup() {
        assert!(interval!(0.0, 0.0).unwrap().sup().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().sup().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().sup().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().sup().is_sign_positive());
    }

    #[test]
    fn wid() {
        assert!(interval!(0.0, 0.0).unwrap().wid().is_sign_positive());
        assert!(interval!(0.0, -0.0).unwrap().wid().is_sign_positive());
        assert!(interval!(-0.0, 0.0).unwrap().wid().is_sign_positive());
        assert!(interval!(-0.0, -0.0).unwrap().wid().is_sign_positive());

        // Check if the result is rounded up.
        assert_eq!(
            interval!(-f64::MIN_POSITIVE, f64::MAX).unwrap().wid(),
            f64::INFINITY
        );
    }
}
