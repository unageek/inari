use crate::{interval::*, simd::*};

impl Interval {
    /// Returns $\hull(\self ∪ \rhs)$, the tightest interval that contains both `self` and `rhs` as its subsets.
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$                      |
    /// | :----------------: | :--------: | :------------------------------------: |
    /// | $\self = ∅$        | $∅$        | $\[c, d\]$                             |
    /// | $\self = \[a, b\]$ | $\[a, b\]$ | $\[\min \set{a, c}, \max \set{b, d}\]$ |
    #[must_use]
    pub fn convex_hull(self, rhs: Self) -> Self {
        if self.is_empty() {
            return rhs;
        }
        if rhs.is_empty() {
            return self;
        }

        // [min(a, c), max(b, d)]
        //   = [-min(a, c); max(b, d)] = [max(-a, -c); max(b, d)] = .max([-a; b], [-c; d])
        Self {
            rep: max(self.rep, rhs.rep),
        }
    }

    /// Returns $\self ∩ \rhs$, the intersection of `self` and `rhs`.
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$                      |
    /// | :----------------: | :--------: | :------------------------------------: |
    /// | $\self = ∅$        | $∅$        | $∅$                                    |
    /// | $\self = \[a, b\]$ | $∅$        | $\[\max \set{a, c}, \min \set{b, d}\]$ |
    #[must_use]
    pub fn intersection(self, rhs: Self) -> Self {
        if self.either_empty(rhs) {
            return Self::EMPTY;
        }

        // [max(a, c), min(b, d)]
        //   = [-max(a, c); min(b, d)] = [min(-a, -c); min(b, d)] = .min([-a; b], [-c; d])
        let i = Self {
            rep: min(self.rep, rhs.rep),
        };

        if i.inf_raw() > i.sup_raw() {
            Self::EMPTY
        } else {
            i
        }
    }
}

macro_rules! impl_dec {
    ($f:ident) => {
        #[doc = concat!("Applies [`Interval::", stringify!($f), "`] to the interval parts of `self` and `rhs`")]
        /// and returns the result decorated with [`Decoration::Trv`].
        ///
        /// A NaI is returned if `self` or `rhs` is NaI.
        #[must_use]
        pub fn $f(self, rhs: Self) -> Self {
            if self.is_nai() || rhs.is_nai() {
                return Self::NAI;
            }

            Self::new_unchecked(self.x.$f(rhs.x), Decoration::Trv)
        }
    };
}

impl DecInterval {
    impl_dec!(convex_hull);
    impl_dec!(intersection);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    #[test]
    fn empty() {
        assert_eq!(I::EMPTY.convex_hull(I::PI), I::PI);
        assert_eq!(I::PI.convex_hull(I::EMPTY), I::PI);

        assert!(I::EMPTY.intersection(I::PI).is_empty());
        assert!(I::PI.intersection(I::EMPTY).is_empty());

        assert_eq!(DI::EMPTY.convex_hull(DI::PI), DI::PI);
        assert_eq!(DI::PI.convex_hull(DI::EMPTY), DI::PI);

        assert!(DI::EMPTY.intersection(DI::PI).is_empty());
        assert!(DI::PI.intersection(DI::EMPTY).is_empty());
    }

    #[test]
    fn nai() {
        assert!(DI::NAI.convex_hull(DI::PI).is_nai());
        assert!(DI::PI.convex_hull(DI::NAI).is_nai());

        assert!(DI::NAI.intersection(DI::PI).is_nai());
        assert!(DI::PI.intersection(DI::NAI).is_nai());
    }
}
