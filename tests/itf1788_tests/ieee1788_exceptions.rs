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
#![rustfmt::skip]
#![allow(unused_attributes, unused_imports)]

//Test library imports

//Arithmetic library imports

//Preamble
use crate::util::*;
use hexf::*;
type D = inari::Decoration;
type DI = inari::DecoratedInterval;
type I = inari::Interval;

#[test]
fn exceptions() {
    assert_eq!(t2i("[+infinity]"), I::EMPTY);
    assert_eq!(n2i(f64::INFINITY, f64::NEG_INFINITY), I::EMPTY);
    assert_eq!(interval_part(DI::NAI), I::EMPTY);
    assert_eq!(t2i("[1.0000000000000001, 1.0000000000000002]"), n2i(1.0, hexf64!("0x1.0000000000001p+0")));
}
