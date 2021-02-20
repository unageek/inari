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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

#[test]
fn mpfi_abs() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).abs(), n2i(7.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).abs(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0).abs(), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 8.0).abs(), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(74565.4044342041, 74565.40467834473).abs(), n2i(74565.4044342041, 74565.40467834473));
    assert_eq2!(n2i(-74565.4044342041, 74565.40467834473).abs(), n2i(0.0, 74565.40467834473));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_acos() {
    // special values
    assert_eq2!(n2i(-1.0, 0.0).acos(), n2i(1.5707963267948966, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).acos(), n2i(1.5707963267948966, 1.5707963267948968));
    assert_eq2!(n2i(0.0, 1.0).acos(), n2i(0.0, 1.5707963267948968));
    // regular values
    assert_eq2!(n2i(-1.0, -0.5).acos(), n2i(2.0943951023931953, 3.1415926535897936));
    assert_eq2!(n2i(-0.75, -0.25).acos(), n2i(1.8234765819369751, 2.418858405776378));
    assert_eq2!(n2i(-0.5, 0.5).acos(), n2i(1.0471975511965976, 2.0943951023931957));
    assert_eq2!(n2i(0.25, 0.625).acos(), n2i(0.8956647938578649, 1.318116071652818));
    assert_eq2!(n2i(-1.0, 1.0).acos(), n2i(0.0, 3.1415926535897936));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_acosh() {
    // special values
    assert_eq2!(n2i(1.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(1.5, f64::INFINITY).acosh(), n2i(0.9624236501192068, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(1.0, 1.5).acosh(), n2i(0.0, 0.9624236501192069));
    assert_eq2!(n2i(1.5, 1.5).acosh(), n2i(0.9624236501192068, 0.9624236501192069));
    assert_eq2!(n2i(2.0, 1000.0).acosh(), n2i(1.3169578969248166, 7.600902209541989));
}

#[test]
fn mpfi_add() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) + n2i(-1.0, 8.0), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) + n2i(8.0, f64::INFINITY), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) + n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 16.0));
    assert_eq2!(I::ENTIRE + n2i(0.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) + n2i(f64::NEG_INFINITY, -7.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq2!(n2i(0.0, 8.0) + n2i(-7.0, 0.0), n2i(-7.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0) + n2i(0.0, 8.0), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) + n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) + n2i(8.0, f64::INFINITY), n2i(8.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) + I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 8.0) + n2i(0.0, 8.0), n2i(0.0, 16.0));
    assert_eq2!(n2i(0.0, 0.0) + n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) + n2i(-7.0, 8.0), n2i(-7.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-0.375, -5.693566843317115e-73) + n2i(-0.125, 5.659799424266695e-73), n2i(-0.5, -3.376741905041928e-75));
    assert_eq2!(n2i(-4.909093465297727e-91, 320255847038976.0) + n2i(-4.547473508864641e-13, 126462925.0), n2i(-4.547473508864642e-13, 320255973501901.0));
    assert_eq2!(n2i(-4.0, 7.0) + n2i(-2443359172.8355484, 3e+300), n2i(-2443359176.8355484, 3.000000000000001e+300));
    assert_eq2!(n2i(7.205869356633318e+16, 1.152921504606847e+18) + n2i(281479271743489.0, 3e+300), n2i(7.234017283807667e+16, 3.000000000000001e+300));
    // signed zeros
    assert_eq2!(n2i(4.0, 8.0) + n2i(-4.0, -2.0), n2i(0.0, 6.0));
    assert_eq2!(n2i(4.0, 8.0) + n2i(-9.0, -8.0), n2i(-5.0, 0.0));
}

#[test]
fn mpfi_add_d() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) + n2i(-4e-17, -4e-17), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) + n2i(4e-17, 4e-17), n2i(f64::NEG_INFINITY, -6.999999999999999));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) + n2i(-8e-17, -8e-17), n2i(f64::NEG_INFINITY, -8e-17));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) + n2i(8e-17, 8e-17), n2i(f64::NEG_INFINITY, 8e-17));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) + n2i(-1.6e+18, -1.6e+18), n2i(f64::NEG_INFINITY, -1.5999999999999997e+18));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) + n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) + n2i(1.6e+18, 1.6e+18), n2i(f64::NEG_INFINITY, 1.6000000000000003e+18));
    assert_eq2!(I::ENTIRE + n2i(-1.6e-16, -1.6e-16), I::ENTIRE);
    assert_eq2!(I::ENTIRE + n2i(0.0, 0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE + n2i(1.6e-16, 1.6e-16), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) + n2i(-1e-17, -1e-17), n2i(-1e-17, -1e-17));
    assert_eq2!(n2i(0.0, 0.0) + n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) + n2i(1e-17, 1e-17), n2i(1e-17, 1e-17));
    assert_eq2!(n2i(0.0, 8.0) + n2i(-3e-17, -3e-17), n2i(-3e-17, 8.0));
    assert_eq2!(n2i(0.0, 8.0) + n2i(0.0, 0.0), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, 8.0) + n2i(2.9999999999999994e-17, 2.9999999999999994e-17), n2i(2.9999999999999994e-17, 8.000000000000002));
    assert_eq2!(n2i(0.0, f64::INFINITY) + n2i(-7e-17, -7e-17), n2i(-7e-17, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY) + n2i(0.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY) + n2i(6.999999999999999e-17, 6.999999999999999e-17), n2i(6.999999999999999e-17, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-32.0, -17.0) + n2i(-31.41592653589793, -31.41592653589793), n2i(-63.41592653589793, -48.41592653589793));
    assert_eq2!(n2i(-31.41592653589793, -17.0) + n2i(31.41592653589793, 31.41592653589793), n2i(0.0, 14.415926535897931));
    assert_eq2!(n2i(-32.0, -15.707963267948966) + n2i(15.707963267948966, 15.707963267948966), n2i(-16.292036732051034, 0.0));
    assert_eq2!(n2i(18.204444444444444, 320255973501901.94) + n2i(3.5, 3.5), n2i(21.704444444444444, 320255973501905.44));
    assert_eq2!(n2i(0.07111111111111111, 320255973501901.94) + n2i(3.5, 3.5), n2i(3.5711111111111107, 320255973501905.44));
    assert_eq2!(n2i(-255.0, 1.1377777777777778) + n2i(256.5, 256.5), n2i(1.5, 257.6377777777778));
    assert_eq2!(n2i(-1.9999999999999998, -2.7133285516175262e-166) + n2i(4097.5, 4097.5), n2i(4095.5, 4097.5));
    assert_eq2!(n2i(18.204444444444444, 320255973501901.94) + n2i(-3.5, -3.5), n2i(14.704444444444444, 320255973501898.44));
    assert_eq2!(n2i(0.07111111111111111, 320255973501901.94) + n2i(-3.5, -3.5), n2i(-3.4288888888888893, 320255973501898.44));
    assert_eq2!(n2i(-255.0, 1.1377777777777778) + n2i(-256.5, -256.5), n2i(-511.5, -255.36222222222221));
    assert_eq2!(n2i(-1.9999999999999998, -2.7133285516175262e-166) + n2i(-4097.5, -4097.5), n2i(-4099.5, -4097.5));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_asin() {
    // special values
    assert_eq2!(n2i(-1.0, 0.0).asin(), n2i(-1.5707963267948968, 0.0));
    assert_eq2!(n2i(0.0, 0.0).asin(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).asin(), n2i(0.0, 1.5707963267948968));
    // regular values
    assert_eq2!(n2i(-1.0, -0.5).asin(), n2i(-1.5707963267948968, -0.5235987755982988));
    assert_eq2!(n2i(-0.75, -0.25).asin(), n2i(-0.8480620789814811, -0.25268025514207865));
    assert_eq2!(n2i(-0.5, 0.5).asin(), n2i(-0.5235987755982989, 0.5235987755982989));
    assert_eq2!(n2i(0.25, 0.625).asin(), n2i(0.25268025514207865, 0.6751315329370317));
    assert_eq2!(n2i(-1.0, 1.0).asin(), n2i(-1.5707963267948968, 1.5707963267948968));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_asinh() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).asinh(), n2i(f64::NEG_INFINITY, -2.644120761058629));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).asinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).asinh(), n2i(f64::NEG_INFINITY, 2.776472280723718));
    assert_eq2!(I::ENTIRE.asinh(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 0.0).asinh(), n2i(-0.881373587019543, 0.0));
    assert_eq2!(n2i(0.0, 0.0).asinh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).asinh(), n2i(0.0, 0.881373587019543));
    assert_eq2!(n2i(0.0, 8.0).asinh(), n2i(0.0, 2.776472280723718));
    assert_eq2!(n2i(0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-6.0, -4.0).asinh(), n2i(-2.491779852644912, -2.0947125472611012));
    assert_eq2!(n2i(-2.0, -0.5).asinh(), n2i(-1.4436354751788105, -0.4812118250596034));
    assert_eq2!(n2i(-1.0, -0.5).asinh(), n2i(-0.881373587019543, -0.4812118250596034));
    assert_eq2!(n2i(-0.75, -0.25).asinh(), n2i(-0.6931471805599454, -0.24746646154726343));
    assert_eq2!(n2i(-0.5, 0.5).asinh(), n2i(-0.48121182505960347, 0.48121182505960347));
    assert_eq2!(n2i(0.25, 0.625).asinh(), n2i(0.24746646154726343, 0.5901436857819591));
    assert_eq2!(n2i(-1.0, 1.0).asinh(), n2i(-0.881373587019543, 0.881373587019543));
    assert_eq2!(n2i(0.125, 17.0).asinh(), n2i(0.12467674692144273, 3.527224456199966));
    assert_eq2!(n2i(17.0, 42.0).asinh(), n2i(3.5272244561999657, 4.430958492080544));
    assert_eq2!(n2i(-42.0, 17.0).asinh(), n2i(-4.430958492080544, 3.527224456199966));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_atan() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).atan(), n2i(-1.5707963267948968, -1.4288992721907325));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).atan(), n2i(-1.5707963267948968, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).atan(), n2i(-1.5707963267948968, 1.4464413322481353));
    assert_eq2!(I::ENTIRE.atan(), n2i(-1.5707963267948968, 1.5707963267948968));
    assert_eq2!(n2i(-1.0, 0.0).atan(), n2i(-0.7853981633974484, 0.0));
    assert_eq2!(n2i(0.0, 0.0).atan(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).atan(), n2i(0.0, 0.7853981633974484));
    assert_eq2!(n2i(0.0, 8.0).atan(), n2i(0.0, 1.4464413322481353));
    assert_eq2!(n2i(0.0, f64::INFINITY).atan(), n2i(0.0, 1.5707963267948968));
    // regular values
    assert_eq2!(n2i(-6.0, -4.0).atan(), n2i(-1.4056476493802699, -1.3258176636680323));
    assert_eq2!(n2i(-2.0, -0.5).atan(), n2i(-1.1071487177940906, -0.4636476090008061));
    assert_eq2!(n2i(-1.0, -0.5).atan(), n2i(-0.7853981633974484, -0.4636476090008061));
    assert_eq2!(n2i(-0.75, -0.25).atan(), n2i(-0.6435011087932845, -0.24497866312686414));
    assert_eq2!(n2i(-0.5, 0.5).atan(), n2i(-0.46364760900080615, 0.46364760900080615));
    assert_eq2!(n2i(0.25, 0.625).atan(), n2i(0.24497866312686414, 0.5585993153435624));
    assert_eq2!(n2i(-1.0, 1.0).atan(), n2i(-0.7853981633974484, 0.7853981633974484));
    assert_eq2!(n2i(0.125, 17.0).atan(), n2i(0.12435499454676142, 1.512040504079174));
    assert_eq2!(n2i(17.0, 42.0).atan(), n2i(1.5120405040791738, 1.5469913006098268));
    assert_eq2!(n2i(-42.0, 17.0).atan(), n2i(-1.5469913006098268, 1.512040504079174));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_atan2() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).atan2(n2i(-1.0, 8.0)), n2i(-1.7126933813990606, -0.7188299996216244));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).atan2(n2i(8.0, f64::INFINITY)), n2i(-1.5707963267948968, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).atan2(n2i(0.0, 8.0)), n2i(-1.5707963267948968, 1.5707963267948968));
    assert_eq2!(I::ENTIRE.atan2(n2i(0.0, 8.0)), n2i(-1.5707963267948968, 1.5707963267948968));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(f64::NEG_INFINITY, -7.0)), n2i(3.141592653589793, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 8.0).atan2(n2i(-7.0, 0.0)), n2i(1.5707963267948966, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 8.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).atan2(n2i(0.0, 8.0)), n2i(0.0, 1.5707963267948968));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(8.0, f64::INFINITY)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).atan2(I::ENTIRE), n2i(0.0, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 8.0).atan2(n2i(-7.0, 8.0)), n2i(0.0, 3.1415926535897936));
    assert_eq2!(n2i(0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, f64::INFINITY).atan2(n2i(0.0, 8.0)), n2i(0.0, 1.5707963267948968));
    // regular values
    assert_eq2!(n2i(-17.0, -5.0).atan2(n2i(-4002.0, -1.0)), n2i(-3.140343278927515, -1.629552149510619));
    assert_eq2!(n2i(-17.0, -5.0).atan2(n2i(1.0, 4002.0)), n2i(-1.512040504079174, -0.0012493746622783569));
    assert_eq2!(n2i(5.0, 17.0).atan2(n2i(1.0, 4002.0)), n2i(0.0012493746622783569, 1.512040504079174));
    assert_eq2!(n2i(5.0, 17.0).atan2(n2i(-4002.0, -1.0)), n2i(1.629552149510619, 3.140343278927515));
    assert_eq2!(n2i(-17.0, 5.0).atan2(n2i(-4002.0, 1.0)), n2i(-3.1415926535897936, 3.1415926535897936));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_atanh() {
    // special values
    assert_eq2!(n2i(-1.0, 0.0).atanh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 0.0).atanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).atanh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-1.0, -0.5).atanh(), n2i(f64::NEG_INFINITY, -0.5493061443340548));
    assert_eq2!(n2i(-0.75, -0.25).atanh(), n2i(-0.9729550745276567, -0.2554128118829953));
    assert_eq2!(n2i(-0.5, 0.5).atanh(), n2i(-0.5493061443340549, 0.5493061443340549));
    assert_eq2!(n2i(0.25, 0.625).atanh(), n2i(0.2554128118829953, 0.7331685343967136));
    assert_eq2!(n2i(-1.0, 1.0).atanh(), I::ENTIRE);
    assert_eq2!(n2i(0.125, 1.0).atanh(), n2i(0.12565721414045303, f64::INFINITY));
}

