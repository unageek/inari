use crate::{interval::*, simd::*};

// NOTE: `eq` is implemented in interval.rs

impl Interval {
    /// Returns `true` if `rhs` is a member of `self` ($\rhs ∈ \self$).
    ///
    /// If `rhs` is not a real number, `false` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).contains(1.0));
    /// assert!(!Interval::EMPTY.contains(1.0));
    /// assert!(Interval::ENTIRE.contains(1.0));
    /// ```
    ///
    /// $±∞$ and NaN are not real numbers, thus do not belong to any interval:
    ///
    /// ```
    /// use inari::*;
    /// assert!(!Interval::ENTIRE.contains(f64::INFINITY));
    /// assert!(!Interval::ENTIRE.contains(f64::NEG_INFINITY));
    /// assert!(!Interval::ENTIRE.contains(f64::NAN));
    /// ```
    pub fn contains(self, rhs: f64) -> bool {
        rhs.is_finite() & {
            // a ≤ c ∧ c ≤ b
            //   = all([a; c] .≤ [c; b])
            //   = all([-c; c] .≤ [-a; b])
            all(le(neg0(splat(rhs)), self.rep))
        }
    }

    /// Returns `true` if `self` and `rhs` are disjoint ($\self ∩ \rhs = ∅$).
    ///
    /// The formal definition is:
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x ≠ y.
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 3.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 5.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(Interval::EMPTY.disjoint(Interval::EMPTY));
    /// assert!(Interval::EMPTY.disjoint(Interval::ENTIRE));
    /// ```
    pub fn disjoint(self, rhs: Self) -> bool {
        self.either_empty(rhs) | {
            // b < c ∨ d < a
            //   = any([b; d] .< [c; a])
            any(lt(
                shuffle13(self.rep, rhs.rep),
                neg(shuffle02(rhs.rep, self.rep)),
            ))
        }
    }

