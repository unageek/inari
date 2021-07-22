use crate::{interval::*, simd::*};

impl Interval {
    /// Rounds `self` to the closest integer toward $+∞$.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).ceil(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).ceil(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).ceil(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).ceil(), const_interval!(-1.0, 0.0));
    /// assert_eq!(Interval::EMPTY.ceil(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.ceil(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::floor`], [`Interval::trunc`].
    pub fn ceil(self) -> Self {
        // _mm_ceil_pd/_mm_floor_pd are slow, better to avoid shuffling them.
        // ceil([a, b]) = [-ceil(a); ceil(b)]
        let x = neg0(self.rep); // [a; b]
        let r = ceil(x); // [ceil(a); ceil(b)]
        Self { rep: neg0(r) }
    }

    /// Rounds `self` to the closest integer toward $-∞$.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).floor(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.8, 1.8).floor(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).floor(), const_interval!(-2.0, -1.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).floor(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.floor(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.floor(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::ceil`], [`Interval::trunc`].
    pub fn floor(self) -> Self {
        // floor([a, b]) = [-floor(a); floor(b)]
        let x = neg0(self.rep); // [a; b]
        let r = floor(x); // [floor(a); floor(b)]
        Self { rep: neg0(r) }
    }

    /// Rounds `self` to the closest integer, away from zero in case of ties.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).round(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.5, 1.5).round(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).round(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).round(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.5, -0.5).round(), const_interval!(-2.0, -1.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).round(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.round(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.round(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::round_ties_to_even`].
    pub fn round(self) -> Self {
        Self {
            rep: round(self.rep),
        }
    }

    /// Rounds `self` to the closest integer, the even number in case of ties.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).round_ties_to_even(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.5, 1.5).round_ties_to_even(), const_interval!(0.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).round_ties_to_even(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).round_ties_to_even(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.5, -0.5).round_ties_to_even(), const_interval!(-2.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).round_ties_to_even(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.round_ties_to_even(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.round_ties_to_even(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::round`].
    pub fn round_ties_to_even(self) -> Self {
        Self {
            rep: round_ties_to_even(self.rep),
        }
    }

    /// Returns the sign of `self`.
    ///
    /// Note the difference in definition between [`f64::signum`] and this function;
    /// `+0.0_f64.signum()` and `-0.0_f64.signum()` return `+1.0` and `-1.0`, respectively,
    /// while the sign of zero is just zero,
    ///
    /// | Domain | Range            |
    /// | ------ | ---------------- |
    /// | $\R$   | $\set{-1, 0, 1}$ |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-10.0, -0.1).sign(), const_interval!(-1.0, -1.0));
    /// assert_eq!(const_interval!(0.0, 0.0).sign(), const_interval!(0.0, 0.0));
    /// assert_eq!(const_interval!(0.1, 10.0).sign(), const_interval!(1.0, 1.0));
    /// assert_eq!(Interval::EMPTY.sign(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.sign(), const_interval!(-1.0, 1.0));
    /// ```
    pub fn sign(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let zero = splat(0.0);
        let gt_zero_mask = gt(self.rep, zero);
        let lt_zero_mask = lt(self.rep, zero);
        // [-(a ≤ 0), b ≥ 0] = [-a ≥ 0; b ≥ 0]
        let one_or_zero = and(splat(1.0), gt_zero_mask);
        // [a ≥ 0, -(b ≤ 0)] = [-(-a ≤ 0); -(b ≤ 0)]
        let m_one_or_zero = and(splat(-1.0), lt_zero_mask);
        // Gives the same result as addition, but faster.
        let r = or(one_or_zero, m_one_or_zero);
        Self { rep: r }
    }

    /// Rounds `self` to the closest integer toward zero.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// Tightness: tightest
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).trunc(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.8, 1.8).trunc(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).trunc(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).trunc(), const_interval!(-1.0, 0.0));
    /// assert_eq!(Interval::EMPTY.trunc(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.trunc(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::ceil`], [`Interval::floor`].
    pub fn trunc(self) -> Self {
        Self {
            rep: trunc(self.rep),
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
        #[doc = concat!("See [`Interval::", stringify!($f), "`].")]
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
impl DecInterval {
    // Discontinuities: ℤ.
    impl_dec!(ceil, x, y, x.sup_raw() == y.sup_raw()); // No need to check inf.
    impl_dec!(floor, x, y, x.inf_raw() == y.inf_raw()); // No need to check sup.

    // Discontinuities: {x + 0.5 ∣ x ∈ ℤ}.
    impl_dec!(round, x, y, {
        let abs_a = x.inf_raw().abs();
        let abs_b = x.sup_raw().abs();
        (abs_a - abs_a.trunc() == 0.5) || (abs_b - abs_b.trunc() == 0.5)
    });
    impl_dec!(round_ties_to_even, x, y, {
        let abs_a = x.inf_raw().abs();
        let abs_b = x.sup_raw().abs();
        (abs_a - abs_a.trunc() == 0.5) || (abs_b - abs_b.trunc() == 0.5)
    });

    // Discontinuities: {0}.
    impl_dec!(sign, x, y, x.inf_raw() == 0.0); // No need to check sup.

    // Discontinuities: ℤ ∖ {0}.
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
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    #[test]
    fn empty() {
        assert!(I::EMPTY.ceil().is_empty());
        assert!(I::EMPTY.floor().is_empty());
        assert!(I::EMPTY.round().is_empty());
        assert!(I::EMPTY.round_ties_to_even().is_empty());
        assert!(I::EMPTY.sign().is_empty());
        assert!(I::EMPTY.trunc().is_empty());

        assert!(DI::EMPTY.ceil().is_empty());
        assert!(DI::EMPTY.floor().is_empty());
        assert!(DI::EMPTY.round().is_empty());
        assert!(DI::EMPTY.round_ties_to_even().is_empty());
        assert!(DI::EMPTY.sign().is_empty());
        assert!(DI::EMPTY.trunc().is_empty());
    }

    #[test]
    fn nai() {
        assert!(DI::NAI.ceil().is_nai());
        assert!(DI::NAI.floor().is_nai());
        assert!(DI::NAI.round().is_nai());
        assert!(DI::NAI.round_ties_to_even().is_nai());
        assert!(DI::NAI.sign().is_nai());
        assert!(DI::NAI.trunc().is_nai());
    }
}
