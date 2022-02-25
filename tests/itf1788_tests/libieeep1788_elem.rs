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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

#[test]
fn minimal_neg_test() {
    assert_eq2!(-n2i(1.0, 2.0), n2i(-2.0, -1.0));
    assert_eq2!(-I::EMPTY, I::EMPTY);
    assert_eq2!(-I::ENTIRE, I::ENTIRE);
    assert_eq2!(-n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq2!(-n2i(f64::NEG_INFINITY, 1.0), n2i(-1.0, f64::INFINITY));
    assert_eq2!(-n2i(0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq2!(-n2i(-0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq2!(-n2i(-2.0, 0.0), n2i(0.0, 2.0));
    assert_eq2!(-n2i(-2.0, -0.0), n2i(0.0, 2.0));
    assert_eq2!(-n2i(0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(-n2i(-0.0, -0.0), n2i(0.0, 0.0));
}

#[test]
fn minimal_neg_dec_test() {
    assert_eq2!(-DI::NAI, DI::NAI);
    assert_eq2!(-DI::EMPTY, DI::EMPTY);
    assert_eq2!(
        -nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(-nd2di(1.0, 2.0, D::Com), nd2di(-2.0, -1.0, D::Com));
}

#[test]
fn minimal_add_test() {
    assert_eq2!(I::EMPTY + I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0) + I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY + n2i(-1.0, 1.0), I::EMPTY);
    assert_eq2!(I::EMPTY + I::ENTIRE, I::EMPTY);
    assert_eq2!(I::ENTIRE + I::EMPTY, I::EMPTY);
    assert_eq2!(I::ENTIRE + n2i(f64::NEG_INFINITY, 1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE + n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE + n2i(-1.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE + I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.0) + I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, 1.0) + I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY) + I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) + n2i(f64::NEG_INFINITY, 4.0),
        n2i(f64::NEG_INFINITY, 6.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, 4.0),
        n2i(f64::NEG_INFINITY, 6.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 2.0) + n2i(f64::NEG_INFINITY, 4.0),
        n2i(f64::NEG_INFINITY, 6.0)
    );
    assert_eq2!(n2i(1.0, 2.0) + n2i(3.0, 4.0), n2i(4.0, 6.0));
    assert_eq2!(
        n2i(1.0, 2.0) + n2i(3.0, f64::INFINITY),
        n2i(4.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) + n2i(f64::NEG_INFINITY, 4.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) + n2i(3.0, 4.0),
        n2i(4.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) + n2i(3.0, f64::INFINITY),
        n2i(4.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) + n2i(3.0, 4.0),
        n2i(4.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, 2.0) + n2i(-3.0, 4.0),
        n2i(f64::NEG_INFINITY, 6.0)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, 2.0) + n2i(-3.0, 1.7976931348623157e+308),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) + n2i(0.0, 0.0),
        n2i(1.0, 1.7976931348623157e+308)
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) + n2i(-0.0, -0.0),
        n2i(1.0, 1.7976931348623157e+308)
    );
    assert_eq2!(n2i(0.0, 0.0) + n2i(-3.0, 4.0), n2i(-3.0, 4.0));
    assert_eq2!(
        n2i(-0.0, -0.0) + n2i(-3.0, 1.7976931348623157e+308),
        n2i(-3.0, 1.7976931348623157e+308)
    );
    assert_eq2!(
        n2i(1.9999999999999964, 1.9999999999999964) + n2i(0.1, 0.1),
        n2i(2.099999999999996, 2.0999999999999965)
    );
    assert_eq2!(
        n2i(1.9999999999999964, 1.9999999999999964) + n2i(-0.1, -0.1),
        n2i(1.8999999999999964, 1.8999999999999966)
    );
    assert_eq2!(
        n2i(-1.9999999999999964, 1.9999999999999964) + n2i(0.1, 0.1),
        n2i(-1.8999999999999966, 2.0999999999999965)
    );
}

#[test]
fn minimal_add_dec_test() {
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Com),
        nd2di(6.0, 9.0, D::Com)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Def),
        nd2di(6.0, 9.0, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 1.7976931348623157e+308, D::Com),
        nd2di(6.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, 2.0, D::Com) + nd2di(-0.1, 5.0, D::Com),
        nd2di(f64::NEG_INFINITY, 7.0, D::Dac)
    );
    assert_eq2!(nd2di(1.0, 2.0, D::Trv) + DI::EMPTY, DI::EMPTY);
    assert_eq2!(DI::NAI + nd2di(1.0, 2.0, D::Trv), DI::NAI);
}

#[test]
fn minimal_sub_test() {
    assert_eq2!(I::EMPTY - I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0) - I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY - n2i(-1.0, 1.0), I::EMPTY);
    assert_eq2!(I::EMPTY - I::ENTIRE, I::EMPTY);
    assert_eq2!(I::ENTIRE - I::EMPTY, I::EMPTY);
    assert_eq2!(I::ENTIRE - n2i(f64::NEG_INFINITY, 1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE - n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE - n2i(-1.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.0) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, 1.0) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) - n2i(f64::NEG_INFINITY, 4.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, 4.0),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(1.0, 2.0) - n2i(f64::NEG_INFINITY, 4.0),
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, 2.0) - n2i(3.0, 4.0), n2i(-3.0, -1.0));
    assert_eq2!(
        n2i(1.0, 2.0) - n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) - n2i(f64::NEG_INFINITY, 4.0),
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) - n2i(3.0, 4.0),
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY) - n2i(3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) - n2i(-3.0, 4.0),
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, 2.0) - n2i(3.0, 4.0),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, 2.0) - n2i(-1.7976931348623157e+308, 4.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) - n2i(0.0, 0.0),
        n2i(1.0, 1.7976931348623157e+308)
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308) - n2i(-0.0, -0.0),
        n2i(1.0, 1.7976931348623157e+308)
    );
    assert_eq2!(n2i(0.0, 0.0) - n2i(-3.0, 4.0), n2i(-4.0, 3.0));
    assert_eq2!(
        n2i(-0.0, -0.0) - n2i(-3.0, 1.7976931348623157e+308),
        n2i(-1.7976931348623157e+308, 3.0)
    );
    assert_eq2!(
        n2i(1.9999999999999964, 1.9999999999999964) - n2i(0.1, 0.1),
        n2i(1.8999999999999964, 1.8999999999999966)
    );
    assert_eq2!(
        n2i(1.9999999999999964, 1.9999999999999964) - n2i(-0.1, -0.1),
        n2i(2.099999999999996, 2.0999999999999965)
    );
    assert_eq2!(
        n2i(-1.9999999999999964, 1.9999999999999964) - n2i(0.1, 0.1),
        n2i(-2.0999999999999965, 1.8999999999999966)
    );
}

#[test]
fn minimal_sub_dec_test() {
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Com),
        nd2di(-6.0, -3.0, D::Com)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Def),
        nd2di(-6.0, -3.0, D::Def)
    );
    assert_eq2!(
        nd2di(-1.0, 2.0, D::Com) - nd2di(5.0, 1.7976931348623157e+308, D::Com),
        nd2di(f64::NEG_INFINITY, -3.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, 2.0, D::Com) - nd2di(-1.0, 5.0, D::Com),
        nd2di(f64::NEG_INFINITY, 3.0, D::Dac)
    );
    assert_eq2!(nd2di(1.0, 2.0, D::Trv) - DI::EMPTY, DI::EMPTY);
    assert_eq2!(DI::NAI - nd2di(1.0, 2.0, D::Trv), DI::NAI);
}

