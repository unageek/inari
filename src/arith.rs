use crate::{classify::*, interval::*, simd::*};
use std::{
    arch::x86_64::*,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        // [-b, -a] = [-a; b]
        Self {
            rep: swap(self.rep),
        }
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // [a + c, b + d] = [b + d; -a - c] = [b; -a] + [d; -c]
        let x = self.rep;
        let y = rhs.rep;
        Self { rep: add_ru(x, y) }
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        // [a - d, b - c] = [b - c; -a + d] = [b; -a] + [-c; d]
        let x = self.rep;
        let y = swap(rhs.rep); // [-c; d]
        Self { rep: add_ru(x, y) }
    }
}

impl Mul for Interval {
    type Output = Self;

    #[allow(clippy::many_single_char_names)]
    fn mul(self, rhs: Self) -> Self {
        // [a, b] * [c, d]
        //    |   E   |      M     |      N     |      P     |   Z
        // ---+-------+------------+------------+------------+-------
        //  E | empty |      empty |      empty |      empty | empty
        //  M | empty |     *1     | [b*c, a*c] | [a*d, b*d] |  zero
        //  N | empty | [a*d, a*c] | [b*d, a*c] | [a*d, b*c] |  zero
        //  P | empty | [b*c, b*d] | [b*c, a*d] | [a*c, b*d] |  zero
        //  Z | empty |       zero |       zero |       zero |  zero
        // *1 [min(a*d, b*c), max(a*c, b*d)]

        match self.classify2(rhs) {
            C_E_E | C_E_M | C_E_N0 | C_E_N1 | C_E_P0 | C_E_P1 | C_E_Z | C_M_E | C_N0_E | C_N1_E
            | C_P0_E | C_P1_E | C_Z_E => Self::EMPTY,
            C_M_Z | C_N0_Z | C_N1_Z | C_P0_Z | C_P1_Z | C_Z_M | C_Z_N0 | C_Z_N1 | C_Z_P0
            | C_Z_P1 | C_Z_Z => Self::zero(),
            C_M_M => {
                // M * M => [min(a*d, b*c), max(a*c, b*d)]
                //   = [max(a*c, b*d); -min(a*d, b*c)]
                //   = [max(a*c, b*d); max(-a*d, -b*c)]
                //   = simd_max([a*c; -a*d], [b*d; -b*c])
                //   = simd_max([a; -a] * [c; d], [b; -b] * [d; c])
                //   = simd_max([-a; -a] * [-c; d], [b; b] * [d; -c])
                let x = dup_lo(self.rep); // [-a; -a]
                let y = swap(rhs.rep); // [-c; d]
                let xy = mul_ru(x, y);
                let z = dup_hi(self.rep); // [b; b]
                let w = rhs.rep;
                let zw = mul_ru(z, w);
                let r = unsafe { _mm_max_pd(xy, zw) };
                Self { rep: r }
            }
            C_M_N0 | C_M_N1 => {
                // M * N => [b*c, a*c] = [a*c; -b*c] = [a; -b] * [c; c] = [-a; b] * [-c; -c]
                let x = swap(self.rep); // [-a; b]
                let y = dup_lo(rhs.rep); // [-c; -c]
                Self { rep: mul_ru(x, y) }
            }
            C_M_P0 | C_M_P1 => {
                // M * P => [a*d, b*d] = [b*d; -a*d] = [b; -a] * [d; d]
                let x = self.rep;
                let y = dup_hi(rhs.rep); // [d; d]
                Self { rep: mul_ru(x, y) }
            }
            C_N0_M | C_N1_M => {
                // N * M => [a*d, a*c] = [a*c; -a*d] = [a; -a] * [c; d] = [-a; -a] * [-c; d]
                let x = dup_lo(self.rep); // [-a; -a]
                let y = swap(rhs.rep); // [-c; d]
                Self { rep: mul_ru(x, y) }
            }
            C_N0_N0 | C_N0_N1 | C_N1_N0 | C_N1_N1 => {
                // N * N => [b*d, a*c] = [a*c; -b*d] = [a; -b] * [c; d] = [-a; -b] * [-c; d]
                let x0 = swap(self.rep); // [-a; b]
                let x = negate_lo(x0); // [-a; -b]
                let y = swap(rhs.rep); // [d; -c]
                Self { rep: mul_ru(x, y) }
            }
            C_N0_P0 | C_N0_P1 | C_N1_P0 | C_N1_P1 => {
                // N * P => [a*d, b*c] = [b*c; -a*d] = [b; -a] * [c; d]
                let x = self.rep;
                let y0 = negate_lo(rhs.rep); // [d; c]
                let y = swap(y0); // [c; d]
                Self { rep: mul_ru(x, y) }
            }
            C_P0_M | C_P1_M => {
                // P * M => [b*c, b*d] = [b*d; -b*c] = [b; -b] * [d; c] = [b; b] * [d; -c]
                let x = dup_hi(self.rep); // [b; b]
                let y = rhs.rep;
                Self { rep: mul_ru(x, y) }
            }
            C_P0_N0 | C_P0_N1 | C_P1_N0 | C_P1_N1 => {
                // P * N => [b*c, a*d] = [a*d; -b*c] = [a; -b] * [d; c] = [a; b] * [d; -c]
                let x0 = negate_lo(self.rep); // [b; a]
                let x = swap(x0); // [a; b]
                let y = rhs.rep; // [d; -c]
                Self { rep: mul_ru(x, y) }
            }
            C_P0_P0 | C_P0_P1 | C_P1_P0 | C_P1_P1 => {
                // P * P => [a*c, b*d] = [b*d; -a*c] = [b; -a] * [d; c]
                let x = self.rep;
                let y = negate_lo(rhs.rep); // [d; c]
                Self { rep: mul_ru(x, y) }
            }
            _ => unreachable!(),
        }
    }
}

impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // [a, b] / [c, d]
        //    |   E   |    M   |      N0     |     N1     |      P0     |     P1     |   Z
        // ---+-------+--------+-------------+------------+-------------+------------+-------
        //  E | empty |  empty |       empty |      empty |       empty |      empty | empty
        //  M | empty | entire |      entire | [b/d, a/d] |      entire | [a/c, b/c] | empty
        //  N | empty | entire | [ b/c, inf] | [b/c, a/d] | [-inf, b/d] | [a/c, b/d] | empty
        //  P | empty | entire | [-inf, a/c] | [b/d, a/c] | [ a/d, inf] | [a/d, b/c] | empty
        //  Z | empty |   zero |        zero |       zero |        zero |       zero | empty

        match self.classify2(rhs) {
            C_E_E | C_E_M | C_E_N0 | C_E_N1 | C_E_P0 | C_E_P1 | C_E_Z | C_M_E | C_M_Z | C_N0_E
            | C_N0_Z | C_N1_E | C_N1_Z | C_P0_E | C_P0_Z | C_P1_E | C_P1_Z | C_Z_E | C_Z_Z => {
                Self::EMPTY
            }
            C_M_M | C_M_N0 | C_M_P0 | C_N0_M | C_N1_M | C_P0_M | C_P1_M => Self::ENTIRE,
            C_Z_M | C_Z_N0 | C_Z_N1 | C_Z_P0 | C_Z_P1 => Self::zero(),
            C_M_N1 => {
                // M / N1 => [b/d, a/d] = [a/d; -b/d] = [a; -b] / [d; d] = [-a; b] / [-d; -d]
                let x = swap(self.rep); // [-a; b]
                let y0 = swap(rhs.rep); // [-c; d]
                let y1 = negate_lo(y0); // [-c; -d]
                let y = dup_lo(y1); // [-d; -d]
                Self { rep: div_ru(x, y) }
            }
            C_M_P1 => {
                // M / P1 => [a/c, b/c] = [b/c; -a/c] = [b; -a] / [c; c]
                let x = self.rep; // [b; -a]
                let y0 = negate_lo(rhs.rep); // [d; c]
                let y = dup_lo(y0); // [c; c]
                Self { rep: div_ru(x, y) }
            }
            C_N0_N0 | C_N1_N0 => {
                // N / N0 => [b/c, inf] = [inf; -b/c] = [_; b] / [_; -c]
                let x = swap(self.rep); // [-a; b]
                let y = rhs.rep; // [d; -c]
                Self {
                    rep: set_hi_inf(div_ru(x, y)),
                }
            }
            C_N0_N1 | C_N1_N1 => {
                // N / N1 => [b/c, a/d] = [a/d; -b/c] = [a; -b] / [d; c] = [a; b] / [d; -c]
                let x0 = negate_lo(self.rep); // [b; a]
                let x = swap(x0); // [a; b]
                let y = rhs.rep; // [d; -c]
                Self { rep: div_ru(x, y) }
            }
            C_N0_P0 | C_N1_P0 => {
                // N / P0 => [-inf, b/d] = [b/d; inf] = [b; _] / [d; _]
                let x = self.rep; // [b; -a]
                let y = rhs.rep; // [d; -c]
                Self {
                    rep: set_lo_inf(div_ru(x, y)),
                }
            }
            C_N0_P1 | C_N1_P1 => {
                // N / P1 => [a/c, b/d] = [b/d; -a/c] = [b; -a] / [d; c]
                let x = self.rep; // [b; -a]
                let y = negate_lo(rhs.rep); // [d, c]
                Self { rep: div_ru(x, y) }
            }
            C_P0_N0 | C_P1_N0 => {
                // P / N0 => [-inf, a/c] = [a/c; inf] = [-a; _] / [-c; _]
                // Swap after div would be better.
                let x = swap(self.rep); // [-a; b]
                let y = swap(rhs.rep); // [-c; d]
                Self {
                    rep: set_lo_inf(div_ru(x, y)),
                }
            }
            C_P0_N1 | C_P1_N1 => {
                // P / N1 => [b/d, a/c] = [a/c; -b/d] = [a; -b] / [c; d] = [-a; -b] / [-c; d]
                let x0 = swap(self.rep); // [-a; b]
                let x = negate_lo(x0); // [-a; -b]
                let y = swap(rhs.rep); // [-c; d]
                Self { rep: div_ru(x, y) }
            }
            C_P0_P0 | C_P1_P0 => {
                // P / P0 => [a/d, inf] = [inf; -a/d] = [_; -a] / [_; d]
                let x = self.rep; // [b; -a]
                let y = swap(rhs.rep); // [-c; d]
                Self {
                    rep: set_hi_inf(div_ru(x, y)),
                }
            }
            C_P0_P1 | C_P1_P1 => {
                // P / P1 => [a/d, b/c] = [b/c; -a/d] = [b; -a] / [c; d]
                let x = self.rep; // [b; -a]
                let y0 = negate_lo(rhs.rep); // [d; c]
                let y = swap(y0); // [c; d]
                Self { rep: div_ru(x, y) }
            }
            _ => unreachable!(),
        }
    }
}

