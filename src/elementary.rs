use crate::{classify::*, const_interval, interval::*};

#[cfg_attr(feature = "crlibm", allow(dead_code))]
mod mpfr;
#[cfg(not(feature = "crlibm"))]
use self::mpfr::*;

#[cfg(feature = "crlibm")]
pub use crlibm::*;

// Functions not provided by crlibm.
#[cfg(feature = "crlibm")]
pub use self::mpfr::{
    acosh_rd, acosh_ru, asinh_rd, asinh_ru, atan2_rd, atan2_ru,
    atanh_rd, atanh_ru, exp10_rd, exp10_ru, exp2_rd, exp2_ru,
    pow_rd, pow_ru, pown_rd, pown_ru, tanh_rd, tanh_ru};
// Functions provided by crlibm which could be interesting here:
// cospi, sinpi, tanpi, atanpi, asinpi, acospi, exp_m1, ln_1p


fn rem_euclid_2(x: f64) -> f64 {
    if 2.0 * (x / 2.0).floor() == x {
        0.0
    } else {
        1.0
    }
}

macro_rules! impl_log {
    ($(#[$meta:meta])* $f:ident, $f_impl:ident, $f_rd:ident, $f_ru:ident) => {
        $(#[$meta])*
        pub fn $f(self) -> Self {
            self.$f_impl().0
        }

        #[allow(clippy::many_single_char_names)]
        fn $f_impl(self) -> (Self, Decoration) {
            // See the comment in atanh_impl.
            const DOM: Interval = const_interval!(0.0, f64::INFINITY);
            let x = self.intersection(DOM);

            let a = x.inf_raw();
            let b = x.sup_raw();
            if x.is_empty() || b <= 0.0 {
                return (Self::EMPTY, Decoration::Trv);
            }

            let y = Self::with_infsup_raw($f_rd(a), $f_ru(b));
            let d = if self.interior(DOM) {
                Decoration::Com
            } else {
                Decoration::Trv
            };
            (y, d)
        }
    };
}

macro_rules! impl_mono_inc {
    ($(#[$meta:meta])* $f:ident, $f_rd:ident, $f_ru:ident) => {
        $(#[$meta])*
        pub fn $f(self) -> Self {
            if self.is_empty() {
                return self;
            }

            Self::with_infsup_raw($f_rd(self.inf_raw()), $f_ru(self.sup_raw()))
        }
    };
}

impl Interval {
    /// Returns the inverse cosine of `self`.
    ///
    /// | Domain      | Range      |
    /// | ----------- | ---------- |
    /// | $\[-1, 1\]$ | $\[0, π\]$ |
    pub fn acos(self) -> Self {
        self.acos_impl().0
    }

    fn acos_impl(self) -> (Self, Decoration) {
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(acos_rd(x.sup_raw()), acos_ru(x.inf_raw()));
        let d = if self.subset(DOM) {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        (y, d)
    }

    /// Returns the inverse hyperbolic cosine of `self`.
    ///
    /// | Domain    | Range     |
    /// | --------- | --------- |
    /// | $\[1, ∞)$ | $\[0, ∞)$ |
    pub fn acosh(self) -> Self {
        self.acosh_impl().0
    }

    fn acosh_impl(self) -> (Self, Decoration) {
        const DOM: Interval = const_interval!(1.0, f64::INFINITY);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(acosh_rd(x.inf_raw()), acosh_ru(x.sup_raw()));
        let d = if self.subset(DOM) {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        (y, d)
    }

    /// Returns the inverse sine of `self`.
    ///
    /// | Domain      | Range           |
    /// | ----------- | --------------- |
    /// | $\[-1, 1\]$ | $\[-π/2, π/2\]$ |
    pub fn asin(self) -> Self {
        self.asin_impl().0
    }

    fn asin_impl(self) -> (Self, Decoration) {
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return (x, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(asin_rd(x.inf_raw()), asin_ru(x.sup_raw()));
        let d = if self.subset(DOM) {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        (y, d)
    }

    impl_mono_inc!(
        /// Returns the inverse hyperbolic sine of `self`.
        ///
        /// | Domain | Range |
        /// | ------ | ----- |
        /// | $\R$   | $\R$  |
        asinh,
        asinh_rd,
        asinh_ru
    );
    impl_mono_inc!(
        /// Returns the inverse tangent of `self`.
        ///
        /// | Domain | Range         |
        /// | ------ | ------------- |
        /// | $\R$   | $(-π/2, π/2)$ |
        atan,
        atan_rd,
        atan_ru
    );

    /// Returns the angle of the point $(\rhs, \self)$ measured counterclockwise from the positive
    /// $x$-axis in the Euclidean plane.
    ///
    /// | Domain                | Range      |
    /// | --------------------- | ---------- |
    /// | $\R^2 ∖ \set{(0, 0)}$ | $(-π, π\]$ |
    pub fn atan2(self, rhs: Self) -> Self {
        self.atan2_impl(rhs).0
    }

    #[allow(clippy::many_single_char_names)]
    fn atan2_impl(self, rhs: Self) -> (Self, Decoration) {
        let (x, y) = (rhs, self);
        let a = x.inf_raw();
        let b = x.sup_raw();
        let c = y.inf_raw();
        let d = y.sup_raw();

        use IntervalClass2::*;
        match x.classify2(y) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | N0_E | N1_E | P0_E | P1_E | Z_E
            | Z_Z => (Self::EMPTY, Decoration::Trv),
            M_M | M_N0 | N0_M | N0_N0 => (
                Self::with_infsup_raw(-Self::PI.sup_raw(), Self::PI.sup_raw()),
                Decoration::Trv,
            ),

            // First quadrant
            P0_P0 => (
                Self::with_infsup_raw(0.0, Self::FRAC_PI_2.sup_raw()),
                Decoration::Trv,
            ),
            P0_P1 | P1_P0 | P1_P1 | P1_Z | Z_P1 => (
                Self::with_infsup_raw(atan2_rd(c, b), atan2_ru(d, a)),
                Decoration::Com,
            ),

            // First & second quadrant
            M_P0 | M_Z => (
                Self::with_infsup_raw(0.0, Self::PI.sup_raw()),
                Decoration::Trv,
            ),
            M_P1 => (
                Self::with_infsup_raw(atan2_rd(c, b), atan2_ru(c, a)),
                Decoration::Com,
            ),

            // Second quadrant
            N0_P0 => (
                Self::with_infsup_raw(Self::FRAC_PI_2.inf_raw(), Self::PI.sup_raw()),
                Decoration::Trv,
            ),
            N0_P1 | N1_P1 => (
                Self::with_infsup_raw(atan2_rd(d, b), atan2_ru(c, a)),
                Decoration::Com,
            ),
            N1_P0 => (
                Self::with_infsup_raw(atan2_rd(d, b), Self::PI.sup_raw()),
                Decoration::Dac,
            ),

            // Second & third quadrant
            //N0_M => See above.
            N1_M | N1_N0 => (
                Self::with_infsup_raw(-Self::PI.sup_raw(), Self::PI.sup_raw()),
                Decoration::Def,
            ),

            // Third quadrant
            //N0_N0 => See above.
            N0_N1 | N1_N1 => (
                Self::with_infsup_raw(atan2_rd(d, a), atan2_ru(c, b)),
                Decoration::Com,
            ),
            //N1_N0 => See above.

            // Third & fourth quadrant
            //M_N0 => See above.
            M_N1 => (
                Self::with_infsup_raw(atan2_rd(d, a), atan2_ru(d, b)),
                Decoration::Com,
            ),

            // Fourth quadrant
            P0_N0 => (
                Self::with_infsup_raw(-Self::FRAC_PI_2.sup_raw(), 0.0),
                Decoration::Trv,
            ),
            P0_N1 | P1_N0 | P1_N1 | Z_N1 => (
                Self::with_infsup_raw(atan2_rd(c, a), atan2_ru(d, b)),
                Decoration::Com,
            ),

            // Fourth & first quadrant
            P0_M | Z_M => (
                Self::with_infsup_raw(-Self::FRAC_PI_2.sup_raw(), Self::FRAC_PI_2.sup_raw()),
                Decoration::Trv,
            ),
            P1_M => (
                Self::with_infsup_raw(atan2_rd(c, a), atan2_ru(d, a)),
                Decoration::Com,
            ),

            // X axis
            //M_Z => See above.
            N0_Z => (Self::PI, Decoration::Trv),
            // The next case cannot be merged with N1_P0 unless we replace -0.0 with +0.0
            // since IEEE 754/MPFR's atan2 returns ±π for y = ±0.0, x < 0.0, while we want only +π.
            N1_Z => (Self::PI, Decoration::Dac),
            P0_Z => (Self::zero(), Decoration::Trv),
            //P1_Z => See above.

            // Y axis
            //Z_M => See above.
            Z_N0 => (-Self::FRAC_PI_2, Decoration::Trv),
            //Z_N1 => See above.
            Z_P0 => (Self::FRAC_PI_2, Decoration::Trv),
            //Z_P1 => See above.
        }
    }

    /// Returns the inverse hyperbolic tangent of `self`.
    ///
    /// | Domain    | Range |
    /// | --------- | ----- |
    /// | $(-1, 1)$ | $\R$  |
    pub fn atanh(self) -> Self {
        self.atanh_impl().0
    }

    #[allow(clippy::many_single_char_names)]
    fn atanh_impl(self) -> (Self, Decoration) {
        // Mathematically, the domain of atanh is (-1.0, 1.0), not [-1.0, 1.0].
        // However, IEEE 754/MPFR's atanh returns ±infinity for ±1.0,
        // (and signals the divide-by-zero exception), so we will make use of that.
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        let a = x.inf_raw();
        let b = x.sup_raw();
        if x.is_empty() || b <= -1.0 || a >= 1.0 {
            return (Self::EMPTY, Decoration::Trv);
        }

        let y = Self::with_infsup_raw(atanh_rd(a), atanh_ru(b));
        let d = if self.interior(DOM) {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        (y, d)
    }

    /// Returns the cosine of `self`.
    ///
    /// | Domain | Range       |
    /// | ------ | ----------- |
    /// | $\R$   | $\[-1, 1\]$ |
    pub fn cos(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Self::PI).floor();
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
            const_interval!(-1.0, 1.0)
        }
    }

    /// Returns the hyperbolic cosine of `self`.
    ///
    /// | Domain | Range     |
    /// | ------ | --------- |
    /// | $\R$   | $\[1, ∞)$ |
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

    impl_mono_inc!(
        /// Returns `self` raised to the power of $\e$.
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp,
        exp_rd,
        exp_ru
    );
    impl_mono_inc!(
        /// Returns `self` raised to the power of 10.
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp10,
        exp10_rd,
        exp10_ru
    );
    impl_mono_inc!(
        /// Returns `self` raised to the power of 2.
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp2,
        exp2_rd,
        exp2_ru
    );

    impl_log!(
        /// Returns the natural logarithm of `self`.
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        ln,
        ln_impl,
        ln_rd,
        ln_ru
    );
    impl_log!(
        /// Returns the base-10 logarithm of `self`.
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        log10,
        log10_impl,
        log10_rd,
        log10_ru
    );
    impl_log!(
        /// Returns the base-2 logarithm of `self`.
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        log2,
        log2_impl,
        log2_rd,
        log2_ru
    );

    /// Returns `self` raised to the power of `rhs`.
    ///
    /// | Domain                              | Range     |
    /// | ----------------------------------- | --------- |
    /// | $((0, ∞) × \R) ∪ (\set 0 × (0, ∞))$ | $\[0, ∞)$ |
    pub fn pow(self, rhs: Self) -> Self {
        self.pow_impl(rhs).0
    }

    #[allow(clippy::many_single_char_names)]
    fn pow_impl(self, rhs: Self) -> (Self, Decoration) {
        const DOM: Interval = const_interval!(0.0, f64::INFINITY);
        let x = self.intersection(DOM);

        if x.either_empty(rhs) {
            return (Self::EMPTY, Decoration::Trv);
        }

        let a = x.inf_raw();
        let b = x.sup_raw();
        let c = rhs.inf_raw();
        let d = rhs.sup_raw();

        if d <= 0.0 {
            if b == 0.0 {
                return (Self::EMPTY, Decoration::Trv);
            }

            let dec = if self.interior(DOM) {
                Decoration::Com
            } else {
                Decoration::Trv
            };

            if b < 1.0 {
                (Self::with_infsup_raw(pow_rd(b, d), pow_ru(a, c)), dec)
            } else if a > 1.0 {
                (Self::with_infsup_raw(pow_rd(b, c), pow_ru(a, d)), dec)
            } else {
                (Self::with_infsup_raw(pow_rd(b, c), pow_ru(a, c)), dec)
            }
        } else if c > 0.0 {
            let dec = if self.subset(DOM) {
                Decoration::Com
            } else {
                Decoration::Trv
            };

            if b < 1.0 {
                (Self::with_infsup_raw(pow_rd(a, d), pow_ru(b, c)), dec)
            } else if a > 1.0 {
                (Self::with_infsup_raw(pow_rd(a, c), pow_ru(b, d)), dec)
            } else {
                (Self::with_infsup_raw(pow_rd(a, d), pow_ru(b, d)), dec)
            }
        } else {
            if b == 0.0 {
                return (Self::zero(), Decoration::Trv);
            }

            let z_ac = pow_ru(a, c);
            let z_ad = pow_rd(a, d);
            let z_bc = pow_rd(b, c);
            let z_bd = pow_ru(b, d);
            let dec = if self.interior(DOM) {
                Decoration::Com
            } else {
                Decoration::Trv
            };

            (Self::with_infsup_raw(z_ad.min(z_bc), z_ac.max(z_bd)), dec)
        }
    }

    /// Returns `self` raised to the power of `rhs`.
    ///
    /// For a fixed $n ∈ \Z$, the domain and the range of the point function $\operatorname{pown}(x, n)$ are:
    ///
    /// |                | Domain        | Range         |
    /// | -------------- | ------------- | ------------- |
    /// | $n > 0$, odd   | $\R$          | $\R$          |
    /// | $n > 0$, even  | $\R$          | $\[0, ∞)$     |
    /// | $n = 0$        | $\R$          | $\set 1$      |
    /// | $n < 0$, odd   | $\R ∖ \set 0$ | $\R ∖ \set 0$ |
    /// | $n < 0$, even  | $\R ∖ \set 0$ | $(0, ∞)$      |
    pub fn pown(self, rhs: i32) -> Self {
        self.pown_impl(rhs).0
    }

    fn pown_impl(self, rhs: i32) -> (Self, Decoration) {
        if self.is_empty() {
            return (self, Decoration::Trv);
        }

        let mut a = self.inf_raw();
        let mut b = self.sup_raw();

        #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
        if rhs < 0 {
            let d = if a <= 0.0 && b >= 0.0 {
                Decoration::Trv
            } else {
                Decoration::Com
            };

            if a == 0.0 && b == 0.0 {
                return (Self::EMPTY, d);
            }

            if rhs % 2 == 0 {
                let abs = self.abs();
                (
                    Self::with_infsup_raw(pown_rd(abs.sup_raw(), rhs), pown_ru(abs.inf_raw(), rhs)),
                    d,
                )
            } else {
                if a < 0.0 && b > 0.0 {
                    (Self::ENTIRE, d)
                } else {
                    if a == 0.0 {
                        a = 0.0; // [0, b]
                    }
                    if b == 0.0 {
                        b = -0.0; // [a, 0]
                    }
                    (Self::with_infsup_raw(pown_rd(b, rhs), pown_ru(a, rhs)), d)
                }
            }
        } else {
            if rhs % 2 == 0 {
                let abs = self.abs();
                (
                    Self::with_infsup_raw(pown_rd(abs.inf_raw(), rhs), pown_ru(abs.sup_raw(), rhs)),
                    Decoration::Com,
                )
            } else {
                (
                    Self::with_infsup_raw(pown_rd(a, rhs), pown_ru(b, rhs)),
                    Decoration::Com,
                )
            }
        }
    }

    /// Returns the sine of `self`.
    ///
    /// | Domain | Range       |
    /// | ------ | ----------- |
    /// | $\R$   | $\[-1, 1\]$ |
    pub fn sin(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Self::FRAC_PI_2).floor();
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
            const_interval!(-1.0, 1.0)
        }
    }

    impl_mono_inc!(
        /// Returns the hyperbolic sine of `self`.
        ///
        /// | Domain | Range |
        /// | ------ | ----- |
        /// | $\R$   | $\R$  |
        sinh,
        sinh_rd,
        sinh_ru
    );

    /// Returns the tangent of `self`.
    ///
    /// | Domain                            | Range |
    /// | --------------------------------- | ----- |
    /// | $\R ∖ \set{(n + 1/2) π ∣ n ∈ \Z}$ | $\R$  |
    pub fn tan(self) -> Self {
        self.tan_impl().0
    }

    fn tan_impl(self) -> (Self, Decoration) {
        if self.is_empty() {
            return (self, Decoration::Trv);
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let q_nowrap = (self / Self::FRAC_PI_2).floor();
        let qa = q_nowrap.inf_raw();
        let qb = q_nowrap.sup_raw();
        let n = if a == b { 0.0 } else { qb - qa };
        let q = rem_euclid_2(qa);

        let cont =
            qb != f64::INFINITY && b <= (Self::with_infsup_raw(qb, qb) * Self::FRAC_PI_2).inf_raw();
        if q == 0.0 && (n < 1.0 || n == 1.0 && cont) || q == 1.0 && (n < 2.0 || n == 2.0 && cont) {
            // In case of overflow, the decoration must be corrected by the caller.
            (Self::with_infsup_raw(tan_rd(a), tan_ru(b)), Decoration::Com)
        } else {
            (Self::ENTIRE, Decoration::Trv)
        }
    }

    impl_mono_inc!(
        /// Returns the hyperbolic tangent of `self`.
        ///
        /// | Domain | Range     |
        /// | ------ | --------- |
        /// | $\R$   | $(-1, 1)$ |
        tanh,
        tanh_rd,
        tanh_ru
    );
}

macro_rules! impl_dec {
    ($f:ident) => {
        #[doc = concat!("The decorated version of [`Interval::", stringify!($f), "`].")]
        ///
        /// A NaI is returned if `self` is NaI.
        pub fn $f(self) -> Self {
            Self::set_dec(self.x.$f(), self.d)
        }
    };

    ($f:ident, $f_impl:ident) => {
        #[doc = concat!("The decorated version of [`Interval::", stringify!($f), "`].")]
        ///
        /// A NaI is returned if `self` is NaI.
        pub fn $f(self) -> Self {
            let (y, d) = self.x.$f_impl();
            Self::set_dec(y, self.d.min(d))
        }
    };
}

macro_rules! impl_dec2 {
    ($f:ident, $f_impl:ident) => {
        #[doc = concat!("The decorated version of [`Interval::", stringify!($f), "`].")]
        ///
        /// A NaI is returned if `self` or `rhs` is NaI.
        pub fn $f(self, rhs: Self) -> Self {
            let (z, d) = self.x.$f_impl(rhs.x);
            Self::set_dec(z, self.d.min(rhs.d.min(d)))
        }
    };
}

impl DecInterval {
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
    impl_dec!(ln, ln_impl);
    impl_dec!(log10, log10_impl);
    impl_dec!(log2, log2_impl);
    impl_dec2!(pow, pow_impl);

    /// The decorated version of [`Interval::pown`].
    ///
    /// A NaI is returned if `self` is NaI.
    pub fn pown(self, rhs: i32) -> Self {
        let (y, d) = self.x.pown_impl(rhs);
        Self::set_dec(y, self.d.min(d))
    }

    impl_dec!(sin);
    impl_dec!(sinh);
    impl_dec!(tan, tan_impl);
    impl_dec!(tanh);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

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
        assert!(DI::NAI.ln().is_nai());
        assert!(DI::NAI.log10().is_nai());
        assert!(DI::NAI.log2().is_nai());
        assert!(DI::NAI.pow(DI::EMPTY).is_nai());
        assert!(DI::EMPTY.pow(DI::NAI).is_nai());
        assert!(DI::NAI.pown(1).is_nai());
        assert!(DI::NAI.sin().is_nai());
        assert!(DI::NAI.sinh().is_nai());
        assert!(DI::NAI.tan().is_nai());
        assert!(DI::NAI.tanh().is_nai());
    }

    #[test]
    fn tan() {
        // a, b ∈ (-π/2, π/2)
        assert!(interval!(std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf())
            .unwrap()
            .tan()
            .is_common_interval());
        assert!(interval!(-std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf())
            .unwrap()
            .tan()
            .is_common_interval());

        // a, b ∈ (-3π/2, -π/2)
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