#[test]
fn mpfi_bounded_p() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -8.0).is_common_interval(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).is_common_interval(), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).is_common_interval(), false);
    assert_eq2!(I::ENTIRE.is_common_interval(), false);
    assert!(n2i(-8.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 5.0).is_common_interval());
    assert_eq2!(n2i(0.0, f64::INFINITY).is_common_interval(), false);
    assert_eq2!(n2i(5.0, f64::INFINITY).is_common_interval(), false);
    // regular values
    assert!(n2i(-34.0, -17.0).is_common_interval());
    assert!(n2i(-8.0, -1.0).is_common_interval());
    assert!(n2i(-34.0, 17.0).is_common_interval());
    assert!(n2i(-3.141592653589793, 3.1415926535897936).is_common_interval());
    assert!(n2i(3.141592653589793, 3.1415926535897936).is_common_interval());
    assert!(n2i(8.0, 5.0706024009129063e+30).is_common_interval());
    assert!(n2i(0.9999999999999999, 2.0).is_common_interval());
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_cos() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(I::ENTIRE.cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 0.0).cos(), n2i(0.5403023058681397, 1.0));
    assert_eq2!(n2i(0.0, 0.0).cos(), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).cos(), n2i(0.5403023058681397, 1.0));
    assert_eq2!(n2i(0.0, 8.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    // regular values
    assert_eq2!(n2i(-2.0, -0.5).cos(), n2i(-0.4161468365471424, 0.8775825618903728));
    assert_eq2!(n2i(-1.0, -0.25).cos(), n2i(0.5403023058681397, 0.9689124217106448));
    assert_eq2!(n2i(-0.5, 0.5).cos(), n2i(0.8775825618903726, 1.0));
    assert_eq2!(n2i(-4.5, 0.625).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.0, 3.141592653589793).cos(), n2i(-1.0, 0.5403023058681398));
    assert_eq2!(n2i(0.125, 17.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(17.0, 42.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -1.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -2.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -3.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -4.0).cos(), n2i(-0.6536436208636119, 1.0));
    assert_eq2!(n2i(-7.0, -5.0).cos(), n2i(0.28366218546322625, 1.0));
    assert_eq2!(n2i(-7.0, -6.0).cos(), n2i(0.7539022543433046, 1.0));
    assert_eq2!(n2i(-7.0, -7.0).cos(), n2i(0.7539022543433046, 0.7539022543433047));
    assert_eq2!(n2i(-6.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, -1.0).cos(), n2i(-1.0, 0.9601702866503661));
    assert_eq2!(n2i(-6.0, -2.0).cos(), n2i(-1.0, 0.9601702866503661));
    assert_eq2!(n2i(-6.0, -3.0).cos(), n2i(-1.0, 0.9601702866503661));
    assert_eq2!(n2i(-6.0, -4.0).cos(), n2i(-0.6536436208636119, 0.9601702866503661));
    assert_eq2!(n2i(-6.0, -5.0).cos(), n2i(0.28366218546322625, 0.9601702866503661));
    assert_eq2!(n2i(-6.0, -6.0).cos(), n2i(0.960170286650366, 0.9601702866503661));
    assert_eq2!(n2i(-5.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, -1.0).cos(), n2i(-1.0, 0.5403023058681398));
    assert_eq2!(n2i(-5.0, -2.0).cos(), n2i(-1.0, 0.2836621854632263));
    assert_eq2!(n2i(-5.0, -3.0).cos(), n2i(-1.0, 0.2836621854632263));
    assert_eq2!(n2i(-5.0, -4.0).cos(), n2i(-0.6536436208636119, 0.2836621854632263));
    assert_eq2!(n2i(-5.0, -5.0).cos(), n2i(0.28366218546322625, 0.2836621854632263));
    assert_eq2!(n2i(-4.0, 1.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, -1.0).cos(), n2i(-1.0, 0.5403023058681398));
    assert_eq2!(n2i(-4.0, -2.0).cos(), n2i(-1.0, -0.41614683654714235));
    assert_eq2!(n2i(-4.0, -3.0).cos(), n2i(-1.0, -0.6536436208636118));
    assert_eq2!(n2i(-4.0, -4.0).cos(), n2i(-0.6536436208636119, -0.6536436208636118));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_cosh() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).cosh(), n2i(548.317035155212, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.cosh(), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(-1.0, 0.0).cosh(), n2i(1.0, 1.543080634815244));
    assert_eq2!(n2i(0.0, 0.0).cosh(), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).cosh(), n2i(1.0, 1.543080634815244));
    assert_eq2!(n2i(0.0, 8.0).cosh(), n2i(1.0, 1490.4791612521783));
    assert_eq2!(n2i(0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-0.125, 0.0).cosh(), n2i(1.0, 1.0078226778257109));
    assert_eq2!(n2i(0.0, 0.5000000000000001).cosh(), n2i(1.0, 1.127625965206381));
    assert_eq2!(n2i(-4.5, -0.625).cosh(), n2i(1.2017536929756063, 45.01412014853003));
    assert_eq2!(n2i(1.0, 3.0).cosh(), n2i(1.5430806348152437, 10.067661995777767));
    assert_eq2!(n2i(17.0, 709.089565712824).cosh(), n2i(12077476.376787668, 4.4942328371554195e+307));
}

#[test]
fn mpfi_d_div() {
    // special values
    assert_eq2!(n2i(-3.9999999999999997e-17, -3.9999999999999997e-17) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 5.714285714285714e-18));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(3.9999999999999997e-17, 3.9999999999999997e-17) / n2i(f64::NEG_INFINITY, -7.0), n2i(-5.714285714285714e-18, 0.0));
    assert_eq2!(n2i(-8e-17, -8e-17) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(8e-17, 8e-17) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-1.6e+18, -1.6e+18) / n2i(f64::NEG_INFINITY, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 8.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(1.6e+18, 1.6e+18) / n2i(f64::NEG_INFINITY, 8.0), I::ENTIRE);
    assert_eq2!(n2i(-1.6e-16, -1.6e-16) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(1.6e-16, 1.6e-16) / I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1e-17, -1e-17) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(1e-17, 1e-17) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(-3e-17, -3e-17) / n2i(0.0, 7.0), n2i(f64::NEG_INFINITY, -4.285714285714285e-18));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, 7.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(3e-17, 3e-17) / n2i(0.0, 7.0), n2i(4.285714285714285e-18, f64::INFINITY));
    assert_eq2!(n2i(-7e-17, -7e-17) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(6.999999999999999e-17, 6.999999999999999e-17) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-2.5, -2.5) / n2i(-8.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(-2.5, -2.5) / n2i(-8.0, -5.0), n2i(0.3125, 0.5));
    assert_eq2!(n2i(-2.5, -2.5) / n2i(25.0, 40.0), n2i(-0.1, -0.0625));
    assert_eq2!(n2i(-2.5, -2.5) / n2i(-16.0, -7.0), n2i(0.15625, 0.35714285714285715));
    assert_eq2!(n2i(-2.5, -2.5) / n2i(11.0, 143.0), n2i(-0.2272727272727273, -0.01748251748251748));
    assert_eq2!(n2i(33.125, 33.125) / n2i(8.28125, 530.0), n2i(0.0625, 4.0));
    assert_eq2!(n2i(33.125, 33.125) / n2i(-530.0, -496.875), n2i(-0.06666666666666668, -0.0625));
    assert_eq2!(n2i(33.125, 33.125) / n2i(54.0, 265.0), n2i(0.125, 0.6134259259259259));
    assert_eq2!(n2i(33.125, 33.125) / n2i(52.0, 54.0), n2i(0.6134259259259258, 0.6370192307692308));
}