#[test]
fn minimal_mul_test() {
    assert_eq2!(I::EMPTY * I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0) * I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY * n2i(-1.0, 1.0), I::EMPTY);
    assert_eq2!(I::EMPTY * I::ENTIRE, I::EMPTY);
    assert_eq2!(I::ENTIRE * I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0) * I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY * n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0) * I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY * n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(I::ENTIRE * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(I::ENTIRE * n2i(-5.0, -1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(1.0, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(-5.0, -1.0),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(1.0, 3.0),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(-5.0, -1.0),
        n2i(f64::NEG_INFINITY, 5.0)
    );
    assert_eq2!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(1.0, 3.0),
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(n2i(-1.0, f64::INFINITY) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, -1.0),
        n2i(-15.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, 3.0),
        n2i(f64::NEG_INFINITY, 9.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, -1.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 3.0) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(-0.0, -0.0),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, -1.0),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, 3.0),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, -1.0),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -3.0) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, -1.0),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, -1.0));
    assert_eq2!(n2i(1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq2!(n2i(1.0, 5.0) * n2i(1.0, 3.0), n2i(1.0, 15.0));
    assert_eq2!(
        n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0),
        n2i(f64::NEG_INFINITY, 15.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0) * n2i(-5.0, f64::INFINITY),
        n2i(-25.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0) * n2i(1.0, f64::INFINITY),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, 5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, 5.0));
    //min max
    assert_eq2!(n2i(-1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq2!(n2i(-10.0, 2.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq2!(n2i(-1.0, 5.0) * n2i(-1.0, 10.0), n2i(-10.0, 50.0));
    assert_eq2!(n2i(-2.0, 2.0) * n2i(-5.0, 3.0), n2i(-10.0, 10.0));
    //end min max
    assert_eq2!(n2i(-1.0, 5.0) * n2i(1.0, 3.0), n2i(-3.0, 15.0));
    assert_eq2!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0) * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-10.0, -5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-10.0, -5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-10.0, -5.0) * n2i(-5.0, -1.0), n2i(5.0, 50.0));
    assert_eq2!(n2i(-10.0, -5.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq2!(n2i(-10.0, -5.0) * n2i(1.0, 3.0), n2i(-30.0, -5.0));
    assert_eq2!(
        n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, -1.0),
        n2i(5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, 3.0),
        n2i(-30.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0) * n2i(-5.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 50.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0) * n2i(1.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(n2i(-10.0, -5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(0.1, 1.9999999999999964) * n2i(-1.9999999999999964, f64::INFINITY),
        n2i(-3.9999999999999862, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.9999999999999964) * n2i(-1.9999999999999964, -0.1),
        n2i(-3.9999999999999862, 0.19999999999999968)
    );
    assert_eq2!(
        n2i(-0.1, 0.1) * n2i(-1.9999999999999964, 0.1),
        n2i(-0.19999999999999968, 0.19999999999999968)
    );
    assert_eq2!(
        n2i(-1.9999999999999964, -0.1) * n2i(0.1, 1.9999999999999964),
        n2i(-3.9999999999999862, -0.01)
    );
}

#[test]
fn minimal_mul_dec_test() {
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Com),
        nd2di(5.0, 14.0, D::Com)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Def),
        nd2di(5.0, 14.0, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 1.7976931348623157e+308, D::Com),
        nd2di(5.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, 2.0, D::Com) * nd2di(-1.0, 5.0, D::Com),
        nd2di(f64::NEG_INFINITY, 1.7976931348623157e+308, D::Dac)
    );
    assert_eq2!(nd2di(1.0, 2.0, D::Trv) * DI::EMPTY, DI::EMPTY);
    assert_eq2!(DI::NAI * nd2di(1.0, 2.0, D::Trv), DI::NAI);
}

#[test]
fn minimal_div_test() {
    assert_eq2!(I::EMPTY / I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0) / I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY / n2i(-1.0, 1.0), I::EMPTY);
    assert_eq2!(I::EMPTY / n2i(0.1, 1.0), I::EMPTY);
    assert_eq2!(I::EMPTY / n2i(-1.0, -0.1), I::EMPTY);
    assert_eq2!(I::EMPTY / I::ENTIRE, I::EMPTY);
    assert_eq2!(I::ENTIRE / I::EMPTY, I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0) / I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0) / I::EMPTY, I::EMPTY);
    assert_eq2!(I::EMPTY / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(I::ENTIRE / n2i(-5.0, -3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(3.0, 5.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(f64::NEG_INFINITY, -3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(I::ENTIRE / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(I::ENTIRE / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(I::ENTIRE / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-5.0, -3.0), n2i(3.0, 10.0));
    assert_eq2!(n2i(-30.0, -15.0) / n2i(3.0, 5.0), n2i(-10.0, -3.0));
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, 10.0)
    );
    assert_eq2!(n2i(-30.0, -15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq2!(n2i(-30.0, -15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-3.0, 0.0), n2i(5.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-3.0, -0.0), n2i(5.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(0.0, 3.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, -15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, -15.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-30.0, -15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-5.0, -3.0), n2i(-5.0, 10.0));
    assert_eq2!(n2i(-30.0, 15.0) / n2i(3.0, 5.0), n2i(-10.0, 5.0));
    assert_eq2!(
        n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(-5.0, 10.0)
    );
    assert_eq2!(n2i(-30.0, 15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 5.0));
    assert_eq2!(n2i(-30.0, 15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(15.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, -3.0));
    assert_eq2!(n2i(15.0, 30.0) / n2i(3.0, 5.0), n2i(3.0, 10.0));
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(-10.0, 0.0)
    );
    assert_eq2!(n2i(15.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq2!(n2i(15.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(-3.0, 0.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(n2i(15.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(n2i(15.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(15.0, 30.0) / n2i(0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(15.0, 30.0) / n2i(-0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(15.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(15.0, 30.0) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(15.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-5.0, -3.0),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, 5.0),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 0.0),
        n2i(5.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, -0.0),
        n2i(5.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 3.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(-5.0, -3.0),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, 5.0),
        n2i(f64::NEG_INFINITY, 5.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 5.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 0.0),
        I::ENTIRE
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -0.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(-5.0, -3.0),
        n2i(f64::NEG_INFINITY, 5.0)
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(3.0, 5.0),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(f64::NEG_INFINITY, 5.0)
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0),
        I::ENTIRE
    );
    assert_eq2!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(n2i(-15.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-5.0, -3.0),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(3.0, 5.0),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(15.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-3.0, 0.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(n2i(15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, -5.0)
    );
    assert_eq2!(n2i(15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(0.0, 3.0),
        n2i(5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-0.0, 3.0),
        n2i(5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(15.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq2!(n2i(-30.0, 0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, 10.0)
    );
    assert_eq2!(n2i(-30.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq2!(n2i(-30.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, 0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, 0.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-30.0, 0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq2!(n2i(-30.0, -0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, 10.0)
    );
    assert_eq2!(n2i(-30.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq2!(n2i(-30.0, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30.0, -0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-30.0, -0.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-30.0, -0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq2!(n2i(0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(-10.0, 0.0)
    );
    assert_eq2!(n2i(0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq2!(n2i(0.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(0.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(0.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 30.0) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq2!(n2i(-0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(-10.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq2!(n2i(-0.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(-3.0, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 30.0) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-5.0, -3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, 5.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-5.0, -3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, 5.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, 3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, f64::INFINITY),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-5.0, -3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(3.0, 5.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-3.0, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(0.0, 3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-0.0, 3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-5.0, -3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(3.0, 5.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-3.0, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-3.0, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(0.0, 3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-0.0, 3.0),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(
        n2i(-2.0, -1.0) / n2i(-10.0, -3.0),
        n2i(0.09999999999999999, 0.6666666666666667)
    );
    assert_eq2!(
        n2i(-2.0, -1.0) / n2i(0.0, 10.0),
        n2i(f64::NEG_INFINITY, -0.09999999999999999)
    );
    assert_eq2!(
        n2i(-2.0, -1.0) / n2i(-0.0, 10.0),
        n2i(f64::NEG_INFINITY, -0.09999999999999999)
    );
    assert_eq2!(n2i(-1.0, 2.0) / n2i(10.0, f64::INFINITY), n2i(-0.1, 0.2));
    assert_eq2!(
        n2i(1.0, 3.0) / n2i(f64::NEG_INFINITY, -10.0),
        n2i(-0.30000000000000004, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -1.0) / n2i(1.0, 3.0),
        n2i(f64::NEG_INFINITY, -0.3333333333333333)
    );
}

#[test]
fn minimal_div_dec_test() {
    assert_eq2!(
        nd2di(-2.0, -1.0, D::Com) / nd2di(-10.0, -3.0, D::Com),
        nd2di(0.09999999999999999, 0.6666666666666667, D::Com)
    );
    assert_eq2!(
        nd2di(-200.0, -1.0, D::Com) / nd2di(5e-324, 10.0, D::Com),
        nd2di(f64::NEG_INFINITY, -0.09999999999999999, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -1.0, D::Com) / nd2di(0.0, 10.0, D::Com),
        nd2di(f64::NEG_INFINITY, -0.09999999999999999, D::Trv)
    );
    assert_eq2!(
        nd2di(1.0, 3.0, D::Def) / nd2di(f64::NEG_INFINITY, -10.0, D::Dac),
        nd2di(-0.30000000000000004, 0.0, D::Def)
    );
    assert_eq2!(nd2di(1.0, 2.0, D::Trv) / DI::EMPTY, DI::EMPTY);
    assert_eq2!(DI::NAI / nd2di(1.0, 2.0, D::Trv), DI::NAI);
}

#[test]
fn minimal_recip_test() {
    assert_eq2!(n2i(-50.0, -10.0).recip(), n2i(-0.1, -0.019999999999999997));
    assert_eq2!(n2i(10.0, 50.0).recip(), n2i(0.019999999999999997, 0.1));
    assert_eq2!(n2i(f64::NEG_INFINITY, -10.0).recip(), n2i(-0.1, 0.0));
    assert_eq2!(n2i(10.0, f64::INFINITY).recip(), n2i(0.0, 0.1));
    assert_eq2!(n2i(0.0, 0.0).recip(), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).recip(), I::EMPTY);
    assert_eq2!(
        n2i(-10.0, 0.0).recip(),
        n2i(f64::NEG_INFINITY, -0.09999999999999999)
    );
    assert_eq2!(
        n2i(-10.0, -0.0).recip(),
        n2i(f64::NEG_INFINITY, -0.09999999999999999)
    );
    assert_eq2!(n2i(-10.0, 10.0).recip(), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 10.0).recip(),
        n2i(0.09999999999999999, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 10.0).recip(),
        n2i(0.09999999999999999, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).recip(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).recip(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, 10.0).recip(), I::ENTIRE);
    assert_eq2!(n2i(-10.0, f64::INFINITY).recip(), I::ENTIRE);
    assert_eq2!(n2i(0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.recip(), I::ENTIRE);
}

#[test]
fn minimal_recip_dec_test() {
    assert_eq2!(
        nd2di(10.0, 50.0, D::Com).recip(),
        nd2di(0.019999999999999997, 0.1, D::Com)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -10.0, D::Dac).recip(),
        nd2di(-0.1, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, -5e-324, D::Def).recip(),
        nd2di(f64::NEG_INFINITY, -5.562684646268003e-309, D::Def)
    );
    assert_eq2!(nd2di(0.0, 0.0, D::Com).recip(), DI::EMPTY);
    assert_eq2!(
        nd2di(-10.0, 0.0, D::Com).recip(),
        nd2di(f64::NEG_INFINITY, -0.09999999999999999, D::Trv)
    );
    assert_eq2!(
        nd2di(-10.0, f64::INFINITY, D::Dac).recip(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Dac).recip(),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).recip(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
}

#[test]
fn minimal_sqr_test() {
    assert_eq2!(I::EMPTY.sqr(), I::EMPTY);
    assert_eq2!(I::ENTIRE.sqr(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -5e-324).sqr(),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-1.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-5.0, 3.0).sqr(), n2i(0.0, 25.0));
    assert_eq2!(n2i(-5.0, 0.0).sqr(), n2i(0.0, 25.0));
    assert_eq2!(n2i(-5.0, -0.0).sqr(), n2i(0.0, 25.0));
    assert_eq2!(n2i(0.1, 0.1).sqr(), n2i(0.01, 0.010000000000000002));
    assert_eq2!(
        n2i(-1.9999999999999964, 0.1).sqr(),
        n2i(0.0, 3.9999999999999862)
    );
    assert_eq2!(
        n2i(-1.9999999999999964, -1.9999999999999964).sqr(),
        n2i(3.999999999999986, 3.9999999999999862)
    );
}

#[test]
fn minimal_sqr_dec_test() {
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, -5e-324, D::Com).sqr(),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(nd2di(-1.0, 1.0, D::Def).sqr(), nd2di(0.0, 1.0, D::Def));
    assert_eq2!(nd2di(-5.0, 3.0, D::Com).sqr(), nd2di(0.0, 25.0, D::Com));
    assert_eq2!(
        nd2di(0.1, 0.1, D::Com).sqr(),
        nd2di(0.01, 0.010000000000000002, D::Com)
    );
}

#[test]
fn minimal_sqrt_test() {
    assert_eq2!(I::EMPTY.sqrt(), I::EMPTY);
    assert_eq2!(I::ENTIRE.sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, -5e-324).sqrt(), I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-5.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq2!(n2i(0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq2!(n2i(-0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq2!(n2i(-5.0, f64::INFINITY).sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.1, 0.1).sqrt(),
        n2i(0.31622776601683794, 0.316227766016838)
    );
    assert_eq2!(
        n2i(-1.9999999999999964, 0.1).sqrt(),
        n2i(0.0, 0.316227766016838)
    );
    assert_eq2!(
        n2i(0.1, 1.9999999999999964).sqrt(),
        n2i(0.31622776601683794, 1.4142135623730938)
    );
}

#[test]
fn minimal_sqrt_dec_test() {
    assert_eq2!(nd2di(1.0, 4.0, D::Com).sqrt(), nd2di(1.0, 2.0, D::Com));
    assert_eq2!(nd2di(-5.0, 25.0, D::Com).sqrt(), nd2di(0.0, 5.0, D::Trv));
    assert_eq2!(nd2di(0.0, 25.0, D::Def).sqrt(), nd2di(0.0, 5.0, D::Def));
    assert_eq2!(
        nd2di(-5.0, f64::INFINITY, D::Dac).sqrt(),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
}

#[test]
fn minimal_fma_test() {
    assert_eq2!(I::EMPTY.mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-1.0, 1.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(0.0, 0.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(1.0, 5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    //min max
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    //end min max
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY),
        I::EMPTY
    );
    assert_eq2!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq2!(
        I::EMPTY.mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 7.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 11.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0)
            .mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0)
            .mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0)
            .mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0)
            .mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 7.0)
    );
    //min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 12.0)
    );
    //end min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 2.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(I::EMPTY.mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(I::ENTIRE.mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 7.0)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(-17.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 11.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, -1.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(-27.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-27.0, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-1.0, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 17.0)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-27.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(-27.0, 7.0)
    );
    //min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-27.0, 17.0)
    );
    assert_eq2!(
        n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-32.0, 52.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, 2.0)),
        n2i(-12.0, 52.0)
    );
    assert_eq2!(
        n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-12.0, 12.0)
    );
    //end min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-5.0, 17.0)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)),
        n2i(-2.0, 2.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)),
        n2i(3.0, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-32.0, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)),
        n2i(-32.0, -3.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)),
        n2i(-32.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, 52.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)),
        n2i(f64::NEG_INFINITY, -3.0)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)),
        I::ENTIRE
    );
    assert_eq2!(
        I::EMPTY.mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-17.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0)
            .mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-27.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-27.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-27.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        n2i(-1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(-27.0, f64::INFINITY)
    );
    //min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-27.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-32.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, f64::INFINITY)),
        n2i(-12.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-12.0, f64::INFINITY)
    );
    //end min max
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-5.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-32.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-32.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)),
        n2i(3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)),
        n2i(-32.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)),
        I::ENTIRE
    );
    assert_eq2!(I::EMPTY.mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-1.0, 1.0), I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(I::ENTIRE, I::ENTIRE), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(0.0, 0.0), I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::EMPTY);
    assert_eq2!(I::ENTIRE.mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(I::ENTIRE.mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(0.0, 0.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(1.0, 5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    //min max
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    //end min max
    assert_eq2!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE),
        I::ENTIRE
    );
    assert_eq2!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq2!(
        n2i(0.1, 0.5).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)),
        n2i(-2.6, 1.6)
    );
    assert_eq2!(
        n2i(-0.5, 0.2).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)),
        n2i(-1.6, 2.6)
    );
    assert_eq2!(
        n2i(-0.5, -0.1).mul_add(n2i(2.0, 3.0), n2i(-0.1, 0.1)),
        n2i(-1.6, -0.1)
    );
    assert_eq2!(
        n2i(-0.5, -0.1).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-0.1, 0.1)),
        n2i(-1.6, f64::INFINITY)
    );
}

#[test]
fn minimal_fma_dec_test() {
    assert_eq2!(
        nd2di(-0.5, -0.1, D::Com).mul_add(
            nd2di(f64::NEG_INFINITY, 3.0, D::Dac),
            nd2di(-0.1, 0.1, D::Com)
        ),
        nd2di(-1.6, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com).mul_add(
            nd2di(1.0, 1.7976931348623157e+308, D::Com),
            nd2di(0.0, 1.0, D::Com)
        ),
        nd2di(1.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com).mul_add(nd2di(1.0, 2.0, D::Com), nd2di(2.0, 5.0, D::Com)),
        nd2di(3.0, 9.0, D::Com)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pown_test() {
    assert_eq2!(I::EMPTY.powi(0), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.0).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(13.1, 13.1).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(-7451.145, -7451.145).powi(0), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(0),
        n2i(1.0, 1.0)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(0),
        n2i(1.0, 1.0)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).powi(0), n2i(1.0, 1.0));
    assert_eq2!(n2i(-324.3, 2.5).powi(0), n2i(1.0, 1.0));
    assert_eq2!(I::EMPTY.powi(2), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(2), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).powi(2), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(2), n2i(0.0, 0.0));
    assert_eq2!(n2i(13.1, 13.1).powi(2), n2i(171.60999999999999, 171.61));
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(2),
        n2i(55519561.811025, 55519561.81102501)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(2),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(2),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(2), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(2), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).powi(2), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(2),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(2), n2i(0.0, 105170.49000000002));
    assert_eq2!(
        n2i(0.01, 2.33).powi(2),
        n2i(9.999999999999999e-05, 5.4289000000000005)
    );
    assert_eq2!(n2i(-1.9, -0.33).powi(2), n2i(0.1089, 3.61));
    assert_eq2!(I::EMPTY.powi(8), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(8), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).powi(8), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(8), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(13.1, 13.1).powi(8),
        n2i(867302034.6900622, 867302034.6900623)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(8),
        n2i(9.501323805961965e+30, 9.501323805961966e+30)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(8),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(8),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(8), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(8), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).powi(8), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(8),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(8), n2i(0.0, 1.2234200379867188e+20));
    assert_eq2!(
        n2i(0.01, 2.33).powi(8),
        n2i(1.0000000000000001e-16, 868.6550888106664)
    );
    assert_eq2!(
        n2i(-1.9, -0.33).powi(8),
        n2i(0.00014064086182410005, 169.83563040999996)
    );
    assert_eq2!(I::EMPTY.powi(1), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(1), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(1), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(1), n2i(0.0, 0.0));
    assert_eq2!(n2i(13.1, 13.1).powi(1), n2i(13.1, 13.1));
    assert_eq2!(n2i(-7451.145, -7451.145).powi(1), n2i(-7451.145, -7451.145));
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(1),
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(1),
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(1), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(1), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(1),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(1),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(1), n2i(-324.3, 2.5));
    assert_eq2!(n2i(0.01, 2.33).powi(1), n2i(0.01, 2.33));
    assert_eq2!(n2i(-1.9, -0.33).powi(1), n2i(-1.9, -0.33));
    assert_eq2!(I::EMPTY.powi(3), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(3), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(3), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(3), n2i(0.0, 0.0));
    assert_eq2!(n2i(13.1, 13.1).powi(3), n2i(2248.0909999999994, 2248.091));
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(3),
        n2i(-413684305390.41, -413684305390.4099)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(3),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(3),
        n2i(f64::NEG_INFINITY, -1.7976931348623157e+308)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(3), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(3), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(3),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(3),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(3), n2i(-34106789.907000005, 15.625));
    assert_eq2!(n2i(0.01, 2.33).powi(3), n2i(1e-06, 12.649337000000003));
    assert_eq2!(
        n2i(-1.9, -0.33).powi(3),
        n2i(-6.858999999999999, -0.035937000000000004)
    );
    assert_eq2!(I::EMPTY.powi(7), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(7), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(7), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).powi(7), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(13.1, 13.1).powi(7),
        n2i(66206262.19008108, 66206262.19008109)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(7),
        n2i(-1.2751494979579603e+27, -1.27514949795796e+27)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(7),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(7),
        n2i(f64::NEG_INFINITY, -1.7976931348623157e+308)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(7), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(7), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(7),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(7),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-324.3, 2.5).powi(7),
        n2i(-3.77249472089645e+17, 610.3515625)
    );
    assert_eq2!(n2i(0.01, 2.33).powi(7), n2i(1e-14, 372.8133428371959));
    assert_eq2!(
        n2i(-1.9, -0.33).powi(7),
        n2i(-89.38717389999998, -0.0004261844297700001)
    );
    assert_eq2!(I::EMPTY.powi(-2), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(-2), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).powi(-2), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).powi(-2), I::EMPTY);
    assert_eq2!(
        n2i(13.1, 13.1).powi(-2),
        n2i(0.005827166249053085, 0.005827166249053086)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(-2),
        n2i(1.8011669533771807e-08, 1.801166953377181e-08)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(-2),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(-2),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(-2), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(-2), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(-2),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(-2),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-324.3, 2.5).powi(-2),
        n2i(9.508370646556842e-06, f64::INFINITY)
    );
    assert_eq2!(n2i(0.01, 2.33).powi(-2), n2i(0.18419937740610434, 10000.0));
    assert_eq2!(
        n2i(-1.9, -0.33).powi(-2),
        n2i(0.2770083102493075, 9.182736455463727)
    );
    assert_eq2!(I::EMPTY.powi(-8), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(-8), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).powi(-8), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).powi(-8), I::EMPTY);
    assert_eq2!(
        n2i(13.1, 13.1).powi(-8),
        n2i(1.1530008693653744e-09, 1.1530008693653746e-09)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(-8),
        n2i(1.0524849172833286e-31, 1.0524849172833288e-31)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(-8),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(-8),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(-8), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(-8), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(-8),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(-8),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-324.3, 2.5).powi(-8),
        n2i(8.173807596331488e-21, f64::INFINITY)
    );
    assert_eq2!(n2i(0.01, 2.33).powi(-8), n2i(0.0011512049061603571, 1e+16));
    assert_eq2!(
        n2i(-1.9, -0.33).powi(-8),
        n2i(0.005888045974722156, 7110.309102419345)
    );
    assert_eq2!(I::EMPTY.powi(-1), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(-1), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(-1), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).powi(-1), I::EMPTY);
    assert_eq2!(
        n2i(13.1, 13.1).powi(-1),
        n2i(0.07633587786259541, 0.07633587786259542)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(-1),
        n2i(-0.00013420756138821617, -0.00013420756138821614)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(-1),
        n2i(5.562684646268003e-309, 5.56268464626801e-309)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(-1),
        n2i(-5.56268464626801e-309, -5.562684646268003e-309)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(-1), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(-1), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(-1),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(-1),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(-1), I::ENTIRE);
    assert_eq2!(n2i(0.01, 2.33).powi(-1), n2i(0.42918454935622313, 100.0));
    assert_eq2!(
        n2i(-1.9, -0.33).powi(-1),
        n2i(-3.0303030303030303, -0.5263157894736842)
    );
    assert_eq2!(I::EMPTY.powi(-3), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(-3), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(-3), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).powi(-3), I::EMPTY);
    assert_eq2!(
        n2i(13.1, 13.1).powi(-3),
        n2i(0.0004448218510727546, 0.00044482185107275467)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(-3),
        n2i(-2.4173022446579435e-12, -2.417302244657943e-12)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(-3),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(-3),
        n2i(-5e-324, -0.0)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(-3), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(-3), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(-3),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(-3),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(-3), I::ENTIRE);
    assert_eq2!(
        n2i(0.01, 2.33).powi(-3),
        n2i(0.07905552678373577, 1000000.0)
    );
    assert_eq2!(
        n2i(-1.9, -0.33).powi(-3),
        n2i(-27.82647410746584, -0.14579384749963553)
    );
    assert_eq2!(I::EMPTY.powi(-7), I::EMPTY);
    assert_eq2!(I::ENTIRE.powi(-7), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).powi(-7), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).powi(-7), I::EMPTY);
    assert_eq2!(
        n2i(13.1, 13.1).powi(-7),
        n2i(1.5104311388686403e-08, 1.5104311388686407e-08)
    );
    assert_eq2!(
        n2i(-7451.145, -7451.145).powi(-7),
        n2i(-7.842217728991088e-28, -7.842217728991087e-28)
    );
    assert_eq2!(
        n2i(1.7976931348623157e+308, 1.7976931348623157e+308).powi(-7),
        n2i(0.0, 5e-324)
    );
    assert_eq2!(
        n2i(-1.7976931348623157e+308, -1.7976931348623157e+308).powi(-7),
        n2i(-5e-324, -0.0)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).powi(-7), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).powi(-7), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).powi(-7),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).powi(-7),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(-324.3, 2.5).powi(-7), I::ENTIRE);
    assert_eq2!(
        n2i(0.01, 2.33).powi(-7),
        n2i(0.002682307431353632, 100000000000000.0)
    );
    assert_eq2!(
        n2i(-1.9, -0.33).powi(-7),
        n2i(-2346.402003798384, -0.011187287351972096)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pown_dec_test() {
    assert_eq2!(nd2di(-5.0, 10.0, D::Com).powi(0), nd2di(1.0, 1.0, D::Com));
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 15.0, D::Dac).powi(0),
        nd2di(1.0, 1.0, D::Dac)
    );
    assert_eq2!(nd2di(-3.0, 5.0, D::Def).powi(2), nd2di(0.0, 25.0, D::Def));
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, 2.0, D::Com).powi(2),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-3.0, 5.0, D::Dac).powi(3),
        nd2di(-27.0, 125.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, 2.0, D::Com).powi(3),
        nd2di(f64::NEG_INFINITY, 8.0, D::Dac)
    );
    assert_eq2!(
        nd2di(3.0, 5.0, D::Com).powi(-2),
        nd2di(0.039999999999999994, 0.11111111111111112, D::Com)
    );
    assert_eq2!(
        nd2di(-5.0, -3.0, D::Def).powi(-2),
        nd2di(0.039999999999999994, 0.11111111111111112, D::Def)
    );
    assert_eq2!(
        nd2di(-5.0, 3.0, D::Com).powi(-2),
        nd2di(0.039999999999999994, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(3.0, 5.0, D::Dac).powi(-3),
        nd2di(0.007999999999999998, 0.03703703703703704, D::Dac)
    );
    assert_eq2!(
        nd2di(-3.0, 5.0, D::Com).powi(-3),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pow_test() {
    assert_eq2!(I::EMPTY.pow(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(0.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(-0.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(1.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(-3.0, 5.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(-5.0, -5.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.pow(n2i(5.0, 5.0)), I::EMPTY);
    assert_eq2!(n2i(0.1, 0.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(0.0, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-0.0, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(0.0, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.0, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(0.1, 0.1)),
        n2i(0.7943282347242815, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(0.1, 1.0)),
        n2i(0.1, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(0.1, 2.5)),
        n2i(0.0031622776601683794, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(1.0, 1.0)), n2i(0.1, 0.5));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(1.0, 2.5)),
        n2i(0.0031622776601683794, 0.5)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.5));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(2.5, 2.5)),
        n2i(0.0031622776601683794, 0.1767766952966369)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.1, 0.1)),
        n2i(0.7943282347242815, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.1, 1.0)),
        n2i(0.1, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.1, 2.5)),
        n2i(0.0031622776601683794, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-1.0, 0.1)),
        n2i(0.7943282347242815, 10.0)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-1.0, 1.0)), n2i(0.1, 10.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-1.0, 2.5)),
        n2i(0.0031622776601683794, 10.0)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 10.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, 0.1)),
        n2i(0.7943282347242815, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, 1.0)),
        n2i(0.1, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, 2.5)),
        n2i(0.0031622776601683794, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.7943282347242815, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0031622776601683794, f64::INFINITY)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-1.0, 0.0)), n2i(1.0, 10.0));
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-1.0, -0.0)), n2i(1.0, 10.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, 0.0)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, -0.0)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-0.1, -0.1)),
        n2i(1.0717734625362931, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-1.0, -0.1)),
        n2i(1.0717734625362931, 10.0)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, -0.1)),
        n2i(1.0717734625362931, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(n2i(0.1, 0.5).pow(n2i(-1.0, -1.0)), n2i(2.0, 10.0));
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, -1.0)),
        n2i(2.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(-2.5, -2.5)),
        n2i(5.65685424949238, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(0.0, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-0.0, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(0.0, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.0, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(0.1, 0.1)),
        n2i(0.7943282347242815, 1.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(0.1, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(0.1, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(1.0, 1.0)), n2i(0.1, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(1.0, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(2.5, 2.5)),
        n2i(0.0031622776601683794, 1.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, 0.1)),
        n2i(0.7943282347242815, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, 1.0)),
        n2i(0.1, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, 2.5)),
        n2i(0.0031622776601683794, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-1.0, 0.1)),
        n2i(0.7943282347242815, 10.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, 1.0)), n2i(0.1, 10.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-1.0, 2.5)),
        n2i(0.0031622776601683794, 10.0)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 10.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, 0.1)),
        n2i(0.7943282347242815, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, 1.0)),
        n2i(0.1, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, 2.5)),
        n2i(0.0031622776601683794, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.7943282347242815, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0031622776601683794, f64::INFINITY)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, 0.0)),
        n2i(1.0, 1.2589254117941673)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, -0.0)),
        n2i(1.0, 1.2589254117941673)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, 0.0)), n2i(1.0, 10.0));
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, -0.0)), n2i(1.0, 10.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, 0.0)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, -0.0)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-0.1, -0.1)),
        n2i(1.0, 1.2589254117941673)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, -0.1)), n2i(1.0, 10.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, -0.1)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.1, 1.0).pow(n2i(-1.0, -1.0)), n2i(1.0, 10.0));
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, -1.0)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(-2.5, -2.5)),
        n2i(1.0, 316.2277660168379)
    );
    assert_eq2!(
        n2i(0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(0.0, 1.0)), n2i(0.5, 1.5));
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(-0.0, 1.0)), n2i(0.5, 1.5));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(0.0, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(0.1, 0.1)),
        n2i(0.9330329915368074, 1.0413797439924106)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(0.1, 1.0)), n2i(0.5, 1.5));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(0.1, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(1.0, 1.0)), n2i(0.5, 1.5));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(1.0, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(2.5, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.1, 0.1)),
        n2i(0.9330329915368074, 1.0717734625362934)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(-0.1, 1.0)), n2i(0.5, 1.5));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.1, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, 0.1)),
        n2i(0.6666666666666666, 2.0)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(n2i(-1.0, 1.0)), n2i(0.5, 2.0));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, 2.5)),
        n2i(0.17677669529663687, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, 0.1)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, 1.0)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, 2.5)),
        n2i(0.17677669529663687, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, 2.0)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, 2.0)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, 1.0717734625362934)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, 2.0)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, 2.0)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(0.5, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(0.5, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(0.17677669529663687, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(0.17677669529663687, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(0.9330329915368074, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(0.5, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(0.17677669529663687, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(0.5, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(0.17677669529663687, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(0.17677669529663687, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(-1.0, 0.0)), n2i(0.0, 2.0));
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(-1.0, -0.0)), n2i(0.0, 2.0));
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, 0.0)),
        n2i(0.0, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, -0.0)),
        n2i(0.0, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-0.1, -0.1)),
        n2i(0.0, 1.0717734625362934)
    );
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(-1.0, -0.1)), n2i(0.0, 2.0));
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, -0.1)),
        n2i(0.0, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.5, f64::INFINITY).pow(n2i(-1.0, -1.0)), n2i(0.0, 2.0));
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, -1.0)),
        n2i(0.0, 5.656854249492381)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.5, f64::INFINITY).pow(n2i(-2.5, -2.5)),
        n2i(0.0, 5.656854249492381)
    );
    assert_eq2!(n2i(1.0, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(0.0, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(-0.0, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(0.0, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(0.1, 0.1)),
        n2i(1.0, 1.0413797439924106)
    );
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(0.1, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(0.1, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, 1.5).pow(n2i(1.0, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(1.0, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(2.5, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.1, 0.1)),
        n2i(0.960264500792218, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.1, 1.0)),
        n2i(0.960264500792218, 1.5)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.1, 2.5)),
        n2i(0.960264500792218, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.960264500792218, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, 0.1)),
        n2i(0.6666666666666666, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, 1.0)),
        n2i(0.6666666666666666, 1.5)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, 2.5)),
        n2i(0.6666666666666666, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, 0.1)),
        n2i(0.3628873693012115, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, 1.0)),
        n2i(0.3628873693012115, 1.5)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, 2.5)),
        n2i(0.3628873693012115, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 1.5)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(n2i(1.0, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.0, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-1.0, 0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-1.0, -0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-2.5, 0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-2.5, -0.0)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-0.1, -0.1)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-1.0, -0.1)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-2.5, -0.1)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-1.0, -1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-2.5, -1.0)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).pow(n2i(-2.5, -2.5)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(1.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(n2i(1.1, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.1, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.1, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.1, 1.5).pow(n2i(0.0, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(n2i(1.1, 1.5).pow(n2i(-0.0, 1.0)), n2i(1.0, 1.5));
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.0, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(1.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.1, 0.1)),
        n2i(1.009576582776887, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.1, 1.0)),
        n2i(1.009576582776887, 1.5)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.1, 2.5)),
        n2i(1.009576582776887, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(1.009576582776887, f64::INFINITY)
    );
    assert_eq2!(n2i(1.1, 1.5).pow(n2i(1.0, 1.0)), n2i(1.1, 1.5));
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(1.0, 2.5)),
        n2i(1.1, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(1.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(2.5, 2.5)),
        n2i(1.2690587062858836, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(1.2690587062858836, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.1, 0.1)),
        n2i(0.960264500792218, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.1, 1.0)),
        n2i(0.960264500792218, 1.5)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.1, 2.5)),
        n2i(0.960264500792218, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.960264500792218, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, 0.1)),
        n2i(0.6666666666666666, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, 1.0)),
        n2i(0.6666666666666666, 1.5)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, 2.5)),
        n2i(0.6666666666666666, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, 0.1)),
        n2i(0.3628873693012115, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, 1.0)),
        n2i(0.3628873693012115, 1.5)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, 2.5)),
        n2i(0.3628873693012115, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 1.0413797439924106)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 1.5)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(n2i(1.1, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, 0.7879856109467704)
    );
    assert_eq2!(
        n2i(1.1, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, 0.7879856109467704)
    );
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(1.009576582776887, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(1.009576582776887, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(1.009576582776887, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(1.009576582776887, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(1.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(1.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(1.1, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(1.2690587062858836, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(1.2690587062858836, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(-1.0, 0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(-1.0, -0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(-2.5, 0.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(1.1, f64::INFINITY).pow(n2i(-2.5, -0.0)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, 1.0)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-0.1, -0.1)),
        n2i(0.0, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, -0.1)),
        n2i(0.0, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, -0.1)),
        n2i(0.0, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, 0.9905142582145219)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-1.0, -1.0)),
        n2i(0.0, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, -1.0)),
        n2i(0.0, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, 0.9090909090909091)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(-2.5, -2.5)),
        n2i(0.0, 0.7879856109467704)
    );
    assert_eq2!(
        n2i(1.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, 0.7879856109467704)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(0.1, 1.0)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(1.0, 1.0)), n2i(0.0, 0.5));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(1.0, 2.5)), n2i(0.0, 0.5));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.5));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-0.1, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-1.0, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-2.5, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-1.0, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.5).pow(n2i(-2.5, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(-2.5, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.1, 0.1)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.1, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.1, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(1.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(1.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(2.5, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-0.1, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-1.0, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.0).pow(n2i(-2.5, -2.5)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 1.0413797439924106)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(0.1, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(1.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(1.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 1.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-0.1, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-1.0, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).pow(n2i(-2.5, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(0.1, 1.0)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(1.0, 1.0)), n2i(0.0, 0.5));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(1.0, 2.5)), n2i(0.0, 0.5));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.5));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-0.1, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-1.0, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-2.5, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-1.0, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 0.5).pow(n2i(-2.5, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(-2.5, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 0.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.1, 0.1)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.1, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.1, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(1.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(1.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(2.5, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-0.1, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-1.0, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.0).pow(n2i(-2.5, -2.5)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.0).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 1.0413797439924106)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(0.1, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(1.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(1.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, 1.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.0, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-0.1, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-1.0, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).pow(n2i(-2.5, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(0.1, 1.0)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, 0.9330329915368075)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(1.0, 1.0)), n2i(0.0, 0.5));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(1.0, 2.5)), n2i(0.0, 0.5));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.5));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, 0.1767766952966369)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-0.1, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-1.0, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-2.5, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0717734625362931, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-1.0, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 0.5).pow(n2i(-2.5, -1.0)), n2i(2.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(-2.5, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 0.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(5.65685424949238, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.1, 0.1)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.1, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.1, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(1.0, 1.0)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(1.0, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(2.5, 2.5)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, 0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, -0.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-0.1, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, -0.1)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-1.0, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, -1.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.0).pow(n2i(-2.5, -2.5)), n2i(1.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.0).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(1.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-0.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-0.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(0.1, 0.1)),
        n2i(0.0, 1.0413797439924106)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(0.1, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(0.1, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(1.0, 1.0)), n2i(0.0, 1.5));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(1.0, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(2.5, 2.5)),
        n2i(0.0, 2.7556759606310757)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-0.1, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-0.1, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-0.1, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-1.0, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-1.0, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-1.0, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-2.5, 0.1)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-2.5, 1.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.1, 1.5).pow(n2i(-2.5, 2.5)), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, 1.5).pow(I::ENTIRE), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-1.0, 0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-1.0, -0.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, 0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, -0.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-0.1, -0.1)),
        n2i(0.960264500792218, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-1.0, -0.1)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, -0.1)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-1.0, -1.0)),
        n2i(0.6666666666666666, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, -1.0)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(-2.5, -2.5)),
        n2i(0.3628873693012115, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, 1.5).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-0.1, f64::INFINITY).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.1, f64::INFINITY).pow(n2i(0.0, 0.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.1, f64::INFINITY).pow(n2i(-0.0, -0.0)), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.1, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.1, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.1, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.1, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, f64::INFINITY)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(I::ENTIRE),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, 0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-0.1, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -0.1)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-1.0, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -1.0)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(f64::NEG_INFINITY, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-0.1, f64::INFINITY).pow(n2i(-2.5, -2.5)),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(0.0, 0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, -0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-0.0, 0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(0.0, -0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-1.0, 0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, 0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.1, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.1, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.1, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, 0.1)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, 2.5)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.1)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, 1.0)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, 2.5)),
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-1.0, -0.0).pow(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.0).pow(n2i(-2.5, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.0, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.0, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.0, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.0, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.1, 0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.1, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.1, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(0.1, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(1.0, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(1.0, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(1.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(2.5, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(2.5, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.1, 0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.1, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.1, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.1, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, 0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, 0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, 0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, 1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, 2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-0.1, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, -0.1)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-1.0, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, -1.0)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(f64::NEG_INFINITY, -2.5)), I::EMPTY);
    assert_eq2!(n2i(-1.0, -0.1).pow(n2i(-2.5, -2.5)), I::EMPTY);
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pow_dec_test() {
    assert_eq2!(
        nd2di(0.1, 0.5, D::Com).pow(nd2di(0.0, 1.0, D::Com)),
        nd2di(0.1, 1.0, D::Com)
    );
    assert_eq2!(
        nd2di(0.1, 0.5, D::Com).pow(nd2di(0.1, 0.1, D::Def)),
        nd2di(0.7943282347242815, 0.9330329915368075, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 0.5, D::Trv).pow(nd2di(-2.5, 2.5, D::Dac)),
        nd2di(0.0031622776601683794, 316.2277660168379, D::Trv)
    );
    assert_eq2!(
        nd2di(0.1, 0.5, D::Com).pow(nd2di(-2.5, f64::INFINITY, D::Dac)),
        nd2di(0.0, 316.2277660168379, D::Dac)
    );
    assert_eq2!(
        nd2di(0.1, 0.5, D::Trv).pow(nd2di(f64::NEG_INFINITY, 0.1, D::Dac)),
        nd2di(0.7943282347242815, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Com).pow(nd2di(0.0, 2.5, D::Com)),
        nd2di(0.0031622776601683794, 1.0, D::Com)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Def).pow(nd2di(1.0, 1.0, D::Dac)),
        nd2di(0.1, 1.0, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Trv).pow(nd2di(-2.5, 1.0, D::Def)),
        nd2di(0.1, 316.2277660168379, D::Trv)
    );
    assert_eq2!(
        nd2di(0.5, 1.5, D::Dac).pow(nd2di(0.1, 0.1, D::Com)),
        nd2di(0.9330329915368074, 1.0413797439924106, D::Dac)
    );
    assert_eq2!(
        nd2di(0.5, 1.5, D::Def).pow(nd2di(-2.5, 0.1, D::Trv)),
        nd2di(0.3628873693012115, 5.656854249492381, D::Trv)
    );
    assert_eq2!(
        nd2di(0.5, 1.5, D::Com).pow(nd2di(-2.5, -2.5, D::Com)),
        nd2di(0.3628873693012115, 5.656854249492381, D::Com)
    );
    assert_eq2!(
        nd2di(0.5, f64::INFINITY, D::Dac).pow(nd2di(0.1, 0.1, D::Com)),
        nd2di(0.9330329915368074, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.5, f64::INFINITY, D::Def).pow(nd2di(-2.5, -0.0, D::Com)),
        nd2di(0.0, 5.656854249492381, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 1.5, D::Com).pow(nd2di(-0.1, 0.1, D::Def)),
        nd2di(0.960264500792218, 1.0413797439924106, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 1.5, D::Trv).pow(nd2di(-0.1, -0.1, D::Com)),
        nd2di(0.960264500792218, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(1.0, f64::INFINITY, D::Dac).pow(nd2di(1.0, 1.0, D::Dac)),
        nd2di(1.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, f64::INFINITY, D::Def).pow(nd2di(-1.0, -0.0, D::Dac)),
        nd2di(0.0, 1.0, D::Def)
    );
    assert_eq2!(
        nd2di(1.1, 1.5, D::Def).pow(nd2di(1.0, 2.5, D::Com)),
        nd2di(1.1, 2.7556759606310757, D::Def)
    );
    assert_eq2!(
        nd2di(1.1, 1.5, D::Com).pow(nd2di(-0.1, -0.1, D::Com)),
        nd2di(0.960264500792218, 0.9905142582145219, D::Com)
    );
    assert_eq2!(
        nd2di(1.1, f64::INFINITY, D::Dac).pow(nd2di(0.1, f64::INFINITY, D::Dac)),
        nd2di(1.009576582776887, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(1.1, f64::INFINITY, D::Def).pow(nd2di(-2.5, f64::INFINITY, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(1.1, f64::INFINITY, D::Trv).pow(nd2di(f64::NEG_INFINITY, -1.0, D::Def)),
        nd2di(0.0, 0.9090909090909091, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.5, D::Com).pow(nd2di(0.1, 0.1, D::Com)),
        nd2di(0.0, 0.9330329915368075, D::Com)
    );
    assert_eq2!(
        nd2di(0.0, 0.5, D::Com).pow(nd2di(2.5, f64::INFINITY, D::Dac)),
        nd2di(0.0, 0.1767766952966369, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 0.5, D::Com).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Dac)),
        nd2di(5.65685424949238, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Com).pow(nd2di(0.0, 0.0, D::Com)),
        nd2di(1.0, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Def).pow(nd2di(0.0, 2.5, D::Dac)),
        nd2di(0.0, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).pow(nd2di(1.0, 2.5, D::Com)),
        nd2di(0.0, 1.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Com).pow(nd2di(-2.5, 0.1, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Def).pow(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).pow(nd2di(-0.1, 0.0, D::Com)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Com).pow(nd2di(f64::NEG_INFINITY, 0.0, D::Dac)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Def).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Dac)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.5, D::Com).pow(nd2di(0.0, 2.5, D::Com)),
        nd2di(0.0, 2.7556759606310757, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.5, D::Def).pow(nd2di(2.5, 2.5, D::Dac)),
        nd2di(0.0, 2.7556759606310757, D::Def)
    );
    assert_eq2!(
        nd2di(0.0, 1.5, D::Dac).pow(nd2di(-1.0, 0.0, D::Com)),
        nd2di(0.6666666666666666, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.5, D::Com).pow(nd2di(-2.5, -2.5, D::Def)),
        nd2di(0.3628873693012115, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).pow(nd2di(0.1, 0.1, D::Com)),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Def).pow(nd2di(-1.0, 1.0, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Trv).pow(nd2di(f64::NEG_INFINITY, -1.0, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).pow(nd2di(-2.5, -2.5, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Com).pow(nd2di(0.0, f64::INFINITY, D::Dac)),
        nd2di(0.0, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Def).pow(nd2di(0.1, f64::INFINITY, D::Def)),
        nd2di(0.0, 0.9330329915368075, D::Def)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Dac).pow(nd2di(2.5, 2.5, D::Com)),
        nd2di(0.0, 0.1767766952966369, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Trv).pow(nd2di(-2.5, -0.0, D::Dac)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Com).pow(nd2di(f64::NEG_INFINITY, -0.1, D::Def)),
        nd2di(1.0717734625362931, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.5, D::Def).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Dac)),
        nd2di(5.65685424949238, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Com).pow(nd2di(2.5, 2.5, D::Dac)),
        nd2di(0.0, 1.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).pow(nd2di(-1.0, f64::INFINITY, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Com).pow(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Def).pow(nd2di(-2.5, -2.5, D::Com)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Def)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5, D::Com).pow(nd2di(0.1, 2.5, D::Dac)),
        nd2di(0.0, 2.7556759606310757, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5, D::Def).pow(nd2di(-1.0, 0.0, D::Trv)),
        nd2di(0.6666666666666666, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5, D::Dac).pow(nd2di(-2.5, -0.1, D::Def)),
        nd2di(0.3628873693012115, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5, D::Com).pow(nd2di(-2.5, -2.5, D::Com)),
        nd2di(0.3628873693012115, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5, D::Def).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Dac).pow(nd2di(-0.1, f64::INFINITY, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Def).pow(nd2di(-2.5, -0.0, D::Com)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Trv).pow(nd2di(f64::NEG_INFINITY, 0.0, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Dac).pow(nd2di(f64::NEG_INFINITY, -0.0, D::Trv)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Def).pow(nd2di(f64::NEG_INFINITY, -1.0, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 0.5, D::Def).pow(nd2di(0.1, f64::INFINITY, D::Dac)),
        nd2di(0.0, 0.9330329915368075, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 0.5, D::Com).pow(nd2di(-0.1, -0.1, D::Com)),
        nd2di(1.0717734625362931, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 0.5, D::Dac).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Def)),
        nd2di(5.65685424949238, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.0, D::Com).pow(nd2di(0.0, 0.0, D::Com)),
        nd2di(1.0, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.0, D::Dac).pow(nd2di(f64::NEG_INFINITY, 2.5, D::Dac)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.0, D::Def).pow(nd2di(f64::NEG_INFINITY, -1.0, D::Def)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.0, D::Com).pow(nd2di(-2.5, -2.5, D::Com)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.0, D::Trv).pow(nd2di(f64::NEG_INFINITY, -2.5, D::Trv)),
        nd2di(1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.5, D::Trv).pow(nd2di(0.0, 2.5, D::Com)),
        nd2di(0.0, 2.7556759606310757, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.5, D::Com).pow(nd2di(2.5, 2.5, D::Dac)),
        nd2di(0.0, 2.7556759606310757, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.5, D::Dac).pow(nd2di(-1.0, 0.0, D::Trv)),
        nd2di(0.6666666666666666, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.5, D::Com).pow(nd2di(-0.1, -0.1, D::Com)),
        nd2di(0.960264500792218, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, 1.5, D::Def).pow(nd2di(-2.5, -2.5, D::Def)),
        nd2di(0.3628873693012115, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, f64::INFINITY, D::Dac).pow(nd2di(-0.1, 2.5, D::Com)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, f64::INFINITY, D::Def).pow(nd2di(-2.5, 0.0, D::Def)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.1, f64::INFINITY, D::Dac).pow(nd2di(-2.5, -2.5, D::Trv)),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).pow(nd2di(1.0, f64::INFINITY, D::Dac)),
        nd2di(0.0, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).pow(nd2di(-2.5, 0.1, D::Com)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).pow(nd2di(-1.0, 0.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-1.0, -0.1, D::Com).pow(nd2di(-0.1, 1.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-1.0, -0.1, D::Dac).pow(nd2di(-0.1, 2.5, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-1.0, -0.1, D::Def).pow(nd2di(-0.1, f64::INFINITY, D::Trv)),
        DI::EMPTY
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp_test() {
    assert_eq2!(I::EMPTY.exp(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).exp(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).exp(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.exp(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 709.7827128933841).exp(),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(709.7827128933841, 709.7827128933841).exp(),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 709.7827128933823).exp(),
        n2i(1.0, 1.7976931348592078e+308)
    );
    assert_eq2!(
        n2i(-0.0, 709.7827128933823).exp(),
        n2i(1.0, 1.7976931348592078e+308)
    );
    assert_eq2!(
        n2i(-708.3964185322642, 709.7827128933823).exp(),
        n2i(2.225073858507009e-308, 1.7976931348592078e+308)
    );
    assert_eq2!(
        n2i(-354.1982092661321, 709.7827128933823).exp(),
        n2i(1.491668146239977e-154, 1.7976931348592078e+308)
    );
    assert_eq2!(
        n2i(-354.1982092661321, 0.0).exp(),
        n2i(1.491668146239977e-154, 1.0)
    );
    assert_eq2!(
        n2i(-354.1982092661321, -0.0).exp(),
        n2i(1.491668146239977e-154, 1.0)
    );
    assert_eq2!(
        n2i(-354.1982092661321, 1.0).exp(),
        n2i(1.491668146239977e-154, 2.7182818284590455)
    );
    assert_eq2!(
        n2i(1.0, 5.0).exp(),
        n2i(2.718281828459045, 148.41315910257663)
    );
    assert_eq2!(
        n2i(-3.3219280948873626, 1.8073549220576042).exp(),
        n2i(0.036083192820787195, 6.094306179356356)
    );
    assert_eq2!(
        n2i(0.7655347463629769, 98.83328352961747).exp(),
        n2i(2.1501438488923172, 8.370466551497384e+42)
    );
    assert_eq2!(
        n2i(11.750288269015629, 25.990462258377743).exp(),
        n2i(126790.10339648015, 193871665366.00427)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp_dec_test() {
    assert_eq2!(
        nd2di(709.7827128933841, 709.7827128933841, D::Com).exp(),
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 709.7827128933823, D::Def).exp(),
        nd2di(1.0, 1.7976931348592078e+308, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp2_test() {
    assert_eq2!(I::EMPTY.exp2(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).exp2(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).exp2(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.exp2(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1024.0).exp2(),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(1024.0, 1024.0).exp2(),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1023.0).exp2(), n2i(1.0, 8.98846567431158e+307));
    assert_eq2!(n2i(-0.0, 1023.0).exp2(), n2i(1.0, 8.98846567431158e+307));
    assert_eq2!(
        n2i(-1022.0, 1023.0).exp2(),
        n2i(2.2250738585072014e-308, 8.98846567431158e+307)
    );
    assert_eq2!(n2i(-1022.0, 0.0).exp2(), n2i(2.2250738585072014e-308, 1.0));
    assert_eq2!(n2i(-1022.0, -0.0).exp2(), n2i(2.2250738585072014e-308, 1.0));
    assert_eq2!(n2i(-1022.0, 1.0).exp2(), n2i(2.2250738585072014e-308, 2.0));
    assert_eq2!(n2i(1.0, 5.0).exp2(), n2i(2.0, 32.0));
    assert_eq2!(
        n2i(-3.3219280948873626, 1.8073549220576042).exp2(),
        n2i(0.09999999999999998, 3.5000000000000004)
    );
    assert_eq2!(
        n2i(0.7655347463629769, 98.83328352961747).exp2(),
        n2i(1.6999999999999997, 5.646546544444467e+29)
    );
    assert_eq2!(
        n2i(11.750288269015629, 25.990462258377743).exp2(),
        n2i(3445.0000034002983, 66666666.666666694)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp2_dec_test() {
    assert_eq2!(
        nd2di(1024.0, 1024.0, D::Com).exp2(),
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.7655347463629769, 98.83328352961747, D::Def).exp2(),
        nd2di(1.6999999999999997, 5.646546544444467e+29, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp10_test() {
    assert_eq2!(I::EMPTY.exp10(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).exp10(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).exp10(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).exp10(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).exp10(), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.exp10(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 308.25471555991675).exp10(),
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(308.25471555991675, 308.25471555991675).exp10(),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 308.2547155599167).exp10(),
        n2i(1.0, 1.7976931348620926e+308)
    );
    assert_eq2!(
        n2i(-0.0, 308.2547155599167).exp10(),
        n2i(1.0, 1.7976931348620926e+308)
    );
    assert_eq2!(
        n2i(-307.6526555685888, 308.2547155599167).exp10(),
        n2i(2.225073858507187e-308, 1.7976931348620926e+308)
    );
    assert_eq2!(
        n2i(-145.0, 308.2547155599167).exp10(),
        n2i(1e-145, 1.7976931348620926e+308)
    );
    assert_eq2!(n2i(-145.0, 0.0).exp10(), n2i(1e-145, 1.0));
    assert_eq2!(n2i(-145.0, -0.0).exp10(), n2i(1e-145, 1.0));
    assert_eq2!(n2i(-145.0, 1.0).exp10(), n2i(1e-145, 10.0));
    assert_eq2!(n2i(1.0, 5.0).exp10(), n2i(10.0, 100000.0));
    assert_eq2!(
        n2i(-3.3219280948873626, 1.8073549220576042).exp10(),
        n2i(0.00047650987489022414, 64.17338117537051)
    );
    assert_eq2!(
        n2i(0.7655347463629769, 98.83328352961747).exp10(),
        n2i(5.828204023267978, 6.812139448068794e+98)
    );
    assert_eq2!(
        n2i(11.750288269015629, 25.990462258377743).exp10(),
        n2i(562714710989.2521, 9.782779355126447e+25)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp10_dec_test() {
    assert_eq2!(
        nd2di(308.25471555991675, 308.25471555991675, D::Com).exp10(),
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.7655347463629769, 98.83328352961747, D::Def).exp10(),
        nd2di(5.828204023267978, 6.812139448068794e+98, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log_test() {
    assert_eq2!(I::EMPTY.ln(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).ln(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).ln(), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.0).ln(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-0.0, 1.0).ln(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).ln(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY).ln(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, f64::INFINITY).ln(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.ln(), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 1.7976931348623157e+308).ln(),
        n2i(f64::NEG_INFINITY, 709.7827128933841)
    );
    assert_eq2!(
        n2i(-0.0, 1.7976931348623157e+308).ln(),
        n2i(f64::NEG_INFINITY, 709.7827128933841)
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308).ln(),
        n2i(0.0, 709.7827128933841)
    );
    assert_eq2!(
        n2i(5e-324, 1.7976931348623157e+308).ln(),
        n2i(-744.4400719213813, 709.7827128933841)
    );
    assert_eq2!(n2i(5e-324, 1.0).ln(), n2i(-744.4400719213813, 0.0));
    assert_eq2!(
        n2i(2.718281828459045, 2.718281828459045).ln(),
        n2i(0.9999999999999999, 1.0)
    );
    assert_eq2!(
        n2i(2.7182818284590455, 2.7182818284590455).ln(),
        n2i(1.0, 1.0000000000000002)
    );
    assert_eq2!(
        n2i(5e-324, 2.7182818284590455).ln(),
        n2i(-744.4400719213813, 1.0000000000000002)
    );
    assert_eq2!(
        n2i(2.718281828459045, 32.0).ln(),
        n2i(0.9999999999999999, 3.465735902799727)
    );
    assert_eq2!(
        n2i(0.1, 3.5).ln(),
        n2i(-2.302585092994046, 1.252762968495368)
    );
    assert_eq2!(
        n2i(1.7, 5.646546544444444e+29).ln(),
        n2i(0.5306282510621703, 68.50601182403604)
    );
    assert_eq2!(
        n2i(3445.000003400303, 66666666.6666666).ln(),
        n2i(8.144679184434784, 18.015215635844203)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log_dec_test() {
    assert_eq2!(
        nd2di(5e-324, 1.7976931348623157e+308, D::Com).ln(),
        nd2di(-744.4400719213813, 709.7827128933841, D::Com)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Com).ln(),
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(2.7182818284590455, 2.7182818284590455, D::Def).ln(),
        nd2di(1.0, 1.0000000000000002, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log2_test() {
    assert_eq2!(I::EMPTY.log2(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).log2(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).log2(), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).log2(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY).log2(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, f64::INFINITY).log2(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.log2(), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 1.7976931348623157e+308).log2(),
        n2i(f64::NEG_INFINITY, 1024.0)
    );
    assert_eq2!(
        n2i(-0.0, 1.7976931348623157e+308).log2(),
        n2i(f64::NEG_INFINITY, 1024.0)
    );
    assert_eq2!(n2i(1.0, 1.7976931348623157e+308).log2(), n2i(0.0, 1024.0));
    assert_eq2!(
        n2i(5e-324, 1.7976931348623157e+308).log2(),
        n2i(-1074.0, 1024.0)
    );
    assert_eq2!(n2i(5e-324, 1.0).log2(), n2i(-1074.0, 0.0));
    assert_eq2!(n2i(5e-324, 2.0).log2(), n2i(-1074.0, 1.0));
    assert_eq2!(n2i(2.0, 32.0).log2(), n2i(1.0, 5.0));
    assert_eq2!(
        n2i(0.1, 3.5).log2(),
        n2i(-3.3219280948873626, 1.8073549220576042)
    );
    assert_eq2!(
        n2i(1.7, 5.646546544444444e+29).log2(),
        n2i(0.7655347463629769, 98.83328352961747)
    );
    assert_eq2!(
        n2i(3445.000003400303, 66666666.6666666).log2(),
        n2i(11.750288269015629, 25.990462258377743)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log2_dec_test() {
    assert_eq2!(
        nd2di(5e-324, 1.7976931348623157e+308, D::Com).log2(),
        nd2di(-1074.0, 1024.0, D::Com)
    );
    assert_eq2!(
        nd2di(5e-324, f64::INFINITY, D::Dac).log2(),
        nd2di(-1074.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(nd2di(2.0, 32.0, D::Def).log2(), nd2di(1.0, 5.0, D::Def));
    assert_eq2!(
        nd2di(0.0, 1.7976931348623157e+308, D::Com).log2(),
        nd2di(f64::NEG_INFINITY, 1024.0, D::Trv)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log10_test() {
    assert_eq2!(I::EMPTY.log10(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).log10(), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).log10(), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(1.0, f64::INFINITY).log10(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY).log10(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, f64::INFINITY).log10(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.log10(), I::ENTIRE);
    assert_eq2!(
        n2i(0.0, 1.7976931348623157e+308).log10(),
        n2i(f64::NEG_INFINITY, 308.25471555991675)
    );
    assert_eq2!(
        n2i(-0.0, 1.7976931348623157e+308).log10(),
        n2i(f64::NEG_INFINITY, 308.25471555991675)
    );
    assert_eq2!(
        n2i(1.0, 1.7976931348623157e+308).log10(),
        n2i(0.0, 308.25471555991675)
    );
    assert_eq2!(
        n2i(5e-324, 1.7976931348623157e+308).log10(),
        n2i(-323.3062153431158, 308.25471555991675)
    );
    assert_eq2!(n2i(5e-324, 1.0).log10(), n2i(-323.3062153431158, 0.0));
    assert_eq2!(n2i(5e-324, 10.0).log10(), n2i(-323.3062153431158, 1.0));
    assert_eq2!(n2i(10.0, 100000.0).log10(), n2i(1.0, 5.0));
    assert_eq2!(n2i(0.1, 3.5).log10(), n2i(-1.0, 0.5440680443502757));
    assert_eq2!(n2i(0.1, 0.1).log10(), n2i(-1.0, -0.9999999999999999));
    assert_eq2!(
        n2i(1.7, 5.646546544444444e+29).log10(),
        n2i(0.2304489213782739, 29.751782912377774)
    );
    assert_eq2!(
        n2i(3445.000003400303, 66666666.6666666).log10(),
        n2i(3.537189226672304, 7.823908740944319)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log10_dec_test() {
    assert_eq2!(
        nd2di(5e-324, 1.7976931348623157e+308, D::Com).log10(),
        nd2di(-323.3062153431158, 308.25471555991675, D::Com)
    );
    assert_eq2!(
        nd2di(0.0, 1.7976931348623157e+308, D::Dac).log10(),
        nd2di(f64::NEG_INFINITY, 308.25471555991675, D::Trv)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sin_test() {
    assert_eq2!(I::EMPTY.sin(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(I::ENTIRE.sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.0).sin(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).sin(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.5707963267948966, 1.5707963267948966).sin(),
        n2i(0.9999999999999999, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 1.5707963267948968).sin(),
        n2i(0.9999999999999999, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948966, 1.5707963267948968).sin(),
        n2i(0.9999999999999999, 1.0)
    );
    assert_eq2!(n2i(0.0, 1.5707963267948966).sin(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.5707963267948966).sin(), n2i(0.0, 1.0));
    assert_eq2!(n2i(0.0, 1.5707963267948968).sin(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 1.5707963267948968).sin(), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(3.141592653589793, 3.141592653589793).sin(),
        n2i(1.224646799147353e-16, 1.2246467991473532e-16)
    );
    assert_eq2!(
        n2i(3.1415926535897936, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, -3.2162452993532727e-16)
    );
    assert_eq2!(
        n2i(3.141592653589793, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, 1.2246467991473532e-16)
    );
    assert_eq2!(n2i(0.0, 3.141592653589793).sin(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 3.141592653589793).sin(), n2i(0.0, 1.0));
    assert_eq2!(
        n2i(0.0, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, 1.0)
    );
    assert_eq2!(
        n2i(-0.0, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948966, 3.141592653589793).sin(),
        n2i(1.224646799147353e-16, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948966, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 3.141592653589793).sin(),
        n2i(1.224646799147353e-16, 1.0)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 3.1415926535897936).sin(),
        n2i(-3.216245299353273e-16, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, -1.5707963267948966).sin(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, -1.5707963267948968).sin(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, -1.5707963267948966).sin(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(n2i(-1.5707963267948966, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.5707963267948966, -0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.5707963267948968, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.5707963267948968, -0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(
        n2i(-3.141592653589793, -3.141592653589793).sin(),
        n2i(-1.2246467991473532e-16, -1.224646799147353e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -3.1415926535897936).sin(),
        n2i(3.2162452993532727e-16, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -3.141592653589793).sin(),
        n2i(-1.2246467991473532e-16, 3.216245299353273e-16)
    );
    assert_eq2!(n2i(-3.141592653589793, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-3.141592653589793, -0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(
        n2i(-3.1415926535897936, 0.0).sin(),
        n2i(-1.0, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -0.0).sin(),
        n2i(-1.0, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(-3.141592653589793, -1.5707963267948966).sin(),
        n2i(-1.0, -1.224646799147353e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -1.5707963267948966).sin(),
        n2i(-1.0, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(-3.141592653589793, -1.5707963267948968).sin(),
        n2i(-1.0, -1.224646799147353e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -1.5707963267948968).sin(),
        n2i(-1.0, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948966).sin(),
        n2i(-1.0, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948968).sin(),
        n2i(-1.0, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948966).sin(),
        n2i(-1.0, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948968).sin(),
        n2i(-1.0, 1.0)
    );
    assert_eq2!(
        n2i(-0.7, 0.1).sin(),
        n2i(-0.6442176872376911, 0.09983341664682817)
    );
    assert_eq2!(n2i(1.0, 2.0).sin(), n2i(0.8414709848078965, 1.0));
    assert_eq2!(
        n2i(-3.2, -2.9).sin(),
        n2i(-0.23924932921398243, 0.05837414342758009)
    );
    assert_eq2!(
        n2i(2.0, 3.0).sin(),
        n2i(0.1411200080598672, 0.9092974268256817)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sin_dec_test() {
    assert_eq2!(
        nd2di(-3.141592653589793, -1.5707963267948966, D::Def).sin(),
        nd2di(-1.0, -1.224646799147353e-16, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.0, D::Trv).sin(),
        nd2di(-1.0, 1.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).sin(),
        nd2di(-1.0, 1.0, D::Dac)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cos_test() {
    assert_eq2!(I::EMPTY.cos(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(I::ENTIRE.cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.0).cos(), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, -0.0).cos(), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(1.5707963267948966, 1.5707963267948966).cos(),
        n2i(6.123233995736765e-17, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(1.5707963267948966, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(0.0, 1.5707963267948966).cos(),
        n2i(6.123233995736765e-17, 1.0)
    );
    assert_eq2!(
        n2i(-0.0, 1.5707963267948966).cos(),
        n2i(6.123233995736765e-17, 1.0)
    );
    assert_eq2!(
        n2i(0.0, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(-0.0, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(3.141592653589793, 3.141592653589793).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(3.1415926535897936, 3.1415926535897936).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(3.141592653589793, 3.1415926535897936).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(n2i(0.0, 3.141592653589793).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-0.0, 3.141592653589793).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, 3.1415926535897936).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-0.0, 3.1415926535897936).cos(), n2i(-1.0, 1.0));
    assert_eq2!(
        n2i(1.5707963267948966, 3.141592653589793).cos(),
        n2i(-1.0, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(1.5707963267948966, 3.1415926535897936).cos(),
        n2i(-1.0, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 3.141592653589793).cos(),
        n2i(-1.0, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 3.1415926535897936).cos(),
        n2i(-1.0, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, -1.5707963267948966).cos(),
        n2i(6.123233995736765e-17, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, -1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, -1.5707963267948966).cos(),
        n2i(-1.6081226496766366e-16, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 0.0).cos(),
        n2i(6.123233995736765e-17, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, -0.0).cos(),
        n2i(6.123233995736765e-17, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 0.0).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, -0.0).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(-3.141592653589793, -3.141592653589793).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -3.1415926535897936).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -3.141592653589793).cos(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(n2i(-3.141592653589793, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.141592653589793, -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.1415926535897936, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.1415926535897936, -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(
        n2i(-3.141592653589793, -1.5707963267948966).cos(),
        n2i(-1.0, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -1.5707963267948966).cos(),
        n2i(-1.0, 6.123233995736766e-17)
    );
    assert_eq2!(
        n2i(-3.141592653589793, -1.5707963267948968).cos(),
        n2i(-1.0, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(-3.1415926535897936, -1.5707963267948968).cos(),
        n2i(-1.0, -1.6081226496766364e-16)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948966).cos(),
        n2i(6.123233995736765e-17, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948966).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948968).cos(),
        n2i(-1.6081226496766366e-16, 1.0)
    );
    assert_eq2!(n2i(-0.7, 0.1).cos(), n2i(0.7648421872844884, 1.0));
    assert_eq2!(
        n2i(1.0, 2.0).cos(),
        n2i(-0.4161468365471424, 0.5403023058681398)
    );
    assert_eq2!(n2i(-3.2, -2.9).cos(), n2i(-1.0, -0.9709581651495904));
    assert_eq2!(
        n2i(2.0, 3.0).cos(),
        n2i(-0.9899924966004455, -0.41614683654714235)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cos_dec_test() {
    assert_eq2!(
        nd2di(-1.5707963267948966, -1.5707963267948966, D::Trv).cos(),
        nd2di(6.123233995736765e-17, 6.123233995736766e-17, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.0, D::Def).cos(),
        nd2di(-1.0, 1.0, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cos(),
        nd2di(-1.0, 1.0, D::Dac)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tan_test() {
    assert_eq2!(I::EMPTY.tan(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).tan(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, f64::INFINITY).tan(), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).tan(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.tan(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).tan(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).tan(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.5707963267948966, 1.5707963267948966).tan(),
        n2i(1.6331239353195368e+16, 1.633123935319537e+16)
    );
    assert_eq2!(
        n2i(1.5707963267948968, 1.5707963267948968).tan(),
        n2i(-6218431163823739.0, -6218431163823738.0)
    );
    assert_eq2!(n2i(1.5707963267948966, 1.5707963267948968).tan(), I::ENTIRE);
    assert_eq2!(
        n2i(3.141592653589793, 3.141592653589793).tan(),
        n2i(-1.2246467991473532e-16, -1.224646799147353e-16)
    );
    assert_eq2!(
        n2i(3.1415926535897936, 3.1415926535897936).tan(),
        n2i(3.2162452993532727e-16, 3.216245299353273e-16)
    );
    assert_eq2!(
        n2i(0.0, 1.5707963267948966).tan(),
        n2i(0.0, 1.633123935319537e+16)
    );
    assert_eq2!(
        n2i(-0.0, 1.5707963267948966).tan(),
        n2i(0.0, 1.633123935319537e+16)
    );
    assert_eq2!(n2i(0.0, 1.5707963267948968).tan(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, 1.5707963267948968).tan(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 3.141592653589793).tan(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, 3.141592653589793).tan(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 3.1415926535897936).tan(), I::ENTIRE);
    assert_eq2!(n2i(-0.0, 3.1415926535897936).tan(), I::ENTIRE);
    assert_eq2!(
        n2i(4.440892098500626e-16, 3.141592653589793).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(4.440892098500626e-16, 3.1415926535897936).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(2.220446049250313e-16, 3.141592653589793).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(2.220446049250313e-16, 3.1415926535897936).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948966).tan(),
        n2i(-1.633123935319537e+16, 1.633123935319537e+16)
    );
    assert_eq2!(
        n2i(-1.5707963267948966, 1.5707963267948968).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948966).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-1.5707963267948968, 1.5707963267948968).tan(),
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-0.33333, 0.1).tan(),
        n2i(-0.34624981654314885, 0.10033467208545056)
    );
    assert_eq2!(
        n2i(5345.555, 5346.01).tan(),
        n2i(-7.356840852049277, -1.4932050979825788)
    );
    assert_eq2!(n2i(5345.555, 5446.01).tan(), I::ENTIRE);
    assert_eq2!(
        n2i(0.99, 1.01).tan(),
        n2i(1.5236767410179022, 1.5922060242195706)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tan_dec_test() {
    assert_eq2!(DI::EMPTY.tan(), DI::EMPTY);
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, f64::INFINITY, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.0, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(nd2di(0.0, 0.0, D::Com).tan(), nd2di(0.0, 0.0, D::Com));
    assert_eq2!(nd2di(-0.0, -0.0, D::Def).tan(), nd2di(0.0, 0.0, D::Def));
    assert_eq2!(
        nd2di(1.5707963267948966, 1.5707963267948966, D::Com).tan(),
        nd2di(1.6331239353195368e+16, 1.633123935319537e+16, D::Com)
    );
    assert_eq2!(
        nd2di(1.5707963267948968, 1.5707963267948968, D::Def).tan(),
        nd2di(-6218431163823739.0, -6218431163823738.0, D::Def)
    );
    assert_eq2!(
        nd2di(1.5707963267948966, 1.5707963267948968, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(3.141592653589793, 3.141592653589793, D::Trv).tan(),
        nd2di(-1.2246467991473532e-16, -1.224646799147353e-16, D::Trv)
    );
    assert_eq2!(
        nd2di(3.1415926535897936, 3.1415926535897936, D::Com).tan(),
        nd2di(3.2162452993532727e-16, 3.216245299353273e-16, D::Com)
    );
    assert_eq2!(
        nd2di(0.0, 1.5707963267948966, D::Dac).tan(),
        nd2di(0.0, 1.633123935319537e+16, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5707963267948966, D::Com).tan(),
        nd2di(0.0, 1.633123935319537e+16, D::Com)
    );
    assert_eq2!(
        nd2di(0.0, 1.5707963267948968, D::Trv).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.5707963267948968, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 3.141592653589793, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 3.141592653589793, D::Com).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 3.1415926535897936, D::Trv).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 3.1415926535897936, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(4.440892098500626e-16, 3.141592653589793, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(4.440892098500626e-16, 3.1415926535897936, D::Com).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(2.220446049250313e-16, 3.141592653589793, D::Trv).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(2.220446049250313e-16, 3.1415926535897936, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.5707963267948966, 1.5707963267948966, D::Com).tan(),
        nd2di(-1.633123935319537e+16, 1.633123935319537e+16, D::Com)
    );
    assert_eq2!(
        nd2di(-1.5707963267948966, 1.5707963267948968, D::Trv).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.5707963267948968, 1.5707963267948966, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Dac).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.33333, 0.1, D::Com).tan(),
        nd2di(-0.34624981654314885, 0.10033467208545056, D::Com)
    );
    assert_eq2!(
        nd2di(5345.555, 5346.01, D::Dac).tan(),
        nd2di(-7.356840852049277, -1.4932050979825788, D::Dac)
    );
    assert_eq2!(
        nd2di(5345.555, 5446.01, D::Def).tan(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.99, 1.01, D::Trv).tan(),
        nd2di(1.5236767410179022, 1.5922060242195706, D::Trv)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asin_test() {
    assert_eq2!(I::EMPTY.asin(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).asin(), n2i(0.0, 1.5707963267948968));
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).asin(),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).asin(),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).asin(),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.asin(),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-1.0, 1.0).asin(),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -1.0).asin(),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(1.0, f64::INFINITY).asin(),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-1.0, -1.0).asin(),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(1.0, 1.0).asin(),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(n2i(0.0, 0.0).asin(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).asin(), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0000000000000002).asin(), I::EMPTY);
    assert_eq2!(n2i(1.0000000000000002, f64::INFINITY).asin(), I::EMPTY);
    assert_eq2!(
        n2i(-0.1, 0.1).asin(),
        n2i(-0.10016742116155981, 0.10016742116155981)
    );
    assert_eq2!(
        n2i(-0.33, 0.9999999999999999).asin(),
        n2i(-0.3363035751539804, 1.5707963118937356)
    );
    assert_eq2!(
        n2i(-0.9999999999999999, 0.9999999999999999).asin(),
        n2i(-1.5707963118937356, 1.5707963118937356)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asin_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).asin(),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).asin(),
        nd2di(-1.5707963267948968, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.0, 1.0, D::Com).asin(),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Com)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).asin(),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.33, 0.9999999999999999, D::Def).asin(),
        nd2di(-0.3363035751539804, 1.5707963118937356, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acos_test() {
    assert_eq2!(I::EMPTY.acos(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).acos(), n2i(0.0, 1.5707963267948968));
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).acos(),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).acos(),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).acos(),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(I::ENTIRE.acos(), n2i(0.0, 3.1415926535897936));
    assert_eq2!(n2i(-1.0, 1.0).acos(), n2i(0.0, 3.1415926535897936));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -1.0).acos(),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(n2i(1.0, f64::INFINITY).acos(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(-1.0, -1.0).acos(),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(n2i(1.0, 1.0).acos(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(0.0, 0.0).acos(),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).acos(),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0000000000000002).acos(), I::EMPTY);
    assert_eq2!(n2i(1.0000000000000002, f64::INFINITY).acos(), I::EMPTY);
    assert_eq2!(
        n2i(-0.1, 0.1).acos(),
        n2i(1.4706289056333368, 1.6709637479564565)
    );
    assert_eq2!(
        n2i(-0.33, 0.9999999999999999).acos(),
        n2i(1.4901161193847656e-08, 1.907099901948877)
    );
    assert_eq2!(
        n2i(-0.9999999999999999, 0.9999999999999999).acos(),
        n2i(1.4901161193847656e-08, 3.1415926386886324)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acos_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).acos(),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).acos(),
        nd2di(1.5707963267948966, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.0, 1.0, D::Com).acos(),
        nd2di(0.0, 3.1415926535897936, D::Com)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).acos(),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.33, 0.9999999999999999, D::Def).acos(),
        nd2di(1.4901161193847656e-08, 1.907099901948877, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan_test() {
    assert_eq2!(I::EMPTY.atan(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).atan(), n2i(0.0, 1.5707963267948968));
    assert_eq2!(
        n2i(-0.0, f64::INFINITY).atan(),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).atan(),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).atan(),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.atan(),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(n2i(0.0, 0.0).atan(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).atan(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.0, 43534534.67567).atan(),
        n2i(0.7853981633974483, 1.570796303824627)
    );
    assert_eq2!(
        n2i(-546675434554.668, -565.8889722).atan(),
        n2i(-1.5707963267930676, -1.5690291975377264)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).atan(),
        nd2di(0.0, 1.5707963267948968, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).atan(),
        nd2di(-1.5707963267948968, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan(),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, 43534534.67567, D::Trv).atan(),
        nd2di(0.7853981633974483, 1.570796303824627, D::Trv)
    );
    assert_eq2!(
        nd2di(-546675434554.668, -565.8889722, D::Com).atan(),
        nd2di(-1.5707963267930676, -1.5690291975377264, D::Com)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_test() {
    assert_eq2!(I::EMPTY.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(I::ENTIRE), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-2.0, -0.1)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-2.0, 0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-2.0, -0.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-2.0, 1.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(0.0, 1.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(-0.0, 1.0)), I::EMPTY);
    assert_eq2!(I::EMPTY.atan2(n2i(0.1, 1.0)), I::EMPTY);
    assert_eq2!(I::ENTIRE.atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        I::ENTIRE.atan2(I::ENTIRE),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(0.0, 0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-0.0, 0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-0.0, -0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-2.0, -0.1)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-2.0, 0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-2.0, -0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-2.0, 1.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(0.0, 1.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(-0.0, 1.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        I::ENTIRE.atan2(n2i(0.1, 1.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(n2i(0.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(I::ENTIRE), n2i(0.0, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(
        n2i(0.0, 0.0).atan2(n2i(-2.0, -0.1)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 0.0).atan2(n2i(-2.0, 0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 0.0).atan2(n2i(-2.0, -0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 0.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, 0.0).atan2(I::ENTIRE),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, 0.0).atan2(n2i(-2.0, -0.1)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 0.0).atan2(n2i(-2.0, 0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 0.0).atan2(n2i(-2.0, -0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 0.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(0.0, -0.0).atan2(I::ENTIRE),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(
        n2i(0.0, -0.0).atan2(n2i(-2.0, -0.1)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, -0.0).atan2(n2i(-2.0, 0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, -0.0).atan2(n2i(-2.0, -0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, -0.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, -0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, -0.0).atan2(I::ENTIRE),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, -0.0).atan2(n2i(-2.0, -0.1)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).atan2(n2i(-2.0, 0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).atan2(n2i(-2.0, -0.0)),
        n2i(3.141592653589793, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, -0.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(-2.0, -0.1).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(I::ENTIRE),
        n2i(-3.1415926535897936, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-2.0, -0.1)),
        n2i(-3.0916342578678506, -1.6207547225168393)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-2.0, 0.0)),
        n2i(-3.0916342578678506, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-2.0, -0.0)),
        n2i(-3.0916342578678506, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-2.0, 1.0)),
        n2i(-3.0916342578678506, -0.09966865249116202)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(0.0, 1.0)),
        n2i(-1.5707963267948968, -0.09966865249116202)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(-0.0, 1.0)),
        n2i(-1.5707963267948968, -0.09966865249116202)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).atan2(n2i(0.1, 1.0)),
        n2i(-1.520837931072954, -0.09966865249116202)
    );
    assert_eq2!(n2i(-2.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(I::ENTIRE),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-2.0, -0.1)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-2.0, 0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-2.0, -0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-2.0, 1.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(0.0, 1.0)),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(-0.0, 1.0)),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).atan2(n2i(0.1, 1.0)),
        n2i(-1.520837931072954, 0.0)
    );
    assert_eq2!(n2i(-2.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(I::ENTIRE),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-0.0, 0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-0.0, -0.0)),
        n2i(-1.5707963267948968, -1.5707963267948966)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-2.0, -0.1)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-2.0, 0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-2.0, -0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-2.0, 1.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(0.0, 1.0)),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(-0.0, 1.0)),
        n2i(-1.5707963267948968, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.0).atan2(n2i(0.1, 1.0)),
        n2i(-1.520837931072954, 0.0)
    );
    assert_eq2!(n2i(-2.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(I::ENTIRE),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(0.0, 0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-0.0, 0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(0.0, -0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-0.0, -0.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-2.0, -0.1)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-2.0, 0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-2.0, -0.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-2.0, 1.0)),
        n2i(-3.1415926535897936, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(0.0, 1.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(-0.0, 1.0)),
        n2i(-1.5707963267948968, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-2.0, 1.0).atan2(n2i(0.1, 1.0)),
        n2i(-1.520837931072954, 1.4711276743037347)
    );
    assert_eq2!(n2i(-0.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(I::ENTIRE),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-2.0, -0.1)),
        n2i(1.6704649792860586, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-2.0, 0.0)),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-2.0, -0.0)),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(0.0, 1.0)),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(-0.0, 1.0)),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(-0.0, 1.0).atan2(n2i(0.1, 1.0)),
        n2i(0.0, 1.4711276743037347)
    );
    assert_eq2!(n2i(0.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.0, 1.0).atan2(I::ENTIRE), n2i(0.0, 3.1415926535897936));
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-2.0, -0.1)),
        n2i(1.6704649792860586, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-2.0, 0.0)),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-2.0, -0.0)),
        n2i(1.5707963267948966, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.0, 3.1415926535897936)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(0.0, 1.0)),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(-0.0, 1.0)),
        n2i(0.0, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.0, 1.0).atan2(n2i(0.1, 1.0)),
        n2i(0.0, 1.4711276743037347)
    );
    assert_eq2!(n2i(0.1, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(0.1, 1.0).atan2(I::ENTIRE), n2i(0.0, 3.1415926535897936));
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-0.0, 0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-0.0, -0.0)),
        n2i(1.5707963267948966, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-2.0, -0.1)),
        n2i(1.6704649792860586, 3.0916342578678506)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-2.0, 0.0)),
        n2i(1.5707963267948966, 3.0916342578678506)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-2.0, -0.0)),
        n2i(1.5707963267948966, 3.0916342578678506)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-2.0, 1.0)),
        n2i(0.09966865249116202, 3.0916342578678506)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(0.0, 1.0)),
        n2i(0.09966865249116202, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(-0.0, 1.0)),
        n2i(0.09966865249116202, 1.5707963267948968)
    );
    assert_eq2!(
        n2i(0.1, 1.0).atan2(n2i(0.1, 1.0)),
        n2i(0.09966865249116202, 1.4711276743037347)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_dec_test() {
    assert_eq2!(DI::EMPTY.atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        DI::EMPTY.atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        DI::EMPTY
    );
    assert_eq2!(DI::EMPTY.atan2(nd2di(0.0, 0.0, D::Com)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-0.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(0.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-0.0, -0.0, D::Trv)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-2.0, -0.1, D::Com)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-2.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-2.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-2.0, 1.0, D::Trv)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(0.0, 1.0, D::Com)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(-0.0, 1.0, D::Dac)), DI::EMPTY);
    assert_eq2!(DI::EMPTY.atan2(nd2di(0.1, 1.0, D::Def)), DI::EMPTY);
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(DI::EMPTY),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        )),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, 0.0, D::Com)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, 0.0, D::Dac)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, -0.0, D::Trv)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, -0.1, D::Com)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, 0.0, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, -0.0, D::Def)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, 1.0, D::Trv)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, 1.0, D::Dac)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Dac)
    );
    assert_eq2!(nd2di(0.0, 0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Trv)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Dac)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Com)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Trv)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Def).atan2(nd2di(0.0, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Com)
    );
    assert_eq2!(nd2di(-0.0, 0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Com)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Trv)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Def).atan2(nd2di(-0.0, 1.0, D::Dac)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Com)
    );
    assert_eq2!(nd2di(0.0, -0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(0.0, -0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Dac)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Trv)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Com)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Com)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Trv)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Def).atan2(nd2di(-0.0, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Def)),
        nd2di(0.0, 0.0, D::Def)
    );
    assert_eq2!(nd2di(-0.0, -0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Def)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)),
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Def).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Def)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Def)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Trv)),
        nd2di(3.141592653589793, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Com)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)),
        nd2di(0.0, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(0.0, 0.0, D::Com)
    );
    assert_eq2!(nd2di(-2.0, -0.1, D::Dac).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(-3.1415926535897936, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Trv).atan2(nd2di(0.0, 0.0, D::Com)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, -0.0, D::Dac)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).atan2(nd2di(-0.0, 0.0, D::Def)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Def).atan2(nd2di(-2.0, -0.1, D::Com)),
        nd2di(-3.0916342578678506, -1.6207547225168393, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).atan2(nd2di(-2.0, 0.0, D::Def)),
        nd2di(-3.0916342578678506, -1.5707963267948966, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Trv).atan2(nd2di(-2.0, -0.0, D::Dac)),
        nd2di(-3.0916342578678506, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Def).atan2(nd2di(-2.0, 1.0, D::Trv)),
        nd2di(-3.0916342578678506, -0.09966865249116202, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, 1.0, D::Def)),
        nd2di(-1.5707963267948968, -0.09966865249116202, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).atan2(nd2di(-0.0, 1.0, D::Com)),
        nd2di(-1.5707963267948968, -0.09966865249116202, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(-1.520837931072954, -0.09966865249116202, D::Com)
    );
    assert_eq2!(nd2di(-2.0, 0.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Dac)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Com)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Def).atan2(nd2di(-0.0, -0.0, D::Def)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).atan2(nd2di(-2.0, 0.0, D::Com)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Trv).atan2(nd2di(-2.0, 1.0, D::Com)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Def).atan2(nd2di(0.0, 1.0, D::Def)),
        nd2di(-1.5707963267948968, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-0.0, 1.0, D::Dac)),
        nd2di(-1.5707963267948968, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(-1.520837931072954, 0.0, D::Com)
    );
    assert_eq2!(nd2di(-2.0, -0.0, D::Trv).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Com).atan2(nd2di(0.0, 0.0, D::Com)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-0.0, 0.0, D::Dac)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Trv)),
        nd2di(-1.5707963267948968, -1.5707963267948966, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-2.0, -0.1, D::Com)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-2.0, 0.0, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Def)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Trv)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Trv).atan2(nd2di(0.0, 1.0, D::Dac)),
        nd2di(-1.5707963267948968, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-0.0, 1.0, D::Com)),
        nd2di(-1.5707963267948968, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(-1.520837931072954, 0.0, D::Com)
    );
    assert_eq2!(nd2di(-2.0, 1.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Def)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Com).atan2(nd2di(-0.0, 0.0, D::Dac)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Trv)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Com)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Def)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(-2.0, -0.0, D::Trv)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Com)),
        nd2di(-3.1415926535897936, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Com).atan2(nd2di(0.0, 1.0, D::Dac)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)),
        nd2di(-1.5707963267948968, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(-1.520837931072954, 1.4711276743037347, D::Com)
    );
    assert_eq2!(nd2di(-0.0, 1.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Dac)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Com)),
        nd2di(1.6704649792860586, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Com)),
        nd2di(1.5707963267948966, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Def).atan2(nd2di(-2.0, -0.0, D::Def)),
        nd2di(1.5707963267948966, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(0.0, 1.0, D::Dac)),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Com)),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(0.0, 1.4711276743037347, D::Trv)
    );
    assert_eq2!(nd2di(0.0, 1.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Dac)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Def).atan2(nd2di(-0.0, -0.0, D::Com)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)),
        nd2di(1.6704649792860586, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Trv)),
        nd2di(1.5707963267948966, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Dac)),
        nd2di(1.5707963267948966, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)),
        nd2di(0.0, 3.1415926535897936, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(0.0, 1.0, D::Trv)),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)),
        nd2di(0.0, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)),
        nd2di(0.0, 1.4711276743037347, D::Com)
    );
    assert_eq2!(nd2di(0.1, 1.0, D::Dac).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq2!(
        nd2di(0.1, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)),
        nd2di(0.0, 3.1415926535897936, D::Dac)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Com)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Trv)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Def)),
        nd2di(1.5707963267948966, 1.5707963267948968, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Trv)),
        nd2di(1.6704649792860586, 3.0916342578678506, D::Trv)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, 0.0, D::Dac)),
        nd2di(1.5707963267948966, 3.0916342578678506, D::Dac)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)),
        nd2di(1.5707963267948966, 3.0916342578678506, D::Dac)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Dac)),
        nd2di(0.09966865249116202, 3.0916342578678506, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Def).atan2(nd2di(0.0, 1.0, D::Def)),
        nd2di(0.09966865249116202, 1.5707963267948968, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)),
        nd2di(0.09966865249116202, 1.5707963267948968, D::Def)
    );
    assert_eq2!(
        nd2di(0.1, 1.0, D::Dac).atan2(nd2di(0.1, 1.0, D::Def)),
        nd2di(0.09966865249116202, 1.4711276743037347, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sinh_test() {
    assert_eq2!(I::EMPTY.sinh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).sinh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).sinh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(I::ENTIRE.sinh(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).sinh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).sinh(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.0, 300.5632345).sinh(),
        n2i(1.1752011936438014, 1.705784684221514e+130)
    );
    assert_eq2!(
        n2i(-546675434554.668, -565.8889722).sinh(),
        n2i(f64::NEG_INFINITY, -2.8935300748108013e+245)
    );
    assert_eq2!(
        n2i(-1.1, 2.3).sinh(),
        n2i(-1.335647470124177, 4.936961805545958)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sinh_dec_test() {
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).sinh(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).sinh(),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).sinh(),
        nd2di(f64::NEG_INFINITY, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 300.5632345, D::Com).sinh(),
        nd2di(1.1752011936438014, 1.705784684221514e+130, D::Com)
    );
    assert_eq2!(
        nd2di(-546675434554.668, -565.8889722, D::Com).sinh(),
        nd2di(f64::NEG_INFINITY, -2.8935300748108013e+245, D::Dac)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cosh_test() {
    assert_eq2!(I::EMPTY.cosh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).cosh(), n2i(1.0, 1.0));
    assert_eq2!(n2i(-0.0, -0.0).cosh(), n2i(1.0, 1.0));
    assert_eq2!(
        n2i(1.0, 300.5632345).cosh(),
        n2i(1.5430806348152437, 1.705784684221514e+130)
    );
    assert_eq2!(
        n2i(-546675434554.668, -565.8889722).cosh(),
        n2i(2.8935300748108013e+245, f64::INFINITY)
    );
    assert_eq2!(n2i(-1.1, 2.3).cosh(), n2i(1.0, 5.037220649268762));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cosh_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).cosh(),
        nd2di(1.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).cosh(),
        nd2di(1.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).cosh(),
        nd2di(1.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(1.0, 300.5632345, D::Def).cosh(),
        nd2di(1.5430806348152437, 1.705784684221514e+130, D::Def)
    );
    assert_eq2!(
        nd2di(-546675434554.668, -565.8889722, D::Com).cosh(),
        nd2di(2.8935300748108013e+245, f64::INFINITY, D::Dac)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tanh_test() {
    assert_eq2!(I::EMPTY.tanh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq2!(I::ENTIRE.tanh(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, 0.0).tanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).tanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, 300.5632345).tanh(), n2i(0.7615941559557649, 1.0));
    assert_eq2!(
        n2i(-546675434554.668, -565.8889722).tanh(),
        n2i(-1.0, -0.9999999999999999)
    );
    assert_eq2!(
        n2i(-1.1, 2.3).tanh(),
        n2i(-0.8004990217606298, 0.9800963962661914)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tanh_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).tanh(),
        nd2di(0.0, 1.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).tanh(),
        nd2di(-1.0, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).tanh(),
        nd2di(-1.0, 1.0, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, 300.5632345, D::Com).tanh(),
        nd2di(0.7615941559557649, 1.0, D::Com)
    );
    assert_eq2!(
        nd2di(-546675434554.668, -565.8889722, D::Trv).tanh(),
        nd2di(-1.0, -0.9999999999999999, D::Trv)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asinh_test() {
    assert_eq2!(I::EMPTY.asinh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).asinh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).asinh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(I::ENTIRE.asinh(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).asinh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).asinh(), n2i(0.0, 0.0));
    assert_eq2!(
        n2i(1.0, 300.5632345).asinh(),
        n2i(0.8813735870195429, 6.398808110711463)
    );
    assert_eq2!(
        n2i(-546675434554.668, -565.8889722).asinh(),
        n2i(-27.720268288347388, -7.03154585801716)
    );
    assert_eq2!(
        n2i(-1.1, 2.3).asinh(),
        n2i(-0.9503469298211343, 1.5702785434849782)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asinh_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).asinh(),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv).asinh(),
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).asinh(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(1.0, 300.5632345, D::Com).asinh(),
        nd2di(0.8813735870195429, 6.398808110711463, D::Com)
    );
    assert_eq2!(
        nd2di(-546675434554.668, -565.8889722, D::Def).asinh(),
        nd2di(-27.720268288347388, -7.03154585801716, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acosh_test() {
    assert_eq2!(I::EMPTY.acosh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(1.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.0).acosh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.9999999999999999).acosh(), I::EMPTY);
    assert_eq2!(I::ENTIRE.acosh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(1.0, 1.0).acosh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.0, 300.5632345).acosh(), n2i(0.0, 6.398802575957844));
    assert_eq2!(
        n2i(1.1, 2.3).acosh(),
        n2i(0.4435682543851154, 1.4750447812414251)
    );
    assert_eq2!(
        n2i(35434.45345435, 777747747.5642).acosh(),
        n2i(11.168587067593199, 21.165059978205456)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acosh_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).acosh(),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(1.0, f64::INFINITY, D::Dac).acosh(),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).acosh(),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(nd2di(1.0, 1.0, D::Com).acosh(), nd2di(0.0, 0.0, D::Com));
    assert_eq2!(nd2di(0.9, 1.0, D::Com).acosh(), nd2di(0.0, 0.0, D::Trv));
    assert_eq2!(
        nd2di(1.0, 300.5632345, D::Dac).acosh(),
        nd2di(0.0, 6.398802575957844, D::Dac)
    );
    assert_eq2!(
        nd2di(0.9, 300.5632345, D::Com).acosh(),
        nd2di(0.0, 6.398802575957844, D::Trv)
    );
    assert_eq2!(
        nd2di(35434.45345435, 777747747.5642, D::Def).acosh(),
        nd2di(11.168587067593199, 21.165059978205456, D::Def)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atanh_test() {
    assert_eq2!(I::EMPTY.atanh(), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).atanh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-0.0, f64::INFINITY).atanh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(1.0, f64::INFINITY).atanh(), I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).atanh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.0).atanh(),
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).atanh(), I::EMPTY);
    assert_eq2!(n2i(-1.0, 1.0).atanh(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).atanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).atanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-1.0, -1.0).atanh(), I::EMPTY);
    assert_eq2!(n2i(1.0, 1.0).atanh(), I::EMPTY);
    assert_eq2!(I::ENTIRE.atanh(), I::ENTIRE);
    assert_eq2!(
        n2i(0.3242345, 0.9999999999999999).atanh(),
        n2i(0.3363718566236146, 18.714973875118524)
    );
    assert_eq2!(
        n2i(-0.999454934, 0.1).atanh(),
        n2i(-4.103739140065866, 0.1003353477310756)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atanh_dec_test() {
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).atanh(),
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).atanh(),
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.0, 1.0, D::Com).atanh(),
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(nd2di(0.0, 0.0, D::Com).atanh(), nd2di(0.0, 0.0, D::Com));
    assert_eq2!(nd2di(1.0, 1.0, D::Def).atanh(), DI::EMPTY);
    assert_eq2!(
        nd2di(0.3242345, 0.9999999999999999, D::Com).atanh(),
        nd2di(0.3363718566236146, 18.714973875118524, D::Com)
    );
    assert_eq2!(
        nd2di(-1.0, 0.9999999999999999, D::Com).atanh(),
        nd2di(f64::NEG_INFINITY, 18.714973875118524, D::Trv)
    );
    assert_eq2!(
        nd2di(-0.999454934, 0.1, D::Def).atanh(),
        nd2di(-4.103739140065866, 0.1003353477310756, D::Def)
    );
    assert_eq2!(
        nd2di(-0.999454934, 1.0, D::Com).atanh(),
        nd2di(-4.103739140065866, f64::INFINITY, D::Trv)
    );
}

