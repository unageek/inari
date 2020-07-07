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
            return Self::empty();
        }

        // [max(a, c), min(b, d)] = [min(b, d); min(-a, -c)] = simd_min([b; -a], [d; -c])
        let i = Self {
            rep: unsafe { _mm_min_pd(self.rep, rhs.rep) },
        };

        if i.inf_raw() > i.sup_raw() {
            Self::empty()
        } else {
            i
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type I = Interval;

    #[test]
    fn empty() {
        assert_eq!(I::empty().convex_hull(I::PI), I::PI);
        assert_eq!(I::PI.convex_hull(I::empty()), I::PI);

        assert!(I::empty().intersection(I::PI).is_empty());
        assert!(I::PI.intersection(I::empty()).is_empty());
    }
}
