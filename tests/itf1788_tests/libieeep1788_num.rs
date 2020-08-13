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
use crate::{*, util::*};
use hexf::*;
type D = inari::Decoration;
type DI = inari::DecoratedInterval;
type I = inari::Interval;

#[test]
fn minimal_inf_test() {
    assert_eq2!(I::EMPTY.inf(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).inf(), f64::NEG_INFINITY);
    assert_eq2!(n2i(1.0, 2.0).inf(), 1.0);
    assert_eq2!(n2i(-3.0, -2.0).inf(), -3.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).inf(), f64::NEG_INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).inf(), f64::NEG_INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).inf(), f64::NEG_INFINITY);
    assert_eq2!(n2i(-2.0, f64::INFINITY).inf(), -2.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).inf(), -0.0);
    assert_eq2!(n2i(-0.0, f64::INFINITY).inf(), -0.0);
    assert_eq2!(n2i(-0.0, 0.0).inf(), -0.0);
    assert_eq2!(n2i(0.0, -0.0).inf(), -0.0);
    assert_eq2!(n2i(0.0, 0.0).inf(), -0.0);
    assert_eq2!(n2i(-0.0, -0.0).inf(), -0.0);
}

#[test]
fn minimal_inf_dec_test() {
    assert!(DI::NAI.inf().is_nan());
    assert_eq2!(DI::EMPTY.inf(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).inf(), f64::NEG_INFINITY);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).inf(), 1.0);
    assert_eq2!(nd2di(-3.0, -2.0, D::Trv).inf(), -3.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Dac).inf(), f64::NEG_INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).inf(), f64::NEG_INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).inf(), f64::NEG_INFINITY);
    assert_eq2!(nd2di(-2.0, f64::INFINITY, D::Trv).inf(), -2.0);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).inf(), -0.0);
    assert_eq2!(nd2di(-0.0, f64::INFINITY, D::Trv).inf(), -0.0);
    assert_eq2!(nd2di(-0.0, 0.0, D::Dac).inf(), -0.0);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).inf(), -0.0);
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).inf(), -0.0);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).inf(), -0.0);
}

#[test]
fn minimal_sup_test() {
    assert_eq2!(I::EMPTY.sup(), f64::NEG_INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq2!(n2i(1.0, 2.0).sup(), 2.0);
    assert_eq2!(n2i(-3.0, -2.0).sup(), -2.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).sup(), 2.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).sup(), 0.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).sup(), 0.0);
    assert_eq2!(n2i(-2.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq2!(n2i(0.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq2!(n2i(-0.0, f64::INFINITY).sup(), f64::INFINITY);
    assert_eq2!(n2i(-0.0, 0.0).sup(), 0.0);
    assert_eq2!(n2i(0.0, -0.0).sup(), 0.0);
    assert_eq2!(n2i(0.0, 0.0).sup(), 0.0);
    assert_eq2!(n2i(-0.0, -0.0).sup(), 0.0);
}

#[test]
fn minimal_sup_dec_test() {
    assert!(DI::NAI.sup().is_nan());
    assert_eq2!(DI::EMPTY.sup(), f64::NEG_INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).sup(), f64::INFINITY);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).sup(), 2.0);
    assert_eq2!(nd2di(-3.0, -2.0, D::Trv).sup(), -2.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Dac).sup(), 2.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).sup(), 0.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).sup(), 0.0);
    assert_eq2!(nd2di(-2.0, f64::INFINITY, D::Trv).sup(), f64::INFINITY);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).sup(), f64::INFINITY);
    assert_eq2!(nd2di(-0.0, f64::INFINITY, D::Trv).sup(), f64::INFINITY);
    assert_eq2!(nd2di(-0.0, 0.0, D::Dac).sup(), 0.0);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).sup(), 0.0);
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).sup(), 0.0);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).sup(), 0.0);
}

