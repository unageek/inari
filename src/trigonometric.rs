use crate::interval::*;
use gmp_mpfr_sys::mpfr;
use rug::Float;

fn mpfr_fn(
    f: unsafe extern "C" fn(*mut mpfr::mpfr_t, *const mpfr::mpfr_t, mpfr::rnd_t) -> i32,
    x: f64,
    rnd: mpfr::rnd_t,
) -> f64 {
    let mut x = Float::with_val(f64::MANTISSA_DIGITS, x);
    unsafe {
        f(x.as_raw_mut(), x.as_raw(), rnd);
        mpfr::get_d(x.as_raw(), rnd)
    }
}

fn cos_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::cos, x, mpfr::rnd_t::RNDD)
}

fn cos_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::cos, x, mpfr::rnd_t::RNDU)
}

fn sin_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::sin, x, mpfr::rnd_t::RNDD)
}

fn sin_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::sin, x, mpfr::rnd_t::RNDU)
}

fn tan_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::tan, x, mpfr::rnd_t::RNDD)
}

fn tan_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::tan, x, mpfr::rnd_t::RNDU)
}

fn rem_euclid_2(x: f64) -> f64 {
    if 2.0 * (x / 2.0).floor() == x {
        0.0
    } else {
        1.0
    }
}

impl Interval {
    pub fn cos(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf();
        let b = self.sup();
        let q_nowrap = (self / Interval::PI).floor();
        // n and p are valid for small values.
        let n = if a == b {
            // Ad-hoc treatment for a huge value.
            0.0
        } else {
            q_nowrap.sup() - q_nowrap.inf()
        };
        // "Quadrant" (polarity?) of a.
        let q = rem_euclid_2(q_nowrap.inf());

        // Overestimation is fine.
        if n == 0.0 {
            if q == 0.0 {
                // monotonically decreasing
                Self::with_infsup_raw(cos_rd(b), cos_ru(a))
            } else {
                // monotonically increasing
                Self::with_infsup_raw(cos_rd(a), cos_ru(b))
            }
        } else if n <= 1.0 {
            if q == 0.0 {
                // decreasing, then increasing
                Self::with_infsup_raw(-1.0, cos_ru(a).max(cos_ru(b)))
            } else {
                // increasing, then decreasing
                Self::with_infsup_raw(cos_rd(a).min(cos_rd(b)), 1.0)
            }
        } else {
            Self::with_infsup_raw(-1.0, 1.0)
        }
    }

    pub fn sin(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf();
        let b = self.sup();
        let q_nowrap = (self / Interval::FRAC_PI_2).floor();
        // n and q are valid for small values.
        let n = if a == b {
            0.0
        } else {
            q_nowrap.sup() - q_nowrap.inf()
        };
        // Quadrant of a.
        let q = q_nowrap.inf().rem_euclid(4.0);

        if q == 0.0 && n < 1.0 || q == 3.0 && n < 2.0 {
            // monotonically increasing
            Self::with_infsup_raw(sin_rd(a), sin_ru(b))
        } else if q == 1.0 && n < 2.0 || q == 2.0 && n < 1.0 {
            // monotonically decreasing
            Self::with_infsup_raw(sin_rd(b), sin_ru(a))
        } else if q == 0.0 && n < 3.0 || q == 3.0 && n < 4.0 {
            // increasing, then decreasing
            Self::with_infsup_raw(sin_rd(a).min(sin_rd(b)), 1.0)
        } else if q == 1.0 && n < 4.0 || q == 2.0 && n < 3.0 {
            // decreasing, then increasing
            Self::with_infsup_raw(-1.0, sin_ru(a).max(sin_ru(b)))
        } else {
            Self::with_infsup_raw(-1.0, 1.0)
        }
    }

    pub(crate) fn tan_impl(self) -> (Self, bool) {
        if self.is_empty() {
            return (self, true);
        }

        let a = self.inf();
        let b = self.sup();
        let mut q_nowrap = (self / Interval::FRAC_PI_2).floor();
        if self.sup_raw() == Interval::FRAC_PI_2.inf() {
            // For a strict test case.
            q_nowrap = Self::with_infsup_raw(q_nowrap.inf_raw(), 0.0);
        }
        // n and q are valid for small values.
        let n = if a == b {
            0.0
        } else {
            q_nowrap.sup() - q_nowrap.inf()
        };
        let q = rem_euclid_2(q_nowrap.inf());

        println!("x: {}, q_nowrap: {}, n: {}, q: {}", self, q_nowrap, n, q);
        if q == 0.0 && n < 1.0 || q == 1.0 && n < 2.0 {
            (Self::with_infsup_raw(tan_rd(a), tan_ru(b)), true)
        } else {
            (Self::entire(), false)
        }
    }

    pub fn tan(self) -> Self {
        self.tan_impl().0
    }
}

macro_rules! impl_dec {
    ($f:ident) => {
        pub fn $f(self) -> Self {
            Self::set_dec(self.x.$f(), self.d)
        }
    };
}

impl DecoratedInterval {
    impl_dec!(cos);
    impl_dec!(sin);

    pub fn tan(self) -> Self {
        let (y, def) = self.x.tan_impl();
        let d = if def {
            self.d
        } else {
            self.d.min(Decoration::Trv)
        };
        Self::set_dec(y, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type DI = DecoratedInterval;

    #[test]
    fn nai() {
        assert!(DI::nai().cos().is_nai());
        assert!(DI::nai().sin().is_nai());
        assert!(DI::nai().tan().is_nai());
    }
}
