/*
 *
 * Unit tests from GNU MPFI
 * (Original authors: Philippe Theveny and Nathalie Revol )
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 2009–2012 Spaces project, Inria Lorraine
 *                     and Salsa project, INRIA Rocquencourt,
 *                     and Arenaire project, Inria Rhone-Alpes, France
 *                     and Lab. ANO, USTL (Univ. of Lille), France
 * Copyright 2015-2016 Oliver Heimlich
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
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
fn mpfi_abs() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).abs(), n2i(7.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).abs(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0).abs(), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 8.0).abs(), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("0x1.2345678900000p+16"), hexf64!("0x1.2345679900000p+16")).abs(), n2i(hexf64!("0x1.2345678900000p+16"), hexf64!("0x1.2345679900000p+16")));
    assert_eq!(n2i(hexf64!("-0x1.2345678900000p+16"), hexf64!("0x1.2345679900000p+16")).abs(), n2i(0.0, hexf64!("0x1.2345679900000p+16")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_acos() {
    // special values
    assert_eq!(n2i(-1.0, 0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    // regular values
    assert_eq!(n2i(-1.0, -0.5).acos(), n2i(hexf64!("0x1.0c152382d7365p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.75, -0.25).acos(), n2i(hexf64!("0x1.d2cf5c7c70f0bp+0"), hexf64!("0x1.359d26f93b6c4p+1")));
    assert_eq!(n2i(-0.5, 0.5).acos(), n2i(hexf64!("0x1.0c152382d7365p+0"), hexf64!("0x1.0c152382d7366p+1")));
    assert_eq!(n2i(0.25, 0.625).acos(), n2i(hexf64!("0x1.ca94936b98a21p-1"), hexf64!("0x1.51700e0c14b25p+0")));
    assert_eq!(n2i(-1.0, 1.0).acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_acosh() {
    // special values
    assert_eq!(n2i(1.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.5, f64::INFINITY).acosh(), n2i(hexf64!("0x1.ecc2caec51609p-1"), f64::INFINITY));
    // regular values
    assert_eq!(n2i(1.0, 1.5).acosh(), n2i(0.0, hexf64!("0x1.ecc2caec5160ap-1")));
    assert_eq!(n2i(1.5, 1.5).acosh(), n2i(hexf64!("0x1.ecc2caec51609p-1"), hexf64!("0x1.ecc2caec5160ap-1")));
    assert_eq!(n2i(2.0, 1000.0).acosh(), n2i(hexf64!("0x1.5124271980434p+0"), hexf64!("0x1.e6752e8a84ed4p+2")));
}

#[test]
fn mpfi_add() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) + n2i(-1.0, 8.0), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) + n2i(8.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) + n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 16.0));
    assert_eq!(I::ENTIRE + n2i(0.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) + n2i(f64::NEG_INFINITY, -7.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq!(n2i(0.0, 8.0) + n2i(-7.0, 0.0), n2i(-7.0, 8.0));
    assert_eq!(n2i(0.0, 0.0) + n2i(0.0, 8.0), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, f64::INFINITY) + n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) + n2i(8.0, f64::INFINITY), n2i(8.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) + I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 8.0) + n2i(0.0, 8.0), n2i(0.0, 16.0));
    assert_eq!(n2i(0.0, 0.0) + n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) + n2i(-7.0, 8.0), n2i(-7.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-0.375, hexf64!("-0x1.0187000000000p-240")) + n2i(-0.125, hexf64!("0x1.0000000000000p-240")), n2i(hexf64!("-0x1.0000000000000p-1"), hexf64!("-0x1.8700000000000p-248")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p-300"), hexf64!("0x1.2345600000000p+48")) + n2i(hexf64!("-0x1.0000000000000p-41"), hexf64!("0x1.e26af34000000p+26")), n2i(hexf64!("-0x1.0000000000001p-41"), hexf64!("0x1.23456789abcd0p+48")));
    assert_eq!(n2i(-4.0, 7.0) + n2i(hexf64!("-0x1.23456789abcd0p+31"), 3e+300), n2i(hexf64!("-0x1.23456791abcd0p+31"), hexf64!("0x1.1eb2d66005836p+998")));
    assert_eq!(n2i(hexf64!("0x1.0001000100010p+56"), hexf64!("0x1.0000000000000p+60")) + n2i(hexf64!("0x1.0001000100010p+48"), 3e+300), n2i(hexf64!("0x1.0101010101010p+56"), hexf64!("0x1.1eb2d66005836p+998")));
    // signed zeros
    assert_eq!(n2i(4.0, 8.0) + n2i(-4.0, -2.0), n2i(0.0, 6.0));
    assert_eq!(n2i(4.0, 8.0) + n2i(-9.0, -8.0), n2i(-5.0, 0.0));
}

#[test]
fn mpfi_add_d() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) + n2i(hexf64!("-0x1.70ef54646d497p-55"), hexf64!("-0x1.70ef54646d497p-55")), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) + n2i(hexf64!("0x1.70ef54646d497p-55"), hexf64!("0x1.70ef54646d497p-55")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.bffffffffffffp+2")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) + n2i(hexf64!("-0x1.70ef54646d497p-54"), hexf64!("-0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, -8e-17));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) + n2i(hexf64!("0x1.70ef54646d497p-54"), hexf64!("0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, hexf64!("0x1.70ef54646d497p-54")));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) + n2i(hexf64!("-0x1.6345785d8a000p+60"), hexf64!("-0x1.6345785d8a000p+60")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.6345785d89fffp+60")));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) + n2i(hexf64!("0x1.6345785d8a000p+60"), hexf64!("0x1.6345785d8a000p+60")), n2i(f64::NEG_INFINITY, hexf64!("0x1.6345785d8a001p+60")));
    assert_eq!(I::ENTIRE + n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(I::ENTIRE + n2i(0.0, 0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE + n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) + n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")), n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")));
    assert_eq!(n2i(0.0, 0.0) + n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) + n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")), n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")));
    assert_eq!(n2i(0.0, 8.0) + n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")), n2i(hexf64!("-0x1.14b37f4b51f71p-55"), 8.0));
    assert_eq!(n2i(0.0, 8.0) + n2i(0.0, 0.0), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, 8.0) + n2i(hexf64!("0x1.14b37f4b51f70p-55"), hexf64!("0x1.14b37f4b51f70p-55")), n2i(hexf64!("0x1.14b37f4b51f70p-55"), hexf64!("0x1.0000000000001p+3")));
    assert_eq!(n2i(0.0, f64::INFINITY) + n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")), n2i(hexf64!("-0x1.42d169d7dfa04p-54"), f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) + n2i(0.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) + n2i(hexf64!("0x1.42d169d7dfa03p-54"), hexf64!("0x1.42d169d7dfa03p-54")), n2i(hexf64!("0x1.42d169d7dfa03p-54"), f64::INFINITY));
    // regular values
    assert_eq!(n2i(-32.0, -17.0) + n2i(hexf64!("-0x1.f6a7a2955385ep+4"), hexf64!("-0x1.f6a7a2955385ep+4")), n2i(hexf64!("-0x1.fb53d14aa9c2fp+5"), hexf64!("-0x1.8353d14aa9c2fp+5")));
    assert_eq!(n2i(hexf64!("-0x1.f6a7a2955385ep+4"), -17.0) + n2i(hexf64!("0x1.f6a7a2955385ep+4"), hexf64!("0x1.f6a7a2955385ep+4")), n2i(0.0, hexf64!("0x1.cd4f452aa70bcp+3")));
    assert_eq!(n2i(-32.0, hexf64!("-0x1.f6a7a2955385ep+3")) + n2i(hexf64!("0x1.f6a7a2955385ep+3"), hexf64!("0x1.f6a7a2955385ep+3")), n2i(hexf64!("-0x1.04ac2eb5563d1p+4"), 0.0));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp+4"), hexf64!("0x1.23456789abcdfp+48")) + n2i(3.5, 3.5), n2i(hexf64!("0x1.5b456789abcdfp+4"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp-4"), hexf64!("0x1.23456789abcdfp+48")) + n2i(3.5, 3.5), n2i(hexf64!("0x1.c91a2b3c4d5e6p+1"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(hexf64!("-0x1.fe00000000000p+7"), hexf64!("0x1.23456789abcdfp+0")) + n2i(256.5, 256.5), n2i(hexf64!("0x1.8000000000000p+0"), hexf64!("0x1.01a3456789abdp+8")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+0"), hexf64!("-0x1.0000000000000p-550")) + n2i(4097.5, 4097.5), n2i(hexf64!("0x1.fff0000000000p+11"), hexf64!("0x1.0018000000000p+12")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp+4"), hexf64!("0x1.23456789abcdfp+48")) + n2i(-3.5, -3.5), n2i(hexf64!("0x1.d68acf13579bep+3"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp-4"), hexf64!("0x1.23456789abcdfp+48")) + n2i(-3.5, -3.5), n2i(hexf64!("-0x1.b6e5d4c3b2a1ap+1"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(hexf64!("-0x1.fe00000000000p+7"), hexf64!("0x1.23456789abcdfp+0")) + n2i(-256.5, -256.5), n2i(hexf64!("-0x1.ff80000000000p+8"), hexf64!("-0x1.feb97530eca86p+7")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+0"), hexf64!("-0x1.0000000000000p-550")) + n2i(-4097.5, -4097.5), n2i(hexf64!("-0x1.0038000000000p+12"), hexf64!("-0x1.0018000000000p+12")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_asin() {
    // special values
    assert_eq!(n2i(-1.0, 0.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(0.0, 0.0).asin(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).asin(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    // regular values
    assert_eq!(n2i(-1.0, -0.5).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.0c152382d7365p-1")));
    assert_eq!(n2i(-0.75, -0.25).asin(), n2i(hexf64!("-0x1.b235315c680ddp-1"), hexf64!("-0x1.02be9ce0b87cdp-2")));
    assert_eq!(n2i(-0.5, 0.5).asin(), n2i(hexf64!("-0x1.0c152382d7366p-1"), hexf64!("0x1.0c152382d7366p-1")));
    assert_eq!(n2i(0.25, 0.625).asin(), n2i(hexf64!("0x1.02be9ce0b87cdp-2"), hexf64!("0x1.59aad71ced00fp-1")));
    assert_eq!(n2i(-1.0, 1.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_asinh() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).asinh(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.52728c91b5f1dp+1")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).asinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).asinh(), n2i(f64::NEG_INFINITY, hexf64!("0x1.63637195fabd4p+1")));
    assert_eq!(I::ENTIRE.asinh(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 0.0).asinh(), n2i(hexf64!("-0x1.c34366179d427p-1"), 0.0));
    assert_eq!(n2i(0.0, 0.0).asinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).asinh(), n2i(0.0, hexf64!("0x1.c34366179d427p-1")));
    assert_eq!(n2i(0.0, 8.0).asinh(), n2i(0.0, hexf64!("0x1.63637195fabd4p+1")));
    assert_eq!(n2i(0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-6.0, -4.0).asinh(), n2i(hexf64!("-0x1.3ef2a467f8864p+1"), hexf64!("-0x1.0c1f8a6e80eebp+1")));
    assert_eq!(n2i(-2.0, -0.5).asinh(), n2i(hexf64!("-0x1.719218313d088p+0"), hexf64!("-0x1.ecc2caec51609p-2")));
    assert_eq!(n2i(-1.0, -0.5).asinh(), n2i(hexf64!("-0x1.c34366179d427p-1"), hexf64!("-0x1.ecc2caec51609p-2")));
    assert_eq!(n2i(-0.75, -0.25).asinh(), n2i(hexf64!("-0x1.62e42fefa39f0p-1"), hexf64!("-0x1.facfb2399e636p-3")));
    assert_eq!(n2i(-0.5, 0.5).asinh(), n2i(hexf64!("-0x1.ecc2caec5160ap-2"), hexf64!("0x1.ecc2caec5160ap-2")));
    assert_eq!(n2i(0.25, 0.625).asinh(), n2i(hexf64!("0x1.facfb2399e636p-3"), hexf64!("0x1.2e27502cbfb34p-1")));
    assert_eq!(n2i(-1.0, 1.0).asinh(), n2i(hexf64!("-0x1.c34366179d427p-1"), hexf64!("0x1.c34366179d427p-1")));
    assert_eq!(n2i(0.125, 17.0).asinh(), n2i(hexf64!("0x1.fead0b6996972p-4"), hexf64!("0x1.c37c174a83deep+1")));
    assert_eq!(n2i(17.0, 42.0).asinh(), n2i(hexf64!("0x1.c37c174a83dedp+1"), hexf64!("0x1.1b94d2ed5ad7ap+2")));
    assert_eq!(n2i(-42.0, 17.0).asinh(), n2i(hexf64!("-0x1.1b94d2ed5ad7ap+2"), hexf64!("0x1.c37c174a83deep+1")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_atan() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.6dcc57bb565fcp+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.7249faa996a22p+0")));
    assert_eq!(I::ENTIRE.atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-1.0, 0.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p-1"), 0.0));
    assert_eq!(n2i(0.0, 0.0).atan(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).atan(), n2i(0.0, hexf64!("0x1.921fb54442d19p-1")));
    assert_eq!(n2i(0.0, 8.0).atan(), n2i(0.0, hexf64!("0x1.7249faa996a22p+0")));
    assert_eq!(n2i(0.0, f64::INFINITY).atan(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    // regular values
    assert_eq!(n2i(-6.0, -4.0).atan(), n2i(hexf64!("-0x1.67d8863bc99bdp+0"), hexf64!("-0x1.5368c951e9cfcp+0")));
    assert_eq!(n2i(-2.0, -0.5).atan(), n2i(hexf64!("-0x1.1b6e192ebbe45p+0"), hexf64!("-0x1.dac670561bb4fp-2")));
    assert_eq!(n2i(-1.0, -0.5).atan(), n2i(hexf64!("-0x1.921fb54442d19p-1"), hexf64!("-0x1.dac670561bb4fp-2")));
    assert_eq!(n2i(-0.75, -0.25).atan(), n2i(hexf64!("-0x1.4978fa3269ee2p-1"), hexf64!("-0x1.f5b75f92c80ddp-3")));
    assert_eq!(n2i(-0.5, 0.5).atan(), n2i(hexf64!("-0x1.dac670561bb50p-2"), hexf64!("0x1.dac670561bb50p-2")));
    assert_eq!(n2i(0.25, 0.625).atan(), n2i(hexf64!("0x1.f5b75f92c80ddp-3"), hexf64!("0x1.1e00babdefeb4p-1")));
    assert_eq!(n2i(-1.0, 1.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p-1"), hexf64!("0x1.921fb54442d19p-1")));
    assert_eq!(n2i(0.125, 17.0).atan(), n2i(hexf64!("0x1.fd5ba9aac2f6dp-4"), hexf64!("0x1.831516233f561p+0")));
    assert_eq!(n2i(17.0, 42.0).atan(), n2i(hexf64!("0x1.831516233f560p+0"), hexf64!("0x1.8c079f3350d27p+0")));
    assert_eq!(n2i(-42.0, 17.0).atan(), n2i(hexf64!("-0x1.8c079f3350d27p+0"), hexf64!("0x1.831516233f561p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_atanh() {
    // special values
    assert_eq!(n2i(-1.0, 0.0).atanh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 0.0).atanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).atanh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-1.0, -0.5).atanh(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.193ea7aad030ap-1")));
    assert_eq!(n2i(-0.75, -0.25).atanh(), n2i(hexf64!("-0x1.f2272ae325a58p-1"), hexf64!("-0x1.058aefa811451p-2")));
    assert_eq!(n2i(-0.5, 0.5).atanh(), n2i(hexf64!("-0x1.193ea7aad030bp-1"), hexf64!("0x1.193ea7aad030bp-1")));
    assert_eq!(n2i(0.25, 0.625).atanh(), n2i(hexf64!("0x1.058aefa811451p-2"), hexf64!("0x1.7761ddbb61598p-1")));
    assert_eq!(n2i(-1.0, 1.0).atanh(), I::ENTIRE);
    assert_eq!(n2i(0.125, 1.0).atanh(), n2i(hexf64!("0x1.015891c9eaef7p-3"), f64::INFINITY));
}

#[test]
fn mpfi_bounded_p() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -8.0).is_common_interval(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).is_common_interval(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).is_common_interval(), false);
    assert_eq!(I::ENTIRE.is_common_interval(), false);
    assert!(n2i(-8.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 5.0).is_common_interval());
    assert_eq!(n2i(0.0, f64::INFINITY).is_common_interval(), false);
    assert_eq!(n2i(5.0, f64::INFINITY).is_common_interval(), false);
    // regular values
    assert!(n2i(-34.0, -17.0).is_common_interval());
    assert!(n2i(-8.0, -1.0).is_common_interval());
    assert!(n2i(-34.0, 17.0).is_common_interval());
    assert!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).is_common_interval());
    assert!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).is_common_interval());
    assert!(n2i(8.0, hexf64!("0x1.fffffffffffecp+101")).is_common_interval());
    assert!(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0).is_common_interval());
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_cos() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(I::ENTIRE.cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 0.0).cos(), n2i(hexf64!("0x1.14a280fb5068bp-1"), 1.0));
    assert_eq!(n2i(0.0, 0.0).cos(), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).cos(), n2i(hexf64!("0x1.14a280fb5068bp-1"), 1.0));
    assert_eq!(n2i(0.0, 8.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    // regular values
    assert_eq!(n2i(-2.0, -0.5).cos(), n2i(hexf64!("-0x1.aa22657537205p-2"), hexf64!("0x1.c1528065b7d50p-1")));
    assert_eq!(n2i(-1.0, -0.25).cos(), n2i(hexf64!("0x1.14a280fb5068bp-1"), hexf64!("0x1.f01549f7deea2p-1")));
    assert_eq!(n2i(-0.5, 0.5).cos(), n2i(hexf64!("0x1.c1528065b7d4fp-1"), 1.0));
    assert_eq!(n2i(-4.5, 0.625).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(-1.0, hexf64!("0x1.14a280fb5068cp-1")));
    assert_eq!(n2i(0.125, 17.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(17.0, 42.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -1.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -2.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -3.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -4.0).cos(), n2i(hexf64!("-0x1.4eaa606db24c1p-1"), 1.0));
    assert_eq!(n2i(-7.0, -5.0).cos(), n2i(hexf64!("0x1.22785706b4ad9p-2"), 1.0));
    assert_eq!(n2i(-7.0, -6.0).cos(), n2i(hexf64!("0x1.81ff79ed92017p-1"), 1.0));
    assert_eq!(n2i(-7.0, -7.0).cos(), n2i(hexf64!("0x1.81ff79ed92017p-1"), hexf64!("0x1.81ff79ed92018p-1")));
    assert_eq!(n2i(-6.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, -1.0).cos(), n2i(-1.0, hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-6.0, -2.0).cos(), n2i(-1.0, hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-6.0, -3.0).cos(), n2i(-1.0, hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-6.0, -4.0).cos(), n2i(hexf64!("-0x1.4eaa606db24c1p-1"), hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-6.0, -5.0).cos(), n2i(hexf64!("0x1.22785706b4ad9p-2"), hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-6.0, -6.0).cos(), n2i(hexf64!("0x1.eb9b7097822f5p-1"), hexf64!("0x1.eb9b7097822f6p-1")));
    assert_eq!(n2i(-5.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, -1.0).cos(), n2i(-1.0, hexf64!("0x1.14a280fb5068cp-1")));
    assert_eq!(n2i(-5.0, -2.0).cos(), n2i(-1.0, hexf64!("0x1.22785706b4adap-2")));
    assert_eq!(n2i(-5.0, -3.0).cos(), n2i(-1.0, hexf64!("0x1.22785706b4adap-2")));
    assert_eq!(n2i(-5.0, -4.0).cos(), n2i(hexf64!("-0x1.4eaa606db24c1p-1"), hexf64!("0x1.22785706b4adap-2")));
    assert_eq!(n2i(-5.0, -5.0).cos(), n2i(hexf64!("0x1.22785706b4ad9p-2"), hexf64!("0x1.22785706b4adap-2")));
    assert_eq!(n2i(-4.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, -1.0).cos(), n2i(-1.0, hexf64!("0x1.14a280fb5068cp-1")));
    assert_eq!(n2i(-4.0, -2.0).cos(), n2i(-1.0, hexf64!("-0x1.aa22657537204p-2")));
    assert_eq!(n2i(-4.0, -3.0).cos(), n2i(-1.0, hexf64!("-0x1.4eaa606db24c0p-1")));
    assert_eq!(n2i(-4.0, -4.0).cos(), n2i(hexf64!("-0x1.4eaa606db24c1p-1"), hexf64!("-0x1.4eaa606db24c0p-1")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_cosh() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).cosh(), n2i(hexf64!("0x1.1228949ba3a8bp+9"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 0.0).cosh(), n2i(1.0, hexf64!("0x1.8b07551d9f551p+0")));
    assert_eq!(n2i(0.0, 0.0).cosh(), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).cosh(), n2i(1.0, hexf64!("0x1.8b07551d9f551p+0")));
    assert_eq!(n2i(0.0, 8.0).cosh(), n2i(1.0, hexf64!("0x1.749eaa93f4e77p+10")));
    assert_eq!(n2i(0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-0.125, 0.0).cosh(), n2i(1.0, hexf64!("0x1.0200aac16db6fp+0")));
    assert_eq!(n2i(0.0, hexf64!("0x1.0000000000001p-1")).cosh(), n2i(1.0, hexf64!("0x1.20ac1862ae8d1p+0")));
    assert_eq!(n2i(-4.5, -0.625).cosh(), n2i(hexf64!("0x1.33a621492d6dap+0"), hexf64!("0x1.681ceb0641359p+5")));
    assert_eq!(n2i(1.0, 3.0).cosh(), n2i(hexf64!("0x1.8b07551d9f550p+0"), hexf64!("0x1.422a497d6185fp+3")));
    assert_eq!(n2i(17.0, hexf64!("0x1.628b76e3a7b60p+9")).cosh(), n2i(hexf64!("0x1.709348c0ea503p+23"), hexf64!("0x1.ffffffffffd1ap+1021")));
}

#[test]
fn mpfi_d_div() {
    // special values
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d496p-55"), hexf64!("-0x1.70ef54646d496p-55")) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, hexf64!("0x1.a5a3ce29a1787p-58")));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d496p-55"), hexf64!("0x1.70ef54646d496p-55")) / n2i(f64::NEG_INFINITY, -7.0), n2i(hexf64!("-0x1.a5a3ce29a1787p-58"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-54"), hexf64!("-0x1.70ef54646d497p-54")) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-54"), hexf64!("0x1.70ef54646d497p-54")) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(hexf64!("-0x1.6345785d8a000p+60"), hexf64!("-0x1.6345785d8a000p+60")) / n2i(f64::NEG_INFINITY, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 8.0), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.6345785d8a000p+60"), hexf64!("0x1.6345785d8a000p+60")) / n2i(f64::NEG_INFINITY, 8.0), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")) / n2i(0.0, 7.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.3c3ada9f391a5p-58")));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 7.0), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.14b37f4b51f71p-55"), hexf64!("0x1.14b37f4b51f71p-55")) / n2i(0.0, 7.0), n2i(hexf64!("0x1.3c3ada9f391a5p-58"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.42d169d7dfa03p-54"), hexf64!("0x1.42d169d7dfa03p-54")) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-2.5, -2.5) / n2i(-8.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(-2.5, -2.5) / n2i(-8.0, -5.0), n2i(hexf64!("0x1.4000000000000p-2"), 0.5));
    assert_eq!(n2i(-2.5, -2.5) / n2i(25.0, 40.0), n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.0000000000000p-4")));
    assert_eq!(n2i(-2.5, -2.5) / n2i(-16.0, -7.0), n2i(hexf64!("0x1.4000000000000p-3"), hexf64!("0x1.6db6db6db6db7p-2")));
    assert_eq!(n2i(-2.5, -2.5) / n2i(11.0, 143.0), n2i(hexf64!("-0x1.d1745d1745d18p-3"), hexf64!("-0x1.1e6efe35b4cfap-6")));
    assert_eq!(n2i(33.125, 33.125) / n2i(8.28125, 530.0), n2i(hexf64!("0x1.0000000000000p-4"), 4.0));
    assert_eq!(n2i(33.125, 33.125) / n2i(-530.0, -496.875), n2i(hexf64!("-0x1.1111111111112p-4"), hexf64!("-0x1.0000000000000p-4")));
    assert_eq!(n2i(33.125, 33.125) / n2i(54.0, 265.0), n2i(0.125, hexf64!("0x1.3a12f684bda13p-1")));
    assert_eq!(n2i(33.125, 33.125) / n2i(52.0, 54.0), n2i(hexf64!("0x1.3a12f684bda12p-1"), hexf64!("0x1.4627627627628p-1")));
}

#[test]
fn mpfi_diam_abs() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -8.0).wid(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).wid(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).wid(), f64::INFINITY);
    assert_eq!(I::ENTIRE.wid(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).wid(), f64::INFINITY);
    assert_eq!(n2i(-8.0, 0.0).wid(), 8.0);
    assert_eq!(n2i(0.0, 0.0).wid(), -0.0);
    assert_eq!(n2i(0.0, 5.0).wid(), 5.0);
    assert_eq!(n2i(0.0, f64::INFINITY).wid(), f64::INFINITY);
    // regular values
    assert_eq!(n2i(-34.0, -17.0).wid(), 17.0);
}

#[test]
fn mpfi_div() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) / n2i(-1.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) / n2i(0.0, 8.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(0.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0) / n2i(-7.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 8.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) / n2i(8.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0) / n2i(-7.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    // regular value
    assert_eq!(n2i(hexf64!("-0x1.d6f3454000000p+26"), hexf64!("-0x1.d538000000000p+14")) / n2i(hexf64!("-0x1.1e00000000000p+8"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("0x1.a400000000000p+6"), hexf64!("0x1.a29f5a0000000p+23")));
    assert_eq!(n2i(hexf64!("-0x1.d6f3454000000p+26"), hexf64!("-0x1.489c07caba163p-4")) / n2i(hexf64!("-0x1.7471b72b03825p+5"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("0x1.c3bd854dec5c1p-10"), hexf64!("0x1.a29f5a0000000p+23")));
    assert_eq!(n2i(hexf64!("-0x1.02f0415f9f596p+0"), hexf64!("-0x1.d538000000000p-2")) / n2i(hexf64!("-0x1.1e00000000000p+8"), hexf64!("-0x1.d8b3993eeb34bp-6")), n2i(hexf64!("0x1.a400000000000p-10"), hexf64!("0x1.18772f77ce1b6p+5")));
    assert_eq!(n2i(hexf64!("-0x1.02f0415f9f596p+0"), hexf64!("-0x1.489c07caba163p-4")) / n2i(hexf64!("-0x1.7471b72b03825p+1"), hexf64!("-0x1.d8b3993eeb34bp-6")), n2i(hexf64!("0x1.c3bd854dec5c1p-6"), hexf64!("0x1.18772f77ce1b6p+5")));
    assert_eq!(n2i(hexf64!("-0x1.5960000000000p+267"), hexf64!("-0x1.be40000000000p+10")) / n2i(hexf64!("-0x1.c000000000000p+2"), 0.0), n2i(hexf64!("0x1.fe00000000000p+7"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+8"), hexf64!("-0x1.d7701a141049cp-1")) / n2i(hexf64!("-0x1.7c6d760a831fap+0"), 0.0), n2i(hexf64!("0x1.3d3e48f2088bep-1"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.25f2d73472753p+0"), hexf64!("-0x1.33433fa783f83p-1")) / n2i(hexf64!("-0x1.2761900e99983p-1"), hexf64!("0x1.1e237d75cabdep-2")), I::ENTIRE);
    assert_eq!(n2i(-100.0, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-2.0, hexf64!("-0x1.25f2d73472753p+0")) / n2i(0.0, hexf64!("0x1.2761900e99983p-1")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.fd8457415f917p+0")));
    assert_eq!(n2i(hexf64!("-0x1.2345678900000p+32"), hexf64!("-0x1.d538000000000p+18")) / n2i(hexf64!("0x1.02c0000000000p+10"), hexf64!("0x1.1e00000000000p+12")), n2i(hexf64!("-0x1.202cec0000000p+22"), hexf64!("-0x1.a400000000000p+6")));
    assert_eq!(n2i(hexf64!("-0x1.aceeebc970b10p-1"), hexf64!("-0x1.d538000000000p-39")) / n2i(hexf64!("0x1.221c2461d3ff2p+2"), hexf64!("0x1.1e00000000000p+209")), n2i(hexf64!("-0x1.7a804696fca66p-3"), hexf64!("-0x1.a400000000000p-248")));
    assert_eq!(n2i(hexf64!("-0x1.2345678900000p+32"), hexf64!("-0x1.b0a62934c76e9p+0")) / n2i(hexf64!("0x1.02c0000000000p-7"), hexf64!("0x1.3b0f63cbb4bd2p-3")), n2i(hexf64!("-0x1.202cec0000000p+39"), hexf64!("-0x1.5f8bce671e7c8p+3")));
    assert_eq!(n2i(hexf64!("-0x1.aceeebc970b10p+3"), hexf64!("-0x1.b0a62934c76e9p+0")) / n2i(hexf64!("0x1.221c2461d3ff2p-2"), hexf64!("0x1.3b0f63cbb4bd2p+5")), n2i(hexf64!("-0x1.7a804696fca66p+5"), hexf64!("-0x1.5f8bce671e7c8p-5")));
    assert_eq!(n2i(hexf64!("-0x1.d6f3454000000p+26"), 0.0) / n2i(hexf64!("-0x1.2000000000000p+7"), hexf64!("-0x1.2000000000000p+3")), n2i(0.0, hexf64!("0x1.a29f5a0000000p+23")));
    assert_eq!(n2i(hexf64!("-0x1.4298b2138f2a7p-4"), 0.0) / n2i(hexf64!("-0x1.0000000000000p-8"), hexf64!("-0x1.ebc920193833ep-9")), n2i(0.0, hexf64!("0x1.4fdb41a33d6cep+4")));
    assert_eq!(n2i(hexf64!("-0x1.ddddddddc0000p+35"), 0.0) / n2i(hexf64!("-0x1.5555555540000p+35"), 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.25f2d73472753p+0"), 0.0) / n2i(hexf64!("-0x1.2761900e99983p-1"), hexf64!("0x1.1e237d75cabdep-2")), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.ddddddddc0000p+35"), 0.0) / n2i(0.0, hexf64!("0x1.8000000000000p+1")), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(hexf64!("-0x1.d6f3454000000p+26"), 0.0) / n2i(hexf64!("0x1.2000000000000p+3"), hexf64!("0x1.2000000000000p+7")), n2i(hexf64!("-0x1.a29f5a0000000p+23"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.4298b2138f2a7p-4"), 0.0) / n2i(hexf64!("0x1.ebc920193833ep-9"), hexf64!("0x1.2000000000000p+3")), n2i(hexf64!("-0x1.4fdb41a33d6cep+4"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.d6f3454000000p+26"), hexf64!("0x1.4d00000000000p+15")) / n2i(hexf64!("-0x1.5ec0000000000p+11"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("-0x1.2800000000000p+12"), hexf64!("0x1.a29f5a0000000p+23")));
    assert_eq!(n2i(hexf64!("-0x1.2000000000000p+4"), hexf64!("0x1.0000000000000p+4")) / n2i(hexf64!("-0x1.7777777776000p+39"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("-0x1.c71c71c71c71dp+0"), 2.0));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.d538000000000p-2")) / n2i(hexf64!("-0x1.9999999998000p+39"), hexf64!("-0x1.1e00000000000p+8")), n2i(hexf64!("-0x1.a400000000000p-10"), hexf64!("0x1.ca4b3055ee192p-9")));
    assert_eq!(n2i(hexf64!("-0x1.6b72169a6426cp-1"), hexf64!("0x1.b9a52b19d9ce5p+2")) / n2i(hexf64!("-0x1.bbbbbbbbbba00p+43"), hexf64!("-0x1.9e8b37d3d0021p-1")), n2i(hexf64!("-0x1.10bc816787ec6p+3"), hexf64!("0x1.c0e397f43bcf1p-1")));
    assert_eq!(n2i(hexf64!("-0x1.5960000000000p+267"), hexf64!("0x1.be40000000000p+10")) / n2i(hexf64!("-0x1.c000000000000p+2"), 0.0), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.25f2d73472753p+0"), hexf64!("0x1.33433fa783f83p-1")) / n2i(hexf64!("-0x1.2761900e99983p-1"), hexf64!("0x1.1e237d75cabdep-2")), I::ENTIRE);
    assert_eq!(n2i(0.0, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.d538000000000p+14"), hexf64!("0x1.a1d3b88000000p+39")) / n2i(hexf64!("0x1.1e00000000000p+8"), hexf64!("0x1.7760000000000p+11")), n2i(hexf64!("-0x1.a400000000000p+6"), hexf64!("0x1.75ffc00000000p+31")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+4"), hexf64!("0x1.a1d3b88000000p+39")) / n2i(hexf64!("0x1.1e00000000000p+8"), hexf64!("0x1.7760000000000p+11")), n2i(hexf64!("-0x1.ca4b3055ee192p-5"), hexf64!("0x1.75ffc00000000p+31")));
    assert_eq!(n2i(hexf64!("-0x1.d538000000000p+14"), hexf64!("0x1.0000000000000p+10")) / n2i(hexf64!("0x1.1e00000000000p+8"), hexf64!("0x1.7760000000000p+11")), n2i(hexf64!("-0x1.a400000000000p+6"), hexf64!("0x1.ca4b3055ee192p+1")));
    assert_eq!(n2i(hexf64!("-0x1.18333622af827p+0"), hexf64!("0x1.0a5c1b48394b8p+1")) / n2i(hexf64!("0x1.263147d1f4bcbp+0"), hexf64!("0x1.1100000000000p+8")), n2i(hexf64!("-0x1.e7a5ebb71d8e5p-1"), hexf64!("0x1.cf8fa732de129p+0")));
    assert_eq!(n2i(0.0, hexf64!("0x1.d6f3454000000p+26")) / n2i(hexf64!("-0x1.4000000000000p+3"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("-0x1.a29f5a0000000p+23"), 0.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.acbf1702af6edp+0")) / n2i(hexf64!("-0x1.e000000000000p-1"), hexf64!("-0x1.c7af4b3c57b58p-1")), n2i(hexf64!("-0x1.e1bb896bfda07p+0"), 0.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.4000000000000p+3")) / n2i(hexf64!("-0x1.2000000000000p+3"), 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.4000000000000p+3")) / n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.d6f3454000000p+26")) / n2i(hexf64!("0x1.2000000000000p+3"), hexf64!("0x1.4000000000000p+3")), n2i(0.0, hexf64!("0x1.a29f5a0000000p+23")));
    assert_eq!(n2i(0.0, hexf64!("0x1.5f6b03dc8c66fp+0")) / n2i(hexf64!("0x1.1cd69274096e7p+1"), hexf64!("0x1.4000000000000p+3")), n2i(0.0, hexf64!("0x1.3bd6cb60575c0p-1")));
    assert_eq!(n2i(hexf64!("0x1.d538000000000p+14"), hexf64!("0x1.d6f3454000000p+26")) / n2i(hexf64!("-0x1.1e00000000000p+8"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("-0x1.a29f5a0000000p+23"), hexf64!("-0x1.a400000000000p+6")));
    assert_eq!(n2i(hexf64!("0x1.d538000000000p-2"), hexf64!("0x1.008a3accc766dp+4")) / n2i(hexf64!("-0x1.1e00000000000p+8"), hexf64!("-0x1.24ba01d98e995p+1")), n2i(hexf64!("-0x1.c0b4fb7ef22d8p+2"), hexf64!("-0x1.a400000000000p-10")));
    assert_eq!(n2i(hexf64!("0x1.358825fe3e28fp-1"), hexf64!("0x1.d6f3454000000p+26")) / n2i(hexf64!("-0x1.5232c83a0e726p+4"), hexf64!("-0x1.2000000000000p+3")), n2i(hexf64!("-0x1.a29f5a0000000p+23"), hexf64!("-0x1.d49a02927975ap-6")));
    assert_eq!(n2i(hexf64!("0x1.c2aa54629ac52p-1"), hexf64!("0x1.064c5adfd0042p+0")) / n2i(hexf64!("-0x1.43534c66942c1p-2"), hexf64!("-0x1.6c7a8ef8f1917p-3")), n2i(hexf64!("-0x1.70765f55f6073p+2"), hexf64!("-0x1.64d300622afadp+1")));
    assert_eq!(n2i(hexf64!("0x1.d538000000000p+14"), hexf64!("0x1.dddc000000000p+15")) / n2i(hexf64!("-0x1.1e00000000000p+8"), 0.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.a400000000000p+6")));
    assert_eq!(n2i(hexf64!("0x1.a9016514490e6p-4"), hexf64!("0x1.dddc000000000p+15")) / n2i(hexf64!("-0x1.c62dd0f7c1648p-1"), 0.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.df1cc82e6a583p-4")));
    assert_eq!(n2i(5.0, 6.0) / n2i(hexf64!("-0x1.43534c66942c1p-2"), hexf64!("0x1.6c7a8ef8f1917p-3")), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.d538000000000p+14"), hexf64!("0x1.ddddc00000000p+19")) / n2i(0.0, hexf64!("0x1.1e00000000000p+8")), n2i(hexf64!("0x1.a400000000000p+6"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.7f03f2a978865p+0"), hexf64!("0x1.ddddc00000000p+19")) / n2i(0.0, hexf64!("0x1.48b08624606b9p+0")), n2i(hexf64!("0x1.2a4fcda568430p+0"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.7bf0524800000p+30"), hexf64!("0x1.ba2dc76300000p+32")) / n2i(hexf64!("0x1.7ee8f80000000p+21"), hexf64!("0x1.1136e20000000p+23")), n2i(hexf64!("0x1.6400000000000p+7"), hexf64!("0x1.27a0000000000p+11")));
    assert_eq!(n2i(hexf64!("0x1.d7c06f9ff0706p-8"), hexf64!("0x1.ba2dc76300000p+32")) / n2i(hexf64!("0x1.7ee8f80000000p+1"), hexf64!("0x1.c7af4b3c57b58p+3")), n2i(hexf64!("0x1.0906badf3a5aap-11"), hexf64!("0x1.27a0000000000p+31")));
    assert_eq!(n2i(hexf64!("0x1.7bf0524800000p-2"), hexf64!("0x1.008a3accc766dp+0")) / n2i(hexf64!("0x1.24ba01d98e995p+1"), hexf64!("0x1.1136e20000000p+3")), n2i(hexf64!("0x1.6400000000000p-5"), hexf64!("0x1.c0b4fb7ef22d8p-2")));
    assert_eq!(n2i(hexf64!("0x1.0881cfacb7cd6p-5"), hexf64!("0x1.cccc174f5704fp+1")) / n2i(hexf64!("0x1.053f51a0cb3c9p-1"), hexf64!("0x1.827a5faec5c95p-1")), n2i(hexf64!("0x1.5e6a30ed1640cp-5"), hexf64!("0x1.c38ab2b552167p+2")));
}

#[test]
fn mpfi_div_d() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) / n2i(-7.0, -7.0), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) / n2i(7.0, 7.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(hexf64!("-0x1.70ef54646d497p-54"), hexf64!("-0x1.70ef54646d497p-54")), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(hexf64!("0x1.70ef54646d497p-54"), hexf64!("0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) / n2i(-3.0, -3.0), n2i(hexf64!("-0x1.5555555555556p+1"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) / n2i(3.0, 3.0), n2i(f64::NEG_INFINITY, hexf64!("0x1.5555555555556p+1")));
    assert_eq!(I::ENTIRE / n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(I::ENTIRE / n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) / n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0) / n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")), n2i(hexf64!("-0x1.d9b1f5d20d556p+57"), 0.0));
    assert_eq!(n2i(0.0, 8.0) / n2i(hexf64!("0x1.14b37f4b51f71p-55"), hexf64!("0x1.14b37f4b51f71p-55")), n2i(0.0, hexf64!("0x1.d9b1f5d20d556p+57")));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(hexf64!("0x1.42d169d7dfa03p-54"), hexf64!("0x1.42d169d7dfa03p-54")), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+32"), hexf64!("-0x1.0000000000001p-1")) / n2i(-1.0, -1.0), n2i(hexf64!("0x1.0000000000001p-1"), hexf64!("0x1.0000000000001p+32")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000002p+32"), hexf64!("-0x1.0000000000001p-1")) / n2i(hexf64!("0x1.0000000000001p-1"), hexf64!("0x1.0000000000001p-1")), n2i(hexf64!("-0x1.0000000000001p+33"), -1.0));
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+32"), hexf64!("-0x1.0000020000001p-1")) / n2i(hexf64!("0x1.0000000000001p-1"), hexf64!("0x1.0000000000001p-1")), n2i(hexf64!("-0x1.0000000000000p+33"), hexf64!("-0x1.000001fffffffp+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000002p+32"), hexf64!("-0x1.0000020000001p-1")) / n2i(hexf64!("0x1.0000000000001p-1"), hexf64!("0x1.0000000000001p-1")), n2i(hexf64!("-0x1.0000000000001p+33"), hexf64!("-0x1.000001fffffffp+0")));
    assert_eq!(n2i(hexf64!("-0x1.23456789abcdfp-1"), hexf64!("0x1.23456789abcdfp+45")) / n2i(hexf64!("-0x1.23456789abcdfp+52"), hexf64!("-0x1.23456789abcdfp+52")), n2i(hexf64!("-0x1.0000000000000p-7"), hexf64!("0x1.0000000000000p-53")));
    assert_eq!(n2i(hexf64!("-0x1.23456789abcdfp-1"), hexf64!("0x1.0000000000001p-1")) / n2i(hexf64!("-0x1.23456789abcdfp+52"), hexf64!("-0x1.23456789abcdfp+52")), n2i(hexf64!("-0x1.c200000000002p-54"), hexf64!("0x1.0000000000000p-53")));
    assert_eq!(n2i(-1.0, hexf64!("0x1.23456789abcdfp+45")) / n2i(hexf64!("-0x1.23456789abcdfp+52"), hexf64!("-0x1.23456789abcdfp+52")), n2i(hexf64!("-0x1.0000000000000p-7"), hexf64!("0x1.c200000000001p-53")));
    assert_eq!(n2i(-1.0, hexf64!("0x1.0000000000001p-1")) / n2i(hexf64!("-0x1.23456789abcdfp+52"), hexf64!("-0x1.23456789abcdfp+52")), n2i(hexf64!("-0x1.c200000000002p-54"), hexf64!("0x1.c200000000001p-53")));
}

#[test]
fn mpfi_d_sub() {
    // special values
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-55"), hexf64!("-0x1.70ef54646d497p-55")) - n2i(f64::NEG_INFINITY, -7.0), n2i(hexf64!("0x1.bffffffffffffp+2"), f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-55"), hexf64!("0x1.70ef54646d497p-55")) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-44"), hexf64!("-0x1.70ef54646d497p-44")) - n2i(f64::NEG_INFINITY, 0.0), n2i(hexf64!("-0x1.70ef54646d497p-44"), f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-44"), hexf64!("0x1.70ef54646d497p-44")) - n2i(f64::NEG_INFINITY, 0.0), n2i(hexf64!("0x1.70ef54646d497p-44"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.6345785d8a000p+60"), hexf64!("-0x1.6345785d8a000p+60")) - n2i(f64::NEG_INFINITY, 8.0), n2i(hexf64!("-0x1.6345785d8a001p+60"), f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.6345785d8a000p+60"), hexf64!("0x1.6345785d8a000p+60")) - n2i(f64::NEG_INFINITY, 8.0), n2i(hexf64!("0x1.6345785d89fffp+60"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")) - n2i(0.0, 0.0), n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")) - n2i(0.0, 0.0), n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")));
    assert_eq!(n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")) - n2i(0.0, 8.0), n2i(hexf64!("-0x1.0000000000001p+3"), hexf64!("-0x1.14b37f4b51f71p-55")));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.14b37f4b51f71p-55"), hexf64!("0x1.14b37f4b51f71p-55")) - n2i(0.0, 8.0), n2i(-8.0, hexf64!("0x1.14b37f4b51f71p-55")));
    assert_eq!(n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, hexf64!("-0x1.42d169d7dfa04p-54")));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(hexf64!("-0x1.42d169d7dfa03p-54"), hexf64!("-0x1.42d169d7dfa03p-54")) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, hexf64!("-0x1.42d169d7dfa03p-54")));
    // regular values
    assert_eq!(n2i(hexf64!("-0x1.f6a7a2955385ep+4"), hexf64!("-0x1.f6a7a2955385ep+4")) - n2i(17.0, 32.0), n2i(hexf64!("-0x1.fb53d14aa9c2fp+5"), hexf64!("-0x1.8353d14aa9c2fp+5")));
    assert_eq!(n2i(hexf64!("0x1.f6a7a2955385ep+4"), hexf64!("0x1.f6a7a2955385ep+4")) - n2i(17.0, hexf64!("0x1.f6a7a2955385ep+4")), n2i(0.0, hexf64!("0x1.cd4f452aa70bcp+3")));
    assert_eq!(n2i(hexf64!("0x1.f6a7a2955385ep+3"), hexf64!("0x1.f6a7a2955385ep+3")) - n2i(hexf64!("0x1.f6a7a2955385ep+3"), 32.0), n2i(hexf64!("-0x1.04ac2eb5563d1p+4"), 0.0));
    assert_eq!(n2i(3.5, 3.5) - n2i(hexf64!("-0x1.23456789abcdfp+48"), hexf64!("-0x1.23456789abcdfp+4")), n2i(hexf64!("0x1.5b456789abcdfp+4"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(3.5, 3.5) - n2i(hexf64!("-0x1.23456789abcdfp+48"), hexf64!("-0x1.23456789abcdfp-4")), n2i(hexf64!("0x1.c91a2b3c4d5e6p+1"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(256.5, 256.5) - n2i(hexf64!("-0x1.23456789abcdfp+0"), hexf64!("0x1.fe00000000000p+7")), n2i(hexf64!("0x1.8000000000000p+0"), hexf64!("0x1.01a3456789abdp+8")));
    assert_eq!(n2i(4097.5, 4097.5) - n2i(hexf64!("0x1.0000000000000p-550"), hexf64!("0x1.fffffffffffffp+0")), n2i(hexf64!("0x1.fff0000000000p+11"), hexf64!("0x1.0018000000000p+12")));
    assert_eq!(n2i(-3.5, -3.5) - n2i(hexf64!("-0x1.23456789abcdfp+48"), hexf64!("-0x1.23456789abcdfp+4")), n2i(hexf64!("0x1.d68acf13579bep+3"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(-3.5, -3.5) - n2i(hexf64!("-0x1.23456789abcdfp+48"), hexf64!("-0x1.23456789abcdfp-4")), n2i(hexf64!("-0x1.b6e5d4c3b2a1ap+1"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(-256.5, -256.5) - n2i(hexf64!("-0x1.23456789abcdfp+0"), hexf64!("0x1.fe00000000000p+7")), n2i(hexf64!("-0x1.ff80000000000p+8"), hexf64!("-0x1.feb97530eca86p+7")));
    assert_eq!(n2i(-4097.5, -4097.5) - n2i(hexf64!("0x1.0000000000000p-550"), hexf64!("0x1.fffffffffffffp+0")), n2i(hexf64!("-0x1.0038000000000p+12"), hexf64!("-0x1.0018000000000p+12")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_exp() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).exp(), n2i(0.0, hexf64!("0x1.de16b9c24a98fp-11")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).exp(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0).exp(), n2i(0.0, hexf64!("0x1.5bf0a8b14576ap+1")));
    assert_eq!(I::ENTIRE.exp(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).exp(), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).exp(), n2i(1.0, hexf64!("0x1.5bf0a8b14576ap+1")));
    assert_eq!(n2i(0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-123.0, -17.0).exp(), n2i(hexf64!("0x1.766b45dd84f17p-178"), hexf64!("0x1.639e3175a689dp-25")));
    assert_eq!(n2i(-0.125, 0.25).exp(), n2i(hexf64!("0x1.c3d6a24ed8221p-1"), hexf64!("0x1.48b5e3c3e8187p+0")));
    assert_eq!(n2i(-0.125, 0.0).exp(), n2i(hexf64!("0x1.c3d6a24ed8221p-1"), 1.0));
    assert_eq!(n2i(0.0, 0.25).exp(), n2i(1.0, hexf64!("0x1.48b5e3c3e8187p+0")));
    assert_eq!(n2i(hexf64!("0x1.4000000000000p-44"), hexf64!("0x1.6000000000000p-44")).exp(), n2i(hexf64!("0x1.0000000000140p+0"), hexf64!("0x1.0000000000161p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_exp2() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0).exp2(), n2i(0.0, 0.5));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).exp2(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0).exp2(), n2i(0.0, 2.0));
    assert_eq!(I::ENTIRE.exp2(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).exp2(), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).exp2(), n2i(1.0, 2.0));
    assert_eq!(n2i(0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-123.0, -17.0).exp2(), n2i(hexf64!("0x1.0000000000000p-123"), hexf64!("0x1.0000000000000p-17")));
    assert_eq!(n2i(-7.0, 7.0).exp2(), n2i(hexf64!("0x1.0000000000000p-7"), hexf64!("0x1.0000000000000p+7")));
    assert_eq!(n2i(-0.125, 0.25).exp2(), n2i(hexf64!("0x1.d5818dcfba487p-1"), hexf64!("0x1.306fe0a31b716p+0")));
    assert_eq!(n2i(-0.125, 0.0).exp2(), n2i(hexf64!("0x1.d5818dcfba487p-1"), 1.0));
    assert_eq!(n2i(0.0, 0.25).exp2(), n2i(1.0, hexf64!("0x1.306fe0a31b716p+0")));
    assert_eq!(n2i(hexf64!("0x1.4000000000000p-44"), hexf64!("0x1.6000000000000p-44")).exp2(), n2i(hexf64!("0x1.00000000000ddp+0"), hexf64!("0x1.00000000000f4p+0")));
}

#[test]
fn mpfi_intersect() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).intersection(n2i(-1.0, 8.0)), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).intersection(n2i(8.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq!(I::ENTIRE.intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, 0.0).intersection(n2i(f64::NEG_INFINITY, -7.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 8.0).intersection(n2i(-7.0, 0.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0).intersection(n2i(0.0, 8.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, 0.0).intersection(n2i(8.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).intersection(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0).intersection(n2i(-7.0, 8.0)), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, 0.0).intersection(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    // regular values
    assert_eq!(n2i(hexf64!("0x1.2000000000000p+4"), hexf64!("0x1.2000000000000p+7")).intersection(n2i(hexf64!("-0x1.a000000000000p+3"), hexf64!("0x1.a000000000000p+5"))), n2i(hexf64!("0x1.2000000000000p+4"), hexf64!("0x1.a000000000000p+5")));
}

#[test]
fn mpfi_inv() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -0.25).recip(), n2i(-4.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 4.0).recip(), I::ENTIRE);
    assert_eq!(I::ENTIRE.recip(), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).recip(), I::EMPTY);
    assert_eq!(n2i(0.0, 2.0).recip(), n2i(0.5, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-8.0, -2.0).recip(), n2i(-0.5, -0.125));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-4"), hexf64!("0x1.440c131282cd9p-1")).recip(), n2i(hexf64!("0x1.947bfce1bc417p+0"), hexf64!("0x1.0000000000000p+4")));
    assert_eq!(n2i(hexf64!("0x1.9f1a539c91fddp-3"), 64.0).recip(), n2i(0.015625, hexf64!("0x1.3bc205a76b3fdp+2")));
    assert_eq!(n2i(hexf64!("-0x1.5d0772bdffad2p-1"), hexf64!("-0x1.8f8f2d3b5ca8cp-3")).recip(), n2i(hexf64!("-0x1.480a9b5772a23p+2"), hexf64!("-0x1.77887d65484c9p+0")));
}

#[test]
fn mpfi_is_neg() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(f64::NEG_INFINITY, 0.0).precedes(n2i(0.0, 0.0)));
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(I::ENTIRE.precedes(n2i(0.0, 0.0)), false);
    assert!(n2i(-8.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert_eq!(n2i(0.0, 5.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, f64::INFINITY).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(5.0, f64::INFINITY).precedes(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).precedes(n2i(0.0, 0.0)));
    assert_eq!(n2i(-34.0, 17.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(8.0, hexf64!("0x1.fffffffffffecp+101")).precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0).precedes(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_nonneg() {
    // special values
    assert_eq!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(I::ENTIRE), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(-8.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 5.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, f64::INFINITY)));
    assert!(n2i(0.0, 0.0).less(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq!(n2i(0.0, 0.0).less(n2i(-34.0, -17.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(-8.0, -1.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(-34.0, 17.0)), false);
    assert_eq!(n2i(0.0, 0.0).less(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))), false);
    assert!(n2i(0.0, 0.0).less(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))));
    assert!(n2i(0.0, 0.0).less(n2i(8.0, hexf64!("0x1.fffffffffffecp+101"))));
    assert!(n2i(0.0, 0.0).less(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0)));
}

#[test]
fn mpfi_is_nonpos() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).less(n2i(0.0, 0.0)));
    assert!(n2i(f64::NEG_INFINITY, 0.0).less(n2i(0.0, 0.0)));
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).less(n2i(0.0, 0.0)), false);
    assert_eq!(I::ENTIRE.less(n2i(0.0, 0.0)), false);
    assert!(n2i(-8.0, 0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 0.0)));
    assert_eq!(n2i(0.0, 5.0).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, f64::INFINITY).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(5.0, f64::INFINITY).less(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).less(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).less(n2i(0.0, 0.0)));
    assert_eq!(n2i(-34.0, 17.0).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(8.0, hexf64!("0x1.fffffffffffecp+101")).less(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0).less(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_pos() {
    // special values
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(I::ENTIRE), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(-8.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 5.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, f64::INFINITY)));
    assert!(n2i(0.0, 0.0).precedes(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(-34.0, -17.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(-8.0, -1.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(-34.0, 17.0)), false);
    assert_eq!(n2i(0.0, 0.0).precedes(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))));
    assert!(n2i(0.0, 0.0).precedes(n2i(8.0, hexf64!("0x1.fffffffffffecp+101"))));
    assert!(n2i(0.0, 0.0).precedes(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0)));
}

#[test]
fn mpfi_is_strictly_neg() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).strict_precedes(n2i(0.0, 0.0)));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(I::ENTIRE.strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(-8.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, 5.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, f64::INFINITY).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(5.0, f64::INFINITY).strict_precedes(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).strict_precedes(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).strict_precedes(n2i(0.0, 0.0)));
    assert_eq!(n2i(-34.0, 17.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(8.0, hexf64!("0x1.fffffffffffecp+101")).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0).strict_precedes(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_strictly_pos() {
    // special values
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(I::ENTIRE), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(-8.0, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 5.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, f64::INFINITY)), false);
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(-34.0, -17.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(-8.0, -1.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(-34.0, 17.0)), false);
    assert_eq!(n2i(0.0, 0.0).strict_precedes(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))), false);
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"))));
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(8.0, hexf64!("0x1.fffffffffffecp+101"))));
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(hexf64!("0x1.fffffffffffffp-1"), 2.0)));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log() {
    // special values
    assert_eq!(n2i(0.0, 1.0).log(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).log(), I::ENTIRE);
    // regular values
    assert_eq!(n2i(1.0, 1.0).log(), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.d1504611d7f18p+35"), hexf64!("0x1.463ceb440d6bdp+38")).log(), n2i(hexf64!("0x1.8db9145250af2p+4"), hexf64!("0x1.a9500bc7ffcc5p+4")));
    assert_eq!(n2i(hexf64!("0x1.6c2d5716d076ap-1"), 1.0).log(), n2i(hexf64!("-0x1.5cdc58fd97dc8p-2"), 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.158e9b265f5c6p+30")).log(), n2i(0.0, hexf64!("0x1.4e011555da624p+4")));
    assert_eq!(n2i(hexf64!("0x1.30c8995fb226cp+34"), hexf64!("0x1.35a2e97c9afc4p+39")).log(), n2i(hexf64!("0x1.7bdce4519fdbep+4"), hexf64!("0x1.b3913fc99f555p+4")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log2() {
    // special values
    assert_eq!(n2i(0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).log2(), I::ENTIRE);
    // regular values
    assert_eq!(n2i(1.0, 1.0).log2(), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.6c2d5716d076ap-1"), 1.0).log2(), n2i(hexf64!("-0x1.f74cb5d105b3ap-2"), 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.158e9b265f5c6p+30")).log2(), n2i(0.0, hexf64!("0x1.e1ddc27c2c70fp+4")));
    assert_eq!(n2i(hexf64!("0x1.30c8995fb226cp+34"), hexf64!("0x1.35a2e97c9afc4p+39")).log2(), n2i(hexf64!("0x1.12035c9390c07p+5"), hexf64!("0x1.3a3208f61f10cp+5")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log10() {
    // special values
    assert_eq!(n2i(0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).log10(), I::ENTIRE);
    // regular values
    assert_eq!(n2i(1.0, 1.0).log10(), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.d1504611d7f18p+35"), hexf64!("0x1.463ceb440d6bdp+38")).log10(), n2i(hexf64!("0x1.59753104a9401p+3"), hexf64!("0x1.716c01a04b570p+3")));
    assert_eq!(n2i(hexf64!("0x1.6c2d5716d076ap-1"), 1.0).log10(), n2i(hexf64!("-0x1.2f043ec00f8d6p-3"), 0.0));
    assert_eq!(n2i(100.0, hexf64!("0x1.158e9b265f5c6p+30")).log10(), n2i(2.0, hexf64!("0x1.221cc590b9946p+3")));
    assert_eq!(n2i(hexf64!("0x1.30c8995fb226cp+34"), hexf64!("0x1.35a2e97c9afc4p+39")).log10(), n2i(hexf64!("0x1.49f1d70168f49p+3"), hexf64!("0x1.7a543a94fb65ep+3")));
}

#[test]
fn mpfi_mag() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -8.0).mag(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).mag(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).mag(), f64::INFINITY);
    assert_eq!(I::ENTIRE.mag(), f64::INFINITY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).mag(), f64::INFINITY);
    assert_eq!(n2i(-8.0, 0.0).mag(), 8.0);
    assert_eq!(n2i(0.0, 0.0).mag(), 0.0);
    assert_eq!(n2i(0.0, 5.0).mag(), 5.0);
    assert_eq!(n2i(0.0, f64::INFINITY).mag(), f64::INFINITY);
    // regular values
    assert_eq!(n2i(-34.0, -17.0).mag(), 34.0);
}

#[test]
fn mpfi_mid() {
    // special values
    assert_eq!(n2i(-8.0, 0.0).mid(), -4.0);
    assert_eq!(n2i(0.0, 0.0).mid(), 0.0);
    assert_eq!(n2i(0.0, 5.0).mid(), 2.5);
    // regular values
    assert_eq!(n2i(-34.0, -17.0).mid(), hexf64!("-0x1.9800000000000p+4"));
    assert_eq!(n2i(-34.0, 17.0).mid(), -8.5);
    assert_eq!(n2i(0.0, hexf64!("0x1.23456789abcd0p+46")).mid(), hexf64!("0x1.23456789abcd0p+45"));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).mid(), hexf64!("0x1.921fb54442d18p+1"));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d18p+1")).mid(), hexf64!("-0x1.921fb54442d18p+1"));
    assert_eq!(n2i(-4.0, hexf64!("-0x1.ffffffffffff4p-1")).mid(), hexf64!("-0x1.3fffffffffffep+1"));
    assert_eq!(n2i(-8.0, hexf64!("-0x1.fffffffffffecp-1")).mid(), hexf64!("-0x1.1ffffffffffffp+2"));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp-1"), 2.0).mid(), 0.5);
}

#[test]
fn mpfi_mig() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -8.0).mig(), 8.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).mig(), 0.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 5.0).mig(), 0.0);
    assert_eq!(I::ENTIRE.mig(), 0.0);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).mig(), 0.0);
    assert_eq!(n2i(-8.0, 0.0).mig(), 0.0);
    assert_eq!(n2i(0.0, 0.0).mig(), 0.0);
    assert_eq!(n2i(0.0, 5.0).mig(), 0.0);
    assert_eq!(n2i(0.0, f64::INFINITY).mig(), 0.0);
    // regular values
    assert_eq!(n2i(-34.0, -17.0).mig(), 17.0);
}

#[test]
fn mpfi_mul() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) * n2i(-1.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) * n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) * n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 64.0));
    assert_eq!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(I::ENTIRE * n2i(0.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0) * n2i(-7.0, 0.0), n2i(-56.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(0.0, 8.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) * n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) * n2i(8.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0) * n2i(-7.0, 8.0), n2i(-56.0, 64.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) * n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-3.0, 7.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    // regular values
    assert_eq!(n2i(hexf64!("-0x1.a000000000000p+3"), hexf64!("-0x1.2000000000000p+3")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.2000000000000p+4"), hexf64!("0x1.a000000000000p+5")));
    assert_eq!(n2i(hexf64!("-0x1.a000000000000p+3"), hexf64!("-0x1.be1cf24fa48f8p-1")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.4821095fe91f0p-5")), n2i(hexf64!("0x1.1de75443b74e9p-5"), hexf64!("0x1.a000000000000p+5")));
    assert_eq!(n2i(hexf64!("-0x1.c4d93d3d6cf69p-1"), hexf64!("-0x1.046fa5d71622fp-1")) * n2i(hexf64!("-0x1.63226681c1b54p-2"), hexf64!("-0x1.9a272bd1bdaf4p-3")), n2i(hexf64!("0x1.a142a930de328p-4"), hexf64!("0x1.3a1b0d0d33649p-2")));
    assert_eq!(n2i(hexf64!("-0x1.b800000000000p+5"), hexf64!("-0x1.c000000000000p+2")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.1000000000000p+5")), n2i(hexf64!("-0x1.d380000000000p+10"), hexf64!("0x1.b800000000000p+5")));
    assert_eq!(n2i(hexf64!("-0x1.c0c7e4cfdaa34p-1"), hexf64!("-0x1.9800000000000p-3")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.777ab178b4a1ep+0")), n2i(hexf64!("-0x1.491df346a9f15p+0"), hexf64!("0x1.c0c7e4cfdaa34p-1")));
    assert_eq!(n2i(hexf64!("-0x1.cb540b71699a8p+4"), hexf64!("-0x1.9800000000000p-3")) * n2i(hexf64!("-0x1.64dcaaa101f18p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.cb540b71699a8p+4"), hexf64!("0x1.402667251fa17p+5")));
    assert_eq!(n2i(hexf64!("-0x1.cb540b71699a8p+4"), hexf64!("-0x1.9800000000000p-3")) * n2i(hexf64!("-0x1.64dcaaa101f18p+0"), hexf64!("0x1.eb67a1a6ef725p+4")), n2i(hexf64!("-0x1.b8da116740bfap+9"), hexf64!("0x1.402667251fa17p+5")));
    assert_eq!(n2i(hexf64!("-0x1.23456789a0000p+36"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+4")), n2i(hexf64!("-0x1.23456789a0000p+40"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.6d8cfa746faa8p-1"), hexf64!("-0x1.0000000000000p-1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.05f72745d9ef8p+1")), n2i(hexf64!("-0x1.7611a672948a5p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.492faa678765cp-5")) * n2i(hexf64!("0x1.7a491be66e7cdp-1"), hexf64!("0x1.0000000000000p+2")), n2i(hexf64!("-0x1.0000000000000p+4"), hexf64!("-0x1.e66e6414cb50dp-6")));
    assert_eq!(n2i(hexf64!("-0x1.6d8cfa746faa8p-1"), hexf64!("-0x1.492faa678765cp-5")) * n2i(hexf64!("0x1.7a491be66e7cdp-1"), hexf64!("0x1.05f72745d9ef8p+1")), n2i(hexf64!("-0x1.7611a672948a5p+0"), hexf64!("-0x1.e66e6414cb50dp-6")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.1000000000000p+4")) * n2i(hexf64!("-0x1.c000000000000p+2"), hexf64!("-0x1.0000000000000p+2")), n2i(hexf64!("-0x1.dc00000000000p+6"), hexf64!("0x1.c000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.d94fbbfb70ae4p-1")) * n2i(hexf64!("-0x1.1da31130a291ap+1"), hexf64!("-0x1.0000000000000p-4")), n2i(hexf64!("-0x1.080da0e9ea45cp+1"), hexf64!("0x1.1da31130a291ap+1")));
    assert_eq!(n2i(hexf64!("-0x1.1d069e75e8741p+8"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.1da31130a291ap+1"), hexf64!("-0x1.0000000000000p-4")), n2i(hexf64!("-0x1.1da31130a291ap+1"), hexf64!("0x1.3e05ecc3bfa02p+9")));
    assert_eq!(n2i(hexf64!("-0x1.d94fbbfb70ae4p-1"), hexf64!("0x1.1d069e75e8741p+8")) * n2i(hexf64!("-0x1.1da31130a291ap+1"), hexf64!("-0x1.0000000000000p-4")), n2i(hexf64!("-0x1.3e05ecc3bfa02p+9"), hexf64!("0x1.080da0e9ea45cp+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+4")) * n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p+5"), hexf64!("0x1.8000000000000p+5")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.6d848e752c9fdp-3")) * n2i(hexf64!("-0x1.35ff9312fdb8ep-3"), hexf64!("0x1.0000000000000p-8")), n2i(hexf64!("-0x1.ba9de8fb90f7ap-6"), hexf64!("0x1.35ff9312fdb8ep-3")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.b8847fbf085a8p-2")) * n2i(hexf64!("-0x1.0000000000000p-4"), hexf64!("0x1.8e3fe93a4ea52p+0")), n2i(hexf64!("-0x1.8e3fe93a4ea52p+0"), hexf64!("0x1.56a5fc45ae4f1p-1")));
    assert_eq!(n2i(hexf64!("-0x1.15e079e49a0ddp+0"), hexf64!("0x1.0000000000000p-8")) * n2i(hexf64!("-0x1.3bfe42314d301p+1"), hexf64!("0x1.0710b265e27e0p-1")), n2i(hexf64!("-0x1.1d8bbce624b7dp-1"), hexf64!("0x1.56ff328e9d8cdp+1")));
    assert_eq!(n2i(hexf64!("-0x1.c000000000000p+2"), hexf64!("0x1.c000000000000p+2")) * n2i(hexf64!("0x1.3000000000000p+4"), hexf64!("0x1.2000000000000p+5")), n2i(hexf64!("-0x1.f800000000000p+7"), hexf64!("0x1.f800000000000p+7")));
    assert_eq!(n2i(hexf64!("-0x1.500e3f0e024d8p-1"), hexf64!("0x1.0000000000000p+4")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.19c0841f3e9dap+1")), n2i(hexf64!("-0x1.71dc5b5607781p+0"), hexf64!("0x1.19c0841f3e9dap+5")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.90aa487ecf153p+0")) * n2i(hexf64!("0x1.0000000000000p-53"), hexf64!("0x1.442e2695ac81ap+0")), n2i(hexf64!("-0x1.442e2695ac81ap+0"), hexf64!("0x1.fb5fbebd0cbc6p+0")));
    assert_eq!(n2i(hexf64!("-0x1.c40db77f2f6fcp+0"), hexf64!("0x1.8eb70bbd94478p+0")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.a288c31a91ae3p+1")), n2i(hexf64!("-0x1.7187f2b64237dp+2"), hexf64!("0x1.45edf1244c209p+2")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+3"), hexf64!("0x1.6800000000000p+5")) * n2i(hexf64!("-0x1.9e40000000000p+10"), hexf64!("-0x1.ca00000000000p+7")), n2i(hexf64!("-0x1.2345000000000p+16"), hexf64!("-0x1.5780000000000p+11")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+3"), hexf64!("0x1.1833fdcab4c4ap+10")) * n2i(hexf64!("-0x1.26057e2829166p+41"), hexf64!("-0x1.ca00000000000p+7")), n2i(hexf64!("-0x1.41d1b8904c91ap+51"), hexf64!("-0x1.5780000000000p+11")));
    assert_eq!(n2i(hexf64!("0x1.671e3f61de861p+3"), hexf64!("0x1.6800000000000p+5")) * n2i(hexf64!("-0x1.9e40000000000p+10"), hexf64!("-0x1.48ee3afa18c08p+3")), n2i(hexf64!("-0x1.2345000000000p+16"), hexf64!("-0x1.cd6cf21002b79p+6")));
    assert_eq!(n2i(hexf64!("0x1.e106cf3099438p-1"), hexf64!("0x1.579ed8d97c682p+3")) * n2i(hexf64!("-0x1.72f1117a654b1p+2"), hexf64!("-0x1.45682d3dcc47fp-7")), n2i(hexf64!("-0x1.f1e7296a509cep+5"), hexf64!("-0x1.31b8b267979a3p-7")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+3")) * n2i(hexf64!("-0x1.ca00000000000p+7"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.5780000000000p+11"), hexf64!("0x1.8000000000000p+3")));
    assert_eq!(n2i(hexf64!("0x1.2300000000000p-44"), hexf64!("0x1.ec24910ac6aecp+0")) * n2i(hexf64!("-0x1.552e4cfead537p-1"), hexf64!("0x1.0000000000000p+32")), n2i(hexf64!("-0x1.47f2dbe4ef916p+0"), hexf64!("0x1.ec24910ac6aecp+32")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.cafa94c7bd026p+2")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.552e4cfead537p-1")), n2i(hexf64!("-0x1.cafa94c7bd026p+2"), hexf64!("0x1.31d9659fe51a0p+2")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p-3"), hexf64!("0x1.552e4cfead537p-1")) * n2i(hexf64!("-0x1.ec24910ac6aecp+0"), hexf64!("0x1.cafa94c7bd026p+2")), n2i(hexf64!("-0x1.47f2dbe4ef916p+0"), hexf64!("0x1.31d9659fe51a0p+2")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.c000000000000p+2")) * n2i(hexf64!("0x1.4000000000000p+2"), hexf64!("0x1.6000000000000p+3")), n2i(hexf64!("0x1.e000000000000p+3"), hexf64!("0x1.3400000000000p+6")));
    assert_eq!(n2i(hexf64!("0x1.241c01197b60bp+1"), hexf64!("0x1.c000000000000p+2")) * n2i(hexf64!("0x1.b8e5b629f3477p+1"), hexf64!("0x1.6000000000000p+3")), n2i(hexf64!("0x1.f7163ecc8eb5ep+2"), hexf64!("0x1.3400000000000p+6")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.b8e5b629f3477p+1")) * n2i(hexf64!("0x1.4000000000000p-23"), hexf64!("0x1.241c01197b60bp+1")), n2i(hexf64!("0x1.e000000000000p-22"), hexf64!("0x1.f7163ecc8eb5fp+2")));
    assert_eq!(n2i(hexf64!("0x1.88745302ab900p-3"), hexf64!("0x1.241c01197b60bp+1")) * n2i(hexf64!("0x1.87b1c60a429d8p-1"), hexf64!("0x1.4f3ed82901e44p+1")), n2i(hexf64!("0x1.2c3d196815e02p-3"), hexf64!("0x1.7e885adf0831bp+2")));
}

#[test]
fn mpfi_mul_d() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) * n2i(hexf64!("-0x1.7000000000000p+4"), hexf64!("-0x1.7000000000000p+4")), n2i(hexf64!("0x1.4200000000000p+7"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) * n2i(hexf64!("0x1.70ef54646d497p-55"), hexf64!("0x1.70ef54646d497p-55")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.42d169d7dfa04p-52")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) * n2i(hexf64!("-0x1.70ef54646d497p-54"), hexf64!("-0x1.70ef54646d497p-54")), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) * n2i(hexf64!("0x1.70ef54646d497p-54"), hexf64!("0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) * n2i(hexf64!("-0x1.6345785d8a000p+60"), hexf64!("-0x1.6345785d8a000p+60")), n2i(hexf64!("-0x1.6345785d8a000p+63"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) * n2i(hexf64!("0x1.6345785d8a000p+60"), hexf64!("0x1.6345785d8a000p+60")), n2i(f64::NEG_INFINITY, hexf64!("0x1.6345785d8a000p+63")));
    assert_eq!(I::ENTIRE * n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(I::ENTIRE * n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) * n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 7.0) * n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")), n2i(hexf64!("-0x1.e43a1ec3cf706p-53"), 0.0));
    assert_eq!(n2i(0.0, 8.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 9.0) * n2i(hexf64!("0x1.14b37f4b51f71p-51"), hexf64!("0x1.14b37f4b51f71p-51")), n2i(0.0, hexf64!("0x1.3749ef34bc360p-48")));
    assert_eq!(n2i(0.0, f64::INFINITY) * n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) * n2i(hexf64!("0x1.42d169d7dfa03p-54"), hexf64!("0x1.42d169d7dfa03p-54")), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("-0x1.7171700000000p+24"), hexf64!("-0x1.5555555555554p-72")) * n2i(-1.5, -1.5), n2i(hexf64!("0x1.ffffffffffffep-72"), hexf64!("0x1.1515140000000p+25")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555554p+51"), hexf64!("0x1.7171700000000p+425")) * n2i(-1.5, -1.5), n2i(hexf64!("-0x1.1515140000000p+426"), hexf64!("0x1.ffffffffffffep+51")));
    assert_eq!(n2i(hexf64!("0x1.0000000000010p+52"), hexf64!("0x1.1111111111100p+701")) * n2i(-2.125, -2.125), n2i(hexf64!("-0x1.2222222222210p+702"), hexf64!("-0x1.1000000000011p+53")));
    assert_eq!(n2i(hexf64!("-0x1.7171700000000p+24"), hexf64!("-0x1.5555555555554p-72")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.1515140000000p+25"), hexf64!("-0x1.ffffffffffffep-72")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555554p+51"), hexf64!("0x1.7171700000000p+425")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.ffffffffffffep+51"), hexf64!("0x1.1515140000000p+426")));
    assert_eq!(n2i(hexf64!("0x1.0000000000010p+52"), hexf64!("0x1.1111111111100p+701")) * n2i(2.125, 2.125), n2i(hexf64!("0x1.1000000000011p+53"), hexf64!("0x1.2222222222210p+702")));
    assert_eq!(n2i(hexf64!("-0x1.7171700000000p+60"), hexf64!("-0x1.0000000000001p+52")) * n2i(-1.5, -1.5), n2i(hexf64!("0x1.8000000000001p+52"), hexf64!("0x1.1515140000000p+61")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555554p+51"), hexf64!("0x1.0000000000001p+52")) * n2i(-1.5, -1.5), n2i(hexf64!("-0x1.8000000000002p+52"), hexf64!("0x1.ffffffffffffep+51")));
    assert_eq!(n2i(hexf64!("0x1.0000000000010p+52"), hexf64!("0x1.1111111111111p+52")) * n2i(-2.125, -2.125), n2i(hexf64!("-0x1.2222222222223p+53"), hexf64!("-0x1.1000000000011p+53")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("-0x1.5555555555554p-72")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.8000000000002p+52"), hexf64!("-0x1.ffffffffffffep-72")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555556p+51"), hexf64!("0x1.7171700000000p+425")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("0x1.1515140000000p+426")));
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+52"), hexf64!("0x1.1111111111100p+701")) * n2i(2.125, 2.125), n2i(hexf64!("0x1.1000000000001p+53"), hexf64!("0x1.2222222222210p+702")));
    assert_eq!(n2i(hexf64!("-0x1.1717171717171p+52"), hexf64!("-0x1.5555555555554p-72")) * n2i(-1.5, -1.5), n2i(hexf64!("0x1.ffffffffffffep-72"), hexf64!("0x1.a2a2a2a2a2a2ap+52")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("0x1.7171700000000p+425")) * n2i(-1.5, -1.5), n2i(hexf64!("-0x1.1515140000000p+426"), hexf64!("0x1.8000000000002p+52")));
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+52"), hexf64!("0x1.1111111111100p+701")) * n2i(-2.125, -2.125), n2i(hexf64!("-0x1.2222222222210p+702"), hexf64!("-0x1.1000000000001p+53")));
    assert_eq!(n2i(hexf64!("-0x1.7171700000000p+24"), hexf64!("-0x1.aaaaaaaaaaaaap-71")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.1515140000000p+25"), hexf64!("-0x1.3ffffffffffffp-70")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555554p+51"), hexf64!("0x1.1717171717171p+52")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.ffffffffffffep+51"), hexf64!("0x1.a2a2a2a2a2a2ap+52")));
    assert_eq!(n2i(hexf64!("0x1.0000000000010p+52"), hexf64!("0x1.8888888888889p+52")) * n2i(2.125, 2.125), n2i(hexf64!("0x1.1000000000011p+53"), hexf64!("0x1.a111111111112p+53")));
    assert_eq!(n2i(hexf64!("-0x1.1717171717171p+52"), hexf64!("-0x1.0000000000001p+52")) * n2i(-1.5, -1.5), n2i(hexf64!("0x1.8000000000001p+52"), hexf64!("0x1.a2a2a2a2a2a2ap+52")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("0x1.0000000000001p+52")) * n2i(-1.5, -1.5), n2i(hexf64!("-0x1.8000000000002p+52"), hexf64!("0x1.8000000000002p+52")));
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+52"), hexf64!("0x1.1111111111111p+52")) * n2i(-2.125, -2.125), n2i(hexf64!("-0x1.2222222222223p+53"), hexf64!("-0x1.1000000000001p+53")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("-0x1.aaaaaaaaaaaaap-71")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.8000000000002p+52"), hexf64!("-0x1.3ffffffffffffp-70")));
    assert_eq!(n2i(hexf64!("-0x1.5555555555556p+51"), hexf64!("0x1.1717171717171p+52")) * n2i(1.5, 1.5), n2i(hexf64!("-0x1.0000000000001p+52"), hexf64!("0x1.a2a2a2a2a2a2ap+52")));
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+52"), hexf64!("0x1.8888888888889p+52")) * n2i(2.125, 2.125), n2i(hexf64!("0x1.1000000000001p+53"), hexf64!("0x1.a111111111112p+53")));
}

#[test]
fn mpfi_neg() {
    // special values
    assert_eq!(-n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq!(-n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(-n2i(f64::NEG_INFINITY, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq!(-I::ENTIRE, I::ENTIRE);
    assert_eq!(-n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(-n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq!(-n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    // regular values
    assert_eq!(-n2i(hexf64!("0x1.2345678900000p+16"), hexf64!("0x1.2345679900000p+16")), n2i(hexf64!("-0x1.2345679900000p+16"), hexf64!("-0x1.2345678900000p+16")));
}

#[test]
fn mpfi_put_d() {
    // special values
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(-8.0, -8.0)), n2i(-8.0, 0.0));
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(5.0, 5.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 5.0));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_sin() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(I::ENTIRE.sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 0.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), 0.0));
    assert_eq!(n2i(0.0, 0.0).sin(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).sin(), n2i(0.0, hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(0.0, 8.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    // regular values
    assert_eq!(n2i(0.125, 17.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("0x1.fffffffffffffp-1"), 1.0));
    assert_eq!(n2i(-2.0, -0.5).sin(), n2i(-1.0, hexf64!("-0x1.eaee8744b05efp-2")));
    assert_eq!(n2i(-4.5, 0.625).sin(), n2i(-1.0, hexf64!("0x1.f47ed3dc74081p-1")));
    assert_eq!(n2i(-1.0, -0.25).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), hexf64!("-0x1.faaeed4f31576p-3")));
    assert_eq!(n2i(-0.5, 0.5).sin(), n2i(hexf64!("-0x1.eaee8744b05f0p-2"), hexf64!("0x1.eaee8744b05f0p-2")));
    assert_eq!(n2i(hexf64!("0x1.c400000000000p+82"), hexf64!("0x1.c400000000000p+82")).sin(), n2i(hexf64!("0x1.bde6c11cbfc46p-3"), hexf64!("0x1.bde6c11cbfc47p-3")));
    assert_eq!(n2i(-7.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-7.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), 1.0));
    assert_eq!(n2i(-7.0, -3.0).sin(), n2i(hexf64!("-0x1.50608c26d0a09p-1"), 1.0));
    assert_eq!(n2i(-7.0, -4.0).sin(), n2i(hexf64!("-0x1.50608c26d0a09p-1"), 1.0));
    assert_eq!(n2i(-7.0, -5.0).sin(), n2i(hexf64!("-0x1.50608c26d0a09p-1"), hexf64!("0x1.eaf81f5e09934p-1")));
    assert_eq!(n2i(-7.0, -6.0).sin(), n2i(hexf64!("-0x1.50608c26d0a09p-1"), hexf64!("0x1.1e1f18ab0a2c1p-2")));
    assert_eq!(n2i(-7.0, -7.0).sin(), n2i(hexf64!("-0x1.50608c26d0a09p-1"), hexf64!("-0x1.50608c26d0a08p-1")));
    assert_eq!(n2i(-6.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-6.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), 1.0));
    assert_eq!(n2i(-6.0, -3.0).sin(), n2i(hexf64!("-0x1.210386db6d55cp-3"), 1.0));
    assert_eq!(n2i(-6.0, -4.0).sin(), n2i(hexf64!("0x1.1e1f18ab0a2c0p-2"), 1.0));
    assert_eq!(n2i(-6.0, -5.0).sin(), n2i(hexf64!("0x1.1e1f18ab0a2c0p-2"), hexf64!("0x1.eaf81f5e09934p-1")));
    assert_eq!(n2i(-6.0, -6.0).sin(), n2i(hexf64!("0x1.1e1f18ab0a2c0p-2"), hexf64!("0x1.1e1f18ab0a2c1p-2")));
    assert_eq!(n2i(-5.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-5.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), 1.0));
    assert_eq!(n2i(-5.0, -3.0).sin(), n2i(hexf64!("-0x1.210386db6d55cp-3"), 1.0));
    assert_eq!(n2i(-5.0, -4.0).sin(), n2i(hexf64!("0x1.837b9dddc1eaep-1"), 1.0));
    assert_eq!(n2i(-5.0, -5.0).sin(), n2i(hexf64!("0x1.eaf81f5e09933p-1"), hexf64!("0x1.eaf81f5e09934p-1")));
    assert_eq!(n2i(-4.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-4.0, 1.0).sin(), n2i(-1.0, hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(-4.0, 0.0).sin(), n2i(-1.0, hexf64!("0x1.837b9dddc1eafp-1")));
    assert_eq!(n2i(-4.0, -1.0).sin(), n2i(-1.0, hexf64!("0x1.837b9dddc1eafp-1")));
    assert_eq!(n2i(-4.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), hexf64!("0x1.837b9dddc1eafp-1")));
    assert_eq!(n2i(-4.0, -3.0).sin(), n2i(hexf64!("-0x1.210386db6d55cp-3"), hexf64!("0x1.837b9dddc1eafp-1")));
    assert_eq!(n2i(-4.0, -4.0).sin(), n2i(hexf64!("0x1.837b9dddc1eaep-1"), hexf64!("0x1.837b9dddc1eafp-1")));
    assert_eq!(n2i(-3.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-3.0, 1.0).sin(), n2i(-1.0, hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(-3.0, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-3.0, -1.0).sin(), n2i(-1.0, hexf64!("-0x1.210386db6d55bp-3")));
    assert_eq!(n2i(-3.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), hexf64!("-0x1.210386db6d55bp-3")));
    assert_eq!(n2i(-3.0, -3.0).sin(), n2i(hexf64!("-0x1.210386db6d55cp-3"), hexf64!("-0x1.210386db6d55bp-3")));
    assert_eq!(n2i(-2.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-2.0, 1.0).sin(), n2i(-1.0, hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(-2.0, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-2.0, -1.0).sin(), n2i(-1.0, hexf64!("-0x1.aed548f090ceep-1")));
    assert_eq!(n2i(-2.0, -2.0).sin(), n2i(hexf64!("-0x1.d18f6ead1b446p-1"), hexf64!("-0x1.d18f6ead1b445p-1")));
    assert_eq!(n2i(-1.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 4.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), 1.0));
    assert_eq!(n2i(-1.0, 3.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), 1.0));
    assert_eq!(n2i(-1.0, 2.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), 1.0));
    assert_eq!(n2i(-1.0, 1.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(-1.0, 0.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), 0.0));
    assert_eq!(n2i(-1.0, -1.0).sin(), n2i(hexf64!("-0x1.aed548f090cefp-1"), hexf64!("-0x1.aed548f090ceep-1")));
    assert_eq!(n2i(1.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(1.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(1.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(1.0, 4.0).sin(), n2i(hexf64!("-0x1.837b9dddc1eafp-1"), 1.0));
    assert_eq!(n2i(1.0, 3.0).sin(), n2i(hexf64!("0x1.210386db6d55bp-3"), 1.0));
    assert_eq!(n2i(1.0, 2.0).sin(), n2i(hexf64!("0x1.aed548f090ceep-1"), 1.0));
    assert_eq!(n2i(1.0, 1.0).sin(), n2i(hexf64!("0x1.aed548f090ceep-1"), hexf64!("0x1.aed548f090cefp-1")));
    assert_eq!(n2i(2.0, 7.0).sin(), n2i(-1.0, hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(2.0, 6.0).sin(), n2i(-1.0, hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(2.0, 5.0).sin(), n2i(-1.0, hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(2.0, 4.0).sin(), n2i(hexf64!("-0x1.837b9dddc1eafp-1"), hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(2.0, 3.0).sin(), n2i(hexf64!("0x1.210386db6d55bp-3"), hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(2.0, 2.0).sin(), n2i(hexf64!("0x1.d18f6ead1b445p-1"), hexf64!("0x1.d18f6ead1b446p-1")));
    assert_eq!(n2i(3.0, 7.0).sin(), n2i(-1.0, hexf64!("0x1.50608c26d0a09p-1")));
    assert_eq!(n2i(3.0, 6.0).sin(), n2i(-1.0, hexf64!("0x1.210386db6d55cp-3")));
    assert_eq!(n2i(3.0, 5.0).sin(), n2i(-1.0, hexf64!("0x1.210386db6d55cp-3")));
    assert_eq!(n2i(3.0, 4.0).sin(), n2i(hexf64!("-0x1.837b9dddc1eafp-1"), hexf64!("0x1.210386db6d55cp-3")));
    assert_eq!(n2i(3.0, 3.0).sin(), n2i(hexf64!("0x1.210386db6d55bp-3"), hexf64!("0x1.210386db6d55cp-3")));
    assert_eq!(n2i(4.0, 7.0).sin(), n2i(-1.0, hexf64!("0x1.50608c26d0a09p-1")));
    assert_eq!(n2i(4.0, 6.0).sin(), n2i(-1.0, hexf64!("-0x1.1e1f18ab0a2c0p-2")));
    assert_eq!(n2i(4.0, 5.0).sin(), n2i(-1.0, hexf64!("-0x1.837b9dddc1eaep-1")));
    assert_eq!(n2i(4.0, 4.0).sin(), n2i(hexf64!("-0x1.837b9dddc1eafp-1"), hexf64!("-0x1.837b9dddc1eaep-1")));
    assert_eq!(n2i(5.0, 7.0).sin(), n2i(hexf64!("-0x1.eaf81f5e09934p-1"), hexf64!("0x1.50608c26d0a09p-1")));
    assert_eq!(n2i(5.0, 6.0).sin(), n2i(hexf64!("-0x1.eaf81f5e09934p-1"), hexf64!("-0x1.1e1f18ab0a2c0p-2")));
    assert_eq!(n2i(5.0, 5.0).sin(), n2i(hexf64!("-0x1.eaf81f5e09934p-1"), hexf64!("-0x1.eaf81f5e09933p-1")));
    assert_eq!(n2i(6.0, 7.0).sin(), n2i(hexf64!("-0x1.1e1f18ab0a2c1p-2"), hexf64!("0x1.50608c26d0a09p-1")));
    assert_eq!(n2i(6.0, 6.0).sin(), n2i(hexf64!("-0x1.1e1f18ab0a2c1p-2"), hexf64!("-0x1.1e1f18ab0a2c0p-2")));
    assert_eq!(n2i(7.0, 7.0).sin(), n2i(hexf64!("0x1.50608c26d0a08p-1"), hexf64!("0x1.50608c26d0a09p-1")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_sinh() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).sinh(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.122876ba380c9p+9")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).sinh(), n2i(f64::NEG_INFINITY, hexf64!("0x1.749ea514eca66p+10")));
    assert_eq!(I::ENTIRE.sinh(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 0.0).sinh(), n2i(hexf64!("-0x1.2cd9fc44eb983p+0"), 0.0));
    assert_eq!(n2i(0.0, 0.0).sinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).sinh(), n2i(0.0, hexf64!("0x1.2cd9fc44eb983p+0")));
    assert_eq!(n2i(0.0, 8.0).sinh(), n2i(0.0, hexf64!("0x1.749ea514eca66p+10")));
    assert_eq!(n2i(0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-0.125, 0.0).sinh(), n2i(hexf64!("-0x1.00aaccd00d2f1p-3"), 0.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.0000000000001p-1")).sinh(), n2i(0.0, hexf64!("0x1.0acd00fe63b98p-1")));
    assert_eq!(n2i(-4.5, -0.625).sinh(), n2i(hexf64!("-0x1.68062ab5fa9fdp+5"), hexf64!("-0x1.553e795dc19ccp-1")));
    assert_eq!(n2i(1.0, 3.0).sinh(), n2i(hexf64!("0x1.2cd9fc44eb982p+0"), hexf64!("0x1.40926e70949aep+3")));
}

#[test]
fn mpfi_sqr() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).sqr(), n2i(49.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).sqr(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 8.0).sqr(), n2i(0.0, 64.0));
    assert_eq!(n2i(0.0, f64::INFINITY).sqr(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("0x1.0c6e9b0000000p-1"), hexf64!("0x1.f8ec948000000p+9")).sqr(), n2i(hexf64!("0x1.1977c45191d90p-2"), hexf64!("0x1.f1f231ad11122p+19")));
    assert_eq!(n2i(hexf64!("0x1.9852170cf02c5p+6"), hexf64!("0x1.2345600000000p+20")).sqr(), n2i(hexf64!("0x1.45a2e1e601f53p+13"), hexf64!("0x1.4b66cb0ce4000p+40")));
    assert_eq!(n2i(hexf64!("-0x1.64722ad2480c9p+0"), hexf64!("0x1.0000000000000p+0")).sqr(), n2i(0.0, hexf64!("0x1.f04dba0302d4dp+0")));
    assert_eq!(n2i(hexf64!("0x1.6b079248747a2p+0"), hexf64!("0x1.58208bb6931fbp+1")).sqr(), n2i(hexf64!("0x1.01673c8966efbp+1"), hexf64!("0x1.ce977b9de549cp+2")));
}

#[test]
fn mpfi_sqrt() {
    // special values
    assert_eq!(n2i(0.0, 0.0).sqrt(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 9.0).sqrt(), n2i(0.0, 3.0));
    assert_eq!(n2i(0.0, f64::INFINITY).sqrt(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("0x1.5542000000000p+15"), hexf64!("0x1.4b66cb0ce4000p+40")).sqrt(), n2i(hexf64!("0x1.a200000000000p+7"), hexf64!("0x1.2345600000000p+20")));
    assert_eq!(n2i(hexf64!("0x1.c935cf2d3c836p-1"), hexf64!("0x1.5542000000000p+15")).sqrt(), n2i(hexf64!("0x1.e3d48504364f5p-1"), hexf64!("0x1.a200000000000p+7")));
    assert_eq!(n2i(hexf64!("0x1.5542000000000p-1"), hexf64!("0x1.0c348f804c7a9p+0")).sqrt(), n2i(hexf64!("0x1.a200000000000p-1"), hexf64!("0x1.06081714eef1dp+0")));
    assert_eq!(n2i(hexf64!("0x1.c935cf2d3c836p-1"), hexf64!("0x1.0c348f804c7a9p+0")).sqrt(), n2i(hexf64!("0x1.e3d48504364f5p-1"), hexf64!("0x1.06081714eef1dp+0")));
}

#[test]
fn mpfi_sub() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) - n2i(-1.0, 8.0), n2i(f64::NEG_INFINITY, -6.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) - n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -8.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) - n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq!(I::ENTIRE - n2i(0.0, 8.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 8.0) - n2i(-7.0, 0.0), n2i(0.0, 15.0));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) - n2i(0.0, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0) - n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -8.0));
    assert_eq!(n2i(0.0, 0.0) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 8.0) - n2i(-7.0, 8.0), n2i(-8.0, 15.0));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) - n2i(0.0, 8.0), n2i(-8.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(-5.0, 59.0) - n2i(17.0, 81.0), n2i(-86.0, 42.0));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p-300"), hexf64!("0x1.2345600000000p+48")) - n2i(hexf64!("-0x1.e26af34000000p+26"), hexf64!("0x1.0000000000000p-41")), n2i(hexf64!("-0x1.0000000000001p-41"), hexf64!("0x1.23456789abcd0p+48")));
    assert_eq!(n2i(-4.0, 7.0) - n2i(-3e+300, hexf64!("0x1.23456789abcd0p+31")), n2i(hexf64!("-0x1.23456791abcd0p+31"), hexf64!("0x1.1eb2d66005836p+998")));
    assert_eq!(n2i(hexf64!("-0x1.0001000100010p+56"), hexf64!("0x1.0000000000000p+60")) - n2i(-3e+300, hexf64!("0x1.0001000100010p+48")), n2i(hexf64!("-0x1.0101010101011p+56"), hexf64!("0x1.1eb2d66005836p+998")));
    assert_eq!(n2i(-5.0, 1.0) - n2i(1.0, hexf64!("0x1.0000000000000p+70")), n2i(hexf64!("-0x1.0000000000001p+70"), 0.0));
    assert_eq!(n2i(5.0, hexf64!("0x1.0000000000000p+70")) - n2i(3.0, 5.0), n2i(0.0, hexf64!("0x1.0000000000000p+70")));
}

#[test]
fn mpfi_sub_d() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) - n2i(hexf64!("-0x1.70ef54646d497p-55"), hexf64!("-0x1.70ef54646d497p-55")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.bffffffffffffp+2")));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0) - n2i(hexf64!("0x1.70ef54646d497p-55"), hexf64!("0x1.70ef54646d497p-55")), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) - n2i(hexf64!("-0x1.70ef54646d497p-54"), hexf64!("-0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, hexf64!("0x1.70ef54646d497p-54")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) - n2i(hexf64!("0x1.70ef54646d497p-54"), hexf64!("0x1.70ef54646d497p-54")), n2i(f64::NEG_INFINITY, -8e-17));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) - n2i(hexf64!("-0x1.6345785d8a000p+60"), hexf64!("-0x1.6345785d8a000p+60")), n2i(f64::NEG_INFINITY, hexf64!("0x1.6345785d8a001p+60")));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0) - n2i(hexf64!("0x1.6345785d8a000p+60"), hexf64!("0x1.6345785d8a000p+60")), n2i(f64::NEG_INFINITY, hexf64!("-0x1.6345785d89fffp+60")));
    assert_eq!(I::ENTIRE - n2i(hexf64!("-0x1.70ef54646d497p-53"), hexf64!("-0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(I::ENTIRE - n2i(0.0, 0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE - n2i(hexf64!("0x1.70ef54646d497p-53"), hexf64!("0x1.70ef54646d497p-53")), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) - n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")), n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")));
    assert_eq!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) - n2i(hexf64!("0x1.70ef54646d497p-57"), hexf64!("0x1.70ef54646d497p-57")), n2i(hexf64!("-0x1.70ef54646d497p-57"), hexf64!("-0x1.70ef54646d497p-57")));
    assert_eq!(n2i(0.0, 8.0) - n2i(hexf64!("-0x1.14b37f4b51f71p-55"), hexf64!("-0x1.14b37f4b51f71p-55")), n2i(hexf64!("0x1.14b37f4b51f71p-55"), hexf64!("0x1.0000000000001p+3")));
    assert_eq!(n2i(0.0, 8.0) - n2i(0.0, 0.0), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, 8.0) - n2i(hexf64!("0x1.14b37f4b51f71p-55"), hexf64!("0x1.14b37f4b51f71p-55")), n2i(hexf64!("-0x1.14b37f4b51f71p-55"), 8.0));
    assert_eq!(n2i(0.0, f64::INFINITY) - n2i(hexf64!("-0x1.42d169d7dfa04p-54"), hexf64!("-0x1.42d169d7dfa04p-54")), n2i(hexf64!("0x1.42d169d7dfa04p-54"), f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) - n2i(0.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) - n2i(hexf64!("0x1.42d169d7dfa03p-54"), hexf64!("0x1.42d169d7dfa03p-54")), n2i(hexf64!("-0x1.42d169d7dfa03p-54"), f64::INFINITY));
    // regular values
    assert_eq!(n2i(-32.0, -17.0) - n2i(hexf64!("0x1.f6a7a2955385ep+4"), hexf64!("0x1.f6a7a2955385ep+4")), n2i(hexf64!("-0x1.fb53d14aa9c2fp+5"), hexf64!("-0x1.8353d14aa9c2fp+5")));
    assert_eq!(n2i(hexf64!("-0x1.f6a7a2955385ep+4"), -17.0) - n2i(hexf64!("-0x1.f6a7a2955385ep+4"), hexf64!("-0x1.f6a7a2955385ep+4")), n2i(0.0, hexf64!("0x1.cd4f452aa70bcp+3")));
    assert_eq!(n2i(-32.0, hexf64!("-0x1.f6a7a2955385ep+3")) - n2i(hexf64!("-0x1.f6a7a2955385ep+3"), hexf64!("-0x1.f6a7a2955385ep+3")), n2i(hexf64!("-0x1.04ac2eb5563d1p+4"), 0.0));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp+4"), hexf64!("0x1.23456789abcdfp+48")) - n2i(-3.5, -3.5), n2i(hexf64!("0x1.5b456789abcdfp+4"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp-4"), hexf64!("0x1.23456789abcdfp+48")) - n2i(-3.5, -3.5), n2i(hexf64!("0x1.c91a2b3c4d5e6p+1"), hexf64!("0x1.23456789abd17p+48")));
    assert_eq!(n2i(hexf64!("-0x1.fe00000000000p+7"), hexf64!("0x1.23456789abcdfp+0")) - n2i(-256.5, -256.5), n2i(hexf64!("0x1.8000000000000p+0"), hexf64!("0x1.01a3456789abdp+8")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+0"), hexf64!("-0x1.0000000000000p-550")) - n2i(-4097.5, -4097.5), n2i(hexf64!("0x1.fff0000000000p+11"), hexf64!("0x1.0018000000000p+12")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp+4"), hexf64!("0x1.23456789abcdfp+48")) - n2i(3.5, 3.5), n2i(hexf64!("0x1.d68acf13579bep+3"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(hexf64!("0x1.23456789abcdfp-4"), hexf64!("0x1.23456789abcdfp+48")) - n2i(3.5, 3.5), n2i(hexf64!("-0x1.b6e5d4c3b2a1ap+1"), hexf64!("0x1.23456789abca7p+48")));
    assert_eq!(n2i(hexf64!("-0x1.fe00000000000p+7"), hexf64!("0x1.23456789abcdfp+0")) - n2i(256.5, 256.5), n2i(hexf64!("-0x1.ff80000000000p+8"), hexf64!("-0x1.feb97530eca86p+7")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+0"), hexf64!("-0x1.0000000000000p-550")) - n2i(4097.5, 4097.5), n2i(hexf64!("-0x1.0038000000000p+12"), hexf64!("-0x1.0018000000000p+12")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_tan() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).tan(), I::ENTIRE);
    assert_eq!(I::ENTIRE.tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 0.0).tan(), n2i(hexf64!("-0x1.8eb245cbee3a6p+0"), 0.0));
    assert_eq!(n2i(0.0, 0.0).tan(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).tan(), n2i(0.0, hexf64!("0x1.8eb245cbee3a6p+0")));
    assert_eq!(n2i(0.0, 8.0).tan(), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY).tan(), I::ENTIRE);
    // regular values
    assert_eq!(n2i(0.125, 17.0).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, -0.5).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.5, 0.625).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, -0.25).tan(), n2i(hexf64!("-0x1.8eb245cbee3a6p+0"), hexf64!("-0x1.05785a43c4c55p-2")));
    assert_eq!(n2i(-0.5, 0.5).tan(), n2i(hexf64!("-0x1.17b4f5bf3474bp-1"), hexf64!("0x1.17b4f5bf3474bp-1")));
    assert_eq!(n2i(hexf64!("0x1.c400000000000p+82"), hexf64!("0x1.c400000000000p+82")).tan(), n2i(hexf64!("-0x1.c8dc87ddcc134p-3"), hexf64!("-0x1.c8dc87ddcc133p-3")));
    assert_eq!(n2i(-7.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, -2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, -3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, -4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-7.0, -5.0).tan(), n2i(hexf64!("-0x1.be2e6e13eea79p-1"), hexf64!("0x1.b0b4b739bbb07p+1")));
    assert_eq!(n2i(-7.0, -6.0).tan(), n2i(hexf64!("-0x1.be2e6e13eea79p-1"), hexf64!("0x1.29fd86ebb95bfp-2")));
    assert_eq!(n2i(-7.0, -7.0).tan(), n2i(hexf64!("-0x1.be2e6e13eea79p-1"), hexf64!("-0x1.be2e6e13eea78p-1")));
    assert_eq!(n2i(-6.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, -2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, -3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, -4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-6.0, -5.0).tan(), n2i(hexf64!("0x1.29fd86ebb95bep-2"), hexf64!("0x1.b0b4b739bbb07p+1")));
    assert_eq!(n2i(-6.0, -6.0).tan(), n2i(hexf64!("0x1.29fd86ebb95bep-2"), hexf64!("0x1.29fd86ebb95bfp-2")));
    assert_eq!(n2i(-5.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, -2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, -3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, -4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-5.0, -5.0).tan(), n2i(hexf64!("0x1.b0b4b739bbb06p+1"), hexf64!("0x1.b0b4b739bbb07p+1")));
    assert_eq!(n2i(-4.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-4.0, -2.0).tan(), n2i(hexf64!("-0x1.2866f9be4de14p+0"), hexf64!("0x1.17af62e0950f9p+1")));
    assert_eq!(n2i(-4.0, -3.0).tan(), n2i(hexf64!("-0x1.2866f9be4de14p+0"), hexf64!("0x1.23ef71254b870p-3")));
    assert_eq!(n2i(-4.0, -4.0).tan(), n2i(hexf64!("-0x1.2866f9be4de14p+0"), hexf64!("-0x1.2866f9be4de13p+0")));
    assert_eq!(n2i(-3.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-3.0, -2.0).tan(), n2i(hexf64!("0x1.23ef71254b86fp-3"), hexf64!("0x1.17af62e0950f9p+1")));
    assert_eq!(n2i(-3.0, -3.0).tan(), n2i(hexf64!("0x1.23ef71254b86fp-3"), hexf64!("0x1.23ef71254b870p-3")));
    assert_eq!(n2i(-2.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, -1.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-2.0, -2.0).tan(), n2i(hexf64!("0x1.17af62e0950f8p+1"), hexf64!("0x1.17af62e0950f9p+1")));
    assert_eq!(n2i(-1.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(-1.0, 1.0).tan(), n2i(hexf64!("-0x1.8eb245cbee3a6p+0"), hexf64!("0x1.8eb245cbee3a6p+0")));
    assert_eq!(n2i(-1.0, 0.0).tan(), n2i(hexf64!("-0x1.8eb245cbee3a6p+0"), 0.0));
    assert_eq!(n2i(-1.0, -1.0).tan(), n2i(hexf64!("-0x1.8eb245cbee3a6p+0"), hexf64!("-0x1.8eb245cbee3a5p+0")));
    assert_eq!(n2i(1.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 4.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 3.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 2.0).tan(), I::ENTIRE);
    assert_eq!(n2i(1.0, 1.0).tan(), n2i(hexf64!("0x1.8eb245cbee3a5p+0"), hexf64!("0x1.8eb245cbee3a6p+0")));
    assert_eq!(n2i(2.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(2.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(2.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(2.0, 4.0).tan(), n2i(hexf64!("-0x1.17af62e0950f9p+1"), hexf64!("0x1.2866f9be4de14p+0")));
    assert_eq!(n2i(2.0, 3.0).tan(), n2i(hexf64!("-0x1.17af62e0950f9p+1"), hexf64!("-0x1.23ef71254b86fp-3")));
    assert_eq!(n2i(2.0, 2.0).tan(), n2i(hexf64!("-0x1.17af62e0950f9p+1"), hexf64!("-0x1.17af62e0950f8p+1")));
    assert_eq!(n2i(3.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(3.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(3.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(3.0, 4.0).tan(), n2i(hexf64!("-0x1.23ef71254b870p-3"), hexf64!("0x1.2866f9be4de14p+0")));
    assert_eq!(n2i(3.0, 3.0).tan(), n2i(hexf64!("-0x1.23ef71254b870p-3"), hexf64!("-0x1.23ef71254b86fp-3")));
    assert_eq!(n2i(4.0, 7.0).tan(), I::ENTIRE);
    assert_eq!(n2i(4.0, 6.0).tan(), I::ENTIRE);
    assert_eq!(n2i(4.0, 5.0).tan(), I::ENTIRE);
    assert_eq!(n2i(4.0, 4.0).tan(), n2i(hexf64!("0x1.2866f9be4de13p+0"), hexf64!("0x1.2866f9be4de14p+0")));
    assert_eq!(n2i(5.0, 7.0).tan(), n2i(hexf64!("-0x1.b0b4b739bbb07p+1"), hexf64!("0x1.be2e6e13eea79p-1")));
    assert_eq!(n2i(5.0, 6.0).tan(), n2i(hexf64!("-0x1.b0b4b739bbb07p+1"), hexf64!("-0x1.29fd86ebb95bep-2")));
    assert_eq!(n2i(5.0, 5.0).tan(), n2i(hexf64!("-0x1.b0b4b739bbb07p+1"), hexf64!("-0x1.b0b4b739bbb06p+1")));
    assert_eq!(n2i(6.0, 7.0).tan(), n2i(hexf64!("-0x1.29fd86ebb95bfp-2"), hexf64!("0x1.be2e6e13eea79p-1")));
    assert_eq!(n2i(6.0, 6.0).tan(), n2i(hexf64!("-0x1.29fd86ebb95bfp-2"), hexf64!("-0x1.29fd86ebb95bep-2")));
    assert_eq!(n2i(7.0, 7.0).tan(), n2i(hexf64!("0x1.be2e6e13eea78p-1"), hexf64!("0x1.be2e6e13eea79p-1")));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_tanh() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).tanh(), n2i(-1.0, hexf64!("-0x1.ffffc832750f1p-1")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).tanh(), n2i(-1.0, hexf64!("0x1.fffff872a91f9p-1")));
    assert_eq!(I::ENTIRE.tanh(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 0.0).tanh(), n2i(hexf64!("-0x1.85efab514f395p-1"), 0.0));
    assert_eq!(n2i(0.0, 0.0).tanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 1.0).tanh(), n2i(0.0, hexf64!("0x1.85efab514f395p-1")));
    assert_eq!(n2i(0.0, 8.0).tanh(), n2i(0.0, hexf64!("0x1.fffff872a91f9p-1")));
    assert_eq!(n2i(0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    // regular values
    assert_eq!(n2i(-0.125, 0.0).tanh(), n2i(hexf64!("-0x1.fd5992bc4b835p-4"), 0.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.0000000000001p-1")).tanh(), n2i(0.0, hexf64!("0x1.d9353d7568af5p-2")));
    assert_eq!(n2i(-4.5, -0.625).tanh(), n2i(hexf64!("-0x1.ffdfa72153984p-1"), hexf64!("-0x1.1bf47eabb8f95p-1")));
    assert_eq!(n2i(1.0, 3.0).tanh(), n2i(hexf64!("0x1.85efab514f394p-1"), hexf64!("0x1.fd77d111a0b00p-1")));
    assert_eq!(n2i(17.0, 18.0).tanh(), n2i(hexf64!("0x1.fffffffffffe1p-1"), hexf64!("0x1.ffffffffffffcp-1")));
}

#[test]
fn mpfi_union() {
    // special values
    assert_eq!(n2i(f64::NEG_INFINITY, -7.0).convex_hull(n2i(-1.0, 8.0)), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).convex_hull(n2i(8.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 8.0).convex_hull(n2i(0.0, 8.0)), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq!(I::ENTIRE.convex_hull(n2i(0.0, 8.0)), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(f64::NEG_INFINITY, -7.0)), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 8.0).convex_hull(n2i(-7.0, 0.0)), n2i(-7.0, 8.0));
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq!(n2i(0.0, f64::INFINITY).convex_hull(n2i(0.0, 8.0)), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(8.0, f64::INFINITY)), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).convex_hull(I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 8.0).convex_hull(n2i(-7.0, 8.0)), n2i(-7.0, 8.0));
    assert_eq!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY).convex_hull(n2i(0.0, 8.0)), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq!(n2i(hexf64!("0x1.2000000000000p+4"), hexf64!("0x1.2000000000000p+7")).convex_hull(n2i(hexf64!("-0x1.a000000000000p+3"), hexf64!("0x1.a000000000000p+5"))), n2i(hexf64!("-0x1.a000000000000p+3"), hexf64!("0x1.2000000000000p+7")));
}
