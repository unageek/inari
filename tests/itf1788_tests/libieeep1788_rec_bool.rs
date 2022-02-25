/*
 *
 * Unit tests from libieeep1788 for recommended interval boolean operations
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
fn minimal_is_common_interval_test() {
    assert!(n2i(-27.0, -27.0).is_common_interval());
    assert!(n2i(-27.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 0.0).is_common_interval());
    assert!(n2i(-0.0, -0.0).is_common_interval());
    assert!(n2i(-0.0, 0.0).is_common_interval());
    assert!(n2i(0.0, -0.0).is_common_interval());
    assert!(n2i(5.0, 12.4).is_common_interval());
    assert!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).is_common_interval());
    assert_eq2!(I::ENTIRE.is_common_interval(), false);
    assert_eq2!(I::EMPTY.is_common_interval(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).is_common_interval(), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).is_common_interval(), false);
}

#[test]
fn minimal_is_common_interval_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Com).is_common_interval());
    assert!(nd2di(-27.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(0.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(-0.0, -0.0, D::Com).is_common_interval());
    assert!(nd2di(-0.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(0.0, -0.0, D::Com).is_common_interval());
    assert!(nd2di(5.0, 12.4, D::Com).is_common_interval());
    assert!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).is_common_interval());
    assert!(nd2di(-27.0, -27.0, D::Trv).is_common_interval());
    assert!(nd2di(-27.0, 0.0, D::Def).is_common_interval());
    assert!(nd2di(0.0, 0.0, D::Dac).is_common_interval());
    assert!(nd2di(-0.0, -0.0, D::Trv).is_common_interval());
    assert!(nd2di(-0.0, 0.0, D::Def).is_common_interval());
    assert!(nd2di(0.0, -0.0, D::Dac).is_common_interval());
    assert!(nd2di(5.0, 12.4, D::Def).is_common_interval());
    assert!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Trv).is_common_interval());
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).is_common_interval(), false);
    assert_eq2!(DI::NAI.is_common_interval(), false);
    assert_eq2!(DI::EMPTY.is_common_interval(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).is_common_interval(), false);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).is_common_interval(), false);
}

#[test]
fn minimal_is_singleton_test() {
    assert!(n2i(-27.0, -27.0).is_singleton());
    assert!(n2i(-2.0, -2.0).is_singleton());
    assert!(n2i(12.0, 12.0).is_singleton());
    assert!(n2i(17.1, 17.1).is_singleton());
    assert!(n2i(-0.0, -0.0).is_singleton());
    assert!(n2i(0.0, 0.0).is_singleton());
    assert!(n2i(-0.0, 0.0).is_singleton());
    assert!(n2i(0.0, -0.0).is_singleton());
    assert_eq2!(I::EMPTY.is_singleton(), false);
    assert_eq2!(I::ENTIRE.is_singleton(), false);
    assert_eq2!(n2i(-1.0, 0.0).is_singleton(), false);
    assert_eq2!(n2i(-1.0, -0.5).is_singleton(), false);
    assert_eq2!(n2i(1.0, 2.0).is_singleton(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.7976931348623157e+308).is_singleton(), false);
    assert_eq2!(n2i(-1.0, f64::INFINITY).is_singleton(), false);
}

#[test]
fn minimal_is_singleton_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Def).is_singleton());
    assert!(nd2di(-2.0, -2.0, D::Trv).is_singleton());
    assert!(nd2di(12.0, 12.0, D::Dac).is_singleton());
    assert!(nd2di(17.1, 17.1, D::Com).is_singleton());
    assert!(nd2di(-0.0, -0.0, D::Def).is_singleton());
    assert!(nd2di(0.0, 0.0, D::Com).is_singleton());
    assert!(nd2di(-0.0, 0.0, D::Def).is_singleton());
    assert!(nd2di(0.0, -0.0, D::Dac).is_singleton());
    assert_eq2!(DI::EMPTY.is_singleton(), false);
    assert_eq2!(DI::NAI.is_singleton(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).is_singleton(), false);
    assert_eq2!(nd2di(-1.0, 0.0, D::Dac).is_singleton(), false);
    assert_eq2!(nd2di(-1.0, -0.5, D::Com).is_singleton(), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Def).is_singleton(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.7976931348623157e+308, D::Dac).is_singleton(), false);
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Trv).is_singleton(), false);
}

#[test]
fn minimal_is_member_test() {
    assert!(n2i(-27.0, -27.0).contains(-27.0));
    assert!(n2i(-27.0, 0.0).contains(-27.0));
    assert!(n2i(-27.0, 0.0).contains(-7.0));
    assert!(n2i(-27.0, 0.0).contains(0.0));
    assert!(n2i(0.0, 0.0).contains(-0.0));
    assert!(n2i(0.0, 0.0).contains(0.0));
    assert!(n2i(-0.0, -0.0).contains(0.0));
    assert!(n2i(-0.0, 0.0).contains(0.0));
    assert!(n2i(0.0, -0.0).contains(0.0));
    assert!(n2i(5.0, 12.4).contains(5.0));
    assert!(n2i(5.0, 12.4).contains(6.3));
    assert!(n2i(5.0, 12.4).contains(12.4));
    assert!(I::ENTIRE.contains(0.0));
    assert!(I::ENTIRE.contains(5.0));
    assert!(I::ENTIRE.contains(6.3));
    assert!(I::ENTIRE.contains(12.4));
    assert!(I::ENTIRE.contains(1.7976931348623157e+308));
    assert!(I::ENTIRE.contains(-1.7976931348623157e+308));
    assert!(I::ENTIRE.contains(2.2250738585072014e-308));
    assert!(I::ENTIRE.contains(-2.2250738585072014e-308));
    assert_eq2!(n2i(-27.0, 0.0).contains(-71.0), false);
    assert_eq2!(n2i(-27.0, 0.0).contains(0.1), false);
    assert_eq2!(n2i(0.0, 0.0).contains(-0.01), false);
    assert_eq2!(n2i(0.0, 0.0).contains(1e-06), false);
    assert_eq2!(n2i(-0.0, -0.0).contains(111110.0), false);
    assert_eq2!(n2i(5.0, 12.4).contains(4.9), false);
    assert_eq2!(n2i(5.0, 12.4).contains(-6.3), false);
    assert_eq2!(I::EMPTY.contains(0.0), false);
    assert_eq2!(I::EMPTY.contains(-4535.3), false);
    assert_eq2!(I::EMPTY.contains(f64::NEG_INFINITY), false);
    assert_eq2!(I::EMPTY.contains(f64::INFINITY), false);
    assert_eq2!(I::EMPTY.contains(f64::NAN), false);
    assert_eq2!(I::ENTIRE.contains(f64::NEG_INFINITY), false);
    assert_eq2!(I::ENTIRE.contains(f64::INFINITY), false);
    assert_eq2!(I::ENTIRE.contains(f64::NAN), false);
}

#[test]
fn minimal_is_member_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Trv).contains(-27.0));
    assert!(nd2di(-27.0, 0.0, D::Def).contains(-27.0));
    assert!(nd2di(-27.0, 0.0, D::Dac).contains(-7.0));
    assert!(nd2di(-27.0, 0.0, D::Com).contains(0.0));
    assert!(nd2di(0.0, 0.0, D::Trv).contains(-0.0));
    assert!(nd2di(0.0, 0.0, D::Def).contains(0.0));
    assert!(nd2di(-0.0, -0.0, D::Dac).contains(0.0));
    assert!(nd2di(-0.0, 0.0, D::Com).contains(0.0));
    assert!(nd2di(0.0, -0.0, D::Trv).contains(0.0));
    assert!(nd2di(5.0, 12.4, D::Def).contains(5.0));
    assert!(nd2di(5.0, 12.4, D::Dac).contains(6.3));
    assert!(nd2di(5.0, 12.4, D::Com).contains(12.4));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(0.0));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(5.0));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(6.3));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(12.4));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(1.7976931348623157e+308));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(-1.7976931348623157e+308));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(2.2250738585072014e-308));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(-2.2250738585072014e-308));
    assert_eq2!(nd2di(-27.0, 0.0, D::Trv).contains(-71.0), false);
    assert_eq2!(nd2di(-27.0, 0.0, D::Def).contains(0.1), false);
    assert_eq2!(nd2di(0.0, 0.0, D::Dac).contains(-0.01), false);
    assert_eq2!(nd2di(0.0, 0.0, D::Com).contains(1e-06), false);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).contains(111110.0), false);
    assert_eq2!(nd2di(5.0, 12.4, D::Def).contains(4.9), false);
    assert_eq2!(nd2di(5.0, 12.4, D::Dac).contains(-6.3), false);
    assert_eq2!(DI::EMPTY.contains(0.0), false);
    assert_eq2!(DI::EMPTY.contains(0.0), false);
    assert_eq2!(DI::EMPTY.contains(-4535.3), false);
    assert_eq2!(DI::EMPTY.contains(-4535.3), false);
    assert_eq2!(DI::EMPTY.contains(f64::NEG_INFINITY), false);
    assert_eq2!(DI::NAI.contains(f64::NEG_INFINITY), false);
    assert_eq2!(DI::EMPTY.contains(f64::INFINITY), false);
    assert_eq2!(DI::NAI.contains(f64::INFINITY), false);
    assert_eq2!(DI::EMPTY.contains(f64::NAN), false);
    assert_eq2!(DI::NAI.contains(f64::NAN), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(f64::NEG_INFINITY), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(f64::INFINITY), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(f64::NAN), false);
}
