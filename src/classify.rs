use crate::interval::Interval;
use core::arch::x86_64::*;

impl Interval {
    // For a given interval [a, b], returns the bit pattern
    // that encodes [b <= 0, -a <= 0, b >= 0, -a >= 0].
    //
    // The following codes are used to represent the class of an interval:
    //
    // - `E` - empty
    // - `M` - a < 0 < b
    // - `N` - a < 0, b <= 0 (used only in comments)
    // - `N0` - a < 0, b = 0
    // - `N1` - b < 0
    // - `P` - a = 0, 0 <= b (used only in comments)
    // - `P0` - a = 0, 0 < b
    // - `P1` - 0 < a
    // - `Z` - a = b = 0
    pub(crate) fn classify(self) -> i32 {
        let zero = unsafe { _mm_setzero_pd() };
        let ge_zero = unsafe { _mm_movemask_pd(_mm_cmpge_pd(self.rep, zero)) };
        let le_zero = unsafe { _mm_movemask_pd(_mm_cmple_pd(self.rep, zero)) };
        (le_zero << 2) | ge_zero
    }
}

pub(crate) const C_E: i32 = 0b0000; // empty
pub(crate) const C_M: i32 = 0b0011; // b > 0 && -a > 0
pub(crate) const C_N0: i32 = 0b1011; // b == 0 && -a > 0
pub(crate) const C_N1: i32 = 0b1001; // b < 0 && -a > 0
pub(crate) const C_P0: i32 = 0b0111; // b > 0 && -a == 0
pub(crate) const C_P1: i32 = 0b0110; // b > 0 && -a < 0
pub(crate) const C_Z: i32 = 0b1111; // b == 0 && -a == 0

pub(crate) const C_E_E: i32 = (C_E << 4) | C_E;
pub(crate) const C_E_M: i32 = (C_E << 4) | C_M;
pub(crate) const C_E_N0: i32 = (C_E << 4) | C_N0;
pub(crate) const C_E_N1: i32 = (C_E << 4) | C_N1;
pub(crate) const C_E_P0: i32 = (C_E << 4) | C_P0;
pub(crate) const C_E_P1: i32 = (C_E << 4) | C_P1;
pub(crate) const C_E_Z: i32 = (C_E << 4) | C_Z;

pub(crate) const C_M_E: i32 = (C_M << 4) | C_E;
pub(crate) const C_M_M: i32 = (C_M << 4) | C_M;
pub(crate) const C_M_N0: i32 = (C_M << 4) | C_N0;
pub(crate) const C_M_N1: i32 = (C_M << 4) | C_N1;
pub(crate) const C_M_P0: i32 = (C_M << 4) | C_P0;
pub(crate) const C_M_P1: i32 = (C_M << 4) | C_P1;
pub(crate) const C_M_Z: i32 = (C_M << 4) | C_Z;

pub(crate) const C_N0_E: i32 = (C_N0 << 4) | C_E;
pub(crate) const C_N0_M: i32 = (C_N0 << 4) | C_M;
pub(crate) const C_N0_N0: i32 = (C_N0 << 4) | C_N0;
pub(crate) const C_N0_N1: i32 = (C_N0 << 4) | C_N1;
pub(crate) const C_N0_P0: i32 = (C_N0 << 4) | C_P0;
pub(crate) const C_N0_P1: i32 = (C_N0 << 4) | C_P1;
pub(crate) const C_N0_Z: i32 = (C_N0 << 4) | C_Z;

pub(crate) const C_N1_E: i32 = (C_N1 << 4) | C_E;
pub(crate) const C_N1_M: i32 = (C_N1 << 4) | C_M;
pub(crate) const C_N1_N0: i32 = (C_N1 << 4) | C_N0;
pub(crate) const C_N1_N1: i32 = (C_N1 << 4) | C_N1;
pub(crate) const C_N1_P0: i32 = (C_N1 << 4) | C_P0;
pub(crate) const C_N1_P1: i32 = (C_N1 << 4) | C_P1;
pub(crate) const C_N1_Z: i32 = (C_N1 << 4) | C_Z;

pub(crate) const C_P0_E: i32 = (C_P0 << 4) | C_E;
pub(crate) const C_P0_M: i32 = (C_P0 << 4) | C_M;
pub(crate) const C_P0_N0: i32 = (C_P0 << 4) | C_N0;
pub(crate) const C_P0_N1: i32 = (C_P0 << 4) | C_N1;
pub(crate) const C_P0_P0: i32 = (C_P0 << 4) | C_P0;
pub(crate) const C_P0_P1: i32 = (C_P0 << 4) | C_P1;
pub(crate) const C_P0_Z: i32 = (C_P0 << 4) | C_Z;

pub(crate) const C_P1_E: i32 = (C_P1 << 4) | C_E;
pub(crate) const C_P1_M: i32 = (C_P1 << 4) | C_M;
pub(crate) const C_P1_N0: i32 = (C_P1 << 4) | C_N0;
pub(crate) const C_P1_N1: i32 = (C_P1 << 4) | C_N1;
pub(crate) const C_P1_P0: i32 = (C_P1 << 4) | C_P0;
pub(crate) const C_P1_P1: i32 = (C_P1 << 4) | C_P1;
pub(crate) const C_P1_Z: i32 = (C_P1 << 4) | C_Z;

pub(crate) const C_Z_E: i32 = (C_Z << 4) | C_E;
pub(crate) const C_Z_M: i32 = (C_Z << 4) | C_M;
pub(crate) const C_Z_N0: i32 = (C_Z << 4) | C_N0;
pub(crate) const C_Z_N1: i32 = (C_Z << 4) | C_N1;
pub(crate) const C_Z_P0: i32 = (C_Z << 4) | C_P0;
pub(crate) const C_Z_P1: i32 = (C_Z << 4) | C_P1;
pub(crate) const C_Z_Z: i32 = (C_Z << 4) | C_Z;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval;
    type I = Interval;

    #[test]
    fn classify() {
        assert!(I::empty().classify() == C_E);
        assert!(I::entire().classify() == C_M);
        assert!(interval!(-1.0, 1.0).unwrap().classify() == C_M);
        assert!(interval!(-1.0, 0.0).unwrap().classify() == C_N0);
        assert!(interval!(-1.0, -1.0).unwrap().classify() == C_N1);
        assert!(interval!(0.0, 1.0).unwrap().classify() == C_P0);
        assert!(interval!(1.0, 1.0).unwrap().classify() == C_P1);
        assert!(I::zero().classify() == C_Z);
    }
}
