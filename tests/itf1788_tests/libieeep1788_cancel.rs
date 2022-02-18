/*
 *
 * Unit tests from libieeep1788 for cancellative addition and subtraction
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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

#[test]
fn minimal_cancel_plus_test() {
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_plus(n2i(-5.0, 1.0)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_plus(n2i(-5.0, 1.0)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_plus(n2i(-5.0, 1.0)), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_plus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_plus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_plus(n2i(1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_plus(n2i(f64::NEG_INFINITY, 1.0)), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_plus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_plus(n2i(1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_plus(n2i(f64::NEG_INFINITY, 1.0)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_plus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_plus(n2i(1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_plus(n2i(f64::NEG_INFINITY, 1.0)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_plus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_plus(n2i(1.0, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_plus(n2i(0.9, 5.0)), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_plus(n2i(0.9, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_plus(n2i(-5.0, 10.1)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_plus(n2i(-5.1, 10.0)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_plus(n2i(-5.1, 10.1)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_plus(n2i(-5.0, -0.9)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_plus(n2i(-5.1, -1.0)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_plus(n2i(-5.1, -0.9)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, -1.0).cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_plus(I::EMPTY), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_plus(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_plus(n2i(1.0, 10.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_plus(n2i(-5.0, 10.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_plus(n2i(-5.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-5.1, -0.0).cancel_plus(n2i(0.0, 5.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-5.1, -1.0).cancel_plus(n2i(1.0, 5.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-5.0, -0.9).cancel_plus(n2i(1.0, 5.0)), n2i(0.0, 0.09999999999999998));
    assert_eq2!(n2i(-5.1, -0.9).cancel_plus(n2i(1.0, 5.0)), n2i(-0.09999999999999964, 0.09999999999999998));
    assert_eq2!(n2i(-5.0, -1.0).cancel_plus(n2i(1.0, 5.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-10.1, 5.0).cancel_plus(n2i(-5.0, 10.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-10.0, 5.1).cancel_plus(n2i(-5.0, 10.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(-10.1, 5.1).cancel_plus(n2i(-5.0, 10.0)), n2i(-0.09999999999999964, 0.09999999999999964));
    assert_eq2!(n2i(-10.0, 5.0).cancel_plus(n2i(-5.0, 10.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.9, 5.0).cancel_plus(n2i(-5.0, -1.0)), n2i(-0.09999999999999998, 0.0));
    assert_eq2!(n2i(1.0, 5.1).cancel_plus(n2i(-5.0, -1.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(0.0, 5.1).cancel_plus(n2i(-5.0, -0.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(0.9, 5.1).cancel_plus(n2i(-5.0, -1.0)), n2i(-0.09999999999999998, 0.09999999999999964));
    assert_eq2!(n2i(1.0, 5.0).cancel_plus(n2i(-5.0, -1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 5.0).cancel_plus(n2i(-5.0, -0.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.9999999999999964, 1.9999999999999964).cancel_plus(n2i(-0.1, -0.1)), n2i(1.8999999999999964, 1.8999999999999966));
    assert_eq2!(n2i(-0.1, 1.9999999999999964).cancel_plus(n2i(-0.1, 0.01)), n2i(-0.09000000000000001, 1.8999999999999966));
    assert_eq2!(n2i(1.7976931348623157e+308, 1.7976931348623157e+308).cancel_plus(n2i(1.7976931348623157e+308, 1.7976931348623157e+308)), n2i(1.7976931348623157e+308, f64::INFINITY));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_plus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_plus(n2i(-1.7976931348623155e+308, 1.7976931348623157e+308)), n2i(0.0, 1.99584030953472e+292));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_plus(n2i(-1.7976931348623157e+308, 1.7976931348623155e+308)), n2i(-1.99584030953472e+292, 0.0));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623155e+308).cancel_plus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), I::ENTIRE);
    assert_eq2!(n2i(-1.7976931348623155e+308, 1.7976931348623157e+308).cancel_plus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 2.2204460492503128e-16).cancel_plus(n2i(-1.0, 2.2204460492503126e-16)), n2i(-0.9999999999999999, -0.9999999999999998));
    assert_eq2!(n2i(-1.0, 2.2204460492503126e-16).cancel_plus(n2i(-1.0, 2.2204460492503128e-16)), I::ENTIRE);
}

#[test]
fn minimal_cancel_plus_dec_test() {
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Dac).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Def).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Dac).cancel_plus(nd2di(-5.0, 1.0, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Trv).cancel_plus(nd2di(-5.0, 1.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_plus(nd2di(-5.0, 1.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Dac).cancel_plus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Def).cancel_plus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(1.0, f64::INFINITY, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(f64::NEG_INFINITY, 1.0, D::Trv)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Dac).cancel_plus(nd2di(1.0, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Def).cancel_plus(nd2di(f64::NEG_INFINITY, 1.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Com).cancel_plus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_plus(nd2di(1.0, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_plus(nd2di(f64::NEG_INFINITY, 1.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_plus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Com).cancel_plus(nd2di(1.0, 5.1, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Dac).cancel_plus(nd2di(0.9, 5.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Def).cancel_plus(nd2di(0.9, 5.1, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Trv).cancel_plus(nd2di(-5.0, 10.1, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Com).cancel_plus(nd2di(-5.1, 10.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Dac).cancel_plus(nd2di(-5.1, 10.1, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Def).cancel_plus(nd2di(-5.0, -0.9, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Trv).cancel_plus(nd2di(-5.1, -1.0, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Dac).cancel_plus(nd2di(-5.1, -0.9, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, -1.0, D::Trv).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Def).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Com).cancel_plus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_plus(DI::EMPTY), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(1.0, 10.0, D::Dac)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(-5.0, 10.0, D::Def)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_plus(nd2di(-5.0, -1.0, D::Com)), DI::EMPTY);
    assert_eq2!(nd2di(-5.1, -0.0, D::Com).cancel_plus(nd2di(0.0, 5.0, D::Com)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-5.1, -1.0, D::Com).cancel_plus(nd2di(1.0, 5.0, D::Dac)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-5.0, -0.9, D::Com).cancel_plus(nd2di(1.0, 5.0, D::Def)), nd2di(0.0, 0.09999999999999998, D::Trv));
    assert_eq2!(nd2di(-5.1, -0.9, D::Dac).cancel_plus(nd2di(1.0, 5.0, D::Trv)), nd2di(-0.09999999999999964, 0.09999999999999998, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Dac).cancel_plus(nd2di(1.0, 5.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-10.1, 5.0, D::Dac).cancel_plus(nd2di(-5.0, 10.0, D::Dac)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.1, D::Def).cancel_plus(nd2di(-5.0, 10.0, D::Def)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(-10.1, 5.1, D::Def).cancel_plus(nd2di(-5.0, 10.0, D::Trv)), nd2di(-0.09999999999999964, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Def).cancel_plus(nd2di(-5.0, 10.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(0.9, 5.0, D::Trv).cancel_plus(nd2di(-5.0, -1.0, D::Dac)), nd2di(-0.09999999999999998, 0.0, D::Trv));
    assert_eq2!(nd2di(1.0, 5.1, D::Trv).cancel_plus(nd2di(-5.0, -1.0, D::Def)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(0.0, 5.1, D::Trv).cancel_plus(nd2di(-5.0, -0.0, D::Trv)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(0.9, 5.1, D::Com).cancel_plus(nd2di(-5.0, -1.0, D::Com)), nd2di(-0.09999999999999998, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Dac).cancel_plus(nd2di(-5.0, -1.0, D::Dac)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(0.0, 5.0, D::Def).cancel_plus(nd2di(-5.0, -0.0, D::Trv)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(1.9999999999999964, 1.9999999999999964, D::Com).cancel_plus(nd2di(-0.1, -0.1, D::Com)), nd2di(1.8999999999999964, 1.8999999999999966, D::Trv));
    assert_eq2!(nd2di(-0.1, 1.9999999999999964, D::Dac).cancel_plus(nd2di(-0.1, 0.01, D::Com)), nd2di(-0.09000000000000001, 1.8999999999999966, D::Trv));
    assert_eq2!(nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_plus(nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(1.7976931348623157e+308, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_plus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Dac).cancel_plus(nd2di(-1.7976931348623155e+308, 1.7976931348623157e+308, D::Com)), nd2di(0.0, 1.99584030953472e+292, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Dac).cancel_plus(nd2di(-1.7976931348623157e+308, 1.7976931348623155e+308, D::Com)), nd2di(-1.99584030953472e+292, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623155e+308, D::Com).cancel_plus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623155e+308, 1.7976931348623157e+308, D::Com).cancel_plus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 2.2204460492503128e-16, D::Dac).cancel_plus(nd2di(-1.0, 2.2204460492503126e-16, D::Com)), nd2di(-0.9999999999999999, -0.9999999999999998, D::Trv));
    assert_eq2!(nd2di(-1.0, 2.2204460492503126e-16, D::Def).cancel_plus(nd2di(-1.0, 2.2204460492503128e-16, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}

#[test]
fn minimal_cancel_minus_test() {
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_minus(n2i(-1.0, 5.0)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_minus(n2i(-1.0, 5.0)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_minus(n2i(-1.0, 5.0)), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).cancel_minus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY).cancel_minus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_minus(n2i(f64::NEG_INFINITY, -1.0)), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_minus(n2i(-1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_minus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_minus(n2i(f64::NEG_INFINITY, -1.0)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_minus(n2i(-1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).cancel_minus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_minus(n2i(f64::NEG_INFINITY, -1.0)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_minus(n2i(-1.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.cancel_minus(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_minus(n2i(-5.1, -1.0)), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_minus(n2i(-5.0, -0.9)), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).cancel_minus(n2i(-5.1, -0.9)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_minus(n2i(-10.1, 5.0)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_minus(n2i(-10.0, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_minus(n2i(-10.1, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_minus(n2i(0.9, 5.0)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_minus(n2i(1.0, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_minus(n2i(0.9, 5.1)), I::ENTIRE);
    assert_eq2!(n2i(-10.0, -1.0).cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(-10.0, 5.0).cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).cancel_minus(I::EMPTY), I::ENTIRE);
    assert_eq2!(I::EMPTY.cancel_minus(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_minus(n2i(-10.0, -1.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_minus(n2i(-10.0, 5.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.cancel_minus(n2i(1.0, 5.0)), I::EMPTY);
    assert_eq2!(n2i(-5.1, -0.0).cancel_minus(n2i(-5.0, 0.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-5.1, -1.0).cancel_minus(n2i(-5.0, -1.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-5.0, -0.9).cancel_minus(n2i(-5.0, -1.0)), n2i(0.0, 0.09999999999999998));
    assert_eq2!(n2i(-5.1, -0.9).cancel_minus(n2i(-5.0, -1.0)), n2i(-0.09999999999999964, 0.09999999999999998));
    assert_eq2!(n2i(-5.0, -1.0).cancel_minus(n2i(-5.0, -1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-10.1, 5.0).cancel_minus(n2i(-10.0, 5.0)), n2i(-0.09999999999999964, 0.0));
    assert_eq2!(n2i(-10.0, 5.1).cancel_minus(n2i(-10.0, 5.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(-10.1, 5.1).cancel_minus(n2i(-10.0, 5.0)), n2i(-0.09999999999999964, 0.09999999999999964));
    assert_eq2!(n2i(-10.0, 5.0).cancel_minus(n2i(-10.0, 5.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.9, 5.0).cancel_minus(n2i(1.0, 5.0)), n2i(-0.09999999999999998, 0.0));
    assert_eq2!(n2i(-0.0, 5.1).cancel_minus(n2i(0.0, 5.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(1.0, 5.1).cancel_minus(n2i(1.0, 5.0)), n2i(0.0, 0.09999999999999964));
    assert_eq2!(n2i(0.9, 5.1).cancel_minus(n2i(1.0, 5.0)), n2i(-0.09999999999999998, 0.09999999999999964));
    assert_eq2!(n2i(1.0, 5.0).cancel_minus(n2i(1.0, 5.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-5.0, 1.0).cancel_minus(n2i(-1.0, 5.0)), n2i(-4.0, -4.0));
    assert_eq2!(n2i(-5.0, 0.0).cancel_minus(n2i(-0.0, 5.0)), n2i(-5.0, -5.0));
    assert_eq2!(n2i(1.9999999999999964, 1.9999999999999964).cancel_minus(n2i(0.1, 0.1)), n2i(1.8999999999999964, 1.8999999999999966));
    assert_eq2!(n2i(-0.1, 1.9999999999999964).cancel_minus(n2i(-0.01, 0.1)), n2i(-0.09000000000000001, 1.8999999999999966));
    assert_eq2!(n2i(1.7976931348623157e+308, 1.7976931348623157e+308).cancel_minus(n2i(-1.7976931348623157e+308, -1.7976931348623157e+308)), n2i(1.7976931348623157e+308, f64::INFINITY));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_minus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_minus(n2i(-1.7976931348623157e+308, 1.7976931348623155e+308)), n2i(0.0, 1.99584030953472e+292));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308).cancel_minus(n2i(-1.7976931348623155e+308, 1.7976931348623157e+308)), n2i(-1.99584030953472e+292, 0.0));
    assert_eq2!(n2i(-1.7976931348623157e+308, 1.7976931348623155e+308).cancel_minus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), I::ENTIRE);
    assert_eq2!(n2i(-1.7976931348623155e+308, 1.7976931348623157e+308).cancel_minus(n2i(-1.7976931348623157e+308, 1.7976931348623157e+308)), I::ENTIRE);
    assert_eq2!(n2i(5e-324, 5e-324).cancel_minus(n2i(5e-324, 5e-324)), n2i(0.0, 0.0));
    assert_eq2!(n2i(5e-324, 5e-324).cancel_minus(n2i(-5e-324, -5e-324)), n2i(1e-323, 1e-323));
    assert_eq2!(n2i(2.2250738585072014e-308, 2.2250738585072024e-308).cancel_minus(n2i(2.2250738585072014e-308, 2.225073858507202e-308)), n2i(0.0, 5e-324));
    assert_eq2!(n2i(2.2250738585072014e-308, 2.225073858507202e-308).cancel_minus(n2i(2.2250738585072014e-308, 2.2250738585072024e-308)), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 2.2204460492503128e-16).cancel_minus(n2i(-2.2204460492503126e-16, 1.0)), n2i(-0.9999999999999999, -0.9999999999999998));
    assert_eq2!(n2i(-1.0, 2.2204460492503126e-16).cancel_minus(n2i(-2.2204460492503128e-16, 1.0)), I::ENTIRE);
}

#[test]
fn minimal_cancel_minus_dec_test() {
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Dac).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Def).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Trv).cancel_minus(nd2di(-1.0, 5.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Dac).cancel_minus(nd2di(-1.0, 5.0, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_minus(nd2di(-1.0, 5.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, -1.0, D::Def).cancel_minus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, f64::INFINITY, D::Trv).cancel_minus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(f64::NEG_INFINITY, -1.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(-1.0, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Dac).cancel_minus(nd2di(f64::NEG_INFINITY, -1.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Def).cancel_minus(nd2di(-1.0, f64::INFINITY, D::Trv)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 5.0, D::Com).cancel_minus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_minus(nd2di(f64::NEG_INFINITY, -1.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_minus(nd2di(-1.0, f64::INFINITY, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cancel_minus(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Com).cancel_minus(nd2di(-5.1, -1.0, D::Trv)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Dac).cancel_minus(nd2di(-5.0, -0.9, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Def).cancel_minus(nd2di(-5.1, -0.9, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Trv).cancel_minus(nd2di(-10.1, 5.0, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Com).cancel_minus(nd2di(-10.0, 5.1, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Dac).cancel_minus(nd2di(-10.1, 5.1, D::Trv)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Def).cancel_minus(nd2di(0.9, 5.0, D::Def)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Trv).cancel_minus(nd2di(1.0, 5.1, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Com).cancel_minus(nd2di(0.9, 5.1, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, -1.0, D::Com).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Dac).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Def).cancel_minus(DI::EMPTY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(DI::EMPTY.cancel_minus(DI::EMPTY), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(-10.0, -1.0, D::Com)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(-10.0, 5.0, D::Dac)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.cancel_minus(nd2di(1.0, 5.0, D::Def)), DI::EMPTY);
    assert_eq2!(nd2di(-5.1, -0.0, D::Com).cancel_minus(nd2di(-5.0, 0.0, D::Com)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-5.1, -1.0, D::Dac).cancel_minus(nd2di(-5.0, -1.0, D::Com)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-5.0, -0.9, D::Def).cancel_minus(nd2di(-5.0, -1.0, D::Com)), nd2di(0.0, 0.09999999999999998, D::Trv));
    assert_eq2!(nd2di(-5.1, -0.9, D::Trv).cancel_minus(nd2di(-5.0, -1.0, D::Com)), nd2di(-0.09999999999999964, 0.09999999999999998, D::Trv));
    assert_eq2!(nd2di(-5.0, -1.0, D::Com).cancel_minus(nd2di(-5.0, -1.0, D::Dac)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-10.1, 5.0, D::Dac).cancel_minus(nd2di(-10.0, 5.0, D::Dac)), nd2di(-0.09999999999999964, 0.0, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.1, D::Def).cancel_minus(nd2di(-10.0, 5.0, D::Dac)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(-10.1, 5.1, D::Trv).cancel_minus(nd2di(-10.0, 5.0, D::Def)), nd2di(-0.09999999999999964, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(-10.0, 5.0, D::Com).cancel_minus(nd2di(-10.0, 5.0, D::Def)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(0.9, 5.0, D::Dac).cancel_minus(nd2di(1.0, 5.0, D::Def)), nd2di(-0.09999999999999998, 0.0, D::Trv));
    assert_eq2!(nd2di(-0.0, 5.1, D::Def).cancel_minus(nd2di(0.0, 5.0, D::Def)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(1.0, 5.1, D::Trv).cancel_minus(nd2di(1.0, 5.0, D::Trv)), nd2di(0.0, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(0.9, 5.1, D::Com).cancel_minus(nd2di(1.0, 5.0, D::Trv)), nd2di(-0.09999999999999998, 0.09999999999999964, D::Trv));
    assert_eq2!(nd2di(1.0, 5.0, D::Dac).cancel_minus(nd2di(1.0, 5.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-5.0, 1.0, D::Def).cancel_minus(nd2di(-1.0, 5.0, D::Def)), nd2di(-4.0, -4.0, D::Trv));
    assert_eq2!(nd2di(-5.0, 0.0, D::Trv).cancel_minus(nd2di(-0.0, 5.0, D::Trv)), nd2di(-5.0, -5.0, D::Trv));
    assert_eq2!(nd2di(1.9999999999999964, 1.9999999999999964, D::Com).cancel_minus(nd2di(0.1, 0.1, D::Com)), nd2di(1.8999999999999964, 1.8999999999999966, D::Trv));
    assert_eq2!(nd2di(-0.1, 1.9999999999999964, D::Def).cancel_minus(nd2di(-0.01, 0.1, D::Dac)), nd2di(-0.09000000000000001, 1.8999999999999966, D::Trv));
    assert_eq2!(nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_minus(nd2di(-1.7976931348623157e+308, -1.7976931348623157e+308, D::Com)), nd2di(1.7976931348623157e+308, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_minus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_minus(nd2di(-1.7976931348623157e+308, 1.7976931348623155e+308, D::Com)), nd2di(0.0, 1.99584030953472e+292, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).cancel_minus(nd2di(-1.7976931348623155e+308, 1.7976931348623157e+308, D::Com)), nd2di(-1.99584030953472e+292, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623157e+308, 1.7976931348623155e+308, D::Com).cancel_minus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.7976931348623155e+308, 1.7976931348623157e+308, D::Com).cancel_minus(nd2di(-1.7976931348623157e+308, 1.7976931348623157e+308, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(5e-324, 5e-324, D::Com).cancel_minus(nd2di(5e-324, 5e-324, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(nd2di(5e-324, 5e-324, D::Com).cancel_minus(nd2di(-5e-324, -5e-324, D::Dac)), nd2di(1e-323, 1e-323, D::Trv));
    assert_eq2!(nd2di(2.2250738585072014e-308, 2.2250738585072024e-308, D::Dac).cancel_minus(nd2di(2.2250738585072014e-308, 2.225073858507202e-308, D::Dac)), nd2di(0.0, 5e-324, D::Trv));
    assert_eq2!(nd2di(2.2250738585072014e-308, 2.225073858507202e-308, D::Def).cancel_minus(nd2di(2.2250738585072014e-308, 2.2250738585072024e-308, D::Com)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(-1.0, 2.2204460492503128e-16, D::Com).cancel_minus(nd2di(-2.2204460492503126e-16, 1.0, D::Dac)), nd2di(-0.9999999999999999, -0.9999999999999998, D::Trv));
    assert_eq2!(nd2di(-1.0, 2.2204460492503126e-16, D::Def).cancel_minus(nd2di(-2.2204460492503128e-16, 1.0, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}
