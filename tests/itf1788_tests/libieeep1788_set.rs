/*
 *
 * Unit tests from libieeep1788 for interval set operations
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
fn minimal_intersection_test() {
    assert_eq2!(n2i(1.0, 3.0).intersection(n2i(2.1, 4.0)), n2i(2.1, 3.0));
    assert_eq2!(n2i(1.0, 3.0).intersection(n2i(3.0, 4.0)), n2i(3.0, 3.0));
    assert_eq2!(n2i(1.0, 3.0).intersection(I::EMPTY), I::EMPTY);
    assert_eq2!(I::ENTIRE.intersection(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 3.0).intersection(I::ENTIRE), n2i(1.0, 3.0));
}

#[test]
fn minimal_intersection_dec_test() {
    assert_eq2!(nd2di(1.0, 3.0, D::Com).intersection(nd2di(2.1, 4.0, D::Com)), nd2di(2.1, 3.0, D::Trv));
    assert_eq2!(nd2di(1.0, 3.0, D::Dac).intersection(nd2di(3.0, 4.0, D::Def)), nd2di(3.0, 3.0, D::Trv));
    assert_eq2!(nd2di(1.0, 3.0, D::Def).intersection(DI::EMPTY), DI::EMPTY);
    assert_eq2!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).intersection(DI::EMPTY), DI::EMPTY);
    assert_eq2!(nd2di(1.0, 3.0, D::Dac).intersection(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(1.0, 3.0, D::Trv));
}

#[test]
fn minimal_convex_hull_test() {
    assert_eq2!(n2i(1.0, 3.0).convex_hull(n2i(2.1, 4.0)), n2i(1.0, 4.0));
    assert_eq2!(n2i(1.0, 1.0).convex_hull(n2i(2.1, 4.0)), n2i(1.0, 4.0));
    assert_eq2!(n2i(1.0, 3.0).convex_hull(I::EMPTY), n2i(1.0, 3.0));
    assert_eq2!(I::EMPTY.convex_hull(I::EMPTY), I::EMPTY);
    assert_eq2!(n2i(1.0, 3.0).convex_hull(I::ENTIRE), I::ENTIRE);
}

#[test]
fn minimal_convex_hull_dec_test() {
    assert_eq2!(nd2di(1.0, 3.0, D::Trv).convex_hull(nd2di(2.1, 4.0, D::Trv)), nd2di(1.0, 4.0, D::Trv));
    assert_eq2!(nd2di(1.0, 1.0, D::Trv).convex_hull(nd2di(2.1, 4.0, D::Trv)), nd2di(1.0, 4.0, D::Trv));
    assert_eq2!(nd2di(1.0, 3.0, D::Trv).convex_hull(DI::EMPTY), nd2di(1.0, 3.0, D::Trv));
    assert_eq2!(DI::EMPTY.convex_hull(DI::EMPTY), DI::EMPTY);
    assert_eq2!(nd2di(1.0, 3.0, D::Trv).convex_hull(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}
