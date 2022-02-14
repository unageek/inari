use crate::{interval::*, simd::*};

impl Interval {
    /// Returns the lower bound of `self`.
    ///
    /// The lower bound of an interval $𝒙$ is:
    ///
    /// $$
    /// \inf(𝒙) = \begin{cases}
    ///   +∞ & \if 𝒙 = ∅, \\\\
    ///   a  & \if 𝒙 = \[a, b\].
    ///  \end{cases}
    /// $$
    ///
    /// The exact value is returned.  When $a = 0$, `-0.0` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).inf(), -2.0);
    /// assert_eq!(Interval::EMPTY.inf(), f64::INFINITY);
    /// assert_eq!(Interval::ENTIRE.inf(), f64::NEG_INFINITY);
    /// ```
    ///
    /// See also: [`Interval::sup`].
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

    /// Returns the magnitude of `self` if it is nonempty; otherwise, a NaN.
    ///
    /// The magnitude of a nonempty interval $𝒙 = \[a, b\]$ is:
    ///
    /// $$
    /// \begin{align*}
    ///  \mag(𝒙) &= \sup \set{|x| ∣ x ∈ 𝒙} \\\\
    ///   &= \max \set{|a|, |b|}.
    /// \end{align*}
    /// $$
    ///
    /// The exact value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).mag(), 3.0);
    /// assert!(Interval::EMPTY.mag().is_nan());
    /// assert_eq!(Interval::ENTIRE.mag(), f64::INFINITY);
    /// ```
    ///
    /// See also: [`Interval::mig`].
    pub fn mag(self) -> f64 {
        let abs = abs(self.rep);
        extract0(max(abs, swap(abs)))
    }

    /// Returns the midpoint of `self` if it is nonempty; otherwise, a NaN.
    ///
    /// The midpoint of a nonempty interval $𝒙 = \[a, b\]$ is:
    ///
    /// $$
    /// \mid(𝒙) = \frac{a + b}{2}.
    /// $$
    ///
    /// As an approximation in [`f64`], it returns:
    ///
    /// - `0.0`, if $\self = \[-∞, +∞\]$;
    /// - [`f64::MIN`], if $\self = \[-∞, b\]$, where $b ∈ \R$;
    /// - [`f64::MAX`], if $\self = \[a, +∞\]$, where $a ∈ \R$;
    /// - otherwise, the closest [`f64`] number to $\mid(\self)$, away from zero in case of ties.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).mid(), 0.5);
    /// assert_eq!(const_interval!(f64::NEG_INFINITY, 3.0).mid(), f64::MIN);
    /// assert_eq!(const_interval!(-2.0, f64::INFINITY).mid(), f64::MAX);
    /// assert!(Interval::EMPTY.mid().is_nan());
    /// assert_eq!(Interval::ENTIRE.mid(), 0.0);
    /// ```
    ///
    /// See also: [`Interval::rad`].
    // See Table VII in https://doi.org/10.1145/2493882 for the implementation.
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

    /// Returns the mignitude of `self` if it is nonempty; otherwise, a NaN.
    ///
    /// The mignitude of a nonempty interval $𝒙 = \[a, b\]$ is:
    ///
    /// $$
    /// \begin{align*}
    ///  \mig(𝒙) &= \inf \set{|x| ∣ x ∈ 𝒙} \\\\
    ///   &= \begin{cases}
    ///     \min \set{|a|, |b|} & \if \sgn(a) = \sgn(b), \\\\
    ///     0                   & \otherwise.
    ///    \end{cases}
    /// \end{align*}
    /// $$
    ///
    /// The exact value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).mig(), 0.0);
    /// assert_eq!(const_interval!(2.0, 3.0).mig(), 2.0);
    /// assert!(Interval::EMPTY.mig().is_nan());
    /// assert_eq!(Interval::ENTIRE.mig(), 0.0);
    /// ```
    ///
    /// See also: [`Interval::mag`].
    pub fn mig(self) -> f64 {
        let zero = splat(0.0);
        let contains_zero = all(ge(self.rep, zero));
        if contains_zero {
            return 0.0;
        }

        let abs = abs(self.rep);
        extract0(min(abs, swap(abs)))
    }

    /// Returns the radius of `self` if it is nonempty; otherwise, a NaN.
    ///
    /// The radius of a nonempty interval $𝒙 = \[a, b\]$ is:
    ///
    /// $$
    /// \rad(𝒙) = \frac{b - a}{2}.
    /// $$
    ///
    /// As an approximation in [`f64`], it returns the least [`f64`] number `r` that satisfies
    /// $\self ⊆ \[𝚖 - 𝚛, 𝚖 + 𝚛\]$, where `m` is the midpoint returned by [`Self::mid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).rad(), 2.5);
    /// assert!(Interval::EMPTY.rad().is_nan());
    /// assert_eq!(Interval::ENTIRE.rad(), f64::INFINITY);
    /// ```
    ///
    /// See also: [`Interval::mid`].
    pub fn rad(self) -> f64 {
        let m = self.mid();
        f64::max(sub1_ru(m, self.inf_raw()), sub1_ru(self.sup_raw(), m))
    }

    /// Returns the upper bound of `self`.
    ///
    /// The upper bound of an interval $𝒙$ is:
    ///
    /// $$
    /// \sup(𝒙) = \begin{cases}
    ///   -∞ & \if 𝒙 = ∅, \\\\
    ///   b  & \if 𝒙 = \[a, b\].
    ///  \end{cases}
    /// $$
    ///
    /// The exact value is returned.  When $b = 0$, `0.0` (the
    /// positive zero) is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).sup(), 3.0);
    /// assert_eq!(Interval::EMPTY.sup(), f64::NEG_INFINITY);
    /// assert_eq!(Interval::ENTIRE.sup(), f64::INFINITY);
    /// ```
    ///
    /// See also: [`Interval::inf`].
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

    /// Returns the width of `self` if it is nonempty; otherwise, a NaN.
    ///
    /// The width of a nonempty interval $𝒙 = \[a, b\]$ is:
    ///
    /// $$
    /// \wid(𝒙) = b - a.
    /// $$
    ///
    /// As an approximation in [`f64`], it returns the closest [`f64`] number toward $+∞$.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-2.0, 3.0).wid(), 5.0);
    /// assert_eq!(const_interval!(-1.0, f64::MAX).wid(), f64::INFINITY);
    /// assert!(Interval::EMPTY.wid().is_nan());
    /// assert_eq!(Interval::ENTIRE.wid(), f64::INFINITY);
    /// ```
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
        #[doc = concat!("Applies [`Interval::", stringify!($f), "`] to the interval part of `self`")]
        /// and returns the result.
        ///
        /// A NaN is returned if `self` is NaI.
        pub fn $f(self) -> f64 {
            if self.is_nai() {
                return f64::NAN;
            }

            self.x.$f()
        }
    };
}

impl DecInterval {
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
