/*
 *
 * Test cases for interval inverse tangent function with two arguments.
 *
 * Copyright 2015-2016 Oliver Heimlich (oheim@posteo.de)
 *
 * Copying and distribution of this file, with or without modification,
 * are permitted in any medium without royalty provided the copyright
 * notice and this notice are preserved.  This file is offered as-is,
 * without any warranty.
 *
 */
//Language imports
#![rustfmt::skip]
#![allow(unused_attributes, unused_imports)]

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use hexf::*;
type I = inari::Interval;

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_test() {
    assert_eq2!(I::EMPTY.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(I::ENTIRE), I::EMPTY);
    assert_eq2!(I::ENTIRE.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(I::ENTIRE.atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(f64::NEG_INFINITY, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).atan2(n2i(0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq2!(n2i(hexf64!("-0x1.0000000000000p-1022"), 0.0).atan2(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("-0x1.0000000000000p-1022"))), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(1.0, 1.0).atan2(n2i(-1.0, -1.0)), n2i(hexf64!("0x1.2d97c7f3321d2p+1"), hexf64!("0x1.2d97c7f3321d3p+1")));
    assert_eq2!(n2i(1.0, 1.0).atan2(n2i(1.0, 1.0)), n2i(hexf64!("0x1.921fb54442d18p-1"), hexf64!("0x1.921fb54442d19p-1")));
    assert_eq2!(n2i(-1.0, -1.0).atan2(n2i(1.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p-1"), hexf64!("-0x1.921fb54442d18p-1")));
    assert_eq2!(n2i(-1.0, -1.0).atan2(n2i(-1.0, -1.0)), n2i(hexf64!("-0x1.2d97c7f3321d3p+1"), hexf64!("-0x1.2d97c7f3321d2p+1")));
    assert_eq2!(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022")).atan2(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("-0x1.0000000000000p-1022"))), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022")).atan2(n2i(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022"))), n2i(hexf64!("-0x1.921fb54442d19p-1"), hexf64!("0x1.921fb54442d19p-1")));
    assert_eq2!(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("-0x1.0000000000000p-1022")).atan2(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022"))), n2i(hexf64!("-0x1.2d97c7f3321d3p+1"), hexf64!("-0x1.921fb54442d18p-1")));
    assert_eq2!(n2i(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022")).atan2(n2i(hexf64!("-0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p-1022"))), n2i(hexf64!("0x1.921fb54442d18p-1"), hexf64!("0x1.2d97c7f3321d3p+1")));
    assert_eq2!(n2i(-2.0, 2.0).atan2(n2i(-3.0, -1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(0.0, 2.0).atan2(n2i(-3.0, -1.0)), n2i(hexf64!("0x1.0468a8ace4df6p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-3.0, -1.0)), n2i(hexf64!("0x1.e47df3d0dd4d0p+0"), hexf64!("0x1.68f095fdf593dp+1")));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.56c6e7397f5afp+1")));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-2.0, 2.0)), n2i(hexf64!("0x1.dac670561bb4fp-2"), hexf64!("0x1.56c6e7397f5afp+1")));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(0.0, 2.0)), n2i(hexf64!("0x1.dac670561bb4fp-2"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(1.0, 3.0)), n2i(hexf64!("0x1.4978fa3269ee1p-2"), hexf64!("0x1.3fc176b7a8560p+0")));
    assert_eq2!(n2i(0.0, 2.0).atan2(n2i(1.0, 3.0)), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.1b6e192ebbe45p+0")));
    assert_eq2!(n2i(-2.0, 2.0).atan2(n2i(1.0, 3.0)), n2i(hexf64!("-0x1.1b6e192ebbe45p+0"), hexf64!("0x1.1b6e192ebbe45p+0")));
    assert_eq2!(n2i(-2.0, 0.0).atan2(n2i(1.0, 3.0)), n2i(hexf64!("-0x1.1b6e192ebbe45p+0"), hexf64!("0x0.0p+0")));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(1.0, 3.0)), n2i(hexf64!("-0x1.3fc176b7a8560p+0"), hexf64!("-0x1.4978fa3269ee1p-2")));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(0.0, 2.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.dac670561bb4fp-2")));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-2.0, 2.0)), n2i(hexf64!("-0x1.56c6e7397f5afp+1"), hexf64!("-0x1.dac670561bb4fp-2")));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.56c6e7397f5afp+1"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-3.0, -1.0)), n2i(hexf64!("-0x1.68f095fdf593dp+1"), hexf64!("-0x1.e47df3d0dd4d0p+0")));
    assert_eq2!(n2i(-2.0, 0.0).atan2(n2i(-3.0, -1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(-5.0, 0.0).atan2(n2i(-5.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(0.0, 5.0).atan2(n2i(-5.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq2!(n2i(0.0, 5.0).atan2(n2i(0.0, 5.0)), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq2!(n2i(-5.0, 0.0).atan2(n2i(0.0, 5.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x0.0p+0")));
}