#[test]
fn minimal_mid_test() {
    assert!(I::EMPTY.mid().is_nan());
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).mid(), 0.0);
    assert_eq2!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).mid(), 0.0);
    assert_eq2!(n2i(0.0, 2.0).mid(), 1.0);
    assert_eq2!(n2i(2.0, 2.0).mid(), 2.0);
    assert_eq2!(n2i(-2.0, 2.0).mid(), 0.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).mid(), hexf64!("0x1.fffffffffffffp+1023"));
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.2).mid(), hexf64!("-0x1.fffffffffffffp+1023"));
    assert_eq2!(n2i(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022")).mid(), 0.0);
    assert_eq2!(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022")).mid(), 0.0);
    assert_eq2!(n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1023")).mid(), hexf64!("0x1.7ffffffffffffp+1023"));
    assert_eq2!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000003p-1022")).mid(), hexf64!("0x0.0000000000002p-1022"));
}

#[test]
fn minimal_mid_dec_test() {
    assert!(DI::EMPTY.mid().is_nan());
    assert!(DI::NAI.mid().is_nan());
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).mid(), 0.0);
    assert_eq2!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(0.0, 2.0, D::Com).mid(), 1.0);
    assert_eq2!(nd2di(2.0, 2.0, D::Dac).mid(), 2.0);
    assert_eq2!(nd2di(-2.0, 2.0, D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Trv).mid(), hexf64!("0x1.fffffffffffffp+1023"));
    assert_eq2!(nd2di(f64::NEG_INFINITY, 1.2, D::Trv).mid(), hexf64!("-0x1.fffffffffffffp+1023"));
    assert_eq2!(nd2di(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022"), D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv).mid(), hexf64!("0x1.7ffffffffffffp+1023"));
    assert_eq2!(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000003p-1022"), D::Trv).mid(), hexf64!("0x0.0000000000002p-1022"));
}

#[test]
fn minimal_rad_test() {
    assert_eq2!(n2i(0.0, 2.0).rad(), 1.0);
    assert_eq2!(n2i(2.0, 2.0).rad(), 0.0);
    assert!(I::EMPTY.rad().is_nan());
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq2!(n2i(0.0, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.2).rad(), f64::INFINITY);
    assert_eq2!(n2i(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022")).rad(), hexf64!("0x0.0000000000002p-1022"));
    assert_eq2!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022")).rad(), hexf64!("0x0.0000000000001p-1022"));
    assert_eq2!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000003p+0")).rad(), hexf64!("0x1.0000000000000p-51"));
}

#[test]
fn minimal_rad_dec_test() {
    assert_eq2!(nd2di(0.0, 2.0, D::Trv).rad(), 1.0);
    assert_eq2!(nd2di(2.0, 2.0, D::Com).rad(), 0.0);
    assert!(DI::EMPTY.rad().is_nan());
    assert!(DI::NAI.rad().is_nan());
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).rad(), f64::INFINITY);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).rad(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 1.2, D::Trv).rad(), f64::INFINITY);
    assert_eq2!(nd2di(hexf64!("-0x0.0000000000002p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Trv).rad(), hexf64!("0x0.0000000000002p-1022"));
    assert_eq2!(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000002p-1022"), D::Trv).rad(), hexf64!("0x0.0000000000001p-1022"));
    assert_eq2!(nd2di(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000003p+0"), D::Trv).rad(), hexf64!("0x1.0000000000000p-51"));
}

#[test]
fn minimal_wid_test() {
    assert_eq2!(n2i(2.0, 2.0).wid(), 0.0);
    assert_eq2!(n2i(1.0, 2.0).wid(), 1.0);
    assert_eq2!(n2i(1.0, f64::INFINITY).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).wid(), f64::INFINITY);
    assert!(I::EMPTY.wid().is_nan());
    assert_eq2!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")).wid(), hexf64!("0x1.0000000000000p-52"));
    assert_eq2!(n2i(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000001p-1022")).wid(), hexf64!("0x0.0000000000001p-1022"));
}

#[test]
fn minimal_wid_dec_test() {
    assert_eq2!(nd2di(2.0, 2.0, D::Com).wid(), 0.0);
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).wid(), 1.0);
    assert_eq2!(nd2di(1.0, f64::INFINITY, D::Trv).wid(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Def).wid(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).wid(), f64::INFINITY);
    assert!(DI::EMPTY.wid().is_nan());
    assert!(DI::NAI.wid().is_nan());
    assert_eq2!(nd2di(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0"), D::Trv).wid(), hexf64!("0x1.0000000000000p-52"));
    assert_eq2!(nd2di(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000001p-1022"), D::Trv).wid(), hexf64!("0x0.0000000000001p-1022"));
}

#[test]
fn minimal_mag_test() {
    assert_eq2!(n2i(1.0, 2.0).mag(), 2.0);
    assert_eq2!(n2i(-4.0, 2.0).mag(), 4.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).mag(), f64::INFINITY);
    assert_eq2!(n2i(1.0, f64::INFINITY).mag(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).mag(), f64::INFINITY);
    assert!(I::EMPTY.mag().is_nan());
    assert_eq2!(n2i(-0.0, 0.0).mag(), 0.0);
    assert_eq2!(n2i(-0.0, -0.0).mag(), 0.0);
}

#[test]
fn minimal_mag_dec_test() {
    assert_eq2!(nd2di(1.0, 2.0, D::Com).mag(), 2.0);
    assert_eq2!(nd2di(-4.0, 2.0, D::Trv).mag(), 4.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Trv).mag(), f64::INFINITY);
    assert_eq2!(nd2di(1.0, f64::INFINITY, D::Def).mag(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).mag(), f64::INFINITY);
    assert!(DI::EMPTY.mag().is_nan());
    assert!(DI::NAI.mag().is_nan());
    assert_eq2!(nd2di(-0.0, 0.0, D::Trv).mag(), 0.0);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).mag(), 0.0);
}

#[test]
fn minimal_mig_test() {
    assert_eq2!(n2i(1.0, 2.0).mig(), 1.0);
    assert_eq2!(n2i(-4.0, 2.0).mig(), 0.0);
    assert_eq2!(n2i(-4.0, -2.0).mig(), 2.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).mig(), 0.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, -2.0).mig(), 2.0);
    assert_eq2!(n2i(-1.0, f64::INFINITY).mig(), 0.0);
    assert_eq2!(n2i(1.0, f64::INFINITY).mig(), 1.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).mig(), 0.0);
    assert!(I::EMPTY.mig().is_nan());
    assert_eq2!(n2i(-0.0, 0.0).mig(), 0.0);
    assert_eq2!(n2i(-0.0, -0.0).mig(), 0.0);
}

#[test]
fn minimal_mig_dec_test() {
    assert_eq2!(nd2di(1.0, 2.0, D::Com).mig(), 1.0);
    assert_eq2!(nd2di(-4.0, 2.0, D::Trv).mig(), 0.0);
    assert_eq2!(nd2di(-4.0, -2.0, D::Trv).mig(), 2.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Def).mig(), 0.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -2.0, D::Trv).mig(), 2.0);
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Trv).mig(), 0.0);
    assert_eq2!(nd2di(1.0, f64::INFINITY, D::Trv).mig(), 1.0);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).mig(), 0.0);
    assert!(DI::EMPTY.mig().is_nan());
    assert!(DI::NAI.mig().is_nan());
    assert_eq2!(nd2di(-0.0, 0.0, D::Trv).mig(), 0.0);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).mig(), 0.0);
}
