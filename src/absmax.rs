use crate::{classify::*, interval::*, simd::*};
use std::arch::x86_64::*;

impl Interval {
    /// Returns the absolute value of `self`.
    ///
    /// Tightness: tightest
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

    /// Returns $\[\max(a, c), \max(b, d)\]$ if both $\self = \[a, b\]$ and $\rhs = \[c, d\]$
    /// are nonempty; otherwise, $∅$.
    ///
    /// Tightness: tightest
    pub fn max(self, rhs: Self) -> Self {
        // IEEE 754's min/max do not propagate nan.
        // https://www.agner.org/optimize/blog/read.php?i=1012
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        unsafe {
            let max = _mm_max_pd(self.rep, rhs.rep); // [max(b, d); max(-a, -c)]
            let min = _mm_min_pd(self.rep, rhs.rep); // [min(b, d); min(-a, -c)]
            let r = _mm_move_sd(max, min); // [max(b, d); min(-a, -c)]
            Self { rep: r }
        }
    }

    /// Returns $\[\min(a, c), \min(b, d)\]$ if both $\self = \[a, b\]$ and $\rhs = \[c, d\]$
    /// are nonempty; otherwise $∅$.
    ///
    /// Tightness: tightest
    pub fn min(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        unsafe {
            let min = _mm_min_pd(self.rep, rhs.rep); // [min(b, d); min(-a, -c)]
            let max = _mm_max_pd(self.rep, rhs.rep); // [max(b, d); max(-a, -c)]
            let r = _mm_move_sd(min, max); // [min(b, d); max(-a, -c)]
            Self { rep: r }
        }
    }
}

impl DecoratedInterval {
    pub fn abs(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.abs(), self.d)
    }

    pub fn max(self, rhs: Self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.max(rhs.x), self.d.min(rhs.d))
    }

    pub fn min(self, rhs: Self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.min(rhs.x), self.d.min(rhs.d))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn empty() {
        assert!(I::EMPTY.abs().is_empty());

        assert!(I::EMPTY.max(I::PI).is_empty());
        assert!(I::PI.max(I::EMPTY).is_empty());

        assert!(I::EMPTY.min(I::PI).is_empty());
        assert!(I::PI.min(I::EMPTY).is_empty());

        assert!(DI::EMPTY.abs().is_empty());

        assert!(DI::EMPTY.max(DI::PI).is_empty());
        assert!(DI::PI.max(DI::EMPTY).is_empty());

        assert!(DI::EMPTY.min(DI::PI).is_empty());
        assert!(DI::PI.min(DI::EMPTY).is_empty());
    }

    #[test]
    fn nai() {
        assert!(DI::NAI.abs().is_nai());

        assert!(DI::NAI.max(DI::PI).is_nai());
        assert!(DI::PI.max(DI::NAI).is_nai());

        assert!(DI::NAI.min(DI::PI).is_nai());
        assert!(DI::PI.min(DI::NAI).is_nai());
    }
}