#[test]
fn mpfi_diam_abs() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -8.0).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).wid(), f64::INFINITY);
    assert_eq2!(I::ENTIRE.wid(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).wid(), f64::INFINITY);
    assert_eq2!(n2i(-8.0, 0.0).wid(), 8.0);
    assert_eq2!(n2i(0.0, 0.0).wid(), -0.0);
    assert_eq2!(n2i(0.0, 5.0).wid(), 5.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).wid(), f64::INFINITY);
    // regular values
    assert_eq2!(n2i(-34.0, -17.0).wid(), 17.0);
}

#[test]
fn mpfi_div() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) / n2i(-1.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) / n2i(0.0, 8.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(0.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0) / n2i(-7.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(0.0, 8.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) / n2i(8.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0) / n2i(-7.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    // regular value
    assert_eq2!(n2i(-123456789.0, -30030.0) / n2i(-286.0, -9.0), n2i(105.0, 13717421.0));
    assert_eq2!(n2i(-123456789.0, -0.08022692722045614) / n2i(-46.5555251465269, -9.0), n2i(0.0017232525455991162, 13717421.0));
    assert_eq2!(n2i(-1.0114785059407487, -0.458221435546875) / n2i(-286.0, -0.028851413396910257), n2i(0.0016021728515625, 35.058196006753334));
    assert_eq2!(n2i(-1.0114785059407487, -0.08022692722045614) / n2i(-2.909720321657931, -0.028851413396910257), n2i(0.02757204072958586, 35.058196006753334));
    assert_eq2!(n2i(-3.1993354256270465e+80, -1785.0) / n2i(-7.0, 0.0), n2i(255.0, f64::INFINITY));
    assert_eq2!(n2i(-256.0, -0.9207771443835955) / n2i(-1.4860452438034586, 0.0), n2i(0.6196158214045437, f64::INFINITY));
    assert_eq2!(n2i(-1.148236704163044, -0.6001224414812757) / n2i(-0.5769162195036163, 0.27943225890584056), I::ENTIRE);
    assert_eq2!(n2i(-100.0, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq2!(n2i(-2.0, -1.148236704163044) / n2i(0.0, 0.5769162195036163), n2i(f64::NEG_INFINITY, -1.990300610981256));
    assert_eq2!(n2i(-4886718345.0, -480480.0) / n2i(1035.0, 4576.0), n2i(-4721467.0, -105.0));
    assert_eq2!(n2i(-0.8377603228949244, -3.333999742949345e-12) / n2i(4.5329676585533765, 9.191685613161424e+62), n2i(-0.18481497905993932, -3.6271907931396667e-75));
    assert_eq2!(n2i(-4886718345.0, -1.6900354150633865) / n2i(0.00789642333984375, 0.15383794751559648), n2i(-618852122624.0, -10.985816193966357));
    assert_eq2!(n2i(-13.40416516631879, -1.6900354150633865) / n2i(0.28331047865958603, 39.3825145639927), n2i(-47.312634639344466, -0.042913344507681084));
    assert_eq2!(n2i(-123456789.0, 0.0) / n2i(-144.0, -9.0), n2i(0.0, 13717421.0));
    assert_eq2!(n2i(-0.0787589031323913, 0.0) / n2i(-0.00390625, -0.003752026718402495), n2i(0.0, 20.99102939382174));
    assert_eq2!(n2i(-64138178286.0, 0.0) / n2i(-45812984490.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-1.148236704163044, 0.0) / n2i(-0.5769162195036163, 0.27943225890584056), I::ENTIRE);
    assert_eq2!(n2i(-64138178286.0, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-123456789.0, 0.0) / n2i(9.0, 144.0), n2i(-13717421.0, 0.0));
    assert_eq2!(n2i(-0.0787589031323913, 0.0) / n2i(0.003752026718402495, 9.0), n2i(-20.99102939382174, 0.0));
    assert_eq2!(n2i(-123456789.0, 42624.0) / n2i(-2806.0, -9.0), n2i(-4736.0, 13717421.0));
    assert_eq2!(n2i(-18.0, 16.0) / n2i(-806308527035.0, -9.0), n2i(-1.777777777777778, 2.0));
    assert_eq2!(n2i(-1.0, 0.458221435546875) / n2i(-879609302220.0, -286.0), n2i(-0.0016021728515625, 0.003496503496503497));
    assert_eq2!(n2i(-0.7098547996011582, 6.900706076855589) / n2i(-15246561238493.0, -0.8096558996257991), n2i(-8.523010923584831, 0.8767364011418107));
    assert_eq2!(n2i(-3.1993354256270465e+80, 1785.0) / n2i(-7.0, 0.0), I::ENTIRE);
    assert_eq2!(n2i(-1.148236704163044, 0.6001224414812757) / n2i(-0.5769162195036163, 0.27943225890584056), I::ENTIRE);
    assert_eq2!(n2i(0.0, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq2!(n2i(-30030.0, 897276723200.0) / n2i(286.0, 3003.0), n2i(-105.0, 3137331200.0));
    assert_eq2!(n2i(-16.0, 897276723200.0) / n2i(286.0, 3003.0), n2i(-0.05594405594405595, 3137331200.0));
    assert_eq2!(n2i(-30030.0, 1024.0) / n2i(286.0, 3003.0), n2i(-105.0, 3.580419580419581));
    assert_eq2!(n2i(-1.0945314249688722, 2.0809358694701494) / n2i(1.1491894614780331, 273.0), n2i(-0.9524377499608617, 1.810785722655121));
    assert_eq2!(n2i(0.0, 123456789.0) / n2i(-10.0, -9.0), n2i(-13717421.0, 0.0));
    assert_eq2!(n2i(0.0, 1.6747908002429839) / n2i(-0.9375, -0.8900092612419526), n2i(-1.8817678345345727, 0.0));
    assert_eq2!(n2i(0.0, 10.0) / n2i(-9.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 10.0) / n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 123456789.0) / n2i(9.0, 10.0), n2i(0.0, 13717421.0));
    assert_eq2!(n2i(0.0, 1.3727266705940584) / n2i(2.2252982203088894, 10.0), n2i(0.0, 0.6168731265167295));
    assert_eq2!(n2i(30030.0, 123456789.0) / n2i(-286.0, -9.0), n2i(-13717421.0, -105.0));
    assert_eq2!(n2i(0.458221435546875, 16.033747482221248) / n2i(-286.0, -2.2869264900484247), n2i(-7.011046289416036, -0.0016021728515625));
    assert_eq2!(n2i(0.6045543549399371, 123456789.0) / n2i(-21.137397982390347, -9.0), n2i(-13717421.0, -0.02860117198169774));
    assert_eq2!(n2i(0.8802057619444759, 1.0246025844972029) / n2i(-0.31574744583216946, -0.17796813676086096), n2i(-5.757224878259978, -2.787689254697361));
    assert_eq2!(n2i(30030.0, 61166.0) / n2i(-286.0, 0.0), n2i(f64::NEG_INFINITY, -105.0));
    assert_eq2!(n2i(0.10376109584882762, 61166.0) / n2i(-0.8870683004794921, 0.0), n2i(f64::NEG_INFINITY, -0.11697080798935215));
    assert_eq2!(n2i(5.0, 6.0) / n2i(-0.31574744583216946, 0.17796813676086096), I::ENTIRE);
    assert_eq2!(n2i(30030.0, 978670.0) / n2i(0.0, 286.0), n2i(105.0, f64::INFINITY));
    assert_eq2!(n2i(1.4961539901492185, 978670.0) / n2i(0.0, 1.2839435423669967), n2i(1.1652802017999981, f64::INFINITY));
    assert_eq2!(n2i(1593578642.0, 7418529635.0) / n2i(3136799.0, 8952689.0), n2i(178.0, 2365.0));
    assert_eq2!(n2i(0.007198359729899689, 7418529635.0) / n2i(2.9914846420288086, 14.240148179871241), n2i(0.0005054975298694382, 2479882240.0));
    assert_eq2!(n2i(0.3710339409299195, 1.002109217638828) / n2i(2.2869264900484247, 8.537949562072754), n2i(0.04345703125, 0.4381903930885023));
    assert_eq2!(n2i(0.03228846131764278, 3.5999783646966885) / n2i(0.5102487095624549, 0.7548398876996482), n2i(0.042775245245771115, 7.055340458937599));
}

#[test]
fn mpfi_div_d() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) / n2i(-7.0, -7.0), n2i(1.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) / n2i(7.0, 7.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-8e-17, -8e-17), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) / n2i(8e-17, 8e-17), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) / n2i(-3.0, -3.0), n2i(-2.666666666666667, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) / n2i(3.0, 3.0), n2i(f64::NEG_INFINITY, 2.666666666666667));
    assert_eq2!(I::ENTIRE / n2i(-1.6e-16, -1.6e-16), I::ENTIRE);
    assert_eq2!(I::ENTIRE / n2i(0.0, 0.0), I::EMPTY);
    assert_eq2!(I::ENTIRE / n2i(1.6e-16, 1.6e-16), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) / n2i(-1e-17, -1e-17), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) / n2i(1e-17, 1e-17), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0) / n2i(-3e-17, -3e-17), n2i(-2.666666666666667e+17, 0.0));
    assert_eq2!(n2i(0.0, 8.0) / n2i(3e-17, 3e-17), n2i(0.0, 2.666666666666667e+17));
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(-7e-17, -7e-17), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) / n2i(6.999999999999999e-17, 6.999999999999999e-17), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-4294967296.000001, -0.5000000000000001) / n2i(-1.0, -1.0), n2i(0.5000000000000001, 4294967296.000001));
    assert_eq2!(n2i(-4294967296.000002, -0.5000000000000001) / n2i(0.5000000000000001, 0.5000000000000001), n2i(-8589934592.000002, -1.0));
    assert_eq2!(n2i(-4294967296.000001, -0.5000000596046449) / n2i(0.5000000000000001, 0.5000000000000001), n2i(-8589934592.0, -1.0000001192092893));
    assert_eq2!(n2i(-4294967296.000002, -0.5000000596046449) / n2i(0.5000000000000001, 0.5000000000000001), n2i(-8589934592.000002, -1.0000001192092893));
    assert_eq2!(n2i(-0.5688888888888889, 40031996687737.74) / n2i(-5124095576030431.0, -5124095576030431.0), n2i(-0.0078125, 1.1102230246251565e-16));
    assert_eq2!(n2i(-0.5688888888888889, 0.5000000000000001) / n2i(-5124095576030431.0, -5124095576030431.0), n2i(-9.757819552369542e-17, 1.1102230246251565e-16));
    assert_eq2!(n2i(-1.0, 40031996687737.74) / n2i(-5124095576030431.0, -5124095576030431.0), n2i(-0.0078125, 1.9515639104739082e-16));
    assert_eq2!(n2i(-1.0, 0.5000000000000001) / n2i(-5124095576030431.0, -5124095576030431.0), n2i(-9.757819552369542e-17, 1.9515639104739082e-16));
}

