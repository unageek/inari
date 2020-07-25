use crate::classify::*;
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

fn mpfr_fn2(
    f: unsafe extern "C" fn(
        *mut mpfr::mpfr_t,
        *const mpfr::mpfr_t,
        *const mpfr::mpfr_t,
        mpfr::rnd_t,
    ) -> i32,
    x: f64,
    y: f64,
    rnd: mpfr::rnd_t,
) -> f64 {
    let mut x = Float::with_val(f64::MANTISSA_DIGITS, x);
    let y = Float::with_val(f64::MANTISSA_DIGITS, y);
    unsafe {
        f(x.as_raw_mut(), x.as_raw(), y.as_raw(), rnd);
        mpfr::get_d(x.as_raw(), rnd)
    }
}

macro_rules! mpfr_fn {
    ($mpfr_f:ident, $f_rd:ident, $f_ru:ident) => {
        fn $f_rd(x: f64) -> f64 {
            mpfr_fn(mpfr::$mpfr_f, x, mpfr::rnd_t::RNDD)
        }

        fn $f_ru(x: f64) -> f64 {
            mpfr_fn(mpfr::$mpfr_f, x, mpfr::rnd_t::RNDU)
        }
    };
}

macro_rules! mpfr_fn2 {
    ($mpfr_f:ident, $f_rd:ident, $f_ru:ident) => {
        fn $f_rd(x: f64, y: f64) -> f64 {
            mpfr_fn2(mpfr::$mpfr_f, x, y, mpfr::rnd_t::RNDD)
        }

        fn $f_ru(x: f64, y: f64) -> f64 {
            mpfr_fn2(mpfr::$mpfr_f, x, y, mpfr::rnd_t::RNDU)
        }
    };
}

mpfr_fn!(acos, acos_rd, acos_ru);
mpfr_fn!(acosh, acosh_rd, acosh_ru);
mpfr_fn!(asin, asin_rd, asin_ru);
mpfr_fn!(asinh, asinh_rd, asinh_ru);
mpfr_fn!(atan, atan_rd, atan_ru);
mpfr_fn2!(atan2, atan2_rd, atan2_ru);
mpfr_fn!(atanh, atanh_rd, atanh_ru);
mpfr_fn!(cos, cos_rd, cos_ru);
mpfr_fn!(cosh, cosh_rd, cosh_ru);
mpfr_fn!(exp, exp_rd, exp_ru);
mpfr_fn!(exp10, exp10_rd, exp10_ru);
mpfr_fn!(exp2, exp2_rd, exp2_ru);
mpfr_fn!(log, log_rd, log_ru);
mpfr_fn!(log10, log10_rd, log10_ru);
mpfr_fn!(log2, log2_rd, log2_ru);
mpfr_fn!(sin, sin_rd, sin_ru);
mpfr_fn!(sinh, sinh_rd, sinh_ru);
mpfr_fn!(tan, tan_rd, tan_ru);
mpfr_fn!(tanh, tanh_rd, tanh_ru);

fn rem_euclid_2(x: f64) -> f64 {
    if 2.0 * (x / 2.0).floor() == x {
        0.0
    } else {
        1.0
    }
}

macro_rules! impl_log {
    ($f:ident, $f_impl:ident, $f_rd:ident, $f_ru:ident) => {
        pub fn $f(self) -> Self {
            self.$f_impl().0
        }

        #[allow(clippy::many_single_char_names)]
        pub(crate) fn $f_impl(self) -> (Self, Decoration) {
            // See the comment in atanh_impl.
            let dom = Self::with_infsup_raw(0.0, f64::INFINITY);
            let x = self.intersection(dom);

            let a = x.inf_raw();
            let b = x.sup_raw();
            if x.is_empty() || b <= 0.0 {
                return (Self::EMPTY, Decoration::Trv);
            }

            let y = Self::with_infsup_raw($f_rd(a), $f_ru(b));
            let d = if a > 0.0 {
                Decoration::Com
            } else {
                Decoration::Trv
            };
            (y, d)
        }
    };
}

