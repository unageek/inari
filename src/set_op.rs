use crate::interval::*;
use std::arch::x86_64::*;

impl Interval {
    /// Returns $\[\min(a, c), \max(b, d)\]$ if both $\self = \[a, b\]$ and $\rhs = \[c, d\]$
    /// are nonempty. If either interval is empty, the other is returned.
    /// If both are empty, $∅$ is returned.
    ///
    /// This is equivalent to $\self ∪ \rhs$ if the intervals are not disjoint,
    ///
    /// Tightness: tightest
    pub fn convex_hull(self, rhs: Self) -> Self {
        if self.is_empty() {
            return rhs;
        }
        if rhs.is_empty() {
            return self;
        }

        // [min(a, c), max(b, d)] = [max(b, d); max(-a, -c)] = simd_max([b; -a], [d; -c])
        Self {
            rep: unsafe { _mm_max_pd(self.rep, rhs.rep) },
        }
    }

    /// Returns $\self ∩ \rhs$. If the result is nonempty, it is equivalent to
    /// $\[\max(a, c), \min(b, d)\]$, where both $\self = \[a, b\]$ and $\rhs = \[c, d\]$
    /// are nonempty.
    ///
    /// Tightness: tightest
    pub fn intersection(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        // [max(a, c), min(b, d)] = [min(b, d); min(-a, -c)] = simd_min([b; -a], [d; -c])
        let i = Self {
            rep: unsafe { _mm_min_pd(self.rep, rhs.rep) },
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
    type DI = DecInterval;
    type I = Interval;

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
