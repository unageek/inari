use crate::interval::*;
use core::arch::x86_64::*;

impl Interval {
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

            DecoratedInterval {
                x: self.x.$f(rhs.x),
                d: Decoration::Trv,
            }
        }
    };
}

impl DecoratedInterval {
    impl_dec!(convex_hull);
    impl_dec!(intersection);
}

#[cfg(test)]
mod tests {
    use super::*;
    type DI = DecoratedInterval;
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