macro_rules! impl_mono_inc {
    ($f:ident, $f_rd:ident, $f_ru:ident) => {
        pub fn $f(self) -> Self {
            if self.is_empty() {
                return self;
            }

            Self::with_infsup_raw($f_rd(self.inf_raw()), $f_ru(self.sup_raw()))
        }
    };
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

    pub fn acosh(self) -> Self {
        self.acosh_impl().0
    }

    pub(crate) fn acosh_impl(self) -> (Self, Decoration) {
        let dom = Self::with_infsup_raw(1.0, f64::INFINITY);
        let x = self.intersection(dom);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(acosh_rd(x.inf_raw()), acosh_ru(x.sup_raw()));
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

    impl_mono_inc!(asinh, asinh_rd, asinh_ru);
    impl_mono_inc!(atan, atan_rd, atan_ru);

    pub fn atan2(self, rhs: Self) -> Self {
        self.atan2_impl(rhs).0
    }

    #[allow(clippy::many_single_char_names)]
    pub fn atan2_impl(self, rhs: Self) -> (Self, Decoration) {
        let (x, y) = (rhs, self);
        let a = x.inf_raw();
        let b = x.sup_raw();
        let c = y.inf_raw();
        let d = y.sup_raw();

        // (a, d)      (b, d)
        //      +------+
        //      |      |
        //      |      |
        //      +------+
        // (a, c)      (b, c)

        match x.classify2(y) {
            C_E_E | C_E_M | C_E_N0 | C_E_N1 | C_E_P0 | C_E_P1 | C_E_Z | C_M_E | C_N0_E | C_N1_E
            | C_P0_E | C_P1_E | C_Z_E | C_Z_Z => (Self::EMPTY, Decoration::Trv),
            C_M_M | C_M_N0 | C_N0_M | C_N0_N0 => (
                Self::with_infsup_raw(-Self::PI.sup_raw(), Self::PI.sup_raw()),
                Decoration::Trv,
            ),
            // First quadrant
            C_P0_P0 => (
                Self::with_infsup_raw(0.0, Self::FRAC_PI_2.sup_raw()),
                Decoration::Trv,
            ),
            C_P0_P1 | C_P1_P0 | C_P1_P1 => (
                Self::with_infsup_raw(atan2_rd(c, b), atan2_ru(d, a)),
                Decoration::Dac,
            ),
            // First & second quadrant
            C_M_P0 | C_M_Z => (
                Self::with_infsup_raw(0.0, Self::PI.sup_raw()),
                Decoration::Trv,
            ),
            C_M_P1 => (
                Self::with_infsup_raw(atan2_rd(c, b), atan2_ru(c, a)),
                Decoration::Dac,
            ),
            // Second quadrant
            C_N0_P0 => (
                Self::with_infsup_raw(Self::FRAC_PI_2.inf_raw(), Self::PI.sup_raw()),
                Decoration::Trv,
            ),
            C_N0_P1 | C_N1_P1 => (
                Self::with_infsup_raw(atan2_rd(d, b), atan2_ru(c, a)),
                Decoration::Dac,
            ),
            C_N1_P0 => (
                Self::with_infsup_raw(atan2_rd(d, b), Self::PI.sup_raw()),
                Decoration::Def,
            ),
            // Second & third quadrant
            // C_N0_M => See above.
            C_N1_M | C_N1_N0 => (
                Self::with_infsup_raw(-Self::PI.sup_raw(), Self::PI.sup_raw()),
                Decoration::Def,
            ),
            // Third quadrant
            // C_N0_N0 => See above.
            C_N0_N1 | C_N1_N1 => (
                Self::with_infsup_raw(atan2_rd(d, a), atan2_ru(c, b)),
                Decoration::Dac,
            ),
            // C_N1_N0 => See above.
            // Third & fourth quadrant
            // C_M_N0 => See above.
            C_M_N1 => (
                Self::with_infsup_raw(atan2_rd(d, a), atan2_ru(d, b)),
                Decoration::Dac,
            ),
            // Fourth quadrant
            C_P0_N0 => (
                Self::with_infsup_raw(-Self::FRAC_PI_2.sup_raw(), 0.0),
                Decoration::Trv,
            ),
            C_P0_N1 | C_P1_N0 | C_P1_N1 => (
                Self::with_infsup_raw(atan2_rd(c, a), atan2_ru(d, b)),
                Decoration::Dac,
            ),
            // Fourth & first quadrant
            C_P0_M | C_Z_M => (
                Self::with_infsup_raw(-Self::FRAC_PI_2.sup_raw(), Self::FRAC_PI_2.sup_raw()),
                Decoration::Trv,
            ),
            C_P1_M => (
                Self::with_infsup_raw(atan2_rd(c, a), atan2_ru(d, a)),
                Decoration::Dac,
            ),
            // X axis
            // C_M_Z => See above.
            C_N0_Z => (Self::PI, Decoration::Trv),
            C_N1_Z => (Self::PI, Decoration::Def),
            C_P0_Z => (Self::with_infsup_raw(0.0, 0.0), Decoration::Trv),
            C_P1_Z => (Self::with_infsup_raw(0.0, 0.0), Decoration::Dac),
            // Y axis
            // C_Z_M => See above.
            C_Z_N0 => (-Self::FRAC_PI_2, Decoration::Trv),
            C_Z_N1 => (-Self::FRAC_PI_2, Decoration::Dac),
            C_Z_P0 => (Self::FRAC_PI_2, Decoration::Trv),
            C_Z_P1 => (Self::FRAC_PI_2, Decoration::Dac),
            _ => unreachable!(),
        }
    }

    pub fn atanh(self) -> Self {
        self.atanh_impl().0
    }

    #[allow(clippy::many_single_char_names)]
    pub(crate) fn atanh_impl(self) -> (Self, Decoration) {
        // Mathematically, the domain of atanh is (-1.0, 1.0), not [-1.0, 1.0].
        // However, IEEE 754 and thus MPFR define atanh to return ±infinity for ±1.0
        // (and signal the divideByZero exception), so we will make use of that.
        let dom = Self::with_infsup_raw(-1.0, 1.0);
        let x = self.intersection(dom);

        let a = x.inf_raw();
        let b = x.sup_raw();
        if x.is_empty() || b <= -1.0 || a >= 1.0 {
            return (Self::EMPTY, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(atanh_rd(a), atanh_ru(b));
        let d = if a > -1.0 && b < 1.0 {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        (y, d)
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

    pub fn cosh(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        if b < 0.0 {
            Self::with_infsup_raw(cosh_rd(b), cosh_ru(a))
        } else if a > 0.0 {
            Self::with_infsup_raw(cosh_rd(a), cosh_ru(b))
        } else {
            Self::with_infsup_raw(1.0, cosh_ru((-a).max(b)))
        }
    }

    impl_mono_inc!(exp, exp_rd, exp_ru);
    impl_mono_inc!(exp10, exp10_rd, exp10_ru);
    impl_mono_inc!(exp2, exp2_rd, exp2_ru);

    impl_log!(log, log_impl, log_rd, log_ru);
    impl_log!(log10, log10_impl, log10_rd, log10_ru);
    impl_log!(log2, log2_impl, log2_rd, log2_ru);

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

    impl_mono_inc!(sinh, sinh_rd, sinh_ru);

    pub fn tan(self) -> Self {
        self.tan_impl().0
    }

    pub(crate) fn tan_impl(self) -> (Self, Decoration) {
        if self.is_empty() {
            return (self, Decoration::Trv);
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Interval::FRAC_PI_2).floor();
        let qa = q_nowrap.inf_raw();
        let qb = q_nowrap.sup_raw();
        let n = if a == b { 0.0 } else { qb - qa };
        let q = rem_euclid_2(qa);

        let cont = self.sup()
            <= (Self::with_infsup_raw(q_nowrap.sup(), q_nowrap.sup()) * Interval::FRAC_PI_2).inf();
        if q == 0.0 && (n < 1.0 || n == 1.0 && cont) || q == 1.0 && (n < 2.0 || n == 2.0 && cont) {
            // In case of overflow, the decoration must be corrected by the caller.
            (Self::with_infsup_raw(tan_rd(a), tan_ru(b)), Decoration::Com)
        } else {
            (Self::ENTIRE, Decoration::Trv)
        }
    }

    impl_mono_inc!(tanh, tanh_rd, tanh_ru);
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

macro_rules! impl_dec2 {
    ($f:ident, $f_impl:ident) => {
        pub fn $f(self, rhs: Self) -> Self {
            let (z, d) = self.x.$f_impl(rhs.x);
            Self::set_dec(z, self.d.min(rhs.d.min(d)))
        }
    };
}

impl DecoratedInterval {
    impl_dec!(acos, acos_impl);
    impl_dec!(acosh, acosh_impl);
    impl_dec!(asin, asin_impl);
    impl_dec!(asinh);
    impl_dec!(atan);
    impl_dec2!(atan2, atan2_impl);
    impl_dec!(atanh, atanh_impl);
    impl_dec!(cos);
    impl_dec!(cosh);
    impl_dec!(exp);
    impl_dec!(exp10);
    impl_dec!(exp2);
    impl_dec!(log, log_impl);
    impl_dec!(log10, log10_impl);
    impl_dec!(log2, log2_impl);
    impl_dec!(sin);
    impl_dec!(sinh);
    impl_dec!(tan, tan_impl);
    impl_dec!(tanh);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn nai() {
        assert!(DI::NAI.acos().is_nai());
        assert!(DI::NAI.acosh().is_nai());
        assert!(DI::NAI.asin().is_nai());
        assert!(DI::NAI.asinh().is_nai());
        assert!(DI::NAI.atan().is_nai());
        assert!(DI::NAI.atan2(DI::EMPTY).is_nai());
        assert!(DI::EMPTY.atan2(DI::NAI).is_nai());
        assert!(DI::NAI.atanh().is_nai());
        assert!(DI::NAI.cos().is_nai());
        assert!(DI::NAI.cosh().is_nai());
        assert!(DI::NAI.exp().is_nai());
        assert!(DI::NAI.exp10().is_nai());
        assert!(DI::NAI.exp2().is_nai());
        assert!(DI::NAI.log().is_nai());
        assert!(DI::NAI.log10().is_nai());
        assert!(DI::NAI.log2().is_nai());
        assert!(DI::NAI.sin().is_nai());
        assert!(DI::NAI.sinh().is_nai());
        assert!(DI::NAI.tan().is_nai());
        assert!(DI::NAI.tanh().is_nai());
    }

    #[test]
    fn tan() {
        // a, b ∊ (-π/2, π/2)
        assert!(interval!(std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf())
            .unwrap()
            .tan()
            .is_common_interval());
        assert!(interval!(-std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf())
            .unwrap()
            .tan()
            .is_common_interval());

        // a, b ∊ (-3π/2, -π/2)
        assert!(
            interval!(-3.0 * std::f64::consts::FRAC_PI_4, -I::FRAC_PI_2.sup())
                .unwrap()
                .tan()
                .is_common_interval()
        );
        assert!(
            interval!(-5.0 * std::f64::consts::FRAC_PI_4, -I::FRAC_PI_2.sup())
                .unwrap()
                .tan()
                .is_common_interval()
        );
    }
}
