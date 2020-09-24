use crate::{classify::*, interval::*, simd::*};
use std::arch::x86_64::*;

// NOTE: `neg`, `add`, `sub`, `mul` and `div` are implemented in arith.rs

impl Interval {
    /// Returns $\self × \rhs + \addend$.
    ///
    /// Tightness: tightest
    // Almost a copy-paste of mul. Additions/modifications are indicated with "// *".
    #[allow(clippy::many_single_char_names)]
    pub fn mul_add(self, rhs: Self, addend: Self) -> Self {
        if addend.is_empty() {
            return Self::EMPTY; // *
        }

        match self.classify2(rhs) {
            C_E_E | C_E_M | C_E_N0 | C_E_N1 | C_E_P0 | C_E_P1 | C_E_Z | C_M_E | C_N0_E | C_N1_E
            | C_P0_E | C_P1_E | C_Z_E => Self::EMPTY,
            C_M_Z | C_N0_Z | C_N1_Z | C_P0_Z | C_P1_Z | C_Z_M | C_Z_N0 | C_Z_N1 | C_Z_P0
            | C_Z_P1 | C_Z_Z => addend, // *
            C_M_M => {
                let x = dup_lo(self.rep);
                let y = swap(rhs.rep);
                let xy = mul_add_ru(x, y, addend.rep); // *
                let z = dup_hi(self.rep);
                let w = rhs.rep;
                let zw = mul_add_ru(z, w, addend.rep); // *
                let r = unsafe { _mm_max_pd(xy, zw) };
                Self { rep: r }
            }
            C_M_N0 | C_M_N1 => {
                let x = swap(self.rep);
                let y = dup_lo(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_M_P0 | C_M_P1 => {
                let x = self.rep;
                let y = dup_hi(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_N0_M | C_N1_M => {
                let x = dup_lo(self.rep);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_N0_N0 | C_N0_N1 | C_N1_N0 | C_N1_N1 => {
                let x0 = swap(self.rep);
                let x = negate_lo(x0);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_N0_P0 | C_N0_P1 | C_N1_P0 | C_N1_P1 => {
                let x = self.rep;
                let y0 = negate_lo(rhs.rep);
                let y = swap(y0);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_P0_M | C_P1_M => {
                let x = dup_hi(self.rep);
                let y = rhs.rep;
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_P0_N0 | C_P0_N1 | C_P1_N0 | C_P1_N1 => {
                let x0 = negate_lo(self.rep);
                let x = swap(x0);
                let y = rhs.rep;
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            C_P0_P0 | C_P0_P1 | C_P1_P0 | C_P1_P1 => {
                let x = self.rep;
                let y = negate_lo(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            _ => unreachable!(),
        }
    }

    /// Returns the multiplicative inverse of `self`.
    ///
    /// Tightness: tightest
    pub fn recip(self) -> Self {
        match self.classify() {
            C_E | C_Z => Self::EMPTY,
            C_M => Self::ENTIRE,
            C_N0 => {
                // 1 / N0 => [-inf, 1/a] = [-1/-a; inf]
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { set_lo_inf(div_ru(_mm_set1_pd(-1.0), x)) };
                Self { rep: r }
            }
            C_N1 => {
                // 1 / N1 => [1/b, 1/a] = [1/a; -1/b] = [-1; -1] / [-a; b]
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { div_ru(_mm_set1_pd(-1.0), x) };
                Self { rep: r }
            }
            C_P0 => {
                // 1 / P0 => [1/b, inf] = [inf; -1/b]
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { set_hi_inf(div_ru(_mm_set1_pd(-1.0), x)) };
                Self { rep: r }
            }
            C_P1 => {
                // 1 / P1 => [1/b, 1/a] = [1/a; -1/b] = [-1; -1] / [-a; b]
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { div_ru(_mm_set1_pd(-1.0), x) };
                Self { rep: r }
            }
            _ => unreachable!(),
        }
    }

    /// Returns the square of `self`.
    ///
    /// Tightness: tightest
    pub fn sqr(self) -> Self {
        match self.classify() {
            C_E => Self::EMPTY,
            C_Z => Self::zero(),
            C_M => {
                // [0, max(a^2, b^2)] = [max(a^2, b^2); 0]
                let r0 = self.rep; // [b; -a]
                let r1 = mul_ru(r0, r0); // [b^2; a^2]
                let r2 = unsafe { _mm_max_sd(r1, swap(r1)) }; // [_; max(a^2, b^2)]
                let r = unsafe { _mm_unpacklo_pd(_mm_setzero_pd(), r2) };
                Self { rep: r }
            }
            C_N0 | C_N1 => {
                // [b^2, a^2] = [a^2; -b^2] = [a; -b] * [a; b]
                let x = swap(self.rep); // [a; -b]
                let y = negate_lo(x); // [a; b]
                Self { rep: mul_ru(x, y) }
            }
            C_P0 | C_P1 => {
                // [a^2, b^2] = [b^2; -a^2] = [b; -a] * [b; a]
                let x = self.rep; // [b; -a]
                let y = negate_lo(x); // [b; a]
                Self { rep: mul_ru(x, y) }
            }
            _ => unreachable!(),
        }
    }

    /// Returns the principal square root of `self`.
    ///
    /// Tightness: tightest
    pub fn sqrt(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();

        if b < 0.0 {
            Self::EMPTY
        } else if a <= 0.0 {
            Self::with_infsup_raw(0.0, sqrt1_ru(b))
        } else {
            Self::with_infsup_raw(sqrt1_rd(a), sqrt1_ru(b))
        }
    }
}

impl DecoratedInterval {
    pub fn mul_add(self, rhs: Self, addend: Self) -> Self {
        if self.is_nai() || rhs.is_nai() || addend.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(
            self.x.mul_add(rhs.x, addend.x),
            self.d.min(rhs.d.min(addend.d)),
        )
    }

    pub fn recip(self) -> Self {
        if self.is_nai() {
            return self;
        }

        let d = if self.x.contains(0.0) {
            Decoration::Trv
        } else {
            self.d
        };
        Self::set_dec(self.x.recip(), d)
    }

    pub fn sqr(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.sqr(), self.d)
    }

    pub fn sqrt(self) -> Self {
        if self.is_nai() {
            return self;
        }

        let d = if self.x.inf_raw() < 0.0 {
            Decoration::Trv
        } else {
            self.d
        };
        Self::set_dec(self.x.sqrt(), d)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn empty() {
        assert!((I::EMPTY.mul_add(I::PI, I::PI)).is_empty());
        assert!((I::PI.mul_add(I::EMPTY, I::PI)).is_empty());
        assert!((I::PI.mul_add(I::PI, I::EMPTY)).is_empty());

        assert!(I::EMPTY.recip().is_empty());
        assert!(I::EMPTY.sqrt().is_empty());
        assert!(I::EMPTY.sqr().is_empty());

        assert!((DI::EMPTY.mul_add(DI::PI, DI::PI)).is_empty());
        assert!((DI::PI.mul_add(DI::EMPTY, DI::PI)).is_empty());
        assert!((DI::PI.mul_add(DI::PI, DI::EMPTY)).is_empty());

        assert!(DI::EMPTY.recip().is_empty());
        assert!(DI::EMPTY.sqrt().is_empty());
        assert!(DI::EMPTY.sqr().is_empty());
    }

    #[test]
    fn nai() {
        assert!((DI::NAI.mul_add(DI::PI, DI::PI)).is_nai());
        assert!((DI::PI.mul_add(DI::NAI, DI::PI)).is_nai());
        assert!((DI::PI.mul_add(DI::PI, DI::NAI)).is_nai());

        assert!(DI::NAI.recip().is_nai());
        assert!(DI::NAI.sqrt().is_nai());
        assert!(DI::NAI.sqr().is_nai());
    }
}
