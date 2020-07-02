/*
 *
 * Unit tests from libieeep1788 for the interval overlap operation
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
type O = inari::OverlappingState;

#[test]
fn minimal_overlap_test() {
    assert_eq!(I::empty().overlap(I::empty()), O::BothEmpty);
    assert_eq!(I::empty().overlap(n2i(1.0, 2.0)), O::FirstEmpty);
    assert_eq!(n2i(1.0, 2.0).overlap(I::empty()), O::SecondEmpty);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).overlap(n2i(3.0, f64::INFINITY)), O::Before);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).overlap(n2i(3.0, 4.0)), O::Before);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(3.0, 4.0)), O::Before);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(3.0, 4.0)), O::Before);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(3.0, 3.0)), O::Before);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(3.0, 3.0)), O::Before);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(3.0, f64::INFINITY)), O::Before);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).overlap(n2i(2.0, 3.0)), O::Meets);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(2.0, 3.0)), O::Meets);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(2.0, f64::INFINITY)), O::Meets);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(1.5, 2.5)), O::Overlaps);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(1.0, f64::INFINITY)), O::Starts);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(1.0, 3.0)), O::Starts);
    assert_eq!(n2i(1.0, 1.0).overlap(n2i(1.0, 3.0)), O::Starts);
    assert_eq!(n2i(1.0, 2.0).overlap(I::entire()), O::ContainedBy);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(f64::NEG_INFINITY, 3.0)), O::ContainedBy);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(0.0, 3.0)), O::ContainedBy);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(0.0, 3.0)), O::ContainedBy);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(0.0, f64::INFINITY)), O::ContainedBy);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(f64::NEG_INFINITY, 2.0)), O::Finishes);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(0.0, 2.0)), O::Finishes);
    assert_eq!(n2i(2.0, 2.0).overlap(n2i(0.0, 2.0)), O::Finishes);
    assert_eq!(n2i(1.0, 2.0).overlap(n2i(1.0, 2.0)), O::Equal);
    assert_eq!(n2i(1.0, 1.0).overlap(n2i(1.0, 1.0)), O::Equal);
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0).overlap(n2i(f64::NEG_INFINITY, 1.0)), O::Equal);
    assert_eq!(I::entire().overlap(I::entire()), O::Equal);
    assert_eq!(n2i(3.0, 4.0).overlap(n2i(2.0, 2.0)), O::After);
    assert_eq!(n2i(3.0, 4.0).overlap(n2i(1.0, 2.0)), O::After);
    assert_eq!(n2i(3.0, 3.0).overlap(n2i(1.0, 2.0)), O::After);
    assert_eq!(n2i(3.0, 3.0).overlap(n2i(2.0, 2.0)), O::After);
    assert_eq!(n2i(3.0, f64::INFINITY).overlap(n2i(2.0, 2.0)), O::After);
    assert_eq!(n2i(2.0, 3.0).overlap(n2i(1.0, 2.0)), O::MetBy);
    assert_eq!(n2i(2.0, 3.0).overlap(n2i(f64::NEG_INFINITY, 2.0)), O::MetBy);
    assert_eq!(n2i(1.5, 2.5).overlap(n2i(1.0, 2.0)), O::OverlappedBy);
    assert_eq!(n2i(1.5, 2.5).overlap(n2i(f64::NEG_INFINITY, 2.0)), O::OverlappedBy);
    assert_eq!(n2i(1.0, f64::INFINITY).overlap(n2i(1.0, 2.0)), O::StartedBy);
    assert_eq!(n2i(1.0, 3.0).overlap(n2i(1.0, 2.0)), O::StartedBy);
    assert_eq!(n2i(1.0, 3.0).overlap(n2i(1.0, 1.0)), O::StartedBy);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).overlap(n2i(1.0, 2.0)), O::Contains);
    assert_eq!(I::entire().overlap(n2i(1.0, 2.0)), O::Contains);
    assert_eq!(n2i(0.0, 3.0).overlap(n2i(1.0, 2.0)), O::Contains);
    assert_eq!(n2i(0.0, 3.0).overlap(n2i(2.0, 2.0)), O::Contains);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0).overlap(n2i(1.0, 2.0)), O::FinishedBy);
    assert_eq!(n2i(0.0, 2.0).overlap(n2i(1.0, 2.0)), O::FinishedBy);
    assert_eq!(n2i(0.0, 2.0).overlap(n2i(2.0, 2.0)), O::FinishedBy);
}
