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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_test() {
    assert_eq2!(I::EMPTY.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(I::ENTIRE), I::EMPTY);
    assert_eq2!(I::ENTIRE.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(I::ENTIRE.atan2(I::ENTIRE), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(f64::NEG_INFINITY, 0.0)), n2i(3.141592653589793, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).atan2(n2i(0.0, 0.0)), n2i(1.5707963267948966, 1.5707963267948968));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).atan2(n2i(0.0, 0.0)), n2i(-1.5707963267948968, -1.5707963267948966));
    assert_eq2!(n2i(-2.2250738585072014e-308, 0.0).atan2(n2i(-2.2250738585072014e-308, -2.2250738585072014e-308)), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(1.0, 1.0).atan2(n2i(-1.0, -1.0)), n2i(2.356194490192345, 2.3561944901923453));
    assert_eq2!(n2i(1.0, 1.0).atan2(n2i(1.0, 1.0)), n2i(0.7853981633974483, 0.7853981633974484));
    assert_eq2!(n2i(-1.0, -1.0).atan2(n2i(1.0, 1.0)), n2i(-0.7853981633974484, -0.7853981633974483));
    assert_eq2!(n2i(-1.0, -1.0).atan2(n2i(-1.0, -1.0)), n2i(-2.3561944901923453, -2.356194490192345));
    assert_eq2!(n2i(-2.2250738585072014e-308, 2.2250738585072014e-308).atan2(n2i(-2.2250738585072014e-308, -2.2250738585072014e-308)), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(-2.2250738585072014e-308, 2.2250738585072014e-308).atan2(n2i(2.2250738585072014e-308, 2.2250738585072014e-308)), n2i(-0.7853981633974484, 0.7853981633974484));
    assert_eq2!(n2i(-2.2250738585072014e-308, -2.2250738585072014e-308).atan2(n2i(-2.2250738585072014e-308, 2.2250738585072014e-308)), n2i(-2.3561944901923453, -0.7853981633974483));
    assert_eq2!(n2i(2.2250738585072014e-308, 2.2250738585072014e-308).atan2(n2i(-2.2250738585072014e-308, 2.2250738585072014e-308)), n2i(0.7853981633974483, 2.3561944901923453));
    assert_eq2!(n2i(-2.0, 2.0).atan2(n2i(-3.0, -1.0)), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 2.0).atan2(n2i(-3.0, -1.0)), n2i(2.0344439357957027, 3.1415926535897936));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-3.0, -1.0)), n2i(1.8925468811915387, 2.8198420991931514));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-2.0, 0.0)), n2i(1.5707963267948966, 2.6779450445889874));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(-2.0, 2.0)), n2i(0.4636476090008061, 2.6779450445889874));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(0.0, 2.0)), n2i(0.4636476090008061, 1.5707963267948968));
    assert_eq2!(n2i(1.0, 3.0).atan2(n2i(1.0, 3.0)), n2i(0.3217505543966422, 1.2490457723982544));
    assert_eq2!(n2i(0.0, 2.0).atan2(n2i(1.0, 3.0)), n2i(0.0, 1.1071487177940906));
    assert_eq2!(n2i(-2.0, 2.0).atan2(n2i(1.0, 3.0)), n2i(-1.1071487177940906, 1.1071487177940906));
    assert_eq2!(n2i(-2.0, 0.0).atan2(n2i(1.0, 3.0)), n2i(-1.1071487177940906, 0.0));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(1.0, 3.0)), n2i(-1.2490457723982544, -0.3217505543966422));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(0.0, 2.0)), n2i(-1.5707963267948968, -0.4636476090008061));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-2.0, 2.0)), n2i(-2.6779450445889874, -0.4636476090008061));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-2.0, 0.0)), n2i(-2.6779450445889874, -1.5707963267948966));
    assert_eq2!(n2i(-3.0, -1.0).atan2(n2i(-3.0, -1.0)), n2i(-2.8198420991931514, -1.8925468811915387));
    assert_eq2!(n2i(-2.0, 0.0).atan2(n2i(-3.0, -1.0)), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(-5.0, 0.0).atan2(n2i(-5.0, 0.0)), n2i(-3.1415926535897936, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 5.0).atan2(n2i(-5.0, 0.0)), n2i(1.5707963267948966, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 5.0).atan2(n2i(0.0, 5.0)), n2i(0.0, 1.5707963267948968));
    assert_eq2!(n2i(-5.0, 0.0).atan2(n2i(0.0, 5.0)), n2i(-1.5707963267948968, 0.0));
}
