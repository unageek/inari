/*
 *
 * Unit tests from libieeep1788 for elementary interval functions
 * (Original author: Marco Nehmeier)
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 2013-2015 Marco Nehmeier (nehmeier@informatik.uni-wuerzburg.de)
 * Copyright 2015-2017 Oliver Heimlich (oheim@posteo.de)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
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
fn minimal_neg_test() {
    assert_eq!(-n2i(1.0, 2.0), n2i(-2.0, -1.0));
    assert_eq!(-I::empty(), I::empty());
    assert_eq!(-I::entire(), I::entire());
    assert_eq!(-n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(-n2i(f64::NEG_INFINITY, 1.0), n2i(-1.0, f64::INFINITY));
    assert_eq!(-n2i(0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq!(-n2i(-0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq!(-n2i(-2.0, 0.0), n2i(0.0, 2.0));
    assert_eq!(-n2i(-2.0, -0.0), n2i(0.0, 2.0));
    assert_eq!(-n2i(0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(-n2i(-0.0, -0.0), n2i(0.0, 0.0));
}

#[test]
fn minimal_neg_dec_test() {
    assert!((-DI::nai()).is_nai());
    assert_eq!(-DI::empty(), DI::empty());
    assert_eq!(-nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(-nd2di(1.0, 2.0, D::Com), nd2di(-2.0, -1.0, D::Com));
}

#[test]
fn minimal_add_test() {
    assert_eq!(I::empty() + I::empty(), I::empty());
    assert_eq!(n2i(-1.0, 1.0) + I::empty(), I::empty());
    assert_eq!(I::empty() + n2i(-1.0, 1.0), I::empty());
    assert_eq!(I::empty() + I::entire(), I::empty());
    assert_eq!(I::entire() + I::empty(), I::empty());
    assert_eq!(I::entire() + n2i(f64::NEG_INFINITY, 1.0), I::entire());
    assert_eq!(I::entire() + n2i(-1.0, 1.0), I::entire());
    assert_eq!(I::entire() + n2i(-1.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() + I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0) + I::entire(), I::entire());
    assert_eq!(n2i(-1.0, 1.0) + I::entire(), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) + I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(f64::NEG_INFINITY, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(1.0, 2.0) + n2i(f64::NEG_INFINITY, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(1.0, 2.0) + n2i(3.0, 4.0), n2i(4.0, 6.0));
    assert_eq!(n2i(1.0, 2.0) + n2i(3.0, f64::INFINITY), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(f64::NEG_INFINITY, 4.0), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(3.0, 4.0), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(3.0, f64::INFINITY), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(3.0, 4.0), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) + n2i(-3.0, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) + n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), I::entire());
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(0.0, 0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(-0.0, -0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, 0.0) + n2i(-3.0, 4.0), n2i(-3.0, 4.0));
    assert_eq!(n2i(-0.0, -0.0) + n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("0x1.0ccccccccccc4p+1"), hexf64!("0x1.0ccccccccccc5p+1")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("0x1.e666666666656p+0"), hexf64!("0x1.e666666666657p+0")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.e666666666657p+0"), hexf64!("0x1.0ccccccccccc5p+1")));
}

#[test]
fn minimal_add_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Com), nd2di(6.0, 9.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Def), nd2di(6.0, 9.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(6.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) + nd2di(-0.1, 5.0, D::Com), nd2di(f64::NEG_INFINITY, 7.0, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) + DI::empty(), DI::empty());
    assert!((DI::nai() + nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_sub_test() {
    assert_eq!(I::empty() - I::empty(), I::empty());
    assert_eq!(n2i(-1.0, 1.0) - I::empty(), I::empty());
    assert_eq!(I::empty() - n2i(-1.0, 1.0), I::empty());
    assert_eq!(I::empty() - I::entire(), I::empty());
    assert_eq!(I::entire() - I::empty(), I::empty());
    assert_eq!(I::entire() - n2i(f64::NEG_INFINITY, 1.0), I::entire());
    assert_eq!(I::entire() - n2i(-1.0, 1.0), I::entire());
    assert_eq!(I::entire() - n2i(-1.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() - I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0) - I::entire(), I::entire());
    assert_eq!(n2i(-1.0, 1.0) - I::entire(), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) - I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(f64::NEG_INFINITY, 4.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, 2.0) - n2i(f64::NEG_INFINITY, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.0) - n2i(3.0, 4.0), n2i(-3.0, -1.0));
    assert_eq!(n2i(1.0, 2.0) - n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(f64::NEG_INFINITY, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(3.0, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(-3.0, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) - n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) - n2i(hexf64!("-0x1.fffffffffffffp+1023"), 4.0), I::entire());
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(0.0, 0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(-0.0, -0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, 0.0) - n2i(-3.0, 4.0), n2i(-4.0, 3.0));
    assert_eq!(n2i(-0.0, -0.0) - n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), 3.0));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("0x1.e666666666656p+0"), hexf64!("0x1.e666666666657p+0")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("0x1.0ccccccccccc4p+1"), hexf64!("0x1.0ccccccccccc5p+1")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.0ccccccccccc5p+1"), hexf64!("0x1.e666666666657p+0")));
}

#[test]
fn minimal_sub_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Com), nd2di(-6.0, -3.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Def), nd2di(-6.0, -3.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.0, D::Com) - nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(f64::NEG_INFINITY, -3.0, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) - nd2di(-1.0, 5.0, D::Com), nd2di(f64::NEG_INFINITY, 3.0, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) - DI::empty(), DI::empty());
    assert!((DI::nai() - nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_mul_test() {
    assert_eq!(I::empty() * I::empty(), I::empty());
    assert_eq!(n2i(-1.0, 1.0) * I::empty(), I::empty());
    assert_eq!(I::empty() * n2i(-1.0, 1.0), I::empty());
    assert_eq!(I::empty() * I::entire(), I::empty());
    assert_eq!(I::entire() * I::empty(), I::empty());
    assert_eq!(n2i(0.0, 0.0) * I::empty(), I::empty());
    assert_eq!(I::empty() * n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-0.0, -0.0) * I::empty(), I::empty());
    assert_eq!(I::empty() * n2i(-0.0, -0.0), I::empty());
    assert_eq!(I::entire() * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(I::entire() * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(I::entire() * n2i(-5.0, -1.0), I::entire());
    assert_eq!(I::entire() * n2i(-5.0, 3.0), I::entire());
    assert_eq!(I::entire() * n2i(1.0, 3.0), I::entire());
    assert_eq!(I::entire() * n2i(f64::NEG_INFINITY, -1.0), I::entire());
    assert_eq!(I::entire() * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(I::entire() * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() * n2i(1.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() * I::entire(), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(1.0, 3.0), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) * I::entire(), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(1.0, 3.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY) * I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, -1.0), n2i(-15.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 9.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, -1.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, -1.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * I::entire(), I::entire());
    assert_eq!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * I::entire(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * I::entire(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, -1.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(1.0, 3.0), n2i(1.0, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, f64::INFINITY), n2i(-25.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0) * n2i(1.0, f64::INFINITY), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0) * I::entire(), I::entire());
    assert_eq!(n2i(-1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, 5.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq!(n2i(-10.0, 2.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-1.0, 10.0), n2i(-10.0, 50.0));
    assert_eq!(n2i(-2.0, 2.0) * n2i(-5.0, 3.0), n2i(-10.0, 10.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0) * n2i(1.0, 3.0), n2i(-3.0, 15.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0), I::entire());
    assert_eq!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-1.0, 5.0) * n2i(1.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-1.0, 5.0) * I::entire(), I::entire());
    assert_eq!(n2i(-10.0, -5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, -1.0), n2i(5.0, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(1.0, 3.0), n2i(-30.0, -5.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(-30.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-10.0, -5.0) * I::entire(), I::entire());
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), f64::INFINITY), n2i(hexf64!("-0x1.fffffffffffe1p+1"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("-0x1.fffffffffffe1p+1"), hexf64!("0x1.999999999998ep-3")));
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.999999999998ep-3"), hexf64!("0x1.999999999998ep-3")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.999999999999ap-4")) * n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")), n2i(hexf64!("-0x1.fffffffffffe1p+1"), hexf64!("-0x1.47ae147ae147bp-7")));
}

#[test]
fn minimal_mul_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Com), nd2di(5.0, 14.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Def), nd2di(5.0, 14.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(5.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) * nd2di(-1.0, 5.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) * DI::empty(), DI::empty());
    assert!((DI::nai() * nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_div_test() {
    assert_eq!(I::empty() / I::empty(), I::empty());
    assert_eq!(n2i(-1.0, 1.0) / I::empty(), I::empty());
    assert_eq!(I::empty() / n2i(-1.0, 1.0), I::empty());
    assert_eq!(I::empty() / n2i(0.1, 1.0), I::empty());
    assert_eq!(I::empty() / n2i(-1.0, -0.1), I::empty());
    assert_eq!(I::empty() / I::entire(), I::empty());
    assert_eq!(I::entire() / I::empty(), I::empty());
    assert_eq!(n2i(0.0, 0.0) / I::empty(), I::empty());
    assert_eq!(I::empty() / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-0.0, -0.0) / I::empty(), I::empty());
    assert_eq!(I::empty() / n2i(-0.0, -0.0), I::empty());
    assert_eq!(I::entire() / n2i(-5.0, -3.0), I::entire());
    assert_eq!(I::entire() / n2i(3.0, 5.0), I::entire());
    assert_eq!(I::entire() / n2i(f64::NEG_INFINITY, -3.0), I::entire());
    assert_eq!(I::entire() / n2i(3.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() / n2i(0.0, 0.0), I::empty());
    assert_eq!(I::entire() / n2i(-0.0, -0.0), I::empty());
    assert_eq!(I::entire() / n2i(-3.0, 0.0), I::entire());
    assert_eq!(I::entire() / n2i(-3.0, -0.0), I::entire());
    assert_eq!(I::entire() / n2i(-3.0, 3.0), I::entire());
    assert_eq!(I::entire() / n2i(0.0, 3.0), I::entire());
    assert_eq!(I::entire() / n2i(f64::NEG_INFINITY, 0.0), I::entire());
    assert_eq!(I::entire() / n2i(-0.0, 3.0), I::entire());
    assert_eq!(I::entire() / n2i(f64::NEG_INFINITY, -0.0), I::entire());
    assert_eq!(I::entire() / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(I::entire() / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() / n2i(0.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() / n2i(-0.0, f64::INFINITY), I::entire());
    assert_eq!(I::entire() / I::entire(), I::entire());
    assert_eq!(n2i(-30.0, -15.0) / n2i(-5.0, -3.0), n2i(3.0, 10.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(3.0, 5.0), n2i(-10.0, -3.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, 0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, -0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / I::entire(), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-5.0, -3.0), n2i(-5.0, 10.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(3.0, 5.0), n2i(-10.0, 5.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-5.0, 10.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 5.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, 0.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, -0.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 0.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -0.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, 15.0) / I::entire(), I::entire());
    assert_eq!(n2i(15.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, -3.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(3.0, 5.0), n2i(3.0, 10.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / I::entire(), I::entire());
    assert_eq!(n2i(0.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / I::entire(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / I::entire(), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-5.0, -3.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, -0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-5.0, -3.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 0.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, -0.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 0.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -0.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / I::entire(), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 0.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, -0.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 3.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, 3.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-15.0, f64::INFINITY) / I::entire(), I::entire());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / I::entire(), I::entire());
    assert_eq!(n2i(-30.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / I::entire(), I::entire());
    assert_eq!(n2i(-30.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / I::entire(), I::entire());
    assert_eq!(n2i(0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / I::entire(), I::entire());
    assert_eq!(n2i(-0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-5.0, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / I::entire(), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-5.0, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / I::entire(), I::entire());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / I::entire(), I::entire());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, 0.0), I::empty());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::empty());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::entire());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::entire());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::entire());
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / I::entire(), I::entire());
    assert_eq!(n2i(-2.0, -1.0) / n2i(-10.0, -3.0), n2i(hexf64!("0x1.9999999999999p-4"), hexf64!("0x1.5555555555556p-1")));
    assert_eq!(n2i(-2.0, -1.0) / n2i(0.0, 10.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-2.0, -1.0) / n2i(-0.0, 10.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-1.0, 2.0) / n2i(10.0, f64::INFINITY), n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-3")));
    assert_eq!(n2i(1.0, 3.0) / n2i(f64::NEG_INFINITY, -10.0), n2i(hexf64!("-0x1.3333333333334p-2"), 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0) / n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.5555555555555p-2")));
}

#[test]
fn minimal_div_dec_test() {
    assert_eq!(nd2di(-2.0, -1.0, D::Com) / nd2di(-10.0, -3.0, D::Com), nd2di(hexf64!("0x1.9999999999999p-4"), hexf64!("0x1.5555555555556p-1"), D::Com));
    assert_eq!(nd2di(-200.0, -1.0, D::Com) / nd2di(hexf64!("0x0.0000000000001p-1022"), 10.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Dac));
    assert_eq!(nd2di(-2.0, -1.0, D::Com) / nd2di(0.0, 10.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Trv));
    assert_eq!(nd2di(1.0, 3.0, D::Def) / nd2di(f64::NEG_INFINITY, -10.0, D::Dac), nd2di(hexf64!("-0x1.3333333333334p-2"), 0.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) / DI::empty(), DI::empty());
    assert!((DI::nai() / nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_recip_test() {
    assert_eq!(n2i(-50.0, -10.0).recip(), n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.47ae147ae147ap-6")));
    assert_eq!(n2i(10.0, 50.0).recip(), n2i(hexf64!("0x1.47ae147ae147ap-6"), hexf64!("0x1.999999999999ap-4")));
    assert_eq!(n2i(f64::NEG_INFINITY, -10.0).recip(), n2i(hexf64!("-0x1.999999999999ap-4"), 0.0));
    assert_eq!(n2i(10.0, f64::INFINITY).recip(), n2i(0.0, hexf64!("0x1.999999999999ap-4")));
    assert_eq!(n2i(0.0, 0.0).recip(), I::empty());
    assert_eq!(n2i(-0.0, -0.0).recip(), I::empty());
    assert_eq!(n2i(-10.0, 0.0).recip(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-10.0, -0.0).recip(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-10.0, 10.0).recip(), I::entire());
    assert_eq!(n2i(0.0, 10.0).recip(), n2i(hexf64!("0x1.9999999999999p-4"), f64::INFINITY));
    assert_eq!(n2i(-0.0, 10.0).recip(), n2i(hexf64!("0x1.9999999999999p-4"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 10.0).recip(), I::entire());
    assert_eq!(n2i(-10.0, f64::INFINITY).recip(), I::entire());
    assert_eq!(n2i(0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq!(I::entire().recip(), I::entire());
}

#[test]
fn minimal_recip_dec_test() {
    assert_eq!(nd2di(10.0, 50.0, D::Com).recip(), nd2di(hexf64!("0x1.47ae147ae147ap-6"), hexf64!("0x1.999999999999ap-4"), D::Com));
    assert_eq!(nd2di(f64::NEG_INFINITY, -10.0, D::Dac).recip(), nd2di(hexf64!("-0x1.999999999999ap-4"), 0.0, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x0.0000000000001p-1022"), D::Def).recip(), nd2di(f64::NEG_INFINITY, hexf64!("-0x0.4000000000000p-1022"), D::Def));
    assert_eq!(nd2di(0.0, 0.0, D::Com).recip(), DI::empty());
    assert_eq!(nd2di(-10.0, 0.0, D::Com).recip(), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Trv));
    assert_eq!(nd2di(-10.0, f64::INFINITY, D::Dac).recip(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, f64::INFINITY, D::Dac).recip(), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).recip(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}

#[test]
fn minimal_sqr_test() {
    assert_eq!(I::empty().sqr(), I::empty());
    assert_eq!(I::entire().sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x0.0000000000001p-1022")).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, 3.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(-5.0, 0.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(-5.0, -0.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).sqr(), n2i(hexf64!("0x1.47ae147ae147bp-7"), hexf64!("0x1.47ae147ae147cp-7")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")).sqr(), n2i(0.0, hexf64!("0x1.fffffffffffe1p+1")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.ffffffffffff0p+0")).sqr(), n2i(hexf64!("0x1.fffffffffffe0p+1"), hexf64!("0x1.fffffffffffe1p+1")));
}

#[test]
fn minimal_sqr_dec_test() {
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x0.0000000000001p-1022"), D::Com).sqr(), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(-1.0, 1.0, D::Def).sqr(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-5.0, 3.0, D::Com).sqr(), nd2di(0.0, 25.0, D::Com));
    assert_eq!(nd2di(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4"), D::Com).sqr(), nd2di(hexf64!("0x1.47ae147ae147bp-7"), hexf64!("0x1.47ae147ae147cp-7"), D::Com));
}

#[test]
fn minimal_sqrt_test() {
    assert_eq!(I::empty().sqrt(), I::empty());
    assert_eq!(I::entire().sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x0.0000000000001p-1022")).sqrt(), I::empty());
    assert_eq!(n2i(-1.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(-0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(-5.0, f64::INFINITY).sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).sqrt(), n2i(hexf64!("0x1.43d136248490fp-2"), hexf64!("0x1.43d1362484910p-2")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")).sqrt(), n2i(0.0, hexf64!("0x1.43d1362484910p-2")));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")).sqrt(), n2i(hexf64!("0x1.43d136248490fp-2"), hexf64!("0x1.6a09e667f3bc7p+0")));
}

#[test]
fn minimal_sqrt_dec_test() {
    assert_eq!(nd2di(1.0, 4.0, D::Com).sqrt(), nd2di(1.0, 2.0, D::Com));
    assert_eq!(nd2di(-5.0, 25.0, D::Com).sqrt(), nd2di(0.0, 5.0, D::Trv));
    assert_eq!(nd2di(0.0, 25.0, D::Def).sqrt(), nd2di(0.0, 5.0, D::Def));
    assert_eq!(nd2di(-5.0, f64::INFINITY, D::Dac).sqrt(), nd2di(0.0, f64::INFINITY, D::Trv));
}

#[test]
fn minimal_fma_test() {
    assert_eq!(I::empty().mul_add(I::empty(), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::empty(), I::empty()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-1.0, 1.0), I::empty()), I::empty());
    assert_eq!(I::empty().mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(I::empty(), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::empty(), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::empty(), I::empty()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(I::entire().mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(1.0, 5.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::empty()), I::empty());
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::empty()), I::empty());
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::entire(), I::empty()), I::empty());
    assert_eq!(I::empty().mul_add(I::empty(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::empty(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-1.0, 1.0), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::entire().mul_add(I::empty(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::empty(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::empty(), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), I::empty());
    assert_eq!(I::entire().mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::entire().mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::entire().mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 11.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 12.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::entire(), n2i(f64::NEG_INFINITY, 2.0)), I::entire());
    assert_eq!(I::empty().mul_add(I::empty(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::empty(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-1.0, 1.0), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(I::entire(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::entire().mul_add(I::empty(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::empty(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::empty(), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), I::empty());
    assert_eq!(I::entire().mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(I::entire().mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(I::entire().mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::entire().mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-17.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 11.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::entire(), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::entire(), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-27.0, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-27.0, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-1.0, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-27.0, 7.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-27.0, 17.0));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, 52.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, 2.0)), n2i(-12.0, 52.0));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-12.0, 12.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-5.0, 17.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(3.0, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::entire(), n2i(-2.0, 2.0)), I::entire());
    assert_eq!(I::empty().mul_add(I::empty(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::empty(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-1.0, 1.0), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::empty().mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::entire().mul_add(I::empty(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::empty(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::empty(), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), I::empty());
    assert_eq!(I::entire().mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(I::entire().mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(I::entire().mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::entire().mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-17.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, f64::INFINITY)), n2i(-12.0, f64::INFINITY));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-12.0, f64::INFINITY));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::entire(), n2i(-2.0, f64::INFINITY)), I::entire());
    assert_eq!(I::empty().mul_add(I::empty(), I::entire()), I::empty());
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::empty(), I::entire()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-1.0, 1.0), I::entire()), I::empty());
    assert_eq!(I::empty().mul_add(I::entire(), I::entire()), I::empty());
    assert_eq!(I::entire().mul_add(I::empty(), I::entire()), I::empty());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::empty(), I::entire()), I::empty());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::empty(), I::entire()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(0.0, 0.0), I::entire()), I::empty());
    assert_eq!(I::empty().mul_add(n2i(-0.0, -0.0), I::entire()), I::empty());
    assert_eq!(I::entire().mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(I::entire().mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(0.0, 0.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(1.0, 5.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::entire()), I::entire());
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::entire()), I::entire());
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::entire(), I::entire()), I::entire());
    assert_eq!(n2i(0.1, 0.5).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.4cccccccccccdp+1"), hexf64!("0x1.999999999999ap+0")));
    assert_eq!(n2i(-0.5, 0.2).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), hexf64!("0x1.4cccccccccccdp+1")));
    assert_eq!(n2i(-0.5, -0.1).mul_add(n2i(2.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), hexf64!("-0x1.999999999999ap-4")));
    assert_eq!(n2i(-0.5, -0.1).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), f64::INFINITY));
}

#[test]
fn minimal_fma_dec_test() {
    assert_eq!(nd2di(-0.5, -0.1, D::Com).mul_add(nd2di(f64::NEG_INFINITY, 3.0, D::Dac), nd2di(-0.1, 0.1, D::Com)), nd2di(hexf64!("-0x1.999999999999ap+0"), f64::INFINITY, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Com).mul_add(nd2di(1.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(0.0, 1.0, D::Com)), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Com).mul_add(nd2di(1.0, 2.0, D::Com), nd2di(2.0, 5.0, D::Com)), nd2di(3.0, 9.0, D::Com));
}

#[test]
fn minimal_sign_test() {
    assert_eq!(I::empty().sign(), I::empty());
    assert_eq!(n2i(1.0, 2.0).sign(), n2i(1.0, 1.0));
    assert_eq!(n2i(-1.0, 2.0).sign(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 0.0).sign(), n2i(-1.0, 0.0));
    assert_eq!(n2i(0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, -2.0).sign(), n2i(-1.0, -1.0));
    assert_eq!(n2i(0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(I::entire().sign(), n2i(-1.0, 1.0));
}

#[test]
fn minimal_sign_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com).sign(), nd2di(1.0, 1.0, D::Com));
    assert_eq!(nd2di(-1.0, 2.0, D::Com).sign(), nd2di(-1.0, 1.0, D::Def));
    assert_eq!(nd2di(-1.0, 0.0, D::Com).sign(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(0.0, 2.0, D::Com).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-0.0, 2.0, D::Def).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-5.0, -2.0, D::Trv).sign(), nd2di(-1.0, -1.0, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Dac).sign(), nd2di(0.0, 0.0, D::Dac));
}

#[test]
fn minimal_ceil_test() {
    assert_eq!(I::empty().ceil(), I::empty());
    assert_eq!(I::entire().ceil(), I::entire());
    assert_eq!(n2i(1.1, 2.0).ceil(), n2i(2.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).ceil(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq!(n2i(-1.0, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq!(n2i(0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq!(n2i(-0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).ceil(), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY).ceil(), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).ceil(), n2i(f64::NEG_INFINITY, 3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")).ceil(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")));
}

#[test]
fn minimal_ceil_dec_test() {
    assert_eq!(nd2di(1.1, 2.0, D::Com).ceil(), nd2di(2.0, 2.0, D::Dac));
    assert_eq!(nd2di(-1.1, 2.0, D::Com).ceil(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.1, -0.0, D::Trv).ceil(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq!(nd2di(-1.1, -0.4, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Dac).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq!(nd2di(0.0, 2.2, D::Trv).ceil(), nd2di(0.0, 3.0, D::Trv));
    assert_eq!(nd2di(-0.0, 2.2, D::Def).ceil(), nd2di(0.0, 3.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Trv).ceil(), nd2di(-1.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac).ceil(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Def));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).ceil(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).ceil(), nd2di(f64::NEG_INFINITY, 3.0, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023"), D::Dac).ceil(), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023"), D::Def));
}

#[test]
fn minimal_floor_test() {
    assert_eq!(I::empty().floor(), I::empty());
    assert_eq!(I::entire().floor(), I::entire());
    assert_eq!(n2i(1.1, 2.0).floor(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).floor(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).floor(), n2i(-2.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).floor(), n2i(-2.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).floor(), n2i(-2.0, -1.0));
    assert_eq!(n2i(-1.9, 2.2).floor(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).floor(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).floor(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).floor(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_floor_dec_test() {
    assert_eq!(nd2di(1.1, 2.0, D::Com).floor(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 2.0, D::Def).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).floor(), nd2di(-2.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.2, -1.1, D::Com).floor(), nd2di(-2.0, -2.0, D::Com));
    assert_eq!(nd2di(-1.1, -0.4, D::Def).floor(), nd2di(-2.0, -1.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Trv).floor(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(0.0, 2.2, D::Trv).floor(), nd2di(0.0, 2.0, D::Trv));
    assert_eq!(nd2di(-0.0, 2.2, D::Com).floor(), nd2di(0.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).floor(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).floor(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Com).floor(), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Dac));
}

#[test]
fn minimal_trunc_test() {
    assert_eq!(I::empty().trunc(), I::empty());
    assert_eq!(I::entire().trunc(), I::entire());
    assert_eq!(n2i(1.1, 2.1).trunc(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).trunc(), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).trunc(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_trunc_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).trunc(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(1.1, 1.9, D::Com).trunc(), nd2di(1.0, 1.0, D::Com));
    assert_eq!(nd2di(-1.1, 2.0, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Trv).trunc(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq!(nd2di(-1.1, -0.0, D::Def).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.1, -0.4, D::Com).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Def).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).trunc(), nd2di(-1.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).trunc(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).trunc(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac).trunc(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Def));
}

#[test]
fn minimal_round_ties_to_even_test() {
    assert_eq!(I::empty().round_ties_to_even(), I::empty());
    assert_eq!(I::entire().round_ties_to_even(), I::entire());
    assert_eq!(n2i(1.1, 2.1).round_ties_to_even(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, -0.4).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, 0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq!(n2i(1.5, 2.1).round_ties_to_even(), n2i(2.0, 2.0));
    assert_eq!(n2i(-1.5, 2.0).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.1, -0.5).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).round_ties_to_even(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).round_ties_to_even(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_round_ties_to_even_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).round_ties_to_even(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 2.0, D::Trv).round_ties_to_even(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(-1.6, -1.5, D::Com).round_ties_to_even(), nd2di(-2.0, -2.0, D::Dac));
    assert_eq!(nd2di(-1.6, -1.4, D::Com).round_ties_to_even(), nd2di(-2.0, -1.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).round_ties_to_even(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).round_ties_to_even(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
}

#[test]
fn minimal_round_ties_to_away_test() {
    assert_eq!(I::empty().round_ties_to_away(), I::empty());
    assert_eq!(I::entire().round_ties_to_away(), I::entire());
    assert_eq!(n2i(1.1, 2.1).round_ties_to_away(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).round_ties_to_away(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).round_ties_to_away(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).round_ties_to_away(), n2i(-1.0, -0.0));
    assert_eq!(n2i(-1.1, -0.4).round_ties_to_away(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).round_ties_to_away(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).round_ties_to_away(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.5, 2.1).round_ties_to_away(), n2i(1.0, 2.0));
    assert_eq!(n2i(-2.5, 2.0).round_ties_to_away(), n2i(-3.0, 2.0));
    assert_eq!(n2i(-1.1, -0.5).round_ties_to_away(), n2i(-1.0, -1.0));
    assert_eq!(n2i(-1.9, 2.5).round_ties_to_away(), n2i(-2.0, 3.0));
    assert_eq!(n2i(-1.5, 2.5).round_ties_to_away(), n2i(-2.0, 3.0));
    assert_eq!(n2i(0.0, 2.5).round_ties_to_away(), n2i(0.0, 3.0));
    assert_eq!(n2i(-0.0, 2.5).round_ties_to_away(), n2i(0.0, 3.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).round_ties_to_away(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).round_ties_to_away(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_round_ties_to_away_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).round_ties_to_away(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).round_ties_to_away(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(1.9, 2.2, D::Com).round_ties_to_away(), nd2di(2.0, 2.0, D::Com));
    assert_eq!(nd2di(-1.0, 2.2, D::Trv).round_ties_to_away(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(2.5, 2.6, D::Com).round_ties_to_away(), nd2di(3.0, 3.0, D::Dac));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).round_ties_to_away(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Def).round_ties_to_away(), nd2di(f64::NEG_INFINITY, 2.0, D::Def));
}

#[test]
fn minimal_abs_test() {
    assert_eq!(I::empty().abs(), I::empty());
    assert_eq!(I::entire().abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.1, 2.1).abs(), n2i(1.1, 2.1));
    assert_eq!(n2i(-1.1, 2.0).abs(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).abs(), n2i(0.0, 1.1));
    assert_eq!(n2i(-1.1, -0.0).abs(), n2i(0.0, 1.1));
    assert_eq!(n2i(-1.1, -0.4).abs(), n2i(0.4, 1.1));
    assert_eq!(n2i(-1.9, 0.2).abs(), n2i(0.0, 1.9));
    assert_eq!(n2i(0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq!(n2i(-0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq!(n2i(-1.5, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -2.2).abs(), n2i(2.2, f64::INFINITY));
}

#[test]
fn minimal_abs_dec_test() {
    assert_eq!(nd2di(-1.1, 2.0, D::Com).abs(), nd2di(0.0, 2.0, D::Com));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).abs(), nd2di(0.0, 1.1, D::Dac));
    assert_eq!(nd2di(-1.1, -0.0, D::Def).abs(), nd2di(0.0, 1.1, D::Def));
    assert_eq!(nd2di(-1.1, -0.4, D::Trv).abs(), nd2di(0.4, 1.1, D::Trv));
    assert_eq!(nd2di(-1.9, 0.2, D::Dac).abs(), nd2di(0.0, 1.9, D::Dac));
    assert_eq!(nd2di(0.0, 0.2, D::Def).abs(), nd2di(0.0, 0.2, D::Def));
    assert_eq!(nd2di(-0.0, 0.2, D::Com).abs(), nd2di(0.0, 0.2, D::Com));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).abs(), nd2di(0.0, f64::INFINITY, D::Dac));
}

#[test]
fn minimal_min_test() {
    assert_eq!(I::empty().min(n2i(1.0, 2.0)), I::empty());
    assert_eq!(n2i(1.0, 2.0).min(I::empty()), I::empty());
    assert_eq!(I::empty().min(I::empty()), I::empty());
    assert_eq!(I::entire().min(n2i(1.0, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 2.0).min(I::entire()), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::entire().min(I::entire()), I::entire());
    assert_eq!(I::empty().min(I::entire()), I::empty());
    assert_eq!(n2i(1.0, 5.0).min(n2i(2.0, 4.0)), n2i(1.0, 4.0));
    assert_eq!(n2i(0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(1.0, 5.0).min(n2i(2.0, 8.0)), n2i(1.0, 5.0));
    assert_eq!(n2i(1.0, 5.0).min(I::entire()), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-7.0, -5.0).min(n2i(2.0, 4.0)), n2i(-7.0, -5.0));
    assert_eq!(n2i(-7.0, 0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
    assert_eq!(n2i(-7.0, -0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
}

#[test]
fn minimal_min_dec_test() {
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).min(nd2di(1.0, 2.0, D::Com)), nd2di(f64::NEG_INFINITY, 2.0, D::Dac));
    assert_eq!(nd2di(-7.0, -5.0, D::Trv).min(nd2di(2.0, 4.0, D::Def)), nd2di(-7.0, -5.0, D::Trv));
    assert_eq!(nd2di(-7.0, 0.0, D::Dac).min(nd2di(2.0, 4.0, D::Def)), nd2di(-7.0, 0.0, D::Def));
    assert_eq!(nd2di(-7.0, -0.0, D::Com).min(nd2di(2.0, 4.0, D::Com)), nd2di(-7.0, 0.0, D::Com));
}

#[test]
fn minimal_max_test() {
    assert_eq!(I::empty().max(n2i(1.0, 2.0)), I::empty());
    assert_eq!(n2i(1.0, 2.0).max(I::empty()), I::empty());
    assert_eq!(I::empty().max(I::empty()), I::empty());
    assert_eq!(I::entire().max(n2i(1.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.0).max(I::entire()), n2i(1.0, f64::INFINITY));
    assert_eq!(I::entire().max(I::entire()), I::entire());
    assert_eq!(I::empty().max(I::entire()), I::empty());
    assert_eq!(n2i(1.0, 5.0).max(n2i(2.0, 4.0)), n2i(2.0, 5.0));
    assert_eq!(n2i(1.0, 5.0).max(n2i(2.0, 8.0)), n2i(2.0, 8.0));
    assert_eq!(n2i(-1.0, 5.0).max(I::entire()), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(2.0, 4.0)), n2i(2.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-2.0, 0.0)), n2i(-2.0, 0.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-2.0, -0.0)), n2i(-2.0, 0.0));
}

#[test]
fn minimal_max_dec_test() {
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).max(nd2di(1.0, 2.0, D::Com)), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(-7.0, -5.0, D::Trv).max(nd2di(2.0, 4.0, D::Def)), nd2di(2.0, 4.0, D::Trv));
    assert_eq!(nd2di(-7.0, 5.0, D::Dac).max(nd2di(2.0, 4.0, D::Def)), nd2di(2.0, 5.0, D::Def));
    assert_eq!(nd2di(3.0, 3.5, D::Com).max(nd2di(2.0, 4.0, D::Com)), nd2di(3.0, 4.0, D::Com));
}