#[test]
fn mpfi_d_sub() {
    // special values
    assert_eq2!(n2i(-4e-17, -4e-17) - n2i(f64::NEG_INFINITY, -7.0), n2i(6.999999999999999, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq2!(n2i(4e-17, 4e-17) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq2!(n2i(-8.192e-14, -8.192e-14) - n2i(f64::NEG_INFINITY, 0.0), n2i(-8.192e-14, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(8.192e-14, 8.192e-14) - n2i(f64::NEG_INFINITY, 0.0), n2i(8.192e-14, f64::INFINITY));
    assert_eq2!(n2i(-1.6e+18, -1.6e+18) - n2i(f64::NEG_INFINITY, 8.0), n2i(-1.6000000000000003e+18, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq2!(n2i(1.6e+18, 1.6e+18) - n2i(f64::NEG_INFINITY, 8.0), n2i(1.5999999999999997e+18, f64::INFINITY));
    assert_eq2!(n2i(-1.6e-16, -1.6e-16) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(1.6e-16, 1.6e-16) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(-1e-17, -1e-17) - n2i(0.0, 0.0), n2i(-1e-17, -1e-17));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(1e-17, 1e-17) - n2i(0.0, 0.0), n2i(1e-17, 1e-17));
    assert_eq2!(n2i(-3e-17, -3e-17) - n2i(0.0, 8.0), n2i(-8.000000000000002, -3e-17));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq2!(n2i(3e-17, 3e-17) - n2i(0.0, 8.0), n2i(-8.0, 3e-17));
    assert_eq2!(n2i(-7e-17, -7e-17) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -7e-17));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(-6.999999999999999e-17, -6.999999999999999e-17) - n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -6.999999999999999e-17));
    // regular values
    assert_eq2!(n2i(-31.41592653589793, -31.41592653589793) - n2i(17.0, 32.0), n2i(-63.41592653589793, -48.41592653589793));
    assert_eq2!(n2i(31.41592653589793, 31.41592653589793) - n2i(17.0, 31.41592653589793), n2i(0.0, 14.415926535897931));
    assert_eq2!(n2i(15.707963267948966, 15.707963267948966) - n2i(15.707963267948966, 32.0), n2i(-16.292036732051034, 0.0));
    assert_eq2!(n2i(3.5, 3.5) - n2i(-320255973501901.94, -18.204444444444444), n2i(21.704444444444444, 320255973501905.44));
    assert_eq2!(n2i(3.5, 3.5) - n2i(-320255973501901.94, -0.07111111111111111), n2i(3.5711111111111107, 320255973501905.44));
    assert_eq2!(n2i(256.5, 256.5) - n2i(-1.1377777777777778, 255.0), n2i(1.5, 257.6377777777778));
    assert_eq2!(n2i(4097.5, 4097.5) - n2i(2.7133285516175262e-166, 1.9999999999999998), n2i(4095.5, 4097.5));
    assert_eq2!(n2i(-3.5, -3.5) - n2i(-320255973501901.94, -18.204444444444444), n2i(14.704444444444444, 320255973501898.44));
    assert_eq2!(n2i(-3.5, -3.5) - n2i(-320255973501901.94, -0.07111111111111111), n2i(-3.4288888888888893, 320255973501898.44));
    assert_eq2!(n2i(-256.5, -256.5) - n2i(-1.1377777777777778, 255.0), n2i(-511.5, -255.36222222222221));
    assert_eq2!(n2i(-4097.5, -4097.5) - n2i(2.7133285516175262e-166, 1.9999999999999998), n2i(-4099.5, -4097.5));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_exp() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).exp(), n2i(0.0, 0.0009118819655545162));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).exp(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.0).exp(), n2i(0.0, 2.7182818284590455));
    assert_eq2!(I::ENTIRE.exp(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).exp(), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).exp(), n2i(1.0, 2.7182818284590455));
    assert_eq2!(n2i(0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-123.0, -17.0).exp(), n2i(3.817497188671174e-54, 4.139937718785167e-08));
    assert_eq2!(n2i(-0.125, 0.25).exp(), n2i(0.8824969025845953, 1.2840254166877416));
    assert_eq2!(n2i(-0.125, 0.0).exp(), n2i(0.8824969025845953, 1.0));
    assert_eq2!(n2i(0.0, 0.25).exp(), n2i(1.0, 1.2840254166877416));
    assert_eq2!(n2i(7.105427357601002e-14, 7.815970093361102e-14).exp(), n2i(1.000000000000071, 1.0000000000000784));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_exp2() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -1.0).exp2(), n2i(0.0, 0.5));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).exp2(), n2i(0.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 1.0).exp2(), n2i(0.0, 2.0));
    assert_eq2!(I::ENTIRE.exp2(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).exp2(), n2i(1.0, 1.0));
    assert_eq2!(n2i(0.0, 1.0).exp2(), n2i(1.0, 2.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-123.0, -17.0).exp2(), n2i(9.4039548065783e-38, 7.62939453125e-06));
    assert_eq2!(n2i(-7.0, 7.0).exp2(), n2i(0.0078125, 128.0));
    assert_eq2!(n2i(-0.125, 0.25).exp2(), n2i(0.9170040432046712, 1.1892071150027212));
    assert_eq2!(n2i(-0.125, 0.0).exp2(), n2i(0.9170040432046712, 1.0));
    assert_eq2!(n2i(0.0, 0.25).exp2(), n2i(1.0, 1.1892071150027212));
    assert_eq2!(n2i(7.105427357601002e-14, 7.815970093361102e-14).exp2(), n2i(1.000000000000049, 1.0000000000000542));
}

#[test]
fn mpfi_intersect() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).intersection(n2i(-1.0, 8.0)), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).intersection(n2i(8.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq2!(I::ENTIRE.intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0).intersection(n2i(f64::NEG_INFINITY, -7.0)), I::EMPTY);
    assert_eq2!(n2i(0.0, 8.0).intersection(n2i(-7.0, 0.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).intersection(n2i(0.0, 8.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0).intersection(n2i(8.0, f64::INFINITY)), I::EMPTY);
    assert_eq2!(n2i(0.0, 0.0).intersection(I::ENTIRE), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0).intersection(n2i(-7.0, 8.0)), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0).intersection(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).intersection(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    // regular values
    assert_eq2!(n2i(18.0, 144.0).intersection(n2i(-13.0, 52.0)), n2i(18.0, 52.0));
}

#[test]
fn mpfi_inv() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -0.25).recip(), n2i(-4.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 4.0).recip(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.recip(), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).recip(), I::EMPTY);
    assert_eq2!(n2i(0.0, 2.0).recip(), n2i(0.5, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-8.0, -2.0).recip(), n2i(-0.5, -0.125));
    assert_eq2!(n2i(0.0625, 0.6329046211334585).recip(), n2i(1.5800169039832832, 16.0));
    assert_eq2!(n2i(0.20268693276943753, 64.0).recip(), n2i(0.015625, 4.933717168326436));
    assert_eq2!(n2i(-0.6816974503452349, -0.19509730660827918).recip(), n2i(-5.125647387884359, -1.4669264194747476));
}

#[test]
fn mpfi_is_neg() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(f64::NEG_INFINITY, 0.0).precedes(n2i(0.0, 0.0)));
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(I::ENTIRE.precedes(n2i(0.0, 0.0)), false);
    assert!(n2i(-8.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert_eq2!(n2i(0.0, 5.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(5.0, f64::INFINITY).precedes(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).precedes(n2i(0.0, 0.0)));
    assert_eq2!(n2i(-34.0, 17.0).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-3.141592653589793, 3.1415926535897936).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(3.141592653589793, 3.1415926535897936).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(8.0, 5.0706024009129063e+30).precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.9999999999999999, 2.0).precedes(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_nonneg() {
    // special values
    assert_eq2!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(I::ENTIRE), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(-8.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 5.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, f64::INFINITY)));
    assert!(n2i(0.0, 0.0).less(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq2!(n2i(0.0, 0.0).less(n2i(-34.0, -17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(-8.0, -1.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(-34.0, 17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).less(n2i(-3.141592653589793, 3.1415926535897936)), false);
    assert!(n2i(0.0, 0.0).less(n2i(3.141592653589793, 3.1415926535897936)));
    assert!(n2i(0.0, 0.0).less(n2i(8.0, 5.0706024009129063e+30)));
    assert!(n2i(0.0, 0.0).less(n2i(0.9999999999999999, 2.0)));
}

#[test]
fn mpfi_is_nonpos() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).less(n2i(0.0, 0.0)));
    assert!(n2i(f64::NEG_INFINITY, 0.0).less(n2i(0.0, 0.0)));
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).less(n2i(0.0, 0.0)), false);
    assert_eq2!(I::ENTIRE.less(n2i(0.0, 0.0)), false);
    assert!(n2i(-8.0, 0.0).less(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).less(n2i(0.0, 0.0)));
    assert_eq2!(n2i(0.0, 5.0).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(5.0, f64::INFINITY).less(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).less(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).less(n2i(0.0, 0.0)));
    assert_eq2!(n2i(-34.0, 17.0).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-3.141592653589793, 3.1415926535897936).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(3.141592653589793, 3.1415926535897936).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(8.0, 5.0706024009129063e+30).less(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.9999999999999999, 2.0).less(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_pos() {
    // special values
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(I::ENTIRE), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(-8.0, 0.0)), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 0.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, 5.0)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.0, f64::INFINITY)));
    assert!(n2i(0.0, 0.0).precedes(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(-34.0, -17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(-8.0, -1.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(-34.0, 17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).precedes(n2i(-3.141592653589793, 3.1415926535897936)), false);
    assert!(n2i(0.0, 0.0).precedes(n2i(3.141592653589793, 3.1415926535897936)));
    assert!(n2i(0.0, 0.0).precedes(n2i(8.0, 5.0706024009129063e+30)));
    assert!(n2i(0.0, 0.0).precedes(n2i(0.9999999999999999, 2.0)));
}

#[test]
fn mpfi_is_strictly_neg() {
    // special values
    assert!(n2i(f64::NEG_INFINITY, -8.0).strict_precedes(n2i(0.0, 0.0)));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(I::ENTIRE.strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-8.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, 5.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, f64::INFINITY).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(5.0, f64::INFINITY).strict_precedes(n2i(0.0, 0.0)), false);
    // regular values
    assert!(n2i(-34.0, -17.0).strict_precedes(n2i(0.0, 0.0)));
    assert!(n2i(-8.0, -1.0).strict_precedes(n2i(0.0, 0.0)));
    assert_eq2!(n2i(-34.0, 17.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-3.141592653589793, 3.1415926535897936).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(3.141592653589793, 3.1415926535897936).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(8.0, 5.0706024009129063e+30).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.9999999999999999, 2.0).strict_precedes(n2i(0.0, 0.0)), false);
}

