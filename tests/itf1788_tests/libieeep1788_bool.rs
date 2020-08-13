/*
 *
 * Unit tests from libieeep1788 for interval boolean operations
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
type D = inari::Decoration;
type DI = inari::DecoratedInterval;
type I = inari::Interval;

#[test]
fn minimal_is_empty_test() {
    assert!(I::EMPTY.is_empty());
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).is_empty(), false);
    assert_eq2!(n2i(1.0, 2.0).is_empty(), false);
    assert_eq2!(n2i(-1.0, 2.0).is_empty(), false);
    assert_eq2!(n2i(-3.0, -2.0).is_empty(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).is_empty(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).is_empty(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).is_empty(), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).is_empty(), false);
    assert_eq2!(n2i(-0.0, f64::INFINITY).is_empty(), false);
    assert_eq2!(n2i(-0.0, 0.0).is_empty(), false);
    assert_eq2!(n2i(0.0, -0.0).is_empty(), false);
    assert_eq2!(n2i(0.0, 0.0).is_empty(), false);
    assert_eq2!(n2i(-0.0, -0.0).is_empty(), false);
}

#[test]
fn minimal_is_empty_dec_test() {
    assert_eq2!(DI::NAI.is_empty(), false);
    assert!(DI::EMPTY.is_empty());
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).is_empty(), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).is_empty(), false);
    assert_eq2!(nd2di(-1.0, 2.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(-3.0, -2.0, D::Dac).is_empty(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).is_empty(), false);
    assert_eq2!(nd2di(-0.0, f64::INFINITY, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(-0.0, 0.0, D::Com).is_empty(), false);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).is_empty(), false);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).is_empty(), false);
}

#[test]
fn minimal_is_entire_test() {
    assert_eq2!(I::EMPTY.is_entire(), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).is_entire());
    assert_eq2!(n2i(1.0, 2.0).is_entire(), false);
    assert_eq2!(n2i(-1.0, 2.0).is_entire(), false);
    assert_eq2!(n2i(-3.0, -2.0).is_entire(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.0).is_entire(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).is_entire(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.0).is_entire(), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).is_entire(), false);
    assert_eq2!(n2i(-0.0, f64::INFINITY).is_entire(), false);
    assert_eq2!(n2i(-0.0, 0.0).is_entire(), false);
    assert_eq2!(n2i(0.0, -0.0).is_entire(), false);
    assert_eq2!(n2i(0.0, 0.0).is_entire(), false);
    assert_eq2!(n2i(-0.0, -0.0).is_entire(), false);
}

#[test]
fn minimal_is_entire_dec_test() {
    assert_eq2!(DI::NAI.is_entire(), false);
    assert_eq2!(DI::EMPTY.is_entire(), false);
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).is_entire());
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).is_entire());
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).is_entire());
    assert_eq2!(nd2di(1.0, 2.0, D::Com).is_entire(), false);
    assert_eq2!(nd2di(-1.0, 2.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(-3.0, -2.0, D::Dac).is_entire(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).is_entire(), false);
    assert_eq2!(nd2di(-0.0, f64::INFINITY, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(-0.0, 0.0, D::Com).is_entire(), false);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).is_entire(), false);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).is_entire(), false);
}

#[test]
fn minimal_is_nai_dec_test() {
    assert!(DI::NAI.is_nai());
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).is_nai(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).is_nai(), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).is_nai(), false);
    assert_eq2!(nd2di(-1.0, 2.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(-3.0, -2.0, D::Dac).is_nai(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(0.0, f64::INFINITY, D::Def).is_nai(), false);
    assert_eq2!(nd2di(-0.0, f64::INFINITY, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(-0.0, 0.0, D::Com).is_nai(), false);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).is_nai(), false);
    assert_eq2!(nd2di(-0.0, -0.0, D::Trv).is_nai(), false);
}

#[test]
fn minimal_equal_test() {
    assert!(n2i(1.0, 2.0) == n2i(1.0, 2.0));
    assert_eq2!(n2i(1.0, 2.1) == n2i(1.0, 2.0), false);
    assert!(I::EMPTY == I::EMPTY);
    assert_eq2!(I::EMPTY == n2i(1.0, 2.0), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY) == n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq2!(n2i(1.0, 2.4) == n2i(f64::NEG_INFINITY, f64::INFINITY), false);
    assert!(n2i(1.0, f64::INFINITY) == n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(1.0, 2.4) == n2i(1.0, f64::INFINITY), false);
    assert!(n2i(f64::NEG_INFINITY, 2.0) == n2i(f64::NEG_INFINITY, 2.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 2.4) == n2i(f64::NEG_INFINITY, 2.0), false);
    assert!(n2i(-2.0, 0.0) == n2i(-2.0, 0.0));
    assert!(n2i(-0.0, 2.0) == n2i(0.0, 2.0));
    assert!(n2i(-0.0, -0.0) == n2i(0.0, 0.0));
    assert!(n2i(-0.0, 0.0) == n2i(0.0, 0.0));
    assert!(n2i(0.0, -0.0) == n2i(0.0, 0.0));
}

#[test]
fn minimal_equal_dec_test() {
    assert!(nd2di(1.0, 2.0, D::Def) == nd2di(1.0, 2.0, D::Trv));
    assert_eq2!(nd2di(1.0, 2.1, D::Trv) == nd2di(1.0, 2.0, D::Trv), false);
    assert!(DI::EMPTY == DI::EMPTY);
    assert_eq2!(DI::EMPTY == DI::NAI, false);
    assert_eq2!(DI::NAI == DI::EMPTY, false);
    assert_eq2!(DI::NAI == DI::NAI, false);
    assert_eq2!(DI::EMPTY == nd2di(1.0, 2.0, D::Trv), false);
    assert_eq2!(DI::NAI == nd2di(1.0, 2.0, D::Trv), false);
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def) == nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 2.4, D::Trv) == nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv), false);
    assert!(nd2di(1.0, f64::INFINITY, D::Trv) == nd2di(1.0, f64::INFINITY, D::Trv));
    assert_eq2!(nd2di(1.0, 2.4, D::Def) == nd2di(1.0, f64::INFINITY, D::Trv), false);
    assert!(nd2di(f64::NEG_INFINITY, 2.0, D::Trv) == nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
    assert_eq2!(nd2di(f64::NEG_INFINITY, 2.4, D::Def) == nd2di(f64::NEG_INFINITY, 2.0, D::Trv), false);
    assert!(nd2di(-2.0, 0.0, D::Trv) == nd2di(-2.0, 0.0, D::Trv));
    assert!(nd2di(-0.0, 2.0, D::Def) == nd2di(0.0, 2.0, D::Trv));
    assert!(nd2di(-0.0, -0.0, D::Trv) == nd2di(0.0, 0.0, D::Trv));
    assert!(nd2di(-0.0, 0.0, D::Def) == nd2di(0.0, 0.0, D::Trv));
    assert!(nd2di(0.0, -0.0, D::Trv) == nd2di(0.0, 0.0, D::Trv));
}

#[test]
fn minimal_subset_test() {
    assert!(I::EMPTY.subset(I::EMPTY));
    assert!(I::EMPTY.subset(n2i(0.0, 4.0)));
    assert!(I::EMPTY.subset(n2i(-0.0, 4.0)));
    assert!(I::EMPTY.subset(n2i(-0.1, 1.0)));
    assert!(I::EMPTY.subset(n2i(-0.1, 0.0)));
    assert!(I::EMPTY.subset(n2i(-0.1, -0.0)));
    assert!(I::EMPTY.subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq2!(n2i(0.0, 4.0).subset(I::EMPTY), false);
    assert_eq2!(n2i(-0.0, 4.0).subset(I::EMPTY), false);
    assert_eq2!(n2i(-0.1, 1.0).subset(I::EMPTY), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).subset(I::EMPTY), false);
    assert!(n2i(0.0, 4.0).subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(-0.0, 4.0).subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(-0.1, 1.0).subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(1.0, 2.0).subset(n2i(1.0, 2.0)));
    assert!(n2i(1.0, 2.0).subset(n2i(0.0, 4.0)));
    assert!(n2i(1.0, 2.0).subset(n2i(-0.0, 4.0)));
    assert!(n2i(0.1, 0.2).subset(n2i(0.0, 4.0)));
    assert!(n2i(0.1, 0.2).subset(n2i(-0.0, 4.0)));
    assert!(n2i(-0.1, -0.1).subset(n2i(-4.0, 3.4)));
    assert!(n2i(0.0, 0.0).subset(n2i(-0.0, -0.0)));
    assert!(n2i(-0.0, -0.0).subset(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).subset(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).subset(n2i(0.0, -0.0)));
    assert!(n2i(0.0, -0.0).subset(n2i(0.0, 0.0)));
    assert!(n2i(0.0, -0.0).subset(n2i(-0.0, 0.0)));
}

#[test]
fn minimal_subset_dec_test() {
    assert_eq2!(DI::NAI.subset(DI::NAI), false);
    assert_eq2!(DI::NAI.subset(DI::EMPTY), false);
    assert_eq2!(DI::NAI.subset(nd2di(0.0, 4.0, D::Trv)), false);
    assert!(DI::EMPTY.subset(nd2di(0.0, 4.0, D::Trv)));
    assert!(DI::EMPTY.subset(nd2di(-0.0, 4.0, D::Def)));
    assert!(DI::EMPTY.subset(nd2di(-0.1, 1.0, D::Trv)));
    assert!(DI::EMPTY.subset(nd2di(-0.1, 0.0, D::Trv)));
    assert!(DI::EMPTY.subset(nd2di(-0.1, -0.0, D::Trv)));
    assert!(DI::EMPTY.subset(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert_eq2!(nd2di(0.0, 4.0, D::Trv).subset(DI::EMPTY), false);
    assert_eq2!(nd2di(-0.0, 4.0, D::Def).subset(DI::EMPTY), false);
    assert_eq2!(nd2di(-0.1, 1.0, D::Trv).subset(DI::EMPTY), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).subset(DI::EMPTY), false);
    assert!(nd2di(0.0, 4.0, D::Trv).subset(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(nd2di(-0.0, 4.0, D::Trv).subset(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(nd2di(-0.1, 1.0, D::Trv).subset(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).subset(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(nd2di(1.0, 2.0, D::Trv).subset(nd2di(1.0, 2.0, D::Trv)));
    assert!(nd2di(1.0, 2.0, D::Trv).subset(nd2di(0.0, 4.0, D::Trv)));
    assert!(nd2di(1.0, 2.0, D::Def).subset(nd2di(-0.0, 4.0, D::Def)));
    assert!(nd2di(0.1, 0.2, D::Trv).subset(nd2di(0.0, 4.0, D::Trv)));
    assert!(nd2di(0.1, 0.2, D::Trv).subset(nd2di(-0.0, 4.0, D::Def)));
    assert!(nd2di(-0.1, -0.1, D::Trv).subset(nd2di(-4.0, 3.4, D::Trv)));
    assert!(nd2di(0.0, 0.0, D::Trv).subset(nd2di(-0.0, -0.0, D::Trv)));
    assert!(nd2di(-0.0, -0.0, D::Trv).subset(nd2di(0.0, 0.0, D::Def)));
    assert!(nd2di(-0.0, 0.0, D::Trv).subset(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(-0.0, 0.0, D::Trv).subset(nd2di(0.0, -0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Def).subset(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Trv).subset(nd2di(-0.0, 0.0, D::Trv)));
}

#[test]
fn minimal_less_test() {
    assert!(I::EMPTY.less(I::EMPTY));
    assert_eq2!(n2i(1.0, 2.0).less(I::EMPTY), false);
    assert_eq2!(I::EMPTY.less(n2i(1.0, 2.0)), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq2!(n2i(1.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(0.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(-0.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(1.0, 2.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(0.0, 2.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(-0.0, 2.0)), false);
    assert!(n2i(0.0, 2.0).less(n2i(0.0, 2.0)));
    assert!(n2i(0.0, 2.0).less(n2i(-0.0, 2.0)));
    assert!(n2i(0.0, 2.0).less(n2i(1.0, 2.0)));
    assert!(n2i(-0.0, 2.0).less(n2i(1.0, 2.0)));
    assert!(n2i(1.0, 2.0).less(n2i(1.0, 2.0)));
    assert!(n2i(1.0, 2.0).less(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 3.5).less(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 4.0).less(n2i(3.0, 4.0)));
    assert!(n2i(-2.0, -1.0).less(n2i(-2.0, -1.0)));
    assert!(n2i(-3.0, -1.5).less(n2i(-2.0, -1.0)));
    assert!(n2i(0.0, 0.0).less(n2i(-0.0, -0.0)));
    assert!(n2i(-0.0, -0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).less(n2i(0.0, -0.0)));
    assert!(n2i(0.0, -0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(0.0, -0.0).less(n2i(-0.0, 0.0)));
}

#[test]
fn minimal_less_dec_test() {
    assert_eq2!(DI::NAI.less(DI::NAI), false);
    assert_eq2!(DI::EMPTY.less(DI::NAI), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).less(DI::NAI), false);
    assert_eq2!(DI::NAI.less(nd2di(1.0, 2.0, D::Def)), false);
    assert!(DI::EMPTY.less(DI::EMPTY));
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).less(DI::EMPTY), false);
    assert_eq2!(DI::EMPTY.less(nd2di(1.0, 2.0, D::Trv)), false);
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert_eq2!(nd2di(1.0, 2.0, D::Def).less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(0.0, 2.0, D::Trv).less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(-0.0, 2.0, D::Trv).less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).less(nd2di(1.0, 2.0, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).less(nd2di(0.0, 2.0, D::Def)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).less(nd2di(-0.0, 2.0, D::Trv)), false);
    assert!(nd2di(0.0, 2.0, D::Trv).less(nd2di(0.0, 2.0, D::Trv)));
    assert!(nd2di(0.0, 2.0, D::Trv).less(nd2di(-0.0, 2.0, D::Trv)));
    assert!(nd2di(0.0, 2.0, D::Def).less(nd2di(1.0, 2.0, D::Def)));
    assert!(nd2di(-0.0, 2.0, D::Trv).less(nd2di(1.0, 2.0, D::Trv)));
    assert!(nd2di(1.0, 2.0, D::Trv).less(nd2di(1.0, 2.0, D::Trv)));
    assert!(nd2di(1.0, 2.0, D::Trv).less(nd2di(3.0, 4.0, D::Def)));
    assert!(nd2di(1.0, 3.5, D::Trv).less(nd2di(3.0, 4.0, D::Trv)));
    assert!(nd2di(1.0, 4.0, D::Trv).less(nd2di(3.0, 4.0, D::Trv)));
    assert!(nd2di(-2.0, -1.0, D::Trv).less(nd2di(-2.0, -1.0, D::Trv)));
    assert!(nd2di(-3.0, -1.5, D::Trv).less(nd2di(-2.0, -1.0, D::Trv)));
    assert!(nd2di(0.0, 0.0, D::Trv).less(nd2di(-0.0, -0.0, D::Trv)));
    assert!(nd2di(-0.0, -0.0, D::Trv).less(nd2di(0.0, 0.0, D::Def)));
    assert!(nd2di(-0.0, 0.0, D::Trv).less(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(-0.0, 0.0, D::Trv).less(nd2di(0.0, -0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Def).less(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Trv).less(nd2di(-0.0, 0.0, D::Trv)));
}

#[test]
fn minimal_precedes_test() {
    assert!(I::EMPTY.precedes(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).precedes(I::EMPTY));
    assert!(I::EMPTY.precedes(I::EMPTY));
    assert_eq2!(n2i(1.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(0.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(-0.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).precedes(n2i(1.0, 2.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert!(n2i(1.0, 2.0).precedes(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 3.0).precedes(n2i(3.0, 4.0)));
    assert!(n2i(-3.0, -1.0).precedes(n2i(-1.0, 0.0)));
    assert!(n2i(-3.0, -1.0).precedes(n2i(-1.0, -0.0)));
    assert_eq2!(n2i(1.0, 3.5).precedes(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(1.0, 4.0).precedes(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(-3.0, -0.1).precedes(n2i(-1.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(-0.0, -0.0)));
    assert!(n2i(-0.0, -0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).precedes(n2i(0.0, -0.0)));
    assert!(n2i(0.0, -0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, -0.0).precedes(n2i(-0.0, 0.0)));
}

#[test]
fn minimal_precedes_dec_test() {
    assert_eq2!(DI::NAI.precedes(nd2di(3.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(3.0, 4.0, D::Trv).precedes(DI::NAI), false);
    assert_eq2!(DI::NAI.precedes(DI::EMPTY), false);
    assert_eq2!(DI::NAI.precedes(DI::NAI), false);
    assert!(DI::EMPTY.precedes(nd2di(3.0, 4.0, D::Def)));
    assert!(nd2di(3.0, 4.0, D::Trv).precedes(DI::EMPTY));
    assert!(DI::EMPTY.precedes(DI::EMPTY));
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(0.0, 2.0, D::Trv).precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(-0.0, 2.0, D::Trv).precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).precedes(nd2di(1.0, 2.0, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert!(nd2di(1.0, 2.0, D::Trv).precedes(nd2di(3.0, 4.0, D::Trv)));
    assert!(nd2di(1.0, 3.0, D::Trv).precedes(nd2di(3.0, 4.0, D::Def)));
    assert!(nd2di(-3.0, -1.0, D::Def).precedes(nd2di(-1.0, 0.0, D::Trv)));
    assert!(nd2di(-3.0, -1.0, D::Trv).precedes(nd2di(-1.0, -0.0, D::Trv)));
    assert_eq2!(nd2di(1.0, 3.5, D::Trv).precedes(nd2di(3.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(1.0, 4.0, D::Trv).precedes(nd2di(3.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(-3.0, -0.1, D::Trv).precedes(nd2di(-1.0, 0.0, D::Trv)), false);
    assert!(nd2di(0.0, 0.0, D::Trv).precedes(nd2di(-0.0, -0.0, D::Trv)));
    assert!(nd2di(-0.0, -0.0, D::Trv).precedes(nd2di(0.0, 0.0, D::Def)));
    assert!(nd2di(-0.0, 0.0, D::Trv).precedes(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(-0.0, 0.0, D::Def).precedes(nd2di(0.0, -0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Trv).precedes(nd2di(0.0, 0.0, D::Trv)));
    assert!(nd2di(0.0, -0.0, D::Trv).precedes(nd2di(-0.0, 0.0, D::Trv)));
}

#[test]
fn minimal_interior_test() {
    assert!(I::EMPTY.interior(I::EMPTY));
    assert!(I::EMPTY.interior(n2i(0.0, 4.0)));
    assert_eq2!(n2i(0.0, 4.0).interior(I::EMPTY), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(0.0, 4.0).interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(I::EMPTY.interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).interior(n2i(0.0, 4.0)), false);
    assert_eq2!(n2i(0.0, 4.0).interior(n2i(0.0, 4.0)), false);
    assert!(n2i(1.0, 2.0).interior(n2i(0.0, 4.0)));
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-2.0, 4.0)), false);
    assert!(n2i(-0.0, -0.0).interior(n2i(-2.0, 4.0)));
    assert!(n2i(0.0, 0.0).interior(n2i(-2.0, 4.0)));
    assert_eq2!(n2i(0.0, 0.0).interior(n2i(-0.0, -0.0)), false);
    assert_eq2!(n2i(0.0, 4.4).interior(n2i(0.0, 4.0)), false);
    assert_eq2!(n2i(-1.0, -1.0).interior(n2i(0.0, 4.0)), false);
    assert_eq2!(n2i(2.0, 2.0).interior(n2i(-2.0, -1.0)), false);
}

#[test]
fn minimal_interior_dec_test() {
    assert_eq2!(DI::NAI.interior(DI::NAI), false);
    assert_eq2!(DI::NAI.interior(DI::EMPTY), false);
    assert_eq2!(DI::NAI.interior(nd2di(0.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(0.0, 4.0, D::Def).interior(DI::NAI), false);
    assert!(DI::EMPTY.interior(DI::EMPTY));
    assert!(DI::EMPTY.interior(nd2di(0.0, 4.0, D::Trv)));
    assert_eq2!(nd2di(0.0, 4.0, D::Def).interior(DI::EMPTY), false);
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).interior(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(nd2di(0.0, 4.0, D::Trv).interior(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert!(DI::EMPTY.interior(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).interior(nd2di(0.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(0.0, 4.0, D::Trv).interior(nd2di(0.0, 4.0, D::Trv)), false);
    assert!(nd2di(1.0, 2.0, D::Def).interior(nd2di(0.0, 4.0, D::Trv)));
    assert_eq2!(nd2di(-2.0, 2.0, D::Trv).interior(nd2di(-2.0, 4.0, D::Def)), false);
    assert!(nd2di(-0.0, -0.0, D::Trv).interior(nd2di(-2.0, 4.0, D::Trv)));
    assert!(nd2di(0.0, 0.0, D::Def).interior(nd2di(-2.0, 4.0, D::Trv)));
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).interior(nd2di(-0.0, -0.0, D::Trv)), false);
    assert_eq2!(nd2di(0.0, 4.4, D::Trv).interior(nd2di(0.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(-1.0, -1.0, D::Trv).interior(nd2di(0.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(2.0, 2.0, D::Def).interior(nd2di(-2.0, -1.0, D::Trv)), false);
}

#[test]
fn minimal_strictly_less_test() {
    assert!(I::EMPTY.strict_less(I::EMPTY));
    assert_eq2!(n2i(1.0, 2.0).strict_less(I::EMPTY), false);
    assert_eq2!(I::EMPTY.strict_less(n2i(1.0, 2.0)), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_less(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq2!(n2i(1.0, 2.0).strict_less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_less(n2i(1.0, 2.0)), false);
    assert_eq2!(n2i(1.0, 2.0).strict_less(n2i(1.0, 2.0)), false);
    assert!(n2i(1.0, 2.0).strict_less(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 3.5).strict_less(n2i(3.0, 4.0)));
    assert_eq2!(n2i(1.0, 4.0).strict_less(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(0.0, 4.0).strict_less(n2i(0.0, 4.0)), false);
    assert_eq2!(n2i(-0.0, 4.0).strict_less(n2i(0.0, 4.0)), false);
    assert_eq2!(n2i(-2.0, -1.0).strict_less(n2i(-2.0, -1.0)), false);
    assert!(n2i(-3.0, -1.5).strict_less(n2i(-2.0, -1.0)));
}

#[test]
fn minimal_strictly_less_dec_test() {
    assert_eq2!(DI::NAI.strict_less(DI::NAI), false);
    assert_eq2!(DI::EMPTY.strict_less(DI::NAI), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).strict_less(DI::NAI), false);
    assert_eq2!(DI::NAI.strict_less(nd2di(1.0, 2.0, D::Def)), false);
    assert!(DI::EMPTY.strict_less(DI::EMPTY));
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).strict_less(DI::EMPTY), false);
    assert_eq2!(DI::EMPTY.strict_less(nd2di(1.0, 2.0, D::Def)), false);
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).strict_less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)));
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).strict_less(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).strict_less(nd2di(1.0, 2.0, D::Trv)), false);
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).strict_less(nd2di(1.0, 2.0, D::Trv)), false);
    assert!(nd2di(1.0, 2.0, D::Trv).strict_less(nd2di(3.0, 4.0, D::Trv)));
    assert!(nd2di(1.0, 3.5, D::Def).strict_less(nd2di(3.0, 4.0, D::Trv)));
    assert_eq2!(nd2di(1.0, 4.0, D::Trv).strict_less(nd2di(3.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(0.0, 4.0, D::Trv).strict_less(nd2di(0.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(-0.0, 4.0, D::Def).strict_less(nd2di(0.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(-2.0, -1.0, D::Def).strict_less(nd2di(-2.0, -1.0, D::Def)), false);
    assert!(nd2di(-3.0, -1.5, D::Trv).strict_less(nd2di(-2.0, -1.0, D::Trv)));
}

#[test]
fn minimal_strictly_precedes_test() {
    assert!(I::EMPTY.strict_precedes(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).strict_precedes(I::EMPTY));
    assert!(I::EMPTY.strict_precedes(I::EMPTY));
    assert_eq2!(n2i(1.0, 2.0).strict_precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_precedes(n2i(1.0, 2.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert!(n2i(1.0, 2.0).strict_precedes(n2i(3.0, 4.0)));
    assert_eq2!(n2i(1.0, 3.0).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(-3.0, -1.0).strict_precedes(n2i(-1.0, 0.0)), false);
    assert_eq2!(n2i(-3.0, -0.0).strict_precedes(n2i(0.0, 1.0)), false);
    assert_eq2!(n2i(-3.0, 0.0).strict_precedes(n2i(-0.0, 1.0)), false);
    assert_eq2!(n2i(1.0, 3.5).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(1.0, 4.0).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq2!(n2i(-3.0, -0.1).strict_precedes(n2i(-1.0, 0.0)), false);
}

#[test]
fn minimal_strictly_precedes_dec_test() {
    assert_eq2!(DI::NAI.strict_precedes(nd2di(3.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(3.0, 4.0, D::Def).strict_precedes(DI::NAI), false);
    assert_eq2!(DI::NAI.strict_precedes(DI::EMPTY), false);
    assert_eq2!(DI::NAI.strict_precedes(DI::NAI), false);
    assert!(DI::EMPTY.strict_precedes(nd2di(3.0, 4.0, D::Trv)));
    assert!(nd2di(3.0, 4.0, D::Def).strict_precedes(DI::EMPTY));
    assert!(DI::EMPTY.strict_precedes(DI::EMPTY));
    assert_eq2!(nd2di(1.0, 2.0, D::Trv).strict_precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).strict_precedes(nd2di(1.0, 2.0, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).strict_precedes(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert!(nd2di(1.0, 2.0, D::Trv).strict_precedes(nd2di(3.0, 4.0, D::Trv)));
    assert_eq2!(nd2di(1.0, 3.0, D::Def).strict_precedes(nd2di(3.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(-3.0, -1.0, D::Trv).strict_precedes(nd2di(-1.0, 0.0, D::Def)), false);
    assert_eq2!(nd2di(-3.0, -0.0, D::Def).strict_precedes(nd2di(0.0, 1.0, D::Trv)), false);
    assert_eq2!(nd2di(-3.0, 0.0, D::Trv).strict_precedes(nd2di(-0.0, 1.0, D::Trv)), false);
    assert_eq2!(nd2di(1.0, 3.5, D::Trv).strict_precedes(nd2di(3.0, 4.0, D::Trv)), false);
    assert_eq2!(nd2di(1.0, 4.0, D::Trv).strict_precedes(nd2di(3.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(-3.0, -0.1, D::Trv).strict_precedes(nd2di(-1.0, 0.0, D::Trv)), false);
}

#[test]
fn minimal_disjoint_test() {
    assert!(I::EMPTY.disjoint(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).disjoint(I::EMPTY));
    assert!(I::EMPTY.disjoint(I::EMPTY));
    assert!(n2i(3.0, 4.0).disjoint(n2i(1.0, 2.0)));
    assert_eq2!(n2i(0.0, 0.0).disjoint(n2i(-0.0, -0.0)), false);
    assert_eq2!(n2i(0.0, -0.0).disjoint(n2i(-0.0, 0.0)), false);
    assert_eq2!(n2i(3.0, 4.0).disjoint(n2i(1.0, 7.0)), false);
    assert_eq2!(n2i(3.0, 4.0).disjoint(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).disjoint(n2i(1.0, 7.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, f64::INFINITY).disjoint(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
}

#[test]
fn minimal_disjoint_dec_test() {
    assert_eq2!(DI::NAI.disjoint(nd2di(3.0, 4.0, D::Def)), false);
    assert_eq2!(nd2di(3.0, 4.0, D::Trv).disjoint(DI::NAI), false);
    assert_eq2!(DI::EMPTY.disjoint(DI::NAI), false);
    assert_eq2!(DI::NAI.disjoint(DI::NAI), false);
    assert!(DI::EMPTY.disjoint(nd2di(3.0, 4.0, D::Def)));
    assert!(nd2di(3.0, 4.0, D::Trv).disjoint(DI::EMPTY));
    assert!(DI::EMPTY.disjoint(DI::EMPTY));
    assert!(nd2di(3.0, 4.0, D::Trv).disjoint(nd2di(1.0, 2.0, D::Def)));
    assert_eq2!(nd2di(0.0, 0.0, D::Trv).disjoint(nd2di(-0.0, -0.0, D::Trv)), false);
    assert_eq2!(nd2di(0.0, -0.0, D::Trv).disjoint(nd2di(-0.0, 0.0, D::Trv)), false);
    assert_eq2!(nd2di(3.0, 4.0, D::Def).disjoint(nd2di(1.0, 7.0, D::Def)), false);
    assert_eq2!(nd2di(3.0, 4.0, D::Trv).disjoint(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).disjoint(nd2di(1.0, 7.0, D::Trv)), false);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).disjoint(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), false);
}