#[test]
fn minimal_sign_test() {
    assert_eq2!(I::EMPTY.sign(), I::EMPTY);
    assert_eq2!(n2i(1.0, 2.0).sign(), n2i(1.0, 1.0));
    assert_eq2!(n2i(-1.0, 2.0).sign(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 0.0).sign(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq2!(n2i(-5.0, -2.0).sign(), n2i(-1.0, -1.0));
    assert_eq2!(n2i(0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, -0.0).sign(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq2!(I::ENTIRE.sign(), n2i(-1.0, 1.0));
}

#[test]
fn minimal_sign_dec_test() {
    assert_eq2!(nd2di(1.0, 2.0, D::Com).sign(), nd2di(1.0, 1.0, D::Com));
    assert_eq2!(nd2di(-1.0, 2.0, D::Com).sign(), nd2di(-1.0, 1.0, D::Def));
    assert_eq2!(nd2di(-1.0, 0.0, D::Com).sign(), nd2di(-1.0, 0.0, D::Def));
    assert_eq2!(nd2di(0.0, 2.0, D::Com).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq2!(nd2di(-0.0, 2.0, D::Def).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq2!(nd2di(-5.0, -2.0, D::Trv).sign(), nd2di(-1.0, -1.0, D::Trv));
    assert_eq2!(nd2di(0.0, 0.0, D::Dac).sign(), nd2di(0.0, 0.0, D::Dac));
}

#[test]
fn minimal_ceil_test() {
    assert_eq2!(I::EMPTY.ceil(), I::EMPTY);
    assert_eq2!(I::ENTIRE.ceil(), I::ENTIRE);
    assert_eq2!(n2i(1.1, 2.0).ceil(), n2i(2.0, 2.0));
    assert_eq2!(n2i(-1.1, 2.0).ceil(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(-1.1, 0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.4).ceil(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.9, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq2!(n2i(-1.0, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq2!(n2i(0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq2!(n2i(-0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq2!(n2i(-1.5, f64::INFINITY).ceil(), n2i(-1.0, f64::INFINITY));
    assert_eq2!(
        n2i(1.7976931348623157e+308, f64::INFINITY).ceil(),
        n2i(1.7976931348623157e+308, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.2).ceil(),
        n2i(f64::NEG_INFINITY, 3.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -1.7976931348623157e+308).ceil(),
        n2i(f64::NEG_INFINITY, -1.7976931348623157e+308)
    );
}

#[test]
fn minimal_ceil_dec_test() {
    assert_eq2!(nd2di(1.1, 2.0, D::Com).ceil(), nd2di(2.0, 2.0, D::Dac));
    assert_eq2!(nd2di(-1.1, 2.0, D::Com).ceil(), nd2di(-1.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.1, 0.0, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq2!(nd2di(-1.1, -0.0, D::Trv).ceil(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.1, -0.4, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq2!(nd2di(-1.9, 2.2, D::Com).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq2!(nd2di(-1.0, 2.2, D::Dac).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq2!(nd2di(0.0, 2.2, D::Trv).ceil(), nd2di(0.0, 3.0, D::Trv));
    assert_eq2!(nd2di(-0.0, 2.2, D::Def).ceil(), nd2di(0.0, 3.0, D::Def));
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Trv).ceil(),
        nd2di(-1.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Dac).ceil(),
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).ceil(),
        nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 2.2, D::Trv).ceil(),
        nd2di(f64::NEG_INFINITY, 3.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -1.7976931348623157e+308, D::Dac).ceil(),
        nd2di(f64::NEG_INFINITY, -1.7976931348623157e+308, D::Def)
    );
}

#[test]
fn minimal_floor_test() {
    assert_eq2!(I::EMPTY.floor(), I::EMPTY);
    assert_eq2!(I::ENTIRE.floor(), I::ENTIRE);
    assert_eq2!(n2i(1.1, 2.0).floor(), n2i(1.0, 2.0));
    assert_eq2!(n2i(-1.1, 2.0).floor(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.1, 0.0).floor(), n2i(-2.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.0).floor(), n2i(-2.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.4).floor(), n2i(-2.0, -1.0));
    assert_eq2!(n2i(-1.9, 2.2).floor(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.0, 2.2).floor(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-1.5, f64::INFINITY).floor(), n2i(-2.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.2).floor(),
        n2i(f64::NEG_INFINITY, 2.0)
    );
}

#[test]
fn minimal_floor_dec_test() {
    assert_eq2!(nd2di(1.1, 2.0, D::Com).floor(), nd2di(1.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.1, 2.0, D::Def).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.1, 0.0, D::Dac).floor(), nd2di(-2.0, 0.0, D::Def));
    assert_eq2!(nd2di(-1.2, -1.1, D::Com).floor(), nd2di(-2.0, -2.0, D::Com));
    assert_eq2!(nd2di(-1.1, -0.4, D::Def).floor(), nd2di(-2.0, -1.0, D::Def));
    assert_eq2!(nd2di(-1.9, 2.2, D::Com).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.0, 2.2, D::Trv).floor(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq2!(nd2di(0.0, 2.2, D::Trv).floor(), nd2di(0.0, 2.0, D::Trv));
    assert_eq2!(nd2di(-0.0, 2.2, D::Com).floor(), nd2di(0.0, 2.0, D::Def));
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Dac).floor(),
        nd2di(-2.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 2.2, D::Trv).floor(),
        nd2di(f64::NEG_INFINITY, 2.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.7976931348623157e+308, -1.7976931348623157e+308, D::Com).floor(),
        nd2di(-1.7976931348623157e+308, -1.7976931348623157e+308, D::Dac)
    );
}

#[test]
fn minimal_trunc_test() {
    assert_eq2!(I::EMPTY.trunc(), I::EMPTY);
    assert_eq2!(I::ENTIRE.trunc(), I::ENTIRE);
    assert_eq2!(n2i(1.1, 2.1).trunc(), n2i(1.0, 2.0));
    assert_eq2!(n2i(-1.1, 2.0).trunc(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(-1.1, 0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.4).trunc(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.9, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(-1.0, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-1.5, f64::INFINITY).trunc(), n2i(-1.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.2).trunc(),
        n2i(f64::NEG_INFINITY, 2.0)
    );
}

#[test]
fn minimal_trunc_dec_test() {
    assert_eq2!(nd2di(1.1, 2.1, D::Com).trunc(), nd2di(1.0, 2.0, D::Def));
    assert_eq2!(nd2di(1.1, 1.9, D::Com).trunc(), nd2di(1.0, 1.0, D::Com));
    assert_eq2!(nd2di(-1.1, 2.0, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.1, 0.0, D::Trv).trunc(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq2!(nd2di(-1.1, -0.0, D::Def).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq2!(nd2di(-1.1, -0.4, D::Com).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq2!(nd2di(-1.9, 2.2, D::Def).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.0, 2.2, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Dac).trunc(),
        nd2di(-1.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 2.2, D::Trv).trunc(),
        nd2di(f64::NEG_INFINITY, 2.0, D::Trv)
    );
    assert_eq2!(
        nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Com).trunc(),
        nd2di(1.7976931348623157e+308, 1.7976931348623157e+308, D::Dac)
    );
    assert_eq2!(
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Dac).trunc(),
        nd2di(1.7976931348623157e+308, f64::INFINITY, D::Def)
    );
}

#[test]
fn minimal_round_ties_to_even_test() {
    assert_eq2!(I::EMPTY.round_ties_to_even(), I::EMPTY);
    assert_eq2!(I::ENTIRE.round_ties_to_even(), I::ENTIRE);
    assert_eq2!(n2i(1.1, 2.1).round_ties_to_even(), n2i(1.0, 2.0));
    assert_eq2!(n2i(-1.1, 2.0).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(-1.1, -0.4).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, 0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.9, 2.2).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.0, 2.2).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(1.5, 2.1).round_ties_to_even(), n2i(2.0, 2.0));
    assert_eq2!(n2i(-1.5, 2.0).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.1, -0.5).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.9, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-1.5, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq2!(
        n2i(-1.5, f64::INFINITY).round_ties_to_even(),
        n2i(-2.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.2).round_ties_to_even(),
        n2i(f64::NEG_INFINITY, 2.0)
    );
}

#[test]
fn minimal_round_ties_to_even_dec_test() {
    assert_eq2!(
        nd2di(1.1, 2.1, D::Com).round_ties_to_even(),
        nd2di(1.0, 2.0, D::Def)
    );
    assert_eq2!(
        nd2di(-1.1, 2.0, D::Trv).round_ties_to_even(),
        nd2di(-1.0, 2.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-1.6, -1.5, D::Com).round_ties_to_even(),
        nd2di(-2.0, -2.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-1.6, -1.4, D::Com).round_ties_to_even(),
        nd2di(-2.0, -1.0, D::Def)
    );
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Dac).round_ties_to_even(),
        nd2di(-2.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 2.2, D::Trv).round_ties_to_even(),
        nd2di(f64::NEG_INFINITY, 2.0, D::Trv)
    );
}

#[test]
fn minimal_round_ties_to_away_test() {
    assert_eq2!(I::EMPTY.round(), I::EMPTY);
    assert_eq2!(I::ENTIRE.round(), I::ENTIRE);
    assert_eq2!(n2i(1.1, 2.1).round(), n2i(1.0, 2.0));
    assert_eq2!(n2i(-1.1, 2.0).round(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(-1.1, 0.0).round(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.1, -0.0).round(), n2i(-1.0, -0.0));
    assert_eq2!(n2i(-1.1, -0.4).round(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-1.9, 2.2).round(), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.0, 2.2).round(), n2i(-1.0, 2.0));
    assert_eq2!(n2i(0.5, 2.1).round(), n2i(1.0, 2.0));
    assert_eq2!(n2i(-2.5, 2.0).round(), n2i(-3.0, 2.0));
    assert_eq2!(n2i(-1.1, -0.5).round(), n2i(-1.0, -1.0));
    assert_eq2!(n2i(-1.9, 2.5).round(), n2i(-2.0, 3.0));
    assert_eq2!(n2i(-1.5, 2.5).round(), n2i(-2.0, 3.0));
    assert_eq2!(n2i(0.0, 2.5).round(), n2i(0.0, 3.0));
    assert_eq2!(n2i(-0.0, 2.5).round(), n2i(0.0, 3.0));
    assert_eq2!(n2i(-1.5, f64::INFINITY).round(), n2i(-2.0, f64::INFINITY));
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 2.2).round(),
        n2i(f64::NEG_INFINITY, 2.0)
    );
}

#[test]
fn minimal_round_ties_to_away_dec_test() {
    assert_eq2!(nd2di(1.1, 2.1, D::Com).round(), nd2di(1.0, 2.0, D::Def));
    assert_eq2!(nd2di(-1.9, 2.2, D::Com).round(), nd2di(-2.0, 2.0, D::Def));
    assert_eq2!(nd2di(1.9, 2.2, D::Com).round(), nd2di(2.0, 2.0, D::Com));
    assert_eq2!(nd2di(-1.0, 2.2, D::Trv).round(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq2!(nd2di(2.5, 2.6, D::Com).round(), nd2di(3.0, 3.0, D::Dac));
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Dac).round(),
        nd2di(-2.0, f64::INFINITY, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 2.2, D::Def).round(),
        nd2di(f64::NEG_INFINITY, 2.0, D::Def)
    );
}

#[test]
fn minimal_abs_test() {
    assert_eq2!(I::EMPTY.abs(), I::EMPTY);
    assert_eq2!(I::ENTIRE.abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(1.1, 2.1).abs(), n2i(1.1, 2.1));
    assert_eq2!(n2i(-1.1, 2.0).abs(), n2i(0.0, 2.0));
    assert_eq2!(n2i(-1.1, 0.0).abs(), n2i(0.0, 1.1));
    assert_eq2!(n2i(-1.1, -0.0).abs(), n2i(0.0, 1.1));
    assert_eq2!(n2i(-1.1, -0.4).abs(), n2i(0.4, 1.1));
    assert_eq2!(n2i(-1.9, 0.2).abs(), n2i(0.0, 1.9));
    assert_eq2!(n2i(0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq2!(n2i(-0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq2!(n2i(-1.5, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, -2.2).abs(), n2i(2.2, f64::INFINITY));
}

#[test]
fn minimal_abs_dec_test() {
    assert_eq2!(nd2di(-1.1, 2.0, D::Com).abs(), nd2di(0.0, 2.0, D::Com));
    assert_eq2!(nd2di(-1.1, 0.0, D::Dac).abs(), nd2di(0.0, 1.1, D::Dac));
    assert_eq2!(nd2di(-1.1, -0.0, D::Def).abs(), nd2di(0.0, 1.1, D::Def));
    assert_eq2!(nd2di(-1.1, -0.4, D::Trv).abs(), nd2di(0.4, 1.1, D::Trv));
    assert_eq2!(nd2di(-1.9, 0.2, D::Dac).abs(), nd2di(0.0, 1.9, D::Dac));
    assert_eq2!(nd2di(0.0, 0.2, D::Def).abs(), nd2di(0.0, 0.2, D::Def));
    assert_eq2!(nd2di(-0.0, 0.2, D::Com).abs(), nd2di(0.0, 0.2, D::Com));
    assert_eq2!(
        nd2di(-1.5, f64::INFINITY, D::Dac).abs(),
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
}

#[test]
fn minimal_min_test() {
    assert_eq2!(I::EMPTY.min(n2i(1.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(1.0, 2.0).min(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.min(I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.min(n2i(1.0, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq2!(n2i(1.0, 2.0).min(I::ENTIRE), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq2!(I::ENTIRE.min(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::EMPTY.min(I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).min(n2i(2.0, 4.0)), n2i(1.0, 4.0));
    assert_eq2!(n2i(0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq2!(n2i(-0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq2!(n2i(1.0, 5.0).min(n2i(2.0, 8.0)), n2i(1.0, 5.0));
    assert_eq2!(n2i(1.0, 5.0).min(I::ENTIRE), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq2!(n2i(-7.0, -5.0).min(n2i(2.0, 4.0)), n2i(-7.0, -5.0));
    assert_eq2!(n2i(-7.0, 0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
    assert_eq2!(n2i(-7.0, -0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
}

#[test]
fn minimal_min_dec_test() {
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).min(nd2di(1.0, 2.0, D::Com)),
        nd2di(f64::NEG_INFINITY, 2.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-7.0, -5.0, D::Trv).min(nd2di(2.0, 4.0, D::Def)),
        nd2di(-7.0, -5.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-7.0, 0.0, D::Dac).min(nd2di(2.0, 4.0, D::Def)),
        nd2di(-7.0, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(-7.0, -0.0, D::Com).min(nd2di(2.0, 4.0, D::Com)),
        nd2di(-7.0, 0.0, D::Com)
    );
}

#[test]
fn minimal_max_test() {
    assert_eq2!(I::EMPTY.max(n2i(1.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(1.0, 2.0).max(I::EMPTY), I::EMPTY);
    assert_eq2!(I::EMPTY.max(I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.max(n2i(1.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(1.0, 2.0).max(I::ENTIRE), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.max(I::ENTIRE), I::ENTIRE);
    assert_eq2!(I::EMPTY.max(I::ENTIRE), I::EMPTY);
    assert_eq2!(n2i(1.0, 5.0).max(n2i(2.0, 4.0)), n2i(2.0, 5.0));
    assert_eq2!(n2i(1.0, 5.0).max(n2i(2.0, 8.0)), n2i(2.0, 8.0));
    assert_eq2!(n2i(-1.0, 5.0).max(I::ENTIRE), n2i(-1.0, f64::INFINITY));
    assert_eq2!(n2i(-7.0, -5.0).max(n2i(2.0, 4.0)), n2i(2.0, 4.0));
    assert_eq2!(n2i(-7.0, -5.0).max(n2i(0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq2!(n2i(-7.0, -5.0).max(n2i(-0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq2!(n2i(-7.0, -5.0).max(n2i(-2.0, 0.0)), n2i(-2.0, 0.0));
    assert_eq2!(n2i(-7.0, -5.0).max(n2i(-2.0, -0.0)), n2i(-2.0, 0.0));
}

#[test]
fn minimal_max_dec_test() {
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).max(nd2di(1.0, 2.0, D::Com)),
        nd2di(1.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-7.0, -5.0, D::Trv).max(nd2di(2.0, 4.0, D::Def)),
        nd2di(2.0, 4.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-7.0, 5.0, D::Dac).max(nd2di(2.0, 4.0, D::Def)),
        nd2di(2.0, 5.0, D::Def)
    );
    assert_eq2!(
        nd2di(3.0, 3.5, D::Com).max(nd2di(2.0, 4.0, D::Com)),
        nd2di(3.0, 4.0, D::Com)
    );
}
