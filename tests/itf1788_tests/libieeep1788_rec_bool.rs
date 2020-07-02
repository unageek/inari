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
#![rustfmt::skip]
#![allow(unused_attributes, unused_imports)]

//Test library imports

//Arithmetic library imports

//Preamble
use crate::util::*;
use hexf::*;
type I = inari::Interval;

#[test]
fn minimal_is_common_interval_test() {
    assert!(n2i(-27.0, -27.0).is_common_interval());
    assert!(n2i(-27.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 0.0).is_common_interval());
    assert!(n2i(-0.0, -0.0).is_common_interval());
    assert!(n2i(-0.0, 0.0).is_common_interval());
    assert!(n2i(0.0, -0.0).is_common_interval());
    assert!(n2i(5.0, 12.4).is_common_interval());
    assert!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).is_common_interval());
    assert_eq!(I::entire().is_common_interval(), false);
    assert_eq!(I::empty().is_common_interval(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).is_common_interval(), false);
    assert_eq!(n2i(0.0, f64::INFINITY).is_common_interval(), false);
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
    assert_eq!(I::empty().is_singleton(), false);
    assert_eq!(I::entire().is_singleton(), false);
    assert_eq!(n2i(-1.0, 0.0).is_singleton(), false);
    assert_eq!(n2i(-1.0, -0.5).is_singleton(), false);
    assert_eq!(n2i(1.0, 2.0).is_singleton(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")).is_singleton(), false);
    assert_eq!(n2i(-1.0, f64::INFINITY).is_singleton(), false);
}

#[test]
fn minimal_is_member_test() {
    assert!(I::is_member(-27.0, n2i(-27.0, -27.0)));
    assert!(I::is_member(-27.0, n2i(-27.0, 0.0)));
    assert!(I::is_member(-7.0, n2i(-27.0, 0.0)));
    assert!(I::is_member(0.0, n2i(-27.0, 0.0)));
    assert!(I::is_member(-0.0, n2i(0.0, 0.0)));
    assert!(I::is_member(0.0, n2i(0.0, 0.0)));
    assert!(I::is_member(0.0, n2i(-0.0, -0.0)));
    assert!(I::is_member(0.0, n2i(-0.0, 0.0)));
    assert!(I::is_member(0.0, n2i(0.0, -0.0)));
    assert!(I::is_member(5.0, n2i(5.0, 12.4)));
    assert!(I::is_member(6.3, n2i(5.0, 12.4)));
    assert!(I::is_member(12.4, n2i(5.0, 12.4)));
    assert!(I::is_member(0.0, I::entire()));
    assert!(I::is_member(5.0, I::entire()));
    assert!(I::is_member(6.3, I::entire()));
    assert!(I::is_member(12.4, I::entire()));
    assert!(I::is_member(hexf64!("0x1.fffffffffffffp+1023"), I::entire()));
    assert!(I::is_member(hexf64!("-0x1.fffffffffffffp+1023"), I::entire()));
    assert!(I::is_member(hexf64!("0x1.0000000000000p-1022"), I::entire()));
    assert!(I::is_member(hexf64!("-0x1.0000000000000p-1022"), I::entire()));
    assert_eq!(I::is_member(-71.0, n2i(-27.0, 0.0)), false);
    assert_eq!(I::is_member(0.1, n2i(-27.0, 0.0)), false);
    assert_eq!(I::is_member(-0.01, n2i(0.0, 0.0)), false);
    assert_eq!(I::is_member(1e-06, n2i(0.0, 0.0)), false);
    assert_eq!(I::is_member(111110.0, n2i(-0.0, -0.0)), false);
    assert_eq!(I::is_member(4.9, n2i(5.0, 12.4)), false);
    assert_eq!(I::is_member(-6.3, n2i(5.0, 12.4)), false);
    assert_eq!(I::is_member(0.0, I::empty()), false);
    assert_eq!(I::is_member(-4535.3, I::empty()), false);
    assert_eq!(I::is_member(f64::NEG_INFINITY, I::empty()), false);
    assert_eq!(I::is_member(f64::INFINITY, I::empty()), false);
    assert_eq!(I::is_member(f64::NAN, I::empty()), false);
    assert_eq!(I::is_member(f64::NEG_INFINITY, I::entire()), false);
    assert_eq!(I::is_member(f64::INFINITY, I::entire()), false);
    assert_eq!(I::is_member(f64::NAN, I::entire()), false);
}
