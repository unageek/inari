/*
 *
 * Unit tests from libieeep1788 for interval numeric operations
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
type I = inari::Interval;

#[test]
fn minimal_inf_test() {
    assert_eq!(I::empty().inf(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).inf(), f64::NEG_INFINITY);
    assert_eq!(n2i(1.0, 2.0).inf(), 1.0);
    assert_eq!(n2i(-3.0, -2.0).inf(), -3.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).inf(), f64::NEG_INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).inf(), f64::NEG_INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).inf(), f64::NEG_INFINITY);
    assert_eq!(n2i(-2.0, f64::INFINITY).inf(), -2.0);
    assert_eq!(n2i(0.0, f64::INFINITY).inf(), -0.0);
    assert_eq!(n2i(-0.0, f64::INFINITY).inf(), -0.0);
    assert_eq!(n2i(-0.0, 0.0).inf(), -0.0);
    assert_eq!(n2i(0.0, -0.0).inf(), -0.0);
    assert_eq!(n2i(0.0, 0.0).inf(), -0.0);
    assert_eq!(n2i(-0.0, -0.0).inf(), -0.0);
}

#[test]
fn minimal_sup_test() {
    assert_eq!(I::empty().sup(), f64::NEG_INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq!(n2i(1.0, 2.0).sup(), 2.0);
    assert_eq!(n2i(-3.0, -2.0).sup(), -2.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).sup(), 2.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sup(), 0.0);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).sup(), 0.0);
    assert_eq!(n2i(-2.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq!(n2i(0.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq!(n2i(-0.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq!(n2i(-0.0, 0.0).sup(), 0.0);
    assert_eq!(n2i(0.0, -0.0).sup(), 0.0);
    assert_eq!(n2i(0.0, 0.0).sup(), 0.0);
    assert_eq!(n2i(-0.0, -0.0).sup(), 0.0);
}

#[test]
fn minimal_mid_test() {
    assert!(I::empty().mid().is_nan());
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).mid(), 0.0);
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).mid(), 0.0);
    assert_eq!(n2i(0.0, 2.0).mid(), 1.0);
    assert_eq!(n2i(2.0, 2.0).mid(), 2.0);
    assert_eq!(n2i(-2.0, 2.0).mid(), 0.0);
    assert_eq!(n2i(0.0, f64::INFINITY).mid(), hexf64!("0x1.fffffffffffffp+1023"));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.2).mid(), hexf64!("-0x1.fffffffffffffp+1023"));
    assert_eq!(n2i(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022")).mid(), 0.0);
    assert_eq!(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022")).mid(), 0.0);
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1023")).mid(), hexf64!("0x1.7ffffffffffffp+1023"));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000003p-1022")).mid(), hexf64!("0x0.0000000000002p-1022"));
}

#[test]
fn minimal_rad_test() {
    assert_eq!(n2i(0.0, 2.0).rad(), 1.0);
    assert_eq!(n2i(2.0, 2.0).rad(), 0.0);
    assert!(I::empty().rad().is_nan());
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq!(n2i(0.0, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 1.2).rad(), f64::INFINITY);
    assert_eq!(n2i(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022")).rad(), hexf64!("0x0.0000000000002p-1022"));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022")).rad(), hexf64!("0x0.0000000000001p-1022"));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000003p+0")).rad(), hexf64!("0x1.0000000000000p-51"));
}

#[test]
fn minimal_wid_test() {
    assert_eq!(n2i(2.0, 2.0).wid(), 0.0);
    assert_eq!(n2i(1.0, 2.0).wid(), 1.0);
    assert_eq!(n2i(1.0, f64::INFINITY).wid(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).wid(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).wid(), f64::INFINITY);
    assert!(I::empty().wid().is_nan());
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")).wid(), hexf64!("0x1.0000000000000p-52"));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000001p-1022")).wid(), hexf64!("0x0.0000000000001p-1022"));
}

#[test]
fn minimal_mag_test() {
    assert_eq!(n2i(1.0, 2.0).mag(), 2.0);
    assert_eq!(n2i(-4.0, 2.0).mag(), 4.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).mag(), f64::INFINITY);
    assert_eq!(n2i(1.0, f64::INFINITY).mag(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).mag(), f64::INFINITY);
    assert!(I::empty().mag().is_nan());
    assert_eq!(n2i(-0.0, 0.0).mag(), 0.0);
    assert_eq!(n2i(-0.0, -0.0).mag(), 0.0);
}

#[test]
fn minimal_mig_test() {
    assert_eq!(n2i(1.0, 2.0).mig(), 1.0);
    assert_eq!(n2i(-4.0, 2.0).mig(), 0.0);
    assert_eq!(n2i(-4.0, -2.0).mig(), 2.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).mig(), 0.0);
    assert_eq!(n2i(f64::NEG_INFINITY, -2.0).mig(), 2.0);
    assert_eq!(n2i(-1.0, f64::INFINITY).mig(), 0.0);
    assert_eq!(n2i(1.0, f64::INFINITY).mig(), 1.0);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).mig(), 0.0);
    assert!(I::empty().mig().is_nan());
    assert_eq!(n2i(-0.0, 0.0).mig(), 0.0);
    assert_eq!(n2i(-0.0, -0.0).mig(), 0.0);
}
