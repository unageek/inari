/*
 *
 * Test cases for interval exceptions from IEEE Std 1788-2015
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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

#[cfg(feature = "gmp")]
#[test]
fn exceptions() {
    assert_eq2!(t2i("[+infinity]"), I::EMPTY);
    assert_eq2!(n2i(f64::INFINITY, f64::NEG_INFINITY), I::EMPTY);
    assert_eq2!(DI::NAI.interval().unwrap_or(I::EMPTY), I::EMPTY);
    assert_eq2!(
        t2i("[1.0000000000000001, 1.0000000000000002]"),
        n2i(1.0, 1.0000000000000002)
    );
}
