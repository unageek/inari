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

fn acos_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::acos, x, mpfr::rnd_t::RNDD)
}

fn acos_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::acos, x, mpfr::rnd_t::RNDU)
}

fn asin_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::asin, x, mpfr::rnd_t::RNDD)
}

fn asin_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::asin, x, mpfr::rnd_t::RNDU)
}

fn atan_rd(x: f64) -> f64 {
    mpfr_fn(mpfr::atan, x, mpfr::rnd_t::RNDD)
}

fn atan_ru(x: f64) -> f64 {
    mpfr_fn(mpfr::atan, x, mpfr::rnd_t::RNDU)
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
    pub fn acos(self) -> Self {
        self.acos_impl().0
    }

    pub(crate) fn acos_impl(self) -> (Self, Decoration) {
        let dom = Self::with_infsup_raw(-1.0, 1.0);
        let x = self.intersection(dom);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(acos_rd(x.sup_raw()), acos_ru(x.inf_raw()));
        let d = if x.interior(dom) {
            Decoration::Com
        } else {
            Decoration::Def
        };
        (y, d)
    }

    pub fn asin(self) -> Self {
        self.asin_impl().0
    }

    pub(crate) fn asin_impl(self) -> (Self, Decoration) {
        let dom = Self::with_infsup_raw(-1.0, 1.0);
        let x = self.intersection(dom);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(asin_rd(x.inf_raw()), asin_ru(x.sup_raw()));
        let d = if x.interior(dom) {
            Decoration::Com
        } else {
            Decoration::Def
        };
        (y, d)
    }

    pub fn atan(self) -> Self {
        if self.is_empty() {
            return self;
        }

        Self::with_infsup_raw(atan_rd(self.inf_raw()), atan_ru(self.sup_raw()))
    }

    pub fn cos(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Interval::PI).floor();
        let qa = q_nowrap.inf_raw();
        let qb = q_nowrap.sup_raw();
        // n and q are valid for small values.
        let n = if a == b {
            // For strict test cases on huge values.
            0.0
        } else {
            qb - qa
        };
        let q = rem_euclid_2(qa);

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

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Interval::FRAC_PI_2).floor();
        let qa = q_nowrap.inf_raw();
        let qb = q_nowrap.sup_raw();
        let n = if a == b { 0.0 } else { qb - qa };
        let q = qa.rem_euclid(4.0);

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

    pub fn tan(self) -> Self {
        self.tan_impl().0
    }

    pub(crate) fn tan_impl(self) -> (Self, Decoration) {
        if self.is_empty() {
            return (self, Decoration::Trv);
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let mut q_nowrap = (self / Interval::FRAC_PI_2).floor();
        if b == Interval::FRAC_PI_2.inf_raw() {
            // For strict test cases.
            q_nowrap = Self::with_infsup_raw(q_nowrap.inf_raw(), 0.0);
        }
        let qa = q_nowrap.inf_raw();
        let qb = q_nowrap.sup_raw();
        let n = if a == b { 0.0 } else { qb - qa };
        let q = rem_euclid_2(qa);

        println!("x: {}, q_nowrap: {}, n: {}, q: {}", self, q_nowrap, n, q);
        if q == 0.0 && n < 1.0 || q == 1.0 && n < 2.0 {
            // In case of overflow, the decoration must be corrected by the caller.
            (Self::with_infsup_raw(tan_rd(a), tan_ru(b)), Decoration::Com)
        } else {
            (Self::entire(), Decoration::Trv)
        }
    }
}

macro_rules! impl_dec {
    ($f:ident) => {
        pub fn $f(self) -> Self {
            Self::set_dec(self.x.$f(), self.d)
        }
    };

    ($f:ident, $f_impl:ident) => {
        pub fn $f(self) -> Self {
            let (y, d) = self.x.$f_impl();
            Self::set_dec(y, self.d.min(d))
        }
    };
}

impl DecoratedInterval {
    impl_dec!(acos, acos_impl);
    impl_dec!(asin, asin_impl);
    impl_dec!(atan);
    impl_dec!(cos);
    impl_dec!(sin);
    impl_dec!(tan, tan_impl);
}

#[cfg(test)]
mod tests {
    use super::*;
    type DI = DecoratedInterval;

    #[test]
    fn nai() {
        assert!(DI::nai().acos().is_nai());
        assert!(DI::nai().asin().is_nai());
        assert!(DI::nai().atan().is_nai());
        assert!(DI::nai().cos().is_nai());
        assert!(DI::nai().sin().is_nai());
        assert!(DI::nai().tan().is_nai());
    }
}
