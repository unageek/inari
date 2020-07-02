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
use crate::util::*;
type I = inari::Interval;

#[test]
fn minimal_is_empty_test() {
    assert!(I::empty().is_empty());
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).is_empty(), false);
    assert_eq!(n2i(1.0, 2.0).is_empty(), false);
    assert_eq!(n2i(-1.0, 2.0).is_empty(), false);
    assert_eq!(n2i(-3.0, -2.0).is_empty(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).is_empty(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).is_empty(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).is_empty(), false);
    assert_eq!(n2i(0.0, f64::INFINITY).is_empty(), false);
    assert_eq!(n2i(-0.0, f64::INFINITY).is_empty(), false);
    assert_eq!(n2i(-0.0, 0.0).is_empty(), false);
    assert_eq!(n2i(0.0, -0.0).is_empty(), false);
    assert_eq!(n2i(0.0, 0.0).is_empty(), false);
    assert_eq!(n2i(-0.0, -0.0).is_empty(), false);
}

#[test]
fn minimal_is_entire_test() {
    assert_eq!(I::empty().is_entire(), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).is_entire());
    assert_eq!(n2i(1.0, 2.0).is_entire(), false);
    assert_eq!(n2i(-1.0, 2.0).is_entire(), false);
    assert_eq!(n2i(-3.0, -2.0).is_entire(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).is_entire(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).is_entire(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).is_entire(), false);
    assert_eq!(n2i(0.0, f64::INFINITY).is_entire(), false);
    assert_eq!(n2i(-0.0, f64::INFINITY).is_entire(), false);
    assert_eq!(n2i(-0.0, 0.0).is_entire(), false);
    assert_eq!(n2i(0.0, -0.0).is_entire(), false);
    assert_eq!(n2i(0.0, 0.0).is_entire(), false);
    assert_eq!(n2i(-0.0, -0.0).is_entire(), false);
}

#[test]
fn minimal_equal_test() {
    assert!(n2i(1.0, 2.0) == n2i(1.0, 2.0));
    assert_eq!(n2i(1.0, 2.1) == n2i(1.0, 2.0), false);
    assert!(I::empty() == I::empty());
    assert_eq!(I::empty() == n2i(1.0, 2.0), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY) == n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.4) == n2i(f64::NEG_INFINITY, f64::INFINITY), false);
    assert!(n2i(1.0, f64::INFINITY) == n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.4) == n2i(1.0, f64::INFINITY), false);
    assert!(n2i(f64::NEG_INFINITY, 2.0) == n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.4) == n2i(f64::NEG_INFINITY, 2.0), false);
    assert!(n2i(-2.0, 0.0) == n2i(-2.0, 0.0));
    assert!(n2i(-0.0, 2.0) == n2i(0.0, 2.0));
    assert!(n2i(-0.0, -0.0) == n2i(0.0, 0.0));
    assert!(n2i(-0.0, 0.0) == n2i(0.0, 0.0));
    assert!(n2i(0.0, -0.0) == n2i(0.0, 0.0));
}

