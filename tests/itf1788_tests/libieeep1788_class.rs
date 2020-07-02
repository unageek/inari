/*
 *
 * Unit tests from libieeep1788 for interval constructors
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
fn minimal_nums_to_interval_test() {
    assert_eq!(n2i(-1.0, 1.0), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(-1.0, f64::INFINITY), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(n2i(f64::NAN, f64::NAN), I::empty());
    assert_eq!(n2i(1.0, -1.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, f64::NEG_INFINITY), I::empty());
    assert_eq!(n2i(f64::INFINITY, f64::INFINITY), I::empty());
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_t2i_test() {
    assert_eq!(t2i("[ Empty  ]"), I::empty());
    assert_eq!(t2i("[ Empty  ]_trv"), I::empty());
    assert_eq!(t2i("[  ]"), I::empty());
    assert_eq!(t2i("[  ]_trv"), I::empty());
    assert_eq!(t2i("[,]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[,]_trv"), I::empty());
    assert_eq!(t2i("[ entire  ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ ENTIRE ]_dac"), I::empty());
    assert_eq!(t2i("[ ENTIRE ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ -inf , INF  ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ -inf, INF ]_def"), I::empty());
    assert_eq!(t2i("[-1.0,1.0]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[  -1.0  ,  1.0  ]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[  -1.0  , 1.0]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[-1,]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-1.0, +inf]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-1.0, +infinity]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-Inf, 1.000 ]"), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(t2i("[-Infinity, 1.000 ]"), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(t2i("[1.0E+400 ]"), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(t2i("[ -4/2, 10/5 ]"), n2i(-2.0, 2.0));
    assert_eq!(t2i("[ -1/10, 1/10 ]"), n2i(-0.1, 0.1));
    assert_eq!(t2i("0.0?"), n2i(-0.05, 0.05));
    assert_eq!(t2i("0.0?u"), n2i(0.0, 0.05));
    assert_eq!(t2i("0.0?d"), n2i(-0.05, 0.0));
    assert_eq!(t2i("2.5?"), n2i(hexf64!("0x1.3999999999999p+1"), hexf64!("0x1.4666666666667p+1")));
    assert_eq!(t2i("2.5?u"), n2i(2.5, hexf64!("0x1.4666666666667p+1")));
    assert_eq!(t2i("2.5?d"), n2i(hexf64!("0x1.3999999999999p+1"), 2.5));
    assert_eq!(t2i("0.000?5"), n2i(-0.005, 0.005));
    assert_eq!(t2i("0.000?5u"), n2i(0.0, 0.005));
    assert_eq!(t2i("0.000?5d"), n2i(-0.005, 0.0));
    assert_eq!(t2i("2.500?5"), n2i(hexf64!("0x1.3f5c28f5c28f5p+1"), hexf64!("0x1.40a3d70a3d70bp+1")));
    assert_eq!(t2i("2.500?5u"), n2i(2.5, hexf64!("0x1.40a3d70a3d70bp+1")));
    assert_eq!(t2i("2.500?5d"), n2i(hexf64!("0x1.3f5c28f5c28f5p+1"), 2.5));
    assert_eq!(t2i("0.0??"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("0.0??u"), n2i(0.0, f64::INFINITY));
    assert_eq!(t2i("0.0??d"), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(t2i("2.5??"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("2.5??u"), n2i(2.5, f64::INFINITY));
    assert_eq!(t2i("2.5??d"), n2i(f64::NEG_INFINITY, 2.5));
    assert_eq!(t2i("2.500?5e+27"), n2i(hexf64!("0x1.01fa19a08fe7fp+91"), hexf64!("0x1.0302cc4352683p+91")));
    assert_eq!(t2i("2.500?5ue4"), n2i(hexf64!("0x1.86a0000000000p+14"), hexf64!("0x1.8768000000000p+14")));
    assert_eq!(t2i("2.500?5de-5"), n2i(hexf64!("0x1.a2976f1cee4d5p-16"), hexf64!("0x1.a36e2eb1c432dp-16")));
    assert_eq!(t2i("10?3"), n2i(7.0, 13.0));
    assert_eq!(t2i("10?3e380"), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(t2i("1.0000000000000001?1"), n2i(1.0, hexf64!("0x1.0000000000001p+0")));
    // 10?18 + 308 zeros
    assert_eq!(t2i("10?1800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ Nai  ]"), I::empty());
    assert_eq!(t2i("[ Nai  ]_ill"), I::empty());
    assert_eq!(t2i("[ Nai  ]_trv"), I::empty());
    assert_eq!(t2i("[ Empty  ]_ill"), I::empty());
    assert_eq!(t2i("[  ]_com"), I::empty());
    assert_eq!(t2i("[,]_com"), I::empty());
    assert_eq!(t2i("[   Entire ]_com"), I::empty());
    assert_eq!(t2i("[ -inf ,  INF ]_com"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_ill"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_fooo"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_da"), I::empty());
    assert_eq!(t2i("[-1.0,]_com"), I::empty());
    assert_eq!(t2i("[-Inf, 1.000 ]_ill"), I::empty());
    assert_eq!(t2i("[-I  nf, 1.000 ]"), I::empty());
    assert_eq!(t2i("[-Inf, 1.0  00 ]"), I::empty());
    assert_eq!(t2i("[-Inf ]"), I::empty());
    assert_eq!(t2i("[Inf , INF]"), I::empty());
    assert_eq!(t2i("[ foo ]"), I::empty());
    assert_eq!(t2i("0.0??_com"), I::empty());
    assert_eq!(t2i("0.0??u_ill"), I::empty());
    assert_eq!(t2i("0.0??d_com"), I::empty());
    assert_eq!(t2i("[1.0000000000000002,1.0000000000000001]"), I::empty());
    assert_eq!(t2i("[10000000000000001/10000000000000000,10000000000000002/10000000000000001]"), I::empty());
    assert_eq!(t2i("[0x1.00000000000002p0,0x1.00000000000001p0]"), I::empty());
}