#[test]
fn mpfi_is_strictly_pos() {
    // special values
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, -8.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(f64::NEG_INFINITY, 5.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(I::ENTIRE), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(-8.0, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, 5.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(0.0, f64::INFINITY)), false);
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(5.0, f64::INFINITY)));
    // regular values
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(-34.0, -17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(-8.0, -1.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(-34.0, 17.0)), false);
    assert_eq2!(n2i(0.0, 0.0).strict_precedes(n2i(-3.141592653589793, 3.1415926535897936)), false);
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(3.141592653589793, 3.1415926535897936)));
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(8.0, 5.0706024009129063e+30)));
    assert!(n2i(0.0, 0.0).strict_precedes(n2i(0.9999999999999999, 2.0)));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log() {
    // special values
    assert_eq2!(n2i(0.0, 1.0).ln(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).ln(), I::ENTIRE);
    // regular values
    assert_eq2!(n2i(1.0, 1.0).ln(), n2i(0.0, 0.0));
    assert_eq2!(n2i(62453330062.74823, 350295347459.3553).ln(), n2i(24.85768539575924, 26.582042485472602));
    assert_eq2!(n2i(0.7112834182118892, 1.0).ln(), n2i(-0.34068430946169004, 0.0));
    assert_eq2!(n2i(1.0, 1164158665.5931258).ln(), n2i(0.0, 20.875264487613222));
    assert_eq2!(n2i(20453680510.783615, 664939052621.4927).ln(), n2i(23.741428679318908, 27.222961223208888));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log2() {
    // special values
    assert_eq2!(n2i(0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).log2(), I::ENTIRE);
    // regular values
    assert_eq2!(n2i(1.0, 1.0).log2(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.7112834182118892, 1.0).log2(), n2i(-0.4915035637690611, 0.0));
    assert_eq2!(n2i(1.0, 1164158665.5931258).log2(), n2i(0.0, 30.116640553525084));
    assert_eq2!(n2i(20453680510.783615, 664939052621.4927).log2(), n2i(34.251641419272396, 39.27443115503601));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_log10() {
    // special values
    assert_eq2!(n2i(0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).log10(), I::ENTIRE);
    // regular values
    assert_eq2!(n2i(1.0, 1.0).log10(), n2i(0.0, 0.0));
    assert_eq2!(n2i(62453330062.74823, 350295347459.3553).log10(), n2i(10.79555560026529, 11.544434369158552));
    assert_eq2!(n2i(0.7112834182118892, 1.0).log10(), n2i(-0.1479573156702318, 0.0));
    assert_eq2!(n2i(100.0, 1164158665.5931258).log10(), n2i(2.0, 9.066012175241337));
    assert_eq2!(n2i(20453680510.783615, 664939052621.4927).log10(), n2i(10.31077146792781, 11.822781840305819));
}

#[test]
fn mpfi_mag() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -8.0).mag(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).mag(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).mag(), f64::INFINITY);
    assert_eq2!(I::ENTIRE.mag(), f64::INFINITY);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).mag(), f64::INFINITY);
    assert_eq2!(n2i(-8.0, 0.0).mag(), 8.0);
    assert_eq2!(n2i(0.0, 0.0).mag(), 0.0);
    assert_eq2!(n2i(0.0, 5.0).mag(), 5.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).mag(), f64::INFINITY);
    // regular values
    assert_eq2!(n2i(-34.0, -17.0).mag(), 34.0);
}

#[test]
fn mpfi_mid() {
    // special values
    assert_eq2!(n2i(-8.0, 0.0).mid(), -4.0);
    assert_eq2!(n2i(0.0, 0.0).mid(), 0.0);
    assert_eq2!(n2i(0.0, 5.0).mid(), 2.5);
    // regular values
    assert_eq2!(n2i(-34.0, -17.0).mid(), -25.5);
    assert_eq2!(n2i(-34.0, 17.0).mid(), -8.5);
    assert_eq2!(n2i(0.0, 80063993375475.25).mid(), 40031996687737.625);
    assert_eq2!(n2i(3.141592653589793, 3.1415926535897936).mid(), 3.141592653589793);
    assert_eq2!(n2i(-3.1415926535897936, -3.141592653589793).mid(), -3.141592653589793);
    assert_eq2!(n2i(-4.0, -0.9999999999999987).mid(), -2.499999999999999);
    assert_eq2!(n2i(-8.0, -0.9999999999999978).mid(), -4.499999999999999);
    assert_eq2!(n2i(-0.9999999999999999, 2.0).mid(), 0.5);
}

#[test]
fn mpfi_mig() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -8.0).mig(), 8.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).mig(), 0.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 5.0).mig(), 0.0);
    assert_eq2!(I::ENTIRE.mig(), 0.0);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).mig(), 0.0);
    assert_eq2!(n2i(-8.0, 0.0).mig(), 0.0);
    assert_eq2!(n2i(0.0, 0.0).mig(), 0.0);
    assert_eq2!(n2i(0.0, 5.0).mig(), 0.0);
    assert_eq2!(n2i(0.0, f64::INFINITY).mig(), 0.0);
    // regular values
    assert_eq2!(n2i(-34.0, -17.0).mig(), 17.0);
}

