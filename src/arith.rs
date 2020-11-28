use crate::{classify::*, interval::*, simd::*};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for Interval {
    type Output = Self;

    /// Tightness: tightest
    fn neg(self) -> Self {
        // [-b, -a] = [b; -a]
        Self {
            rep: swap(self.rep),
        }
    }
}

impl Add for Interval {
    type Output = Self;

    /// Tightness: tightest
    fn add(self, rhs: Self) -> Self {
        // [a + c, b + d] = [-a - c; b + d] = [-a; b] .+ [-c; d]
        let x = self.rep; // [-a; b]
        let y = rhs.rep; // [-c; d]
        Self { rep: add_ru(x, y) }
    }
}

impl Sub for Interval {
    type Output = Self;

    /// Tightness: tightest
    fn sub(self, rhs: Self) -> Self {
        // [a - d, b - c] = [-a + d; b - c] = [-a; b] .+ [d; -c]
        let x = self.rep; // [-a; b]
        let y = swap(rhs.rep); // [d; -c]
        Self { rep: add_ru(x, y) }
    }
}

impl Mul for Interval {
    type Output = Self;

    /// Tightness: tightest
    #[allow(clippy::many_single_char_names)]
    fn mul(self, rhs: Self) -> Self {
        // [a, b] * [c, d]
        //    |      M     |      N     |      P     |   Z
        // ---+------------+------------+------------+------
        //  M |     *1     | [b*c, a*c] | [a*d, b*d] | zero
        //  N | [a*d, a*c] | [b*d, a*c] | [a*d, b*c] | zero
        //  P | [b*c, b*d] | [b*c, a*d] | [a*c, b*d] | zero
        //  Z |       zero |       zero |       zero | zero
        // *1 [min(a*d, b*c), max(a*c, b*d)]

        use IntervalClass2::*;
        match self.classify2(rhs) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | N0_E | N1_E | P0_E | P1_E | Z_E => {
                Self::EMPTY
            }
            M_Z | N0_Z | N1_Z | P0_Z | P1_Z | Z_M | Z_N0 | Z_N1 | Z_P0 | Z_P1 | Z_Z => Self::zero(),
            M_M => {
                // M * M => [min(a*d, b*c), max(a*c, b*d)]
                //   = [-min(a*d, b*c); max(a*c, b*d)]
                //   = [max(-a*d, -b*c); max(a*c, b*d)]
                //   = [max(-a*d, b*-c); max(-a*-c, b*d)]
                //   = .max([-a*d; -a*-c], [b*-c; b*d])
                //   = .max([-a; -a] .* [d; -c], [b; b] .* [-c; d])
                let x = shuffle02(self.rep, self.rep); // [-a; -a]
                let y = swap(rhs.rep); // [d; -c]
                let xy = mul_ru(x, y);
                let z = shuffle13(self.rep, self.rep); // [b; b]
                let w = rhs.rep; // [-c; d]
                let zw = mul_ru(z, w);
                let r = max(xy, zw);
                Self { rep: r }
            }
            M_N0 | M_N1 => {
                // M * N => [b*c, a*c] = [-b*c; a*c] = [b*-c; -a*-c] = [b; -a] .* [-c; -c]
                let x = swap(self.rep); // [b; -a]
                let y = shuffle02(rhs.rep, rhs.rep); // [-c; -c]
                Self { rep: mul_ru(x, y) }
            }
            M_P0 | M_P1 => {
                // M * P => [a*d, b*d] = [-a*d; b*d] = [-a; b] .* [d; d]
                let x = self.rep; // [-a; b]
                let y = shuffle13(rhs.rep, rhs.rep); // [d; d]
                Self { rep: mul_ru(x, y) }
            }
            N0_M | N1_M => {
                // N * M => [a*d, a*c] = [-a*d; a*c] = [-a*d; -a*-c] = [-a; -a] .* [d; -c]
                let x = shuffle02(self.rep, self.rep); // [-a; -a]
                let y = swap(rhs.rep); // [d; -c]
                Self { rep: mul_ru(x, y) }
            }
            N0_N0 | N0_N1 | N1_N0 | N1_N1 => {
                // N * N => [b*d, a*c] = [-b*d; a*c] = [-b*d; -a*-c] = [-b; -a] .* [d; -c]
                let x = swap(self.rep); // [b; -a]
                let x = neg0(x); // [-b; -a]
                let y = swap(rhs.rep); // [d; -c]
                Self { rep: mul_ru(x, y) }
            }
            N0_P0 | N0_P1 | N1_P0 | N1_P1 => {
                // N * P => [a*d, b*c] = [-a*d; b*c] = [-a; b] .* [d; c]
                let x = self.rep; // [-a; b]
                let y = neg0(rhs.rep); // [c; d]
                let y = swap(y); // [d; c]
                Self { rep: mul_ru(x, y) }
            }
            P0_M | P1_M => {
                // P * M => [b*c, b*d] = [-b*c; b*d] = [b*-c; b*d] = [b; b] .* [-c; d]
                let x = shuffle13(self.rep, self.rep); // [b; b]
                let y = rhs.rep; // [-c; d]
                Self { rep: mul_ru(x, y) }
            }
            P0_N0 | P0_N1 | P1_N0 | P1_N1 => {
                // P * N => [b*c, a*d] = [-b*c; a*d] = [b*-c; a*d] = [b; a] .* [-c; d]
                let x = neg0(self.rep); // [a; b]
                let x = swap(x); // [b; a]
                let y = rhs.rep; // [-c; d]
                Self { rep: mul_ru(x, y) }
            }
            P0_P0 | P0_P1 | P1_P0 | P1_P1 => {
                // P * P => [a*c, b*d] = [-a*c; b*d] = [-a; b] .* [c; d]
                let x = self.rep; // [-a; b]
                let y = neg0(rhs.rep); // [c; d]
                Self { rep: mul_ru(x, y) }
            }
        }
    }
}

