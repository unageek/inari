use crate::classify::*;
use crate::interval::*;
use crate::simd::*;
use core::arch::x86_64::*;

impl Interval {
    pub fn abs(self) -> Self {
        match self.classify() {
            C_E | C_P0 | C_P1 | C_Z => self,
            C_M => {
                // [0, max(-a, b)] = [max(-a, b); 0]
                let r0 = self.rep; // [b; -a]
                let r1 = unsafe { _mm_max_sd(r0, swap(r0)) }; // [_; max(-a, b)]
                let r = unsafe { _mm_unpacklo_pd(_mm_setzero_pd(), r1) };
                Self { rep: r }
            }
            C_N0 | C_N1 => {
                // [-b, -a] = [-a; b]
                Self {
                    rep: swap(self.rep),
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn max(self, rhs: Self) -> Self {
        // IEEE 754's min/max do not propagate nan.
        // https://www.agner.org/optimize/blog/read.php?i=1012
        if self.is_empty() || rhs.is_empty() {
            return Self::empty();
        }

        unsafe {
            let max = _mm_max_pd(self.rep, rhs.rep); // [max(b, d); max(-a, -c)]
            let min = _mm_min_pd(self.rep, rhs.rep); // [min(b, d); min(-a, -c)]
            let r = _mm_move_sd(max, min); // [min(-a, -c); max(b, d)]
            Self { rep: r }
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::empty();
        }

        unsafe {
            let min = _mm_min_pd(self.rep, rhs.rep); // [min(b, d); min(-a, -c)]
            let max = _mm_max_pd(self.rep, rhs.rep); // [max(b, d); max(-a, -c)]
            let r = _mm_move_sd(min, max); // [max(-a, -c); min(b, d)]
            Self { rep: r }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type I = Interval;

    #[test]
    fn empty() {
        assert!(I::empty().abs().is_empty());

        assert!(I::empty().max(I::PI).is_empty());
        assert!(I::PI.max(I::empty()).is_empty());

        assert!(I::empty().min(I::PI).is_empty());
        assert!(I::PI.min(I::empty()).is_empty());
    }
}
