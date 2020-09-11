use crate::interval::Interval;
use std::arch::x86_64::*;

impl Interval {
    /// For a given interval [a, b], returns the following bit pattern as an integer:
    /// `[0, ..., 0, b ≤ 0, -a ≤ 0, b ≥ 0, -a ≥ 0]`.
    /// If `self` is empty, returns `0`.
    pub(crate) fn classify(self) -> u32 {
        let zero = unsafe { _mm_setzero_pd() };
        let ge_zero = unsafe { _mm_movemask_pd(_mm_cmpge_pd(self.rep, zero)) };
        let le_zero = unsafe { _mm_movemask_pd(_mm_cmple_pd(self.rep, zero)) };
        ((le_zero << 2) | ge_zero) as u32
    }

    pub(crate) fn classify2(self, rhs: Self) -> u32 {
        (self.classify() << 4) | rhs.classify()
    }
}

// The following codes are used to represent the class of an interval:
//
//  Code | Description
// ------+---------------
//     E | Empty
//     M | a < 0 < b
//    N0 | a < 0 ∧ b = 0
//    N1 | b < 0
//    P0 | a = 0 ∧ 0 < b
//    P1 | 0 < a
//     Z | a = b = 0
//    N* | a < 0 ∧ b ≤ 0
//    P* | a = 0 ∧ 0 ≤ b
// * These codes are used only in comments.
pub(crate) const C_E: u32 = 0b0000; // empty
pub(crate) const C_M: u32 = 0b0011; // b > 0 ∧ -a > 0
pub(crate) const C_N0: u32 = 0b1011; // b = 0 ∧ -a > 0
pub(crate) const C_N1: u32 = 0b1001; // b < 0 ∧ -a > 0
pub(crate) const C_P0: u32 = 0b0111; // b > 0 ∧ -a = 0
pub(crate) const C_P1: u32 = 0b0110; // b > 0 ∧ -a < 0
pub(crate) const C_Z: u32 = 0b1111; // b = 0 ∧ -a = 0

pub(crate) const C_E_E: u32 = (C_E << 4) | C_E;
pub(crate) const C_E_M: u32 = (C_E << 4) | C_M;
pub(crate) const C_E_N0: u32 = (C_E << 4) | C_N0;
pub(crate) const C_E_N1: u32 = (C_E << 4) | C_N1;
pub(crate) const C_E_P0: u32 = (C_E << 4) | C_P0;
pub(crate) const C_E_P1: u32 = (C_E << 4) | C_P1;
pub(crate) const C_E_Z: u32 = (C_E << 4) | C_Z;

pub(crate) const C_M_E: u32 = (C_M << 4) | C_E;
pub(crate) const C_M_M: u32 = (C_M << 4) | C_M;
pub(crate) const C_M_N0: u32 = (C_M << 4) | C_N0;
pub(crate) const C_M_N1: u32 = (C_M << 4) | C_N1;
pub(crate) const C_M_P0: u32 = (C_M << 4) | C_P0;
pub(crate) const C_M_P1: u32 = (C_M << 4) | C_P1;
pub(crate) const C_M_Z: u32 = (C_M << 4) | C_Z;

pub(crate) const C_N0_E: u32 = (C_N0 << 4) | C_E;
pub(crate) const C_N0_M: u32 = (C_N0 << 4) | C_M;
pub(crate) const C_N0_N0: u32 = (C_N0 << 4) | C_N0;
pub(crate) const C_N0_N1: u32 = (C_N0 << 4) | C_N1;
pub(crate) const C_N0_P0: u32 = (C_N0 << 4) | C_P0;
pub(crate) const C_N0_P1: u32 = (C_N0 << 4) | C_P1;
pub(crate) const C_N0_Z: u32 = (C_N0 << 4) | C_Z;

pub(crate) const C_N1_E: u32 = (C_N1 << 4) | C_E;
pub(crate) const C_N1_M: u32 = (C_N1 << 4) | C_M;
pub(crate) const C_N1_N0: u32 = (C_N1 << 4) | C_N0;
pub(crate) const C_N1_N1: u32 = (C_N1 << 4) | C_N1;
pub(crate) const C_N1_P0: u32 = (C_N1 << 4) | C_P0;
pub(crate) const C_N1_P1: u32 = (C_N1 << 4) | C_P1;
pub(crate) const C_N1_Z: u32 = (C_N1 << 4) | C_Z;

pub(crate) const C_P0_E: u32 = (C_P0 << 4) | C_E;
pub(crate) const C_P0_M: u32 = (C_P0 << 4) | C_M;
pub(crate) const C_P0_N0: u32 = (C_P0 << 4) | C_N0;
pub(crate) const C_P0_N1: u32 = (C_P0 << 4) | C_N1;
pub(crate) const C_P0_P0: u32 = (C_P0 << 4) | C_P0;
pub(crate) const C_P0_P1: u32 = (C_P0 << 4) | C_P1;
pub(crate) const C_P0_Z: u32 = (C_P0 << 4) | C_Z;

pub(crate) const C_P1_E: u32 = (C_P1 << 4) | C_E;
pub(crate) const C_P1_M: u32 = (C_P1 << 4) | C_M;
pub(crate) const C_P1_N0: u32 = (C_P1 << 4) | C_N0;
pub(crate) const C_P1_N1: u32 = (C_P1 << 4) | C_N1;
pub(crate) const C_P1_P0: u32 = (C_P1 << 4) | C_P0;
pub(crate) const C_P1_P1: u32 = (C_P1 << 4) | C_P1;
pub(crate) const C_P1_Z: u32 = (C_P1 << 4) | C_Z;

pub(crate) const C_Z_E: u32 = (C_Z << 4) | C_E;
pub(crate) const C_Z_M: u32 = (C_Z << 4) | C_M;
pub(crate) const C_Z_N0: u32 = (C_Z << 4) | C_N0;
pub(crate) const C_Z_N1: u32 = (C_Z << 4) | C_N1;
pub(crate) const C_Z_P0: u32 = (C_Z << 4) | C_P0;
pub(crate) const C_Z_P1: u32 = (C_Z << 4) | C_P1;
pub(crate) const C_Z_Z: u32 = (C_Z << 4) | C_Z;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    type I = Interval;

    #[test]
    fn classify() {
        assert!(I::EMPTY.classify() == C_E);
        assert!(I::ENTIRE.classify() == C_M);
        assert!(const_interval!(-1.0, 1.0).classify() == C_M);
        assert!(const_interval!(-1.0, 0.0).classify() == C_N0);
        assert!(const_interval!(-1.0, -1.0).classify() == C_N1);
        assert!(const_interval!(0.0, 1.0).classify() == C_P0);
        assert!(const_interval!(1.0, 1.0).classify() == C_P1);
        assert!(I::zero().classify() == C_Z);
    }
}