impl Div for Interval {
    type Output = Self;

    /// Tightness: tightest
    fn div(self, rhs: Self) -> Self {
        // [a, b] / [c, d]
        //    |    M   |     N0    |     N1     |     P0    |     P1     |   Z
        // ---+--------+-----------+------------+-----------+------------+-------
        //  M | entire |    entire | [b/d, a/d] |    entire | [a/c, b/c] | empty
        //  N | entire | [b/c, +∞] | [b/c, a/d] | [-∞, b/d] | [a/c, b/d] | empty
        //  P | entire | [-∞, a/c] | [b/d, a/c] | [a/d, +∞] | [a/d, b/c] | empty
        //  Z |   zero |      zero |       zero |      zero |       zero | empty

        use IntervalClass2::*;
        match self.classify2(rhs) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | M_Z | N0_E | N0_Z | N1_E | N1_Z
            | P0_E | P0_Z | P1_E | P1_Z | Z_E | Z_Z => Self::EMPTY,
            M_M | M_N0 | M_P0 | N0_M | N1_M | P0_M | P1_M => Self::ENTIRE,
            Z_M | Z_N0 | Z_N1 | Z_P0 | Z_P1 => Self::zero(),
            M_N1 => {
                // M / N1 => [b/d, a/d] = [-b/d; a/d] = [b/-d; -a/-d]  = [b; -a] ./ [-d; -d]
                let x = swap(self.rep); // [b; -a]
                let y = swap(rhs.rep); // [d; -c]
                let y = neg0(y); // [-d; -c]
                let y = shuffle02(y, y); // [-d; -d]
                Self { rep: div_ru(x, y) }
            }
            M_P1 => {
                // M / P1 => [a/c, b/c] = [-a/c; b/c] = [-a; b] ./ [c; c]
                let x = self.rep; // [-a; b]
                let y = neg0(rhs.rep); // [c; d]
                let y = shuffle02(y, y); // [c; c]
                Self { rep: div_ru(x, y) }
            }
            N0_N0 | N1_N0 => {
                // N / N0 => [b/c, +∞] = [-b/c; +∞] = [b/-c; +∞]
                let x = swap(self.rep); // [b; -a]
                let y = rhs.rep; // [-c; d]
                Self {
                    rep: shuffle02(div_ru(x, y), splat(f64::INFINITY)),
                }
            }
            N0_N1 | N1_N1 => {
                // N / N1 => [b/c, a/d] = [-b/c; a/d] = [b/-c; a/d] = [b; a] ./ [-c; d]
                let x = neg0(self.rep); // [a; b]
                let x = swap(x); // [b; a]
                let y = rhs.rep; // [-c; d]
                Self { rep: div_ru(x, y) }
            }
            N0_P0 | N1_P0 => {
                // N / P0 => [-∞, b/d] = [+∞; b/d]
                let x = self.rep; // [-a; b]
                let y = rhs.rep; // [-c; d]
                Self {
                    rep: shuffle03(splat(f64::INFINITY), div_ru(x, y)),
                }
            }
            N0_P1 | N1_P1 => {
                // N / P1 => [a/c, b/d] = [-a/c; b/d] = [-a; b] ./ [c; d]
                let x = self.rep; // [-a; b]
                let y = neg0(rhs.rep); // [c; d]
                Self { rep: div_ru(x, y) }
            }
            P0_N0 | P1_N0 => {
                // P / N0 => [-∞, a/c] = [+∞; a/c] = [+∞; -a/-c]
                let x = self.rep; // [-a; b]
                let y = rhs.rep; // [-c; d]
                Self {
                    rep: shuffle02(splat(f64::INFINITY), div_ru(x, y)),
                }
            }
            P0_N1 | P1_N1 => {
                // P / N1 => [b/d, a/c] = [-b/d; a/c] = [-b/d; -a/-c] = [-b; -a] ./ [d; -c]
                let x = swap(self.rep); // [b; -a]
                let x = neg0(x); // [-b; -a]
                let y = swap(rhs.rep); // [d; -c]
                Self { rep: div_ru(x, y) }
            }
            P0_P0 | P1_P0 => {
                // P / P0 => [a/d, +∞] = [-a/d; +∞]
                let x = self.rep; // [-a; b]
                let y = swap(rhs.rep); // [d; -c]
                Self {
                    rep: shuffle02(div_ru(x, y), splat(f64::INFINITY)),
                }
            }
            P0_P1 | P1_P1 => {
                // P / P1 => [a/d, b/c] = [-a/d; b/c] = [-a; b] ./ [d; c]
                let x = self.rep; // [-a; b]
                let y = neg0(rhs.rep); // [c; d]
                let y = swap(y); // [d; c]
                Self { rep: div_ru(x, y) }
            }
        }
    }
}

impl Neg for DecInterval {
    type Output = Self;

    fn neg(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(-self.x, self.d)
    }
}

impl Add for DecInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x + rhs.x, self.d.min(rhs.d))
    }
}

impl Sub for DecInterval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x - rhs.x, self.d.min(rhs.d))
    }
}

impl Mul for DecInterval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        if self.is_nai() || rhs.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(self.x * rhs.x, self.d.min(rhs.d))
    }
}

impl Div for DecInterval {
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

        impl $OpAssign for DecInterval {
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
    type DI = DecInterval;
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
