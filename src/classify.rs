use crate::{interval::Interval, simd::*};
use std::mem::transmute;

impl Interval {
    pub(crate) fn classify(self) -> IntervalClass {
        let zero = splat(0.0);
        let ge_zero = bitmask(ge(self.rep, zero)) as u8;
        let le_zero = bitmask(le(self.rep, zero)) as u8;
        unsafe { transmute((le_zero << 2) | ge_zero) }
    }

    pub(crate) fn classify2(self, rhs: Self) -> IntervalClass2 {
        unsafe { transmute(((self.classify() as u8) << 4) | rhs.classify() as u8) }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IntervalClass {
    // Each bit represents b ≤ 0, -a ≤ 0, b ≥ 0 and -a ≥ 0, respectively.
    E = 0b0000,  // empty
    M = 0b0011,  // b > 0 ∧ -a > 0
    N0 = 0b1011, // b = 0 ∧ -a > 0
    N1 = 0b1001, // b < 0 ∧ -a > 0
    P0 = 0b0111, // b > 0 ∧ -a = 0
    P1 = 0b0110, // b > 0 ∧ -a < 0
    Z = 0b1111,  // b = 0 ∧ -a = 0
}

macro_rules! discr {
    ($x:ident, $y:ident) => {
        ((IntervalClass::$x as u8) << 4) | IntervalClass::$y as u8
    };
}

#[allow(dead_code)]
#[allow(non_camel_case_types)] // We could rename E_E to ExE, etc., but that would degrade the legibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IntervalClass2 {
    E_E = discr!(E, E),
    E_M = discr!(E, M),
    E_N0 = discr!(E, N0),
    E_N1 = discr!(E, N1),
    E_P0 = discr!(E, P0),
    E_P1 = discr!(E, P1),
    E_Z = discr!(E, Z),

    M_E = discr!(M, E),
    M_M = discr!(M, M),
    M_N0 = discr!(M, N0),
    M_N1 = discr!(M, N1),
    M_P0 = discr!(M, P0),
    M_P1 = discr!(M, P1),
    M_Z = discr!(M, Z),

    N0_E = discr!(N0, E),
    N0_M = discr!(N0, M),
    N0_N0 = discr!(N0, N0),
    N0_N1 = discr!(N0, N1),
    N0_P0 = discr!(N0, P0),
    N0_P1 = discr!(N0, P1),
    N0_Z = discr!(N0, Z),

    N1_E = discr!(N1, E),
    N1_M = discr!(N1, M),
    N1_N0 = discr!(N1, N0),
    N1_N1 = discr!(N1, N1),
    N1_P0 = discr!(N1, P0),
    N1_P1 = discr!(N1, P1),
    N1_Z = discr!(N1, Z),

    P0_E = discr!(P0, E),
    P0_M = discr!(P0, M),
    P0_N0 = discr!(P0, N0),
    P0_N1 = discr!(P0, N1),
    P0_P0 = discr!(P0, P0),
    P0_P1 = discr!(P0, P1),
    P0_Z = discr!(P0, Z),

    P1_E = discr!(P1, E),
    P1_M = discr!(P1, M),
    P1_N0 = discr!(P1, N0),
    P1_N1 = discr!(P1, N1),
    P1_P0 = discr!(P1, P0),
    P1_P1 = discr!(P1, P1),
    P1_Z = discr!(P1, Z),

    Z_E = discr!(Z, E),
    Z_M = discr!(Z, M),
    Z_N0 = discr!(Z, N0),
    Z_N1 = discr!(Z, N1),
    Z_P0 = discr!(Z, P0),
    Z_P1 = discr!(Z, P1),
    Z_Z = discr!(Z, Z),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use Interval as I;

    #[test]
    fn classify() {
        use IntervalClass::*;
        assert_eq!(I::EMPTY.classify(), E);
        assert_eq!(I::ENTIRE.classify(), M);
        assert_eq!(const_interval!(-1.0, 1.0).classify(), M);
        assert_eq!(const_interval!(-1.0, 0.0).classify(), N0);
        assert_eq!(const_interval!(-1.0, -1.0).classify(), N1);
        assert_eq!(const_interval!(0.0, 1.0).classify(), P0);
        assert_eq!(const_interval!(1.0, 1.0).classify(), P1);
        assert_eq!(I::zero().classify(), Z);
    }
}