#[test]
fn minimal_subset_test() {
    assert!(I::empty().subset(I::empty()));
    assert!(I::empty().subset(n2i(0.0, 4.0)));
    assert!(I::empty().subset(n2i(-0.0, 4.0)));
    assert!(I::empty().subset(n2i(-0.1, 1.0)));
    assert!(I::empty().subset(n2i(-0.1, 0.0)));
    assert!(I::empty().subset(n2i(-0.1, -0.0)));
    assert!(I::empty().subset(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq!(n2i(0.0, 4.0).subset(I::empty()), false);
    assert_eq!(n2i(-0.0, 4.0).subset(I::empty()), false);
    assert_eq!(n2i(-0.1, 1.0).subset(I::empty()), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).subset(I::empty()), false);
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
fn minimal_less_test() {
    assert!(I::empty().less(I::empty()));
    assert_eq!(n2i(1.0, 2.0).less(I::empty()), false);
    assert_eq!(I::empty().less(n2i(1.0, 2.0)), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq!(n2i(1.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(0.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(-0.0, 2.0).less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(1.0, 2.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(0.0, 2.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).less(n2i(-0.0, 2.0)), false);
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
fn minimal_precedes_test() {
    assert!(I::empty().precedes(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).precedes(I::empty()));
    assert!(I::empty().precedes(I::empty()));
    assert_eq!(n2i(1.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(0.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(-0.0, 2.0).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).precedes(n2i(1.0, 2.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert!(n2i(1.0, 2.0).precedes(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 3.0).precedes(n2i(3.0, 4.0)));
    assert!(n2i(-3.0, -1.0).precedes(n2i(-1.0, 0.0)));
    assert!(n2i(-3.0, -1.0).precedes(n2i(-1.0, -0.0)));
    assert_eq!(n2i(1.0, 3.5).precedes(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(1.0, 4.0).precedes(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(-3.0, -0.1).precedes(n2i(-1.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(-0.0, -0.0)));
    assert!(n2i(-0.0, -0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-0.0, 0.0).precedes(n2i(0.0, -0.0)));
    assert!(n2i(0.0, -0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, -0.0).precedes(n2i(-0.0, 0.0)));
}

#[test]
fn minimal_interior_test() {
    assert!(I::empty().interior(I::empty()));
    assert!(I::empty().interior(n2i(0.0, 4.0)));
    assert_eq!(n2i(0.0, 4.0).interior(I::empty()), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(n2i(0.0, 4.0).interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert!(I::empty().interior(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).interior(n2i(0.0, 4.0)), false);
    assert_eq!(n2i(0.0, 4.0).interior(n2i(0.0, 4.0)), false);
    assert!(n2i(1.0, 2.0).interior(n2i(0.0, 4.0)));
    assert_eq!(n2i(-2.0, 2.0).interior(n2i(-2.0, 4.0)), false);
    assert!(n2i(-0.0, -0.0).interior(n2i(-2.0, 4.0)));
    assert!(n2i(0.0, 0.0).interior(n2i(-2.0, 4.0)));
    assert_eq!(n2i(0.0, 0.0).interior(n2i(-0.0, -0.0)), false);
    assert_eq!(n2i(0.0, 4.4).interior(n2i(0.0, 4.0)), false);
    assert_eq!(n2i(-1.0, -1.0).interior(n2i(0.0, 4.0)), false);
    assert_eq!(n2i(2.0, 2.0).interior(n2i(-2.0, -1.0)), false);
}

#[test]
fn minimal_strictly_less_test() {
    assert!(I::empty().strict_less(I::empty()));
    assert_eq!(n2i(1.0, 2.0).strict_less(I::empty()), false);
    assert_eq!(I::empty().strict_less(n2i(1.0, 2.0)), false);
    assert!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_less(n2i(f64::NEG_INFINITY, f64::INFINITY)));
    assert_eq!(n2i(1.0, 2.0).strict_less(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_less(n2i(1.0, 2.0)), false);
    assert_eq!(n2i(1.0, 2.0).strict_less(n2i(1.0, 2.0)), false);
    assert!(n2i(1.0, 2.0).strict_less(n2i(3.0, 4.0)));
    assert!(n2i(1.0, 3.5).strict_less(n2i(3.0, 4.0)));
    assert_eq!(n2i(1.0, 4.0).strict_less(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(0.0, 4.0).strict_less(n2i(0.0, 4.0)), false);
    assert_eq!(n2i(-0.0, 4.0).strict_less(n2i(0.0, 4.0)), false);
    assert_eq!(n2i(-2.0, -1.0).strict_less(n2i(-2.0, -1.0)), false);
    assert!(n2i(-3.0, -1.5).strict_less(n2i(-2.0, -1.0)));
}

#[test]
fn minimal_strictly_precedes_test() {
    assert!(I::empty().strict_precedes(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).strict_precedes(I::empty()));
    assert!(I::empty().strict_precedes(I::empty()));
    assert_eq!(n2i(1.0, 2.0).strict_precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_precedes(n2i(1.0, 2.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).strict_precedes(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert!(n2i(1.0, 2.0).strict_precedes(n2i(3.0, 4.0)));
    assert_eq!(n2i(1.0, 3.0).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(-3.0, -1.0).strict_precedes(n2i(-1.0, 0.0)), false);
    assert_eq!(n2i(-3.0, -0.0).strict_precedes(n2i(0.0, 1.0)), false);
    assert_eq!(n2i(-3.0, 0.0).strict_precedes(n2i(-0.0, 1.0)), false);
    assert_eq!(n2i(1.0, 3.5).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(1.0, 4.0).strict_precedes(n2i(3.0, 4.0)), false);
    assert_eq!(n2i(-3.0, -0.1).strict_precedes(n2i(-1.0, 0.0)), false);
}

#[test]
fn minimal_disjoint_test() {
    assert!(I::empty().disjoint(n2i(3.0, 4.0)));
    assert!(n2i(3.0, 4.0).disjoint(I::empty()));
    assert!(I::empty().disjoint(I::empty()));
    assert!(n2i(3.0, 4.0).disjoint(n2i(1.0, 2.0)));
    assert_eq!(n2i(0.0, 0.0).disjoint(n2i(-0.0, -0.0)), false);
    assert_eq!(n2i(0.0, -0.0).disjoint(n2i(-0.0, 0.0)), false);
    assert_eq!(n2i(3.0, 4.0).disjoint(n2i(1.0, 7.0)), false);
    assert_eq!(n2i(3.0, 4.0).disjoint(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).disjoint(n2i(1.0, 7.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY).disjoint(n2i(f64::NEG_INFINITY, f64::INFINITY)), false);
}