#[test]
fn mpfi_mul() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) * n2i(-1.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) * n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) * n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 64.0));
    assert_eq2!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(I::ENTIRE * n2i(0.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, -7.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0) * n2i(-7.0, 0.0), n2i(-56.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(0.0, 8.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) * n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) * n2i(8.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0) * n2i(-7.0, 8.0), n2i(-56.0, 64.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) * n2i(0.0, 8.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(-3.0, 7.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    // regular values
    assert_eq2!(n2i(-13.0, -9.0) * n2i(-4.0, -2.0), n2i(18.0, 52.0));
    assert_eq2!(n2i(-13.0, -0.8713145944805367) * n2i(-4.0, -0.04005481558786872), n2i(0.03490034540093651, 52.0));
    assert_eq2!(n2i(-0.8844699037058322, -0.5086643052155234) * n2i(-0.3468109146783054, -0.20027002557591056), n2i(0.10187021341506564, 0.30674381630965236));
    assert_eq2!(n2i(-55.0, -7.0) * n2i(-1.0, 34.0), n2i(-1870.0, 55.0));
    assert_eq2!(n2i(-0.8765250686386197, -0.19921875) * n2i(-1.0, 1.4667159003917116), n2i(-1.2856132552642, 0.8765250686386197));
    assert_eq2!(n2i(-28.708018725412018, -0.19921875) * n2i(-1.393992103870863, 1.0), n2i(-28.708018725412018, 40.018751421001234));
    assert_eq2!(n2i(-28.708018725412018, -0.19921875) * n2i(-1.393992103870863, 30.712800647822537), n2i(-881.7036561075358, 40.018751421001234));
    assert_eq2!(n2i(-78187493530.0, -1.0) * n2i(1.0, 16.0), n2i(-1250999896480.0, -1.0));
    assert_eq2!(n2i(-0.7139662043699291, -0.5) * n2i(2.0, 2.0466050234862188), n2i(-1.4612068204628852, -1.0));
    assert_eq2!(n2i(-4.0, -0.04018386156188278) * n2i(0.7388390272892537, 4.0), n2i(-16.0, -0.0296894051891075));
    assert_eq2!(n2i(-0.7139662043699291, -0.04018386156188278) * n2i(0.7388390272892537, 2.0466050234862188), n2i(-1.4612068204628852, -0.0296894051891075));
    assert_eq2!(n2i(-1.0, 17.0) * n2i(-7.0, -4.0), n2i(-119.0, 7.0));
    assert_eq2!(n2i(-1.0, 0.9244364494737423) * n2i(-2.23153891443975, -0.0625), n2i(-2.062915910927172, 2.23153891443975));
    assert_eq2!(n2i(-285.0258554165376, 1.0) * n2i(-2.23153891443975, -0.0625), n2i(-2.23153891443975, 636.0462879834815));
    assert_eq2!(n2i(-0.9244364494737423, 285.0258554165376) * n2i(-2.23153891443975, -0.0625), n2i(-636.0462879834815, 2.062915910927172));
    assert_eq2!(n2i(-1.0, 16.0) * n2i(-2.0, 3.0), n2i(-32.0, 48.0));
    assert_eq2!(n2i(-1.0, 0.17847548766219737) * n2i(-0.15136637593942875, 0.00390625), n2i(-0.027015187761449046, 0.15136637593942875));
    assert_eq2!(n2i(-1.0, 0.4301929436060967) * n2i(-0.0625, 1.5556627051688747), n2i(-1.5556627051688747, 0.6692351183948216));
    assert_eq2!(n2i(-1.0854564841375158, 0.00390625) * n2i(-2.468696855614667, 0.5137992619868292), n2i(-0.5577067404686741, 2.679663009296837));
    assert_eq2!(n2i(-7.0, 7.0) * n2i(19.0, 36.0), n2i(-252.0, 252.0));
    assert_eq2!(n2i(-0.6563586907006096, 16.0) * n2i(2.0, 2.201187625175618), n2i(-1.444768627646653, 35.21900200280989));
    assert_eq2!(n2i(-1.0, 1.5650983152000506) * n2i(1.1102230246251565e-16, 1.266329204122036), n2i(-1.266329204122036, 1.9819297038600197));
    assert_eq2!(n2i(-1.7658343015204414, 1.557480558189381) * n2i(2.0, 3.269798648808647), n2i(-5.773922613131501, 5.092647824713375));
    assert_eq2!(n2i(12.0, 45.0) * n2i(-1657.0, -229.0), n2i(-74565.0, -2748.0));
    assert_eq2!(n2i(12.0, 1120.8123652234376) * n2i(-2525625077842.175, -229.0), n2i(-2830751817163917.0, -2748.0));
    assert_eq2!(n2i(11.22244233241162, 45.0) * n2i(-1657.0, -10.279080856771557), n2i(-74565.0, -115.35639214531501));
    assert_eq2!(n2i(0.9395050761055534, 10.738140511300347) * n2i(-5.795963639761951, -0.009930631723906068), n2i(-62.23787196215163, -0.009329878913544592));
    assert_eq2!(n2i(1.0, 12.0) * n2i(-229.0, 1.0), n2i(-2748.0, 12.0));
    assert_eq2!(n2i(6.461498003318411e-14, 1.9224329615887497) * n2i(-0.6663688717732502, 4294967296.0), n2i(-1.2810494836736033, 8256786698.776104));
    assert_eq2!(n2i(3.0, 7.171544261033011) * n2i(-1.0, 0.6663688717732502), n2i(-7.171544261033011, 4.778893858096495));
    assert_eq2!(n2i(0.1875, 0.6663688717732502) * n2i(-1.9224329615887497, 7.171544261033011), n2i(-1.2810494836736033, 4.778893858096495));
    assert_eq2!(n2i(3.0, 7.0) * n2i(5.0, 11.0), n2i(15.0, 77.0));
    assert_eq2!(n2i(2.2821046232627444, 7.0) * n2i(3.444510241015958, 11.0), n2i(7.860732745898387, 77.0));
    assert_eq2!(n2i(3.0, 3.444510241015958) * n2i(1.4901161193847656e-07, 2.2821046232627444), n2i(4.470348358154297e-07, 7.860732745898388));
    assert_eq2!(n2i(0.19162812091730785, 2.2821046232627444) * n2i(0.7650281798863103, 2.6191053581458466), n2i(0.1466009125604018, 5.977072446636863));
}

#[test]
fn mpfi_mul_d() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) * n2i(-23.0, -23.0), n2i(161.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) * n2i(4e-17, 4e-17), n2i(f64::NEG_INFINITY, -2.8e-16));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) * n2i(-8e-17, -8e-17), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) * n2i(8e-17, 8e-17), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) * n2i(-1.6e+18, -1.6e+18), n2i(-1.28e+19, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) * n2i(1.6e+18, 1.6e+18), n2i(f64::NEG_INFINITY, 1.28e+19));
    assert_eq2!(I::ENTIRE * n2i(-1.6e-16, -1.6e-16), I::ENTIRE);
    assert_eq2!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(I::ENTIRE * n2i(1.6e-16, 1.6e-16), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) * n2i(-1e-17, -1e-17), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) * n2i(1e-17, 1e-17), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 7.0) * n2i(-3e-17, -3e-17), n2i(-2.1e-16, 0.0));
    assert_eq2!(n2i(0.0, 8.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 9.0) * n2i(4.8e-16, 4.8e-16), n2i(0.0, 4.320000000000001e-15));
    assert_eq2!(n2i(0.0, f64::INFINITY) * n2i(-7e-17, -7e-17), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) * n2i(6.999999999999999e-17, 6.999999999999999e-17), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-24211824.0, -2.823443157514334e-22) * n2i(-1.5, -1.5), n2i(4.2351647362715008e-22, 36317736.0));
    assert_eq2!(n2i(-3002399751580330.0, 1.2504195914452337e+128) * n2i(-1.5, -1.5), n2i(-1.8756293871678506e+128, 4503599627370495.0));
    assert_eq2!(n2i(4503599627370512.0, 1.122162325663649e+211) * n2i(-2.125, -2.125), n2i(-2.384594942035254e+211, -9570149208162338.0));
    assert_eq2!(n2i(-24211824.0, -2.823443157514334e-22) * n2i(1.5, 1.5), n2i(-36317736.0, -4.2351647362715008e-22));
    assert_eq2!(n2i(-3002399751580330.0, 1.2504195914452337e+128) * n2i(1.5, 1.5), n2i(-4503599627370495.0, 1.8756293871678506e+128));
    assert_eq2!(n2i(4503599627370512.0, 1.122162325663649e+211) * n2i(2.125, 2.125), n2i(9570149208162338.0, 2.384594942035254e+211));
    assert_eq2!(n2i(-1.6638238761041265e+18, -4503599627370497.0) * n2i(-1.5, -1.5), n2i(6755399441055745.0, 2.4957358141561897e+18));
    assert_eq2!(n2i(-3002399751580330.0, 4503599627370497.0) * n2i(-1.5, -1.5), n2i(-6755399441055746.0, 4503599627370495.0));
    assert_eq2!(n2i(4503599627370512.0, 4803839602528529.0) * n2i(-2.125, -2.125), n2i(-1.0208159155373126e+16, -9570149208162338.0));
    assert_eq2!(n2i(-4503599627370497.0, -2.823443157514334e-22) * n2i(1.5, 1.5), n2i(-6755399441055746.0, -4.2351647362715008e-22));
    assert_eq2!(n2i(-3002399751580331.0, 1.2504195914452337e+128) * n2i(1.5, 1.5), n2i(-4503599627370497.0, 1.8756293871678506e+128));
    assert_eq2!(n2i(4503599627370497.0, 1.122162325663649e+211) * n2i(2.125, 2.125), n2i(9570149208162306.0, 2.384594942035254e+211));
    assert_eq2!(n2i(-4909806652584305.0, -2.823443157514334e-22) * n2i(-1.5, -1.5), n2i(4.2351647362715008e-22, 7364709978876458.0));
    assert_eq2!(n2i(-4503599627370497.0, 1.2504195914452337e+128) * n2i(-1.5, -1.5), n2i(-1.8756293871678506e+128, 6755399441055746.0));
    assert_eq2!(n2i(4503599627370497.0, 1.122162325663649e+211) * n2i(-2.125, -2.125), n2i(-2.384594942035254e+211, -9570149208162306.0));
    assert_eq2!(n2i(-24211824.0, -7.058607893785836e-22) * n2i(1.5, 1.5), n2i(-36317736.0, -1.0587911840678752e-21));
    assert_eq2!(n2i(-3002399751580330.0, 4909806652584305.0) * n2i(1.5, 1.5), n2i(-4503599627370495.0, 7364709978876458.0));
    assert_eq2!(n2i(4503599627370512.0, 6905519428634761.0) * n2i(2.125, 2.125), n2i(9570149208162338.0, 1.4674228785848868e+16));
    assert_eq2!(n2i(-4909806652584305.0, -4503599627370497.0) * n2i(-1.5, -1.5), n2i(6755399441055745.0, 7364709978876458.0));
    assert_eq2!(n2i(-4503599627370497.0, 4503599627370497.0) * n2i(-1.5, -1.5), n2i(-6755399441055746.0, 6755399441055746.0));
    assert_eq2!(n2i(4503599627370497.0, 4803839602528529.0) * n2i(-2.125, -2.125), n2i(-1.0208159155373126e+16, -9570149208162306.0));
    assert_eq2!(n2i(-4503599627370497.0, -7.058607893785836e-22) * n2i(1.5, 1.5), n2i(-6755399441055746.0, -1.0587911840678752e-21));
    assert_eq2!(n2i(-3002399751580331.0, 4909806652584305.0) * n2i(1.5, 1.5), n2i(-4503599627370497.0, 7364709978876458.0));
    assert_eq2!(n2i(4503599627370497.0, 6905519428634761.0) * n2i(2.125, 2.125), n2i(9570149208162306.0, 1.4674228785848868e+16));
}

