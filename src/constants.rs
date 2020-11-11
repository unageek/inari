#![allow(clippy::approx_constant)]

use crate::{const_interval, interval::*};
use hexf::*;
use std::mem::transmute;

impl Interval {
    /// $∅$, the empty interval.
    pub const EMPTY: Self = unsafe { transmute([f64::NAN, f64::NAN]) };

    /// $\[-∞, ∞\]$.
    pub const ENTIRE: Self = const_interval!(f64::NEG_INFINITY, f64::INFINITY);

    /// The tightest interval enclosing $\e$, the base of natural logarithms.
    pub const E: Self = const_interval!(
        hexf64!("0x2.b7e151628aed2p0"),
        hexf64!("0x2.b7e151628aed4p0")
    );

    /// The tightest interval enclosing $1 / \pi$.
    pub const FRAC_1_PI: Self = const_interval!(
        hexf64!("0x5.17cc1b7272208p-4"),
        hexf64!("0x5.17cc1b727220cp-4")
    );

    /// The tightest interval enclosing $1 / \sqrt{2}$.
    pub const FRAC_1_SQRT_2: Self = const_interval!(
        hexf64!("0xb.504f333f9de60p-4"),
        hexf64!("0xb.504f333f9de68p-4")
    );

    /// The tightest interval enclosing $2 / \pi$.
    pub const FRAC_2_PI: Self = const_interval!(
        hexf64!("0xa.2f9836e4e4410p-4"),
        hexf64!("0xa.2f9836e4e4418p-4")
    );

    /// The tightest interval enclosing $2 / \sqrt{\pi}$.
    pub const FRAC_2_SQRT_PI: Self = const_interval!(
        hexf64!("0x1.20dd750429b6dp0"),
        hexf64!("0x1.20dd750429b6ep0")
    );

    /// The tightest interval enclosing $\pi / 2$.
    pub const FRAC_PI_2: Self = const_interval!(
        hexf64!("0x1.921fb54442d18p0"),
        hexf64!("0x1.921fb54442d19p0")
    );

    /// The tightest interval enclosing $\pi / 3$.
    pub const FRAC_PI_3: Self = const_interval!(
        hexf64!("0x1.0c152382d7365p0"),
        hexf64!("0x1.0c152382d7366p0")
    );

    /// The tightest interval enclosing $\pi / 4$.
    pub const FRAC_PI_4: Self = const_interval!(
        hexf64!("0xc.90fdaa22168c0p-4"),
        hexf64!("0xc.90fdaa22168c8p-4")
    );

    /// The tightest interval enclosing $\pi / 6$.
    pub const FRAC_PI_6: Self = const_interval!(
        hexf64!("0x8.60a91c16b9b28p-4"),
        hexf64!("0x8.60a91c16b9b30p-4")
    );

    /// The tightest interval enclosing $\pi / 8$.
    pub const FRAC_PI_8: Self = const_interval!(
        hexf64!("0x6.487ed5110b460p-4"),
        hexf64!("0x6.487ed5110b464p-4")
    );

    /// The tightest interval enclosing $\ln 10$.
    pub const LN_10: Self = const_interval!(
        hexf64!("0x2.4d763776aaa2ap0"),
        hexf64!("0x2.4d763776aaa2cp0")
    );

    /// The tightest interval enclosing $\ln 2$.
    pub const LN_2: Self = const_interval!(
        hexf64!("0xb.17217f7d1cf78p-4"),
        hexf64!("0xb.17217f7d1cf80p-4")
    );

    /// The tightest interval enclosing $\log_{10} 2$.
    pub const LOG10_2: Self = const_interval!(
        hexf64!("0x4.d104d427de7f8p-4"),
        hexf64!("0x4.d104d427de7fcp-4")
    );

    /// The tightest interval enclosing $\log_{10} \e$.
    pub const LOG10_E: Self = const_interval!(
        hexf64!("0x6.f2dec549b9438p-4"),
        hexf64!("0x6.f2dec549b943cp-4")
    );

    /// The tightest interval enclosing $\log_2 10$.
    pub const LOG2_10: Self = const_interval!(
        hexf64!("0x3.5269e12f346e2p0"),
        hexf64!("0x3.5269e12f346e4p0")
    );

    /// The tightest interval enclosing $\log_2 \e$.
    pub const LOG2_E: Self = const_interval!(
        hexf64!("0x1.71547652b82fep0"),
        hexf64!("0x1.71547652b82ffp0")
    );

    /// The tightest interval enclosing $\pi$.
    pub const PI: Self = const_interval!(
        hexf64!("0x3.243f6a8885a30p0"),
        hexf64!("0x3.243f6a8885a32p0")
    );

    /// The tightest interval enclosing $\sqrt{2}$.
    pub const SQRT_2: Self = const_interval!(
        hexf64!("0x1.6a09e667f3bccp0"),
        hexf64!("0x1.6a09e667f3bcdp0")
    );

    /// The tightest interval enclosing $2 \pi$.
    pub const TAU: Self = const_interval!(
        hexf64!("0x6.487ed5110b460p0"),
        hexf64!("0x6.487ed5110b464p0")
    );
}

macro_rules! def_com {
    ($c:ident) => {
        pub const $c: Self = Self::new_unchecked(Interval::$c, Decoration::Com);
    };
}

impl DecInterval {
    pub const EMPTY: Self = Self::new_unchecked(Interval::EMPTY, Decoration::Trv);
    pub const ENTIRE: Self = Self::new_unchecked(Interval::ENTIRE, Decoration::Dac);
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
    type I = Interval;
    type DI = DecInterval;

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