impl Neg for DecoratedInterval {
    type Output = Self;

    fn neg(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(-self.x, self.d)
    }
}

impl Add for DecoratedInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x + rhs.x, self.d.min(rhs.d))
    }
}

impl Sub for DecoratedInterval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x - rhs.x, self.d.min(rhs.d))
    }
}

impl Mul for DecoratedInterval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x * rhs.x, self.d.min(rhs.d))
    }
}

impl Div for DecoratedInterval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        let d = if rhs.x.contains(0.0) {
            Decoration::Trv
        } else {
            self.d.min(rhs.d)
        };
        Self::set_dec(self.x / rhs.x, d)
    }
}

macro_rules! impl_op_assign {
    ($OpAssign:ident, $op_assign:ident, $op:ident) => {
        impl $OpAssign for Interval {
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl $OpAssign for DecoratedInterval {
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign, add);
impl_op_assign!(SubAssign, sub_assign, sub);
impl_op_assign!(MulAssign, mul_assign, mul);
impl_op_assign!(DivAssign, div_assign, div);

#[cfg(test)]
mod tests {
    use crate::*;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn add_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i += const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(4.0, 6.0));

        let mut i = const_dec_interval!(3.0, 4.0);
        i += const_dec_interval!(1.0, 2.0);
        assert_eq!(i, const_dec_interval!(4.0, 6.0));
    }

    #[test]
    fn sub_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i -= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(1.0, 3.0));

        let mut i = const_dec_interval!(3.0, 4.0);
        i -= const_dec_interval!(1.0, 2.0);
        assert_eq!(i, const_dec_interval!(1.0, 3.0));
    }

    #[test]
    fn mul_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i *= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(3.0, 8.0));

        let mut i = const_dec_interval!(3.0, 4.0);
        i *= const_dec_interval!(1.0, 2.0);
        assert_eq!(i, const_dec_interval!(3.0, 8.0));
    }

    #[test]
    fn div_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i /= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(1.5, 4.0));

        let mut i = const_dec_interval!(3.0, 4.0);
        i /= const_dec_interval!(1.0, 2.0);
        assert_eq!(i, const_dec_interval!(1.5, 4.0));
    }

    #[test]
    fn empty() {
        assert!((-I::EMPTY).is_empty());

        assert!((I::EMPTY + I::PI).is_empty());
        assert!((I::PI + I::EMPTY).is_empty());

        assert!((I::EMPTY - I::PI).is_empty());
        assert!((I::PI - I::EMPTY).is_empty());

        assert!((I::EMPTY * I::PI).is_empty());
        assert!((I::PI * I::EMPTY).is_empty());

        assert!((I::EMPTY / I::PI).is_empty());
        assert!((I::PI / I::EMPTY).is_empty());

        assert!((-DI::EMPTY).is_empty());

        assert!((DI::EMPTY + DI::PI).is_empty());
        assert!((DI::PI + DI::EMPTY).is_empty());

        assert!((DI::EMPTY - DI::PI).is_empty());
        assert!((DI::PI - DI::EMPTY).is_empty());

        assert!((DI::EMPTY * DI::PI).is_empty());
        assert!((DI::PI * DI::EMPTY).is_empty());

        assert!((DI::EMPTY / DI::PI).is_empty());
        assert!((DI::PI / DI::EMPTY).is_empty());
    }

    #[test]
    fn nai() {
        assert!((-DI::NAI).is_nai());

        assert!((DI::NAI + DI::PI).is_nai());
        assert!((DI::PI + DI::NAI).is_nai());

        assert!((DI::NAI - DI::PI).is_nai());
        assert!((DI::PI - DI::NAI).is_nai());

        assert!((DI::NAI * DI::PI).is_nai());
        assert!((DI::PI * DI::NAI).is_nai());

        assert!((DI::NAI / DI::PI).is_nai());
        assert!((DI::PI / DI::NAI).is_nai());
    }
}