#[test]
fn mpfi_neg() {
    // special values
    assert_eq2!(-n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq2!(-n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(-n2i(f64::NEG_INFINITY, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq2!(-I::ENTIRE, I::ENTIRE);
    assert_eq2!(-n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(-n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq2!(-n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    // regular values
    assert_eq2!(-n2i(74565.4044342041, 74565.40467834473), n2i(-74565.40467834473, -74565.4044342041));
}

#[test]
fn mpfi_put_d() {
    // special values
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(-8.0, -8.0)), n2i(-8.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(5.0, 5.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 5.0));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_sin() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(I::ENTIRE.sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 0.0).sin(), n2i(-0.8414709848078966, 0.0));
    assert_eq2!(n2i(0.0, 0.0).sin(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).sin(), n2i(0.0, 0.8414709848078966));
    assert_eq2!(n2i(0.0, 8.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    // regular values
    assert_eq2!(n2i(0.125, 17.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.5707963267948966, 1.5707963267948968).sin(), n2i(0.9999999999999999, 1.0));
    assert_eq2!(n2i(-2.0, -0.5).sin(), n2i(-1.0, -0.47942553860420295));
    assert_eq2!(n2i(-4.5, 0.625).sin(), n2i(-1.0, 0.9775301176650971));
    assert_eq2!(n2i(-1.0, -0.25).sin(), n2i(-0.8414709848078966, -0.2474039592545229));
    assert_eq2!(n2i(-0.5, 0.5).sin(), n2i(-0.479425538604203, 0.479425538604203));
    assert_eq2!(n2i(8.538038601028319e+24, 8.538038601028319e+24).sin(), n2i(0.21772528523431073, 0.21772528523431076));
    assert_eq2!(n2i(-7.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-7.0, -2.0).sin(), n2i(-0.9092974268256817, 1.0));
    assert_eq2!(n2i(-7.0, -3.0).sin(), n2i(-0.6569865987187892, 1.0));
    assert_eq2!(n2i(-7.0, -4.0).sin(), n2i(-0.6569865987187892, 1.0));
    assert_eq2!(n2i(-7.0, -5.0).sin(), n2i(-0.6569865987187892, 0.9589242746631386));
    assert_eq2!(n2i(-7.0, -6.0).sin(), n2i(-0.6569865987187892, 0.2794154981989259));
    assert_eq2!(n2i(-7.0, -7.0).sin(), n2i(-0.6569865987187892, -0.6569865987187891));
    assert_eq2!(n2i(-6.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-6.0, -2.0).sin(), n2i(-0.9092974268256817, 1.0));
    assert_eq2!(n2i(-6.0, -3.0).sin(), n2i(-0.14112000805986724, 1.0));
    assert_eq2!(n2i(-6.0, -4.0).sin(), n2i(0.27941549819892586, 1.0));
    assert_eq2!(n2i(-6.0, -5.0).sin(), n2i(0.27941549819892586, 0.9589242746631386));
    assert_eq2!(n2i(-6.0, -6.0).sin(), n2i(0.27941549819892586, 0.2794154981989259));
    assert_eq2!(n2i(-5.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, -1.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-5.0, -2.0).sin(), n2i(-0.9092974268256817, 1.0));
    assert_eq2!(n2i(-5.0, -3.0).sin(), n2i(-0.14112000805986724, 1.0));
    assert_eq2!(n2i(-5.0, -4.0).sin(), n2i(0.7568024953079282, 1.0));
    assert_eq2!(n2i(-5.0, -5.0).sin(), n2i(0.9589242746631385, 0.9589242746631386));
    assert_eq2!(n2i(-4.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-4.0, 1.0).sin(), n2i(-1.0, 0.8414709848078966));
    assert_eq2!(n2i(-4.0, 0.0).sin(), n2i(-1.0, 0.7568024953079283));
    assert_eq2!(n2i(-4.0, -1.0).sin(), n2i(-1.0, 0.7568024953079283));
    assert_eq2!(n2i(-4.0, -2.0).sin(), n2i(-0.9092974268256817, 0.7568024953079283));
    assert_eq2!(n2i(-4.0, -3.0).sin(), n2i(-0.14112000805986724, 0.7568024953079283));
    assert_eq2!(n2i(-4.0, -4.0).sin(), n2i(0.7568024953079282, 0.7568024953079283));
    assert_eq2!(n2i(-3.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-3.0, 1.0).sin(), n2i(-1.0, 0.8414709848078966));
    assert_eq2!(n2i(-3.0, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-3.0, -1.0).sin(), n2i(-1.0, -0.1411200080598672));
    assert_eq2!(n2i(-3.0, -2.0).sin(), n2i(-0.9092974268256817, -0.1411200080598672));
    assert_eq2!(n2i(-3.0, -3.0).sin(), n2i(-0.14112000805986724, -0.1411200080598672));
    assert_eq2!(n2i(-2.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 4.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 3.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 2.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 1.0).sin(), n2i(-1.0, 0.8414709848078966));
    assert_eq2!(n2i(-2.0, 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(-2.0, -1.0).sin(), n2i(-1.0, -0.8414709848078965));
    assert_eq2!(n2i(-2.0, -2.0).sin(), n2i(-0.9092974268256817, -0.9092974268256816));
    assert_eq2!(n2i(-1.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 4.0).sin(), n2i(-0.8414709848078966, 1.0));
    assert_eq2!(n2i(-1.0, 3.0).sin(), n2i(-0.8414709848078966, 1.0));
    assert_eq2!(n2i(-1.0, 2.0).sin(), n2i(-0.8414709848078966, 1.0));
    assert_eq2!(n2i(-1.0, 1.0).sin(), n2i(-0.8414709848078966, 0.8414709848078966));
    assert_eq2!(n2i(-1.0, 0.0).sin(), n2i(-0.8414709848078966, 0.0));
    assert_eq2!(n2i(-1.0, -1.0).sin(), n2i(-0.8414709848078966, -0.8414709848078965));
    assert_eq2!(n2i(1.0, 7.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.0, 6.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.0, 5.0).sin(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.0, 4.0).sin(), n2i(-0.7568024953079283, 1.0));
    assert_eq2!(n2i(1.0, 3.0).sin(), n2i(0.1411200080598672, 1.0));
    assert_eq2!(n2i(1.0, 2.0).sin(), n2i(0.8414709848078965, 1.0));
    assert_eq2!(n2i(1.0, 1.0).sin(), n2i(0.8414709848078965, 0.8414709848078966));
    assert_eq2!(n2i(2.0, 7.0).sin(), n2i(-1.0, 0.9092974268256817));
    assert_eq2!(n2i(2.0, 6.0).sin(), n2i(-1.0, 0.9092974268256817));
    assert_eq2!(n2i(2.0, 5.0).sin(), n2i(-1.0, 0.9092974268256817));
    assert_eq2!(n2i(2.0, 4.0).sin(), n2i(-0.7568024953079283, 0.9092974268256817));
    assert_eq2!(n2i(2.0, 3.0).sin(), n2i(0.1411200080598672, 0.9092974268256817));
    assert_eq2!(n2i(2.0, 2.0).sin(), n2i(0.9092974268256816, 0.9092974268256817));
    assert_eq2!(n2i(3.0, 7.0).sin(), n2i(-1.0, 0.6569865987187892));
    assert_eq2!(n2i(3.0, 6.0).sin(), n2i(-1.0, 0.14112000805986724));
    assert_eq2!(n2i(3.0, 5.0).sin(), n2i(-1.0, 0.14112000805986724));
    assert_eq2!(n2i(3.0, 4.0).sin(), n2i(-0.7568024953079283, 0.14112000805986724));
    assert_eq2!(n2i(3.0, 3.0).sin(), n2i(0.1411200080598672, 0.14112000805986724));
    assert_eq2!(n2i(4.0, 7.0).sin(), n2i(-1.0, 0.6569865987187892));
    assert_eq2!(n2i(4.0, 6.0).sin(), n2i(-1.0, -0.27941549819892586));
    assert_eq2!(n2i(4.0, 5.0).sin(), n2i(-1.0, -0.7568024953079282));
    assert_eq2!(n2i(4.0, 4.0).sin(), n2i(-0.7568024953079283, -0.7568024953079282));
    assert_eq2!(n2i(5.0, 7.0).sin(), n2i(-0.9589242746631386, 0.6569865987187892));
    assert_eq2!(n2i(5.0, 6.0).sin(), n2i(-0.9589242746631386, -0.27941549819892586));
    assert_eq2!(n2i(5.0, 5.0).sin(), n2i(-0.9589242746631386, -0.9589242746631385));
    assert_eq2!(n2i(6.0, 7.0).sin(), n2i(-0.2794154981989259, 0.6569865987187892));
    assert_eq2!(n2i(6.0, 6.0).sin(), n2i(-0.2794154981989259, -0.27941549819892586));
    assert_eq2!(n2i(7.0, 7.0).sin(), n2i(0.6569865987187891, 0.6569865987187892));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_sinh() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).sinh(), n2i(f64::NEG_INFINITY, -548.3161232732465));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).sinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).sinh(), n2i(f64::NEG_INFINITY, 1490.4788257895502));
    assert_eq2!(I::ENTIRE.sinh(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 0.0).sinh(), n2i(-1.1752011936438016, 0.0));
    assert_eq2!(n2i(0.0, 0.0).sinh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).sinh(), n2i(0.0, 1.1752011936438016));
    assert_eq2!(n2i(0.0, 8.0).sinh(), n2i(0.0, 1490.4788257895502));
    assert_eq2!(n2i(0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-0.125, 0.0).sinh(), n2i(-0.12532577524111546, 0.0));
    assert_eq2!(n2i(0.0, 0.5000000000000001).sinh(), n2i(0.0, 0.5210953054937475));
    assert_eq2!(n2i(-4.5, -0.625).sinh(), n2i(-45.00301115199179, -0.666492264456616));
    assert_eq2!(n2i(1.0, 3.0).sinh(), n2i(1.1752011936438014, 10.017874927409903));
}

#[test]
fn mpfi_sqr() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).sqr(), n2i(49.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq2!(I::ENTIRE.sqr(), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).sqr(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 8.0).sqr(), n2i(0.0, 64.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).sqr(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(0.5242813527584076, 1009.8482818603516).sqr(), n2i(0.2748709368501858, 1019793.552376304));
    assert_eq2!(n2i(102.08016605583096, 1193046.0).sqr(), n2i(10420.360301986022, 1423358758116.0));
    assert_eq2!(n2i(-1.392367054308169, 1.0).sqr(), n2i(0.0, 1.9386860139228077));
    assert_eq2!(n2i(1.4180842806713163, 2.6884932176092184).sqr(), n2i(2.0109630270870844, 7.227995781130769));
}

#[test]
fn mpfi_sqrt() {
    // special values
    assert_eq2!(n2i(0.0, 0.0).sqrt(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 9.0).sqrt(), n2i(0.0, 3.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).sqrt(), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(43681.0, 1423358758116.0).sqrt(), n2i(209.0, 1193046.0));
    assert_eq2!(n2i(0.8929886572570982, 43681.0).sqrt(), n2i(0.9449807708398611, 209.0));
    assert_eq2!(n2i(0.6665191650390625, 1.0476770103673323).sqrt(), n2i(0.81640625, 1.0235609460932615));
    assert_eq2!(n2i(0.8929886572570982, 1.0476770103673323).sqrt(), n2i(0.9449807708398611, 1.0235609460932615));
}

#[test]
fn mpfi_sub() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) - n2i(-1.0, 8.0), n2i(f64::NEG_INFINITY, -6.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) - n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -8.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) - n2i(0.0, 8.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq2!(I::ENTIRE - n2i(0.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) - n2i(f64::NEG_INFINITY, -7.0), n2i(7.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 8.0) - n2i(-7.0, 0.0), n2i(0.0, 15.0));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, 8.0), n2i(-8.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) - n2i(0.0, 8.0), n2i(-8.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0) - n2i(8.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -8.0));
    assert_eq2!(n2i(0.0, 0.0) - I::ENTIRE, I::ENTIRE);
    assert_eq2!(n2i(0.0, 8.0) - n2i(-7.0, 8.0), n2i(-8.0, 15.0));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) - n2i(0.0, 8.0), n2i(-8.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-5.0, 59.0) - n2i(17.0, 81.0), n2i(-86.0, 42.0));
    assert_eq2!(n2i(-4.909093465297727e-91, 320255847038976.0) - n2i(-126462925.0, 4.547473508864641e-13), n2i(-4.547473508864642e-13, 320255973501901.0));
    assert_eq2!(n2i(-4.0, 7.0) - n2i(-3e+300, 2443359172.8355484), n2i(-2443359176.8355484, 3.000000000000001e+300));
    assert_eq2!(n2i(-7.205869356633318e+16, 1.152921504606847e+18) - n2i(-3e+300, 281479271743489.0), n2i(-7.234017283807669e+16, 3.000000000000001e+300));
    assert_eq2!(n2i(-5.0, 1.0) - n2i(1.0, 1.1805916207174113e+21), n2i(-1.1805916207174116e+21, 0.0));
    assert_eq2!(n2i(5.0, 1.1805916207174113e+21) - n2i(3.0, 5.0), n2i(0.0, 1.1805916207174113e+21));
}

