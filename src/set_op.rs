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
    use crate::interval;
    type I = Interval;

    #[test]
    fn convex_hull() {
        assert_eq!(
            I::empty().convex_hull(interval!(1.0, 2.0).unwrap()),
            interval!(1.0, 2.0).unwrap()
        );

        assert_eq!(
            interval!(1.0, 2.0).unwrap().convex_hull(I::empty()),
            interval!(1.0, 2.0).unwrap()
        );
    }

    #[test]
    fn intersection() {
        assert_eq!(
            I::empty().intersection(interval!(1.0, 2.0).unwrap()),
            I::empty()
        );

        assert_eq!(
            interval!(1.0, 2.0).unwrap().intersection(I::empty()),
            I::empty()
        );
    }
}
