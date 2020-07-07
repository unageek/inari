use crate::classify::*;
use crate::interval::*;
use crate::simd::*;
use core::arch::x86_64::*;

// NOTE: `add`, `sub`, `mul` and `div` are implemented in arith.rs

impl Interval {
    // Almost a copy-paste of mul. Additions/modifications are indicated with "// *".
    #[allow(clippy::many_single_char_names)]
    pub fn mul_add(self, rhs: Self, addend: Self) -> Self {
        if addend.is_empty() {
            return Self::empty(); // *
        }

        match (self.classify() << 4) | rhs.classify() {
            C_E_E | C_E_M | C_E_N0 | C_E_N1 | C_E_P0 | C_E_P1 | C_E_Z | C_M_E | C_N0_E | C_N1_E
            | C_P0_E | C_P1_E | C_Z_E => Self::empty(),
            C_M_Z | C_N0_Z | C_N1_Z | C_P0_Z | C_P1_Z | C_Z_M | C_Z_N0 | C_Z_N1 | C_Z_P0
            | C_Z_P1 | C_Z_Z => addend, // *
            C_M_M => {
                let _r = RoundUpContext::new();
                let x = dup_lo(self.rep);
                let y = swap(rhs.rep);
                let xy = mul_add(x, y, addend.rep); // *
                let z = dup_hi(self.rep);
                let w = rhs.rep;
                let zw = mul_add(z, w, addend.rep); // *
                let r = unsafe { _mm_max_pd(xy, zw) };
                Self { rep: r }
            }
            C_M_N0 | C_M_N1 => {
                let _r = RoundUpContext::new();
                let x = swap(self.rep);
                let y = dup_lo(rhs.rep);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_M_P0 | C_M_P1 => {
                let _r = RoundUpContext::new();
                let x = self.rep;
                let y = dup_hi(rhs.rep);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_N0_M | C_N1_M => {
                let _r = RoundUpContext::new();
                let x = dup_lo(self.rep);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_N0_N0 | C_N0_N1 | C_N1_N0 | C_N1_N1 => {
                let _r = RoundUpContext::new();
                let x0 = swap(self.rep);
                let x = negate_lo(x0);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_N0_P0 | C_N0_P1 | C_N1_P0 | C_N1_P1 => {
                let _r = RoundUpContext::new();
                let x = self.rep;
                let y0 = negate_lo(rhs.rep);
                let y = swap(y0);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_P0_M | C_P1_M => {
                let _r = RoundUpContext::new();
                let x = dup_hi(self.rep);
                let y = rhs.rep;
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_P0_N0 | C_P0_N1 | C_P1_N0 | C_P1_N1 => {
                let _r = RoundUpContext::new();
                let x0 = negate_lo(self.rep);
                let x = swap(x0);
                let y = rhs.rep;
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            C_P0_P0 | C_P0_P1 | C_P1_P0 | C_P1_P1 => {
                let _r = RoundUpContext::new();
                let x = self.rep;
                let y = negate_lo(rhs.rep);
                Self {
                    rep: mul_add(x, y, addend.rep), // *
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn recip(self) -> Self {
        match self.classify() {
            C_E | C_Z => Self::empty(),
            C_M => Self::entire(),
            C_N0 => {
                // 1 / N0 => [-inf, 1/a] = [-1/-a; inf]
                let _r = RoundUpContext::new();
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { set_lo_inf(div(_mm_set1_pd(-1.0), x)) };
                Self { rep: r }
            }
            C_N1 => {
                // 1 / N1 => [1/b, 1/a] = [1/a; -1/b] = [-1; -1] / [-a; b]
                let _r = RoundUpContext::new();
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { div(_mm_set1_pd(-1.0), x) };
                Self { rep: r }
            }
            C_P0 => {
                // 1 / P0 => [1/b, inf] = [inf; -1/b]
                let _r = RoundUpContext::new();
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { set_hi_inf(div(_mm_set1_pd(-1.0), x)) };
                Self { rep: r }
            }
            C_P1 => {
                // 1 / P1 => [1/b, 1/a] = [1/a; -1/b] = [-1; -1] / [-a; b]
                let _r = RoundUpContext::new();
                let x = swap(self.rep); // [-a; b]
                let r = unsafe { div(_mm_set1_pd(-1.0), x) };
                Self { rep: r }
            }
            _ => unreachable!(),
        }
    }

    pub fn sqr(self) -> Self {
        match self.classify() {
            C_E => Self::empty(),
            C_Z => Self::zero(),
            C_M => {
                // [0, max(a^2, b^2)] = [max(a^2, b^2); 0]
                let _r = RoundUpContext::new();
                let r0 = self.rep; // [b; -a]
                let r1 = mul(r0, r0); // [b^2; a^2]
                let r2 = unsafe { _mm_max_sd(r1, swap(r1)) }; // [_; max(a^2, b^2)]
                let r = unsafe { _mm_unpacklo_pd(_mm_setzero_pd(), r2) };
                Self { rep: r }
            }
            C_N0 | C_N1 => {
                // [b^2, a^2] = [a^2; -b^2] = [a; -b] * [a; b]
                let _r = RoundUpContext::new();
                let x = swap(self.rep); // [a; -b]
                let y = negate_lo(x); // [a; b]
                Self { rep: mul(x, y) }
            }
            C_P0 | C_P1 => {
                // [a^2, b^2] = [b^2; -a^2] = [b; -a] * [b; a]
                let _r = RoundUpContext::new();
                let x = self.rep; // [b; -a]
                let y = negate_lo(x); // [b; a]
                Self { rep: mul(x, y) }
            }
            _ => unreachable!(),
        }
    }

    pub fn sqrt(self) -> Self {
        if self.is_empty() {
            return Self::empty();
        }

        let a = self.inf_raw();
        let b = self.sup_raw();

        if b < 0.0 {
            Self::empty()
        } else if a <= 0.0 {
            let _r = RoundUpContext::new();
            Self::with_infsup_raw(0.0, secure(f64::sqrt(secure(b))))
        } else {
            let _r = RoundUpContext::new();
            let rb = secure(f64::sqrt(secure(b)));
            round_down();
            let ra = secure(f64::sqrt(secure(a)));
            Self::with_infsup_raw(ra, rb)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type I = Interval;

    #[test]
    fn empty() {
        assert!((I::empty().mul_add(I::PI, I::PI)).is_empty());
        assert!((I::PI.mul_add(I::empty(), I::PI)).is_empty());
        assert!((I::PI.mul_add(I::PI, I::empty())).is_empty());

        assert!(I::empty().recip().is_empty());
        assert!(I::empty().sqrt().is_empty());
        assert!(I::empty().sqr().is_empty());
    }
}