#[test]
fn mpfi_sub_d() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) - n2i(-4e-17, -4e-17), n2i(f64::NEG_INFINITY, -6.999999999999999));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0) - n2i(4e-17, 4e-17), n2i(f64::NEG_INFINITY, -7.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) - n2i(-8e-17, -8e-17), n2i(f64::NEG_INFINITY, 8e-17));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0) - n2i(8e-17, 8e-17), n2i(f64::NEG_INFINITY, -8e-17));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) - n2i(-1.6e+18, -1.6e+18), n2i(f64::NEG_INFINITY, 1.6000000000000003e+18));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) - n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0) - n2i(1.6e+18, 1.6e+18), n2i(f64::NEG_INFINITY, -1.5999999999999997e+18));
    assert_eq2!(I::ENTIRE - n2i(-1.6e-16, -1.6e-16), I::ENTIRE);
    assert_eq2!(I::ENTIRE - n2i(0.0, 0.0), I::ENTIRE);
    assert_eq2!(I::ENTIRE - n2i(1.6e-16, 1.6e-16), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0) - n2i(-1e-17, -1e-17), n2i(1e-17, 1e-17));
    assert_eq2!(n2i(0.0, 0.0) - n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 0.0) - n2i(1e-17, 1e-17), n2i(-1e-17, -1e-17));
    assert_eq2!(n2i(0.0, 8.0) - n2i(-3e-17, -3e-17), n2i(3e-17, 8.000000000000002));
    assert_eq2!(n2i(0.0, 8.0) - n2i(0.0, 0.0), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, 8.0) - n2i(3e-17, 3e-17), n2i(-3e-17, 8.0));
    assert_eq2!(n2i(0.0, f64::INFINITY) - n2i(-7e-17, -7e-17), n2i(7e-17, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY) - n2i(0.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, f64::INFINITY) - n2i(6.999999999999999e-17, 6.999999999999999e-17), n2i(-6.999999999999999e-17, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(-32.0, -17.0) - n2i(31.41592653589793, 31.41592653589793), n2i(-63.41592653589793, -48.41592653589793));
    assert_eq2!(n2i(-31.41592653589793, -17.0) - n2i(-31.41592653589793, -31.41592653589793), n2i(0.0, 14.415926535897931));
    assert_eq2!(n2i(-32.0, -15.707963267948966) - n2i(-15.707963267948966, -15.707963267948966), n2i(-16.292036732051034, 0.0));
    assert_eq2!(n2i(18.204444444444444, 320255973501901.94) - n2i(-3.5, -3.5), n2i(21.704444444444444, 320255973501905.44));
    assert_eq2!(n2i(0.07111111111111111, 320255973501901.94) - n2i(-3.5, -3.5), n2i(3.5711111111111107, 320255973501905.44));
    assert_eq2!(n2i(-255.0, 1.1377777777777778) - n2i(-256.5, -256.5), n2i(1.5, 257.6377777777778));
    assert_eq2!(n2i(-1.9999999999999998, -2.7133285516175262e-166) - n2i(-4097.5, -4097.5), n2i(4095.5, 4097.5));
    assert_eq2!(n2i(18.204444444444444, 320255973501901.94) - n2i(3.5, 3.5), n2i(14.704444444444444, 320255973501898.44));
    assert_eq2!(n2i(0.07111111111111111, 320255973501901.94) - n2i(3.5, 3.5), n2i(-3.4288888888888893, 320255973501898.44));
    assert_eq2!(n2i(-255.0, 1.1377777777777778) - n2i(256.5, 256.5), n2i(-511.5, -255.36222222222221));
    assert_eq2!(n2i(-1.9999999999999998, -2.7133285516175262e-166) - n2i(4097.5, 4097.5), n2i(-4099.5, -4097.5));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_tan() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).tan(), I::ENTIRE);
    assert_eq2!(I::ENTIRE.tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 0.0).tan(), n2i(-1.5574077246549023, 0.0));
    assert_eq2!(n2i(0.0, 0.0).tan(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).tan(), n2i(0.0, 1.5574077246549023));
    assert_eq2!(n2i(0.0, 8.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(0.0, f64::INFINITY).tan(), I::ENTIRE);
    // regular values
    assert_eq2!(n2i(0.125, 17.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.5707963267948966, 1.5707963267948968).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, -0.5).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.5, 0.625).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, -0.25).tan(), n2i(-1.5574077246549023, -0.2553419212210362));
    assert_eq2!(n2i(-0.5, 0.5).tan(), n2i(-0.5463024898437906, 0.5463024898437906));
    assert_eq2!(n2i(8.538038601028319e+24, 8.538038601028319e+24).tan(), n2i(-0.22307687898274653, -0.2230768789827465));
    assert_eq2!(n2i(-7.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, -2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, -3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, -4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-7.0, -5.0).tan(), n2i(-0.8714479827243188, 3.380515006246586));
    assert_eq2!(n2i(-7.0, -6.0).tan(), n2i(-0.8714479827243188, 0.2910061913847492));
    assert_eq2!(n2i(-7.0, -7.0).tan(), n2i(-0.8714479827243188, -0.8714479827243187));
    assert_eq2!(n2i(-6.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, -2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, -3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, -4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-6.0, -5.0).tan(), n2i(0.29100619138474915, 3.380515006246586));
    assert_eq2!(n2i(-6.0, -6.0).tan(), n2i(0.29100619138474915, 0.2910061913847492));
    assert_eq2!(n2i(-5.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-5.0, -5.0).tan(), n2i(3.3805150062465854, 3.380515006246586));
    assert_eq2!(n2i(-4.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-4.0, -2.0).tan(), n2i(-1.1578212823495777, 2.1850398632615193));
    assert_eq2!(n2i(-4.0, -3.0).tan(), n2i(-1.1578212823495777, 0.14254654307427783));
    assert_eq2!(n2i(-4.0, -4.0).tan(), n2i(-1.1578212823495777, -1.1578212823495775));
    assert_eq2!(n2i(-3.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-3.0, -2.0).tan(), n2i(0.1425465430742778, 2.1850398632615193));
    assert_eq2!(n2i(-3.0, -3.0).tan(), n2i(0.1425465430742778, 0.14254654307427783));
    assert_eq2!(n2i(-2.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 0.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, -1.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-2.0, -2.0).tan(), n2i(2.185039863261519, 2.1850398632615193));
    assert_eq2!(n2i(-1.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 1.0).tan(), n2i(-1.5574077246549023, 1.5574077246549023));
    assert_eq2!(n2i(-1.0, 0.0).tan(), n2i(-1.5574077246549023, 0.0));
    assert_eq2!(n2i(-1.0, -1.0).tan(), n2i(-1.5574077246549023, -1.557407724654902));
    assert_eq2!(n2i(1.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 4.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 3.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 2.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(1.0, 1.0).tan(), n2i(1.557407724654902, 1.5574077246549023));
    assert_eq2!(n2i(2.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(2.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(2.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(2.0, 4.0).tan(), n2i(-2.1850398632615193, 1.1578212823495777));
    assert_eq2!(n2i(2.0, 3.0).tan(), n2i(-2.1850398632615193, -0.1425465430742778));
    assert_eq2!(n2i(2.0, 2.0).tan(), n2i(-2.1850398632615193, -2.185039863261519));
    assert_eq2!(n2i(3.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(3.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(3.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(3.0, 4.0).tan(), n2i(-0.14254654307427783, 1.1578212823495777));
    assert_eq2!(n2i(3.0, 3.0).tan(), n2i(-0.14254654307427783, -0.1425465430742778));
    assert_eq2!(n2i(4.0, 7.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(4.0, 6.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(4.0, 5.0).tan(), I::ENTIRE);
    assert_eq2!(n2i(4.0, 4.0).tan(), n2i(1.1578212823495775, 1.1578212823495777));
    assert_eq2!(n2i(5.0, 7.0).tan(), n2i(-3.380515006246586, 0.8714479827243188));
    assert_eq2!(n2i(5.0, 6.0).tan(), n2i(-3.380515006246586, -0.29100619138474915));
    assert_eq2!(n2i(5.0, 5.0).tan(), n2i(-3.380515006246586, -3.3805150062465854));
    assert_eq2!(n2i(6.0, 7.0).tan(), n2i(-0.2910061913847492, 0.8714479827243188));
    assert_eq2!(n2i(6.0, 6.0).tan(), n2i(-0.2910061913847492, -0.29100619138474915));
    assert_eq2!(n2i(7.0, 7.0).tan(), n2i(0.8714479827243187, 0.8714479827243188));
}

#[cfg(feature = "gmp")]
#[test]
fn mpfi_tanh() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).tanh(), n2i(-1.0, -0.9999983369439446));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).tanh(), n2i(-1.0, 0.999999774929676));
    assert_eq2!(I::ENTIRE.tanh(), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-1.0, 0.0).tanh(), n2i(-0.761594155955765, 0.0));
    assert_eq2!(n2i(0.0, 0.0).tanh(), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, 1.0).tanh(), n2i(0.0, 0.761594155955765));
    assert_eq2!(n2i(0.0, 8.0).tanh(), n2i(0.0, 0.999999774929676));
    assert_eq2!(n2i(0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    // regular values
    assert_eq2!(n2i(-0.125, 0.0).tanh(), n2i(-0.12435300177159621, 0.0));
    assert_eq2!(n2i(0.0, 0.5000000000000001).tanh(), n2i(0.0, 0.46211715726000985));
    assert_eq2!(n2i(-4.5, -0.625).tanh(), n2i(-0.9997532108480276, -0.5545997223493823));
    assert_eq2!(n2i(1.0, 3.0).tanh(), n2i(0.7615941559557649, 0.9950547536867305));
    assert_eq2!(n2i(17.0, 18.0).tanh(), n2i(0.9999999999999966, 0.9999999999999996));
}

#[test]
fn mpfi_union() {
    // special values
    assert_eq2!(n2i(f64::NEG_INFINITY, -7.0).convex_hull(n2i(-1.0, 8.0)), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq2!(n2i(f64::NEG_INFINITY, 0.0).convex_hull(n2i(8.0, f64::INFINITY)), I::ENTIRE);
    assert_eq2!(n2i(f64::NEG_INFINITY, 8.0).convex_hull(n2i(0.0, 8.0)), n2i(f64::NEG_INFINITY, 8.0));
    assert_eq2!(I::ENTIRE.convex_hull(n2i(0.0, 8.0)), I::ENTIRE);
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(f64::NEG_INFINITY, -7.0)), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq2!(n2i(0.0, 8.0).convex_hull(n2i(-7.0, 0.0)), n2i(-7.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 8.0)), n2i(0.0, 8.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).convex_hull(n2i(0.0, 8.0)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(8.0, f64::INFINITY)), n2i(0.0, f64::INFINITY));
    assert_eq2!(n2i(0.0, 0.0).convex_hull(I::ENTIRE), I::ENTIRE);
    assert_eq2!(n2i(0.0, 8.0).convex_hull(n2i(-7.0, 8.0)), n2i(-7.0, 8.0));
    assert_eq2!(n2i(0.0, 0.0).convex_hull(n2i(0.0, 0.0)), n2i(0.0, 0.0));
    assert_eq2!(n2i(0.0, f64::INFINITY).convex_hull(n2i(0.0, 8.0)), n2i(0.0, f64::INFINITY));
    // regular values
    assert_eq2!(n2i(18.0, 144.0).convex_hull(n2i(-13.0, 52.0)), n2i(-13.0, 144.0));
}
