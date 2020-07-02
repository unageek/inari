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
use crate::util::*;
type I = inari::Interval;

#[test]
fn minimal_intersection_test() {
    assert_eq!(n2i(1.0, 3.0).intersection(n2i(2.1, 4.0)), n2i(2.1, 3.0));
    assert_eq!(n2i(1.0, 3.0).intersection(n2i(3.0, 4.0)), n2i(3.0, 3.0));
    assert_eq!(n2i(1.0, 3.0).intersection(I::empty()), I::empty());
    assert_eq!(I::entire().intersection(I::empty()), I::empty());
    assert_eq!(n2i(1.0, 3.0).intersection(I::entire()), n2i(1.0, 3.0));
}

#[test]
fn minimal_convex_hull_test() {
    assert_eq!(n2i(1.0, 3.0).convex_hull(n2i(2.1, 4.0)), n2i(1.0, 4.0));
    assert_eq!(n2i(1.0, 1.0).convex_hull(n2i(2.1, 4.0)), n2i(1.0, 4.0));
    assert_eq!(n2i(1.0, 3.0).convex_hull(I::empty()), n2i(1.0, 3.0));
    assert_eq!(I::empty().convex_hull(I::empty()), I::empty());
    assert_eq!(n2i(1.0, 3.0).convex_hull(I::entire()), I::entire());
}
