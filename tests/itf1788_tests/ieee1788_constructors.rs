/*
 *
 * Test cases for interval constructors from IEEE Std 1788-2015
 *
 * Copyright 2016 Oliver Heimlich (oheim@posteo.de)
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

// According to the examples in Section 7.4.2, unbounded intervals can be constructed with non-common inputs.
#[test]
fn ieee1788_a() {
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY), I::ENTIRE);
}

// Examples from Sections 9.7.1 and 9.8
#[cfg(feature = "gmp")]
#[test]
fn ieee1788_b() {
    assert_eq2!(t2i("[1.2345]"), n2i(1.2345, 1.2345000000000002));
    assert_eq2!(t2i("[1,+infinity]"), n2i(1.0, f64::INFINITY));
    assert_eq2!(t2di("[1,1e3]_com"), nd2di(1.0, 1000.0, D::Com));
    assert_eq2!(t2di("[1,1E3]_COM"), nd2di(1.0, 1000.0, D::Com));
}

// Examples from Table 9.4
#[cfg(feature = "gmp")]
#[test]
fn ieee1788_c() {
    assert_eq2!(t2i("[1.e-3, 1.1e-3]"), n2i(0.0009999999999999998, 0.0011));
    assert_eq2!(t2i("[-0x1.3p-1, 2/3]"), n2i(-0.59375, 0.6666666666666667));
    assert_eq2!(t2i("[3.56]"), n2i(3.5599999999999996, 3.56));
    assert_eq2!(t2i("3.56?1"), n2i(3.55, 3.5700000000000003));
    assert_eq2!(t2i("3.56?1e2"), n2i(355.0, 357.0));
    assert_eq2!(t2i("3.560?2"), n2i(3.558, 3.5620000000000003));
    assert_eq2!(t2i("3.56?"), n2i(3.5549999999999997, 3.5650000000000004));
    assert_eq2!(t2i("3.560?2u"), n2i(3.5599999999999996, 3.5620000000000003));
    assert_eq2!(t2i("-10?"), n2i(-10.5, -9.5));
    assert_eq2!(t2i("-10?u"), n2i(-10.0, -9.5));
    assert_eq2!(t2i("-10?12"), n2i(-22.0, 2.0));
}

// Examples from Section 10.5.1
#[cfg(feature = "gmp")]
#[test]
fn ieee1788_d() {
    assert_eq2!(t2i("[1.234e5,Inf]"), n2i(123400.0, f64::INFINITY));
    assert_eq2!(t2i("3.1416?1"), n2i(3.1414999999999997, 3.1417));
    assert_eq2!(t2i("[Empty]"), I::EMPTY);
}

// Example from Section 11.3
#[test]
fn ieee1788_e() {
    assert_eq2!(n2di(2.0, 1.0), DI::NAI);
}

// Examples from Table 12.1
#[cfg(feature = "gmp")]
#[test]
fn ieee1788_e2() {
    assert_eq2!(t2di("[ ]"), DI::EMPTY);
    assert_eq2!(t2di("[entire]"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq2!(t2di("[1.e-3, 1.1e-3]"), nd2di(0.0009999999999999998, 0.0011, D::Com));
    assert_eq2!(t2di("[-Inf, 2/3]"), nd2di(f64::NEG_INFINITY, 0.6666666666666667, D::Dac));
    assert_eq2!(t2di("[0x1.3p-1,]"), nd2di(0.59375, f64::INFINITY, D::Dac));
    assert_eq2!(t2di("[,]"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq2!(t2di("3.56?1"), nd2di(3.55, 3.5700000000000003, D::Com));
    assert_eq2!(t2di("3.56?1e2"), nd2di(355.0, 357.0, D::Com));
    assert_eq2!(t2di("3.560?2"), nd2di(3.558, 3.5620000000000003, D::Com));
    assert_eq2!(t2di("3.56?"), nd2di(3.5549999999999997, 3.5650000000000004, D::Com));
    assert_eq2!(t2di("3.560?2u"), nd2di(3.5599999999999996, 3.5620000000000003, D::Com));
    assert_eq2!(t2di("-10?"), nd2di(-10.5, -9.5, D::Com));
    assert_eq2!(t2di("-10?u"), nd2di(-10.0, -9.5, D::Com));
    assert_eq2!(t2di("-10?12"), nd2di(-22.0, 2.0, D::Com));
    assert_eq2!(t2di("-10??u"), nd2di(-10.0, f64::INFINITY, D::Dac));
    assert_eq2!(t2di("-10??"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq2!(t2di("[nai]"), DI::NAI);
    assert_eq2!(t2di("3.56?1_def"), nd2di(3.55, 3.5700000000000003, D::Def));
}

// Examples from Section 12.11.3
#[cfg(feature = "gmp")]
#[test]
fn ieee1788_f() {
    assert_eq2!(t2i("[]"), I::EMPTY);
    assert_eq2!(t2i("[empty]"), I::EMPTY);
    assert_eq2!(t2i("[ empty ]"), I::EMPTY);
    assert_eq2!(t2i("[,]"), I::ENTIRE);
    assert_eq2!(t2i("[ entire ]"), I::ENTIRE);
}
