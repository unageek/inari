/*
 *
 * Unit tests from libieeep1788 for interval reverse multiplication
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
fn minimal_mulrevtopair_test() {
    assert_eq2!(I::EMPTY.mul_rev_to_pair(n2i(1.0, 2.0))[0], I::EMPTY);
    assert_eq2!(I::EMPTY.mul_rev_to_pair(n2i(1.0, 2.0))[1], I::EMPTY);
    assert_eq2!(n2i(1.0, 2.0).mul_rev_to_pair(I::EMPTY)[0], I::EMPTY);
    assert_eq2!(n2i(1.0, 2.0).mul_rev_to_pair(I::EMPTY)[1], I::EMPTY);
    assert_eq2!(I::EMPTY.mul_rev_to_pair(I::EMPTY)[0], I::EMPTY);
    assert_eq2!(I::EMPTY.mul_rev_to_pair(I::EMPTY)[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(0.2, 21.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(0.2, f64::INFINITY)
    );
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, -0.3636363636363636)
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        n2i(0.2, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, -0.3636363636363636)
    );
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(-210.00000000000003, -0.3636363636363636)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[0], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(0.0, 21.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, -0.3636363636363636)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        n2i(0.2, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(-210.00000000000003, 0.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(-2.1, -0.4))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(-2.1, -0.4))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        n2i(0.0, 21.0)
    );
    assert_eq2!(n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        n2i(-210.00000000000003, 0.0)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        n2i(0.0, 21.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[0],
        n2i(-210.00000000000003, 0.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(-2.1, 0.0))[0], I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(-2.1, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        n2i(-1.2, 21.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        n2i(-210.00000000000003, 12.0)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        n2i(-1.2, 21.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[0],
        n2i(-210.00000000000003, 12.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-2.1, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(-2.1, 0.12))[0], I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(-2.1, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        n2i(-1.2, 0.0)
    );
    assert_eq2!(n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        n2i(0.0, 12.0)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        n2i(-1.2, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[0],
        n2i(0.0, 12.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(0.0, 0.12))[0], I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(0.0, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(-1.2, -0.005)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, -0.005)
    );
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, -0.005)
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        n2i(0.009090909090909089, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(0.009090909090909089, f64::INFINITY)
    );
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(0.009090909090909089, 12.0)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[0], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(-1.2, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        n2i(0.009090909090909089, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, -0.005)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(0.0, 12.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.01, 0.12))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.01, 0.12))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.01, 0.12))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        n2i(0.0, 0.0)
    );
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[0],
        n2i(0.0, 0.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(0.0, 0.0))[0], I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(n2i(0.0, 0.0))[1], I::EMPTY);
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(0.05, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(0.05, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, -0.0909090909090909)
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        n2i(0.05, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, -0.0909090909090909)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, -0.0909090909090909)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, -0.0909090909090909)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        n2i(0.05, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, -0.1))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[0],
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.0))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        n2i(f64::NEG_INFINITY, 30.0)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        n2i(-3.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        n2i(f64::NEG_INFINITY, 30.0)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[0],
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(f64::NEG_INFINITY, 0.3))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 2.1)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        n2i(-21.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 2.1)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        n2i(-21.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(-0.21, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.0, f64::INFINITY))[0],
        I::ENTIRE
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.0, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, -0.02)
    );
    assert_eq2!(
        n2i(-2.0, -0.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, -0.02)
    );
    assert_eq2!(
        n2i(-2.0, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, -0.02)
    );
    assert_eq2!(
        n2i(-2.0, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        n2i(0.036363636363636355, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(0.036363636363636355, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(0.036363636363636355, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        n2i(0.036363636363636355, f64::INFINITY)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, -0.02)
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        I::EMPTY
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.04, f64::INFINITY))[0],
        n2i(f64::NEG_INFINITY, 0.0)
    );
    assert_eq2!(
        I::ENTIRE.mul_rev_to_pair(n2i(0.04, f64::INFINITY))[1],
        n2i(0.0, f64::INFINITY)
    );
    assert_eq2!(n2i(-2.0, -0.1).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, -0.1).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 0.0).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.1).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 1.1).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(0.01, 1.1).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, -0.1).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 0.0).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(f64::NEG_INFINITY, 1.1).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(-2.0, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.0, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[0],
        I::ENTIRE
    );
    assert_eq2!(
        n2i(0.01, f64::INFINITY).mul_rev_to_pair(I::ENTIRE)[1],
        I::EMPTY
    );
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(I::ENTIRE)[0], I::ENTIRE);
    assert_eq2!(I::ENTIRE.mul_rev_to_pair(I::ENTIRE)[1], I::EMPTY);
}

#[test]
fn minimal_mulrevtopair_dec_test() {
    assert_eq2!(DI::NAI.mul_rev_to_pair(nd2di(1.0, 2.0, D::Def))[0], DI::NAI);
    assert_eq2!(DI::NAI.mul_rev_to_pair(nd2di(1.0, 2.0, D::Def))[1], DI::NAI);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).mul_rev_to_pair(DI::NAI)[0], DI::NAI);
    assert_eq2!(nd2di(1.0, 2.0, D::Com).mul_rev_to_pair(DI::NAI)[1], DI::NAI);
    assert_eq2!(DI::NAI.mul_rev_to_pair(DI::NAI)[0], DI::NAI);
    assert_eq2!(DI::NAI.mul_rev_to_pair(DI::NAI)[1], DI::NAI);
    assert_eq2!(
        DI::EMPTY.mul_rev_to_pair(nd2di(1.0, 2.0, D::Def))[0],
        DI::EMPTY
    );
    assert_eq2!(
        DI::EMPTY.mul_rev_to_pair(nd2di(1.0, 2.0, D::Def))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com).mul_rev_to_pair(DI::EMPTY)[0],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(1.0, 2.0, D::Com).mul_rev_to_pair(DI::EMPTY)[1],
        DI::EMPTY
    );
    assert_eq2!(DI::EMPTY.mul_rev_to_pair(DI::EMPTY)[0], DI::EMPTY);
    assert_eq2!(DI::EMPTY.mul_rev_to_pair(DI::EMPTY)[1], DI::EMPTY);
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[0],
        nd2di(0.2, 21.0, D::Com)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[0],
        nd2di(0.2, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.3636363636363636, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[1],
        nd2di(0.2, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Trv).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[0],
        nd2di(f64::NEG_INFINITY, -0.3636363636363636, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Trv).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[0],
        nd2di(-210.00000000000003, -0.3636363636363636, D::Com)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[0],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[0],
        nd2di(0.0, 21.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[0],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Trv).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[0],
        nd2di(f64::NEG_INFINITY, -0.3636363636363636, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Trv).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))[1],
        nd2di(0.2, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[0],
        nd2di(-210.00000000000003, 0.0, D::Def)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Def))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, -0.4, D::Dac))
            [1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(0.0, 21.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(-210.00000000000003, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(0.0, 21.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[0],
        nd2di(-210.00000000000003, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-2.1, 0.0, D::Com))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(-1.2, 21.0, D::Def)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(-210.00000000000003, 12.0, D::Def)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(-1.2, 21.0, D::Def)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[0],
        nd2di(-210.00000000000003, 12.0, D::Def)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).mul_rev_to_pair(nd2di(-2.1, 0.12, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(-1.2, 0.0, D::Com)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(0.0, 12.0, D::Com)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(-1.2, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[0],
        nd2di(0.0, 12.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.12, D::Com))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(-1.2, -0.005, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.005, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.005, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        nd2di(0.009090909090909089, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(0.009090909090909089, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(0.009090909090909089, 12.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(-1.2, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        nd2di(0.009090909090909089, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.005, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[0],
        nd2di(0.0, 12.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.01, 0.12, D::Dac))
            [1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(0.0, 0.0, D::Com)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(0.0, 0.0, D::Com)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Com).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(0.0, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(0.0, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, 0.0, D::Com))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        nd2di(0.05, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        nd2di(0.05, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.0909090909090909, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        nd2di(0.05, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.0909090909090909, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.0909090909090909, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[0],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[0],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, -0.0909090909090909, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [1],
        nd2di(0.05, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, -0.1, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            -0.1,
            D::Dac
        ))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.0, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.0,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(-3.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 30.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[0],
        nd2di(-3.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 30.0, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, 0.3, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            0.3,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 2.1, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(-21.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 2.1, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[0],
        nd2di(-21.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(-0.21, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            -0.21,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            -0.21,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.0, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            0.0,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            0.0,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.02, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.02, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.02, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        nd2di(0.036363636363636355, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(0.036363636363636355, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(0.036363636363636355, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))
            [1],
        nd2di(0.036363636363636355, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, -0.02, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[0],
        nd2di(0.0, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(0.04, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            0.04,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, 0.0, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            0.04,
            f64::INFINITY,
            D::Dac
        ))[1],
        nd2di(0.0, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(-2.0, -0.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, 1.1, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))
            [1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, 0.0, D::Dac).mul_rev_to_pair(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, -0.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 0.0, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, 1.1, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(-2.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(0.0, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)
    );
    assert_eq2!(
        nd2di(0.01, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[0],
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)
    );
    assert_eq2!(
        nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).mul_rev_to_pair(nd2di(
            f64::NEG_INFINITY,
            f64::INFINITY,
            D::Dac
        ))[1],
        DI::EMPTY
    );
}