    /// Returns `true` if `self` is interior to `rhs`.
    ///
    /// The formal definition is:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x < y) ∧ (∀x ∈ \self, ∃y ∈ \rhs : y < x).
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.1, 1.9).interior(const_interval!(1.0, 2.0)));
    /// assert!(!const_interval!(1.1, 2.0).interior(const_interval!(1.0, 2.0)));
    /// assert!(Interval::EMPTY.interior(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.interior(Interval::ENTIRE));
    /// ```
    pub fn interior(self, rhs: Self) -> bool {
        let l = self.is_empty()
            || self.sup_raw() < rhs.sup_raw()
            || all(eq(shuffle13(self.rep, rhs.rep), splat(f64::INFINITY)));
        let r = self.is_empty()
            || rhs.inf_raw() < self.inf_raw()
            || all(eq(shuffle02(self.rep, rhs.rep), splat(f64::INFINITY)));
        l && r
    }

    /// Returns `true` if `self` is nonempty and bounded.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).is_common_interval());
    /// assert!(!const_interval!(1.0, f64::INFINITY).is_common_interval());
    /// assert!(!Interval::EMPTY.is_common_interval());
    /// assert!(!Interval::ENTIRE.is_common_interval());
    /// ```
    pub fn is_common_interval(self) -> bool {
        all(lt(self.rep, splat(f64::INFINITY)))
    }

    /// Returns `true` if `self` is empty ($\self = ∅$).
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(!const_interval!(1.0, 1.0).is_empty());
    /// assert!(Interval::EMPTY.is_empty());
    /// assert!(!Interval::ENTIRE.is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        self.either_empty(self)
    }

    /// Returns `true` if $\self = \[-∞, +∞\]$.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(!const_interval!(1.0, f64::INFINITY).is_entire());
    /// assert!(!Interval::EMPTY.is_entire());
    /// assert!(Interval::ENTIRE.is_entire());
    /// ```
    pub fn is_entire(self) -> bool {
        all(eq(self.rep, splat(f64::INFINITY)))
    }

    /// Returns `true` if `self` consists of a single real number.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 1.0).is_singleton());
    /// assert!(!const_interval!(1.0, 2.0).is_singleton());
    /// assert!(!Interval::EMPTY.is_singleton());
    /// assert!(!Interval::ENTIRE.is_singleton());
    /// ```
    ///
    /// 0.1 is not a member of `f64`:
    ///
    /// ```
    /// use inari::*;
    /// // The singleton set that consists of the nearest `f64` number to 0.1.
    /// assert!(const_interval!(0.1, 0.1).is_singleton());
    /// // The tightest interval that encloses 0.1.
    /// #[cfg(feature = "gmp")]
    /// assert!(!interval!("[0.1, 0.1]").unwrap().is_singleton());
    /// ```
    pub fn is_singleton(self) -> bool {
        self.inf_raw() == self.sup_raw()
    }

    /// Returns `true` if `self` is weakly less than `rhs`.
    ///
    /// The formal definition is:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x ≤ y) ∧ (∀y ∈ \rhs, ∃x ∈ \self : x ≤ y).
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).less(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).less(const_interval!(2.0, 4.0)));
    /// assert!(const_interval!(1.0, 4.0).less(const_interval!(1.0, 4.0)));
    /// assert!(Interval::EMPTY.less(Interval::EMPTY));
    /// assert!(!Interval::EMPTY.less(Interval::ENTIRE));
    /// assert!(!Interval::ENTIRE.less(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.less(Interval::ENTIRE));
    /// ```
    pub fn less(self, rhs: Self) -> bool {
        let l = self.is_empty() || self.sup_raw() <= rhs.sup_raw();
        let r = rhs.is_empty() || self.inf_raw() <= rhs.inf_raw();
        l && r
    }

    /// Returns `true` if `self` is to the left of but may touch `rhs`.
    ///
    /// The formal definition is:
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x ≤ y.
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).precedes(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).precedes(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 3.0).precedes(const_interval!(2.0, 4.0)));
    /// assert!(Interval::EMPTY.precedes(Interval::EMPTY));
    /// assert!(Interval::EMPTY.precedes(Interval::ENTIRE));
    /// assert!(Interval::ENTIRE.precedes(Interval::EMPTY));
    /// assert!(!Interval::ENTIRE.precedes(Interval::ENTIRE));
    /// ```
    pub fn precedes(self, rhs: Self) -> bool {
        self.either_empty(rhs) | (self.sup_raw() <= rhs.inf_raw())
    }

    /// Returns `true` if `self` is strictly less than `rhs`.
    ///
    /// The formal definition is:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x < y) ∧ (∀y ∈ \self, ∃x ∈ \rhs : x < y).
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).strict_less(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).strict_less(const_interval!(2.0, 4.0)));
    /// assert!(!const_interval!(1.0, 4.0).strict_less(const_interval!(2.0, 4.0)));
    /// assert!(const_interval!(1.0, f64::INFINITY).strict_less(const_interval!(2.0, f64::INFINITY)));
    /// assert!(Interval::EMPTY.strict_less(Interval::EMPTY));
    /// assert!(!Interval::EMPTY.strict_less(Interval::ENTIRE));
    /// assert!(!Interval::ENTIRE.strict_less(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.strict_less(Interval::ENTIRE));
    /// ```
    pub fn strict_less(self, rhs: Self) -> bool {
        let l = self.is_empty()
            || self.sup_raw() < rhs.sup_raw()
            || all(eq(shuffle13(self.rep, rhs.rep), splat(f64::INFINITY)));
        let r = rhs.is_empty()
            || self.inf_raw() < rhs.inf_raw()
            || all(eq(shuffle02(self.rep, rhs.rep), splat(f64::INFINITY)));
        l && r
    }

    /// Returns `true` if `self` is strictly to the left of `rhs`.
    ///
    /// The formal definition is:
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x < y.
    /// $$
    pub fn strict_precedes(self, rhs: Self) -> bool {
        self.either_empty(rhs) | (self.sup_raw() < rhs.inf_raw())
    }

    /// Returns `true` if `self` is a subset of `rhs` ($\self ⊆ \rhs$).
    ///
    /// The formal definition is:
    ///
    /// $$
    /// ∀x ∈ \self, ∃y ∈ \rhs : x = y.
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert!(const_interval!(1.0, 2.0).subset(const_interval!(1.0, 2.0)));
    /// assert!(Interval::EMPTY.subset(Interval::EMPTY));
    /// assert!(Interval::EMPTY.subset(Interval::ENTIRE));
    /// assert!(Interval::ENTIRE.subset(Interval::ENTIRE));
    /// ```
    pub fn subset(self, rhs: Self) -> bool {
        self.is_empty() | {
            // c <= a && b <= d
            //   = all([c; b] .<= [a; d])
            //   = all([-a; b] .<= [-c; d])
            all(le(self.rep, rhs.rep))
        }
    }

    pub(crate) fn both_empty(self, rhs: Self) -> bool {
        let x = shuffle02(self.rep, rhs.rep); // [-a; -c]
        all(unord(x, x))
    }

    pub(crate) fn either_empty(self, rhs: Self) -> bool {
        any(unord(self.rep, rhs.rep))
    }
}

macro_rules! impl_dec {
    ($f:ident, 1) => {
        pub fn $f(self) -> bool {
            if self.is_nai() {
                return false;
            }

            self.x.$f()
        }
    };

    ($f:ident, 2) => {
        pub fn $f(self, rhs: Self) -> bool {
            if self.is_nai() || rhs.is_nai() {
                return false;
            }

            self.x.$f(rhs.x)
        }
    };
}

impl DecInterval {
    pub fn contains(self, rhs: f64) -> bool {
        if self.is_nai() {
            return false;
        }

        Interval::contains(self.x, rhs)
    }

    impl_dec!(disjoint, 2);
    impl_dec!(interior, 2);
    impl_dec!(is_common_interval, 1);
    impl_dec!(is_empty, 1);
    impl_dec!(is_entire, 1);

    pub fn is_nai(self) -> bool {
        self.d == Decoration::Ill
    }

    impl_dec!(is_singleton, 1);
    impl_dec!(less, 2);
    impl_dec!(precedes, 2);
    impl_dec!(strict_less, 2);
    impl_dec!(strict_precedes, 2);
    impl_dec!(subset, 2);
}
