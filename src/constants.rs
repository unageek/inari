#![allow(clippy::approx_constant)]

use crate::{const_interval, interval::*};
use std::mem::transmute;

impl Interval {
    /// $∅$, the empty set.
    pub const EMPTY: Self = unsafe { transmute([f64::NAN, f64::NAN]) };

    /// $\[-∞, +∞\]$.
    pub const ENTIRE: Self = const_interval!(f64::NEG_INFINITY, f64::INFINITY);

    /// The tightest interval enclosing $\e$, the base of natural logarithms.
    pub const E: Self = const_interval!(2.718281828459045, 2.7182818284590455);

    /// The tightest interval enclosing $1 / π$.
    pub const FRAC_1_PI: Self = const_interval!(0.31830988618379064, 0.3183098861837907);

    /// The tightest interval enclosing $1 / \sqrt{2}$.
    pub const FRAC_1_SQRT_2: Self = const_interval!(0.7071067811865475, 0.7071067811865476);

    /// The tightest interval enclosing $2 / π$.
    pub const FRAC_2_PI: Self = const_interval!(0.6366197723675813, 0.6366197723675814);

    /// The tightest interval enclosing $2 / \sqrt{π}$.
    pub const FRAC_2_SQRT_PI: Self = const_interval!(1.1283791670955126, 1.1283791670955128);

    /// The tightest interval enclosing $π / 2$.
    pub const FRAC_PI_2: Self = const_interval!(1.5707963267948966, 1.5707963267948968);

    /// The tightest interval enclosing $π / 3$.
    pub const FRAC_PI_3: Self = const_interval!(1.0471975511965976, 1.0471975511965979);

    /// The tightest interval enclosing $π / 4$.
    pub const FRAC_PI_4: Self = const_interval!(0.7853981633974483, 0.7853981633974484);

    /// The tightest interval enclosing $π / 6$.
    pub const FRAC_PI_6: Self = const_interval!(0.5235987755982988, 0.5235987755982989);

    /// The tightest interval enclosing $π / 8$.
    pub const FRAC_PI_8: Self = const_interval!(0.39269908169872414, 0.3926990816987242);

    /// The tightest interval enclosing $\ln 10$.
    pub const LN_10: Self = const_interval!(2.3025850929940455, 2.302585092994046);

    /// The tightest interval enclosing $\ln 2$.
    pub const LN_2: Self = const_interval!(0.6931471805599453, 0.6931471805599454);

    /// The tightest interval enclosing $\log_{10} 2$.
    pub const LOG10_2: Self = const_interval!(0.30102999566398114, 0.3010299956639812);

    /// The tightest interval enclosing $\log_{10} \e$.
    pub const LOG10_E: Self = const_interval!(0.4342944819032518, 0.43429448190325187);

    /// The tightest interval enclosing $\log_2 10$.
    pub const LOG2_10: Self = const_interval!(3.321928094887362, 3.3219280948873626);

    /// The tightest interval enclosing $\log_2 \e$.
    pub const LOG2_E: Self = const_interval!(1.4426950408889634, 1.4426950408889636);

    /// The tightest interval enclosing $π$.
    pub const PI: Self = const_interval!(3.141592653589793, 3.1415926535897936);

    /// The tightest interval enclosing $\sqrt{2}$.
    pub const SQRT_2: Self = const_interval!(1.414213562373095, 1.4142135623730951);

    /// The tightest interval enclosing $2 π$.
    pub const TAU: Self = const_interval!(6.283185307179586, 6.283185307179587);
}

macro_rules! def_com {
    ($c:ident) => {
        #[doc = concat!("See [`Interval::", stringify!($c), "`].")]
        pub const $c: Self = Self::new_unchecked(Interval::$c, Decoration::Com);
    };
}

impl DecInterval {
    /// $∅$, the empty set, decorated with [`Decoration::Trv`].
    pub const EMPTY: Self = Self::new_unchecked(Interval::EMPTY, Decoration::Trv);

    /// $\[-∞, +∞\]$, decorated with [`Decoration::Dac`].
    pub const ENTIRE: Self = Self::new_unchecked(Interval::ENTIRE, Decoration::Dac);

    /// A NaI (Not an Interval).
    pub const NAI: Self = Self::new_unchecked(Interval::EMPTY, Decoration::Ill);

    def_com!(E);
    def_com!(FRAC_1_PI);
    def_com!(FRAC_1_SQRT_2);
    def_com!(FRAC_2_PI);
    def_com!(FRAC_2_SQRT_PI);
    def_com!(FRAC_PI_2);
    def_com!(FRAC_PI_3);
    def_com!(FRAC_PI_4);
    def_com!(FRAC_PI_6);
    def_com!(FRAC_PI_8);
    def_com!(LN_10);
    def_com!(LN_2);
    def_com!(LOG10_2);
    def_com!(LOG10_E);
    def_com!(LOG2_10);
    def_com!(LOG2_E);
    def_com!(PI);
    def_com!(SQRT_2);
    def_com!(TAU);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    // This only works with positive numbers.
    fn succ(x: f64) -> f64 {
        f64::from_bits(x.to_bits() + 1)
    }

    macro_rules! check {
        ($c:ident) => {
            check!($c, std::f64::consts::$c);
        };

        ($c:ident, $f:expr) => {
            assert!({
                let a = I::$c.inf();
                let b = I::$c.sup();
                b == succ(a) && (a == $f || $f == b)
            });
            assert_eq!(DI::$c, DI::new(I::$c));
        };
    }

    #[test]
    fn constants() {
        check!(E);
        check!(FRAC_1_PI);
        check!(FRAC_1_SQRT_2);
        check!(FRAC_2_PI);
        check!(FRAC_2_SQRT_PI);
        check!(FRAC_PI_2);
        check!(FRAC_PI_3);
        check!(FRAC_PI_4);
        check!(FRAC_PI_6);
        check!(FRAC_PI_8);
        check!(LN_10);
        check!(LN_2);
        check!(LOG10_2);
        check!(LOG10_E);
        check!(LOG2_10);
        check!(LOG2_E);
        check!(PI);
        check!(SQRT_2);
        check!(TAU);
    }
}
