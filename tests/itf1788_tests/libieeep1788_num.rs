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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

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
    assert_eq2!(DI::NAI.inf(), f64::NAN);
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
    assert_eq2!(DI::NAI.sup(), f64::NAN);
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
    assert_eq2!(I::EMPTY.mid(), f64::NAN);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).mid(), 0.0);
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).mid(), 0.0);
    assert_eq2!(n2i(0.0, 2.0).mid(), 1.0);
    assert_eq2!(n2i(2.0, 2.0).mid(), 2.0);
    assert_eq2!(n2i(-2.0, 2.0).mid(), 0.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).mid(), 1.7976931348623157e+308);
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.2).mid(), -1.7976931348623157e+308);
    assert_eq2!(n2i(-1e-323, 5e-324).mid(), 0.0);
    assert_eq2!(n2i(-5e-324, 1e-323).mid(), 0.0);
    assert_eq2!(n2i(8.988465674311579e+307, 1.7976931348623157e+308).mid(), 1.3482698511467367e+308);
    assert_eq2!(n2i(5e-324, 1.5e-323).mid(), 1e-323);
}

#[test]
fn minimal_mid_dec_test() {
    assert_eq2!(DI::EMPTY.mid(), f64::NAN);
    assert_eq2!(DI::NAI.mid(), f64::NAN);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).mid(), 0.0);
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(0.0, 2.0, D::Com).mid(), 1.0);
    assert_eq2!(nd2di(2.0, 2.0, D::Dac).mid(), 2.0);
    assert_eq2!(nd2di(-2.0, 2.0, D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Trv).mid(), 1.7976931348623157e+308);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 1.2, D::Trv).mid(), -1.7976931348623157e+308);
    assert_eq2!(nd2di(-1e-323, 5e-324, D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(-5e-324, 1e-323, D::Trv).mid(), 0.0);
    assert_eq2!(nd2di(8.988465674311579e+307, 1.7976931348623157e+308, D::Trv).mid(), 1.3482698511467367e+308);
    assert_eq2!(nd2di(5e-324, 1.5e-323, D::Trv).mid(), 1e-323);
}

#[test]
fn minimal_rad_test() {
    assert_eq2!(n2i(0.0, 2.0).rad(), 1.0);
    assert_eq2!(n2i(2.0, 2.0).rad(), 0.0);
    assert_eq2!(I::EMPTY.rad(), f64::NAN);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq2!(n2i(0.0, f64::INFINITY).rad(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.2).rad(), f64::INFINITY);
    assert_eq2!(n2i(-1e-323, 5e-324).rad(), 1e-323);
    assert_eq2!(n2i(5e-324, 1e-323).rad(), 5e-324);
    assert_eq2!(n2i(1.0, 1.0000000000000007).rad(), 4.440892098500626e-16);
}

#[test]
fn minimal_rad_dec_test() {
    assert_eq2!(nd2di(0.0, 2.0, D::Trv).rad(), 1.0);
    assert_eq2!(nd2di(2.0, 2.0, D::Com).rad(), 0.0);
    assert_eq2!(DI::EMPTY.rad(), f64::NAN);
    assert_eq2!(DI::NAI.rad(), f64::NAN);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).rad(), f64::INFINITY);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).rad(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 1.2, D::Trv).rad(), f64::INFINITY);
    assert_eq2!(nd2di(-1e-323, 5e-324, D::Trv).rad(), 1e-323);
    assert_eq2!(nd2di(5e-324, 1e-323, D::Trv).rad(), 5e-324);
    assert_eq2!(nd2di(1.0, 1.0000000000000007, D::Trv).rad(), 4.440892098500626e-16);
}

#[test]
fn minimal_wid_test() {
    assert_eq2!(n2i(2.0, 2.0).wid(), 0.0);
    assert_eq2!(n2i(1.0, 2.0).wid(), 1.0);
    assert_eq2!(n2i(1.0, f64::INFINITY).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).wid(), f64::INFINITY);
    assert_eq2!(I::EMPTY.wid(), f64::NAN);
    assert_eq2!(n2i(1.0, 1.0000000000000002).wid(), 2.220446049250313e-16);
    assert_eq2!(n2i(2.2250738585072014e-308, 2.225073858507202e-308).wid(), 5e-324);
}

#[test]
fn minimal_wid_dec_test() {
    assert_eq2!(nd2di(2.0, 2.0, D::Com).wid(), 0.0);
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).wid(), 1.0);
    assert_eq2!(nd2di(1.0, f64::INFINITY, D::Trv).wid(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Def).wid(), f64::INFINITY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).wid(), f64::INFINITY);
    assert_eq2!(DI::EMPTY.wid(), f64::NAN);
    assert_eq2!(DI::NAI.wid(), f64::NAN);
    assert_eq2!(nd2di(1.0, 1.0000000000000002, D::Trv).wid(), 2.220446049250313e-16);
    assert_eq2!(nd2di(2.2250738585072014e-308, 2.225073858507202e-308, D::Trv).wid(), 5e-324);
}

#[test]
fn minimal_mag_test() {
    assert_eq2!(n2i(1.0, 2.0).mag(), 2.0);
    assert_eq2!(n2i(-4.0, 2.0).mag(), 4.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).mag(), f64::INFINITY);
    assert_eq2!(n2i(1.0, f64::INFINITY).mag(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).mag(), f64::INFINITY);
    assert_eq2!(I::EMPTY.mag(), f64::NAN);
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
    assert_eq2!(DI::EMPTY.mag(), f64::NAN);
    assert_eq2!(DI::NAI.mag(), f64::NAN);
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
    assert_eq2!(I::EMPTY.mig(), f64::NAN);
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
    assert_eq2!(DI::EMPTY.mig(), f64::NAN);
    assert_eq2!(DI::NAI.mig(), f64::NAN);
    assert_eq2!(nd2di(-0.0, 0.0, D::Trv).mig(), 0.0);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).mig(), 0.0);
}
