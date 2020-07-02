/*
 *
 * Unit tests from FI_LIB version 1.2
 * (Original authors: Werner Hofschuster and Walter Kraemer)
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 1997-2000 Institut fuer Wissenschaftliches Rechnen
 *                     und Mathematische Modellbildung (IWRMM)
 *                                      and
 *                     Institut fuer Angewandte Mathematik
 *                     Universitaet Karlsruhe, Germany
 * Copyright 2000-2005 Wiss. Rechnen/Softwaretechnologie
 *                     Universitaet Wuppertal, Germany
 * Copyright 2015-2016 Oliver Heimlich (oheim@posteo.de)
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

#[test]
fn fi_lib_addii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) + n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) + n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) + n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0000000000fd0p-1022"), hexf64!("0x0.0000000000fd0p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) + n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("-0x0.0000000000fd0p-1022"), hexf64!("-0x0.0000000000fd0p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.ffffffffffffep+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.ffffffffffffep+1023")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) + n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) + n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+2")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")) + n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")) + n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) + n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
}

#[test]
fn fi_lib_subii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) - n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) - n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) - n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) - n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000001p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.ffffffffffffep+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.ffffffffffffep+1023")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) - n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) - n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")) - n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")) - n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")), n2i(hexf64!("-0x1.4000000000000p+3"), hexf64!("-0x1.0000000000000p+3")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) - n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")), n2i(hexf64!("0x1.0000000000000p+3"), hexf64!("0x1.4000000000000p+3")));
}

#[test]
fn fi_lib_mulii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) * n2i(hexf64!("0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")), n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) * n2i(hexf64!("0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")), n2i(hexf64!("-0x1.fffffffffffffp+1022"), hexf64!("-0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("-0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("-0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.e000000000000p+3"), hexf64!("0x1.4000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+3")), n2i(hexf64!("-0x1.4000000000000p+5"), hexf64!("0x1.0000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("0x1.4000000000000p+2")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+4"), hexf64!("0x1.e000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+4"), hexf64!("0x1.0000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.8000000000000p+2"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
}

#[test]
fn fi_lib_divii() {
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.fffffffffffffp+1022"), hexf64!("-0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.5555555555555p-1"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("-0x1.5555555555555p-1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("-0x1.5555555555555p-1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x1.5555555555555p-1"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+0")));
}

#[test]
fn fi_lib_unary_functions() {
    assert_eq!(n2i(hexf64!("0x1.89694dd6d675ep-261"), hexf64!("0x1.5ba589837c966p+95")).sqrt(), n2i(hexf64!("0x1.c0ce46227e574p-131"), hexf64!("0x1.a5e5061f52645p+47")));
    assert_eq!(n2i(hexf64!("0x1.3109f10d8a44ap-515"), hexf64!("0x1.9981bc265797dp-222")).sqrt(), n2i(hexf64!("0x1.8b322d746dd92p-258"), hexf64!("0x1.43c7c708713a9p-111")));
    assert_eq!(n2i(hexf64!("0x1.21fe595436609p-237"), hexf64!("0x1.95f14b9ba7449p+236")).sqrt(), n2i(hexf64!("0x1.8153a5585df9bp-119"), hexf64!("0x1.425e465111a84p+118")));
    assert_eq!(n2i(hexf64!("0x1.1c55b51bf7d27p-489"), hexf64!("0x1.b3e59df05d8a3p-451")).sqrt(), n2i(hexf64!("0x1.7d8c77fc21799p-245"), hexf64!("0x1.d86b2273c5e01p-226")));
    assert_eq!(n2i(hexf64!("0x1.394270bbcba7ep+196"), hexf64!("0x1.092ede17e8b48p+227")).sqrt(), n2i(hexf64!("0x1.1b2facb63d9c6p+98"), hexf64!("0x1.7079918cd6185p+113")));
    assert_eq!(n2i(hexf64!("0x1.a800284a0e694p-42"), hexf64!("0x1.c36e2ca1ca0adp+277")).sqrt(), n2i(hexf64!("0x1.4975dcfe90520p-21"), hexf64!("0x1.e0c3235697cf6p+138")));
    assert_eq!(n2i(hexf64!("0x1.3c84e4f9c80cep-476"), hexf64!("0x1.23d287387686bp+199")).sqrt(), n2i(hexf64!("0x1.1ca7df0d1338cp-238"), hexf64!("0x1.828a33acd2ffep+99")));
    assert_eq!(n2i(hexf64!("0x1.083a5b6db26a9p-373"), hexf64!("0x1.76e7a53c8c81ap+141")).sqrt(), n2i(hexf64!("0x1.6fcf8a53263bdp-187"), hexf64!("0x1.b61f599c226f6p+70")));
    assert_eq!(n2i(hexf64!("0x1.2c18feebcaeaep-768"), hexf64!("0x1.c369e759df5e3p-328")).sqrt(), n2i(hexf64!("0x1.152c585eddb6ap-384"), hexf64!("0x1.53f1a81caa4a0p-164")));
    assert_eq!(n2i(hexf64!("0x1.3653a458674fcp-578"), hexf64!("0x1.4fb657b5aa89fp-131")).sqrt(), n2i(hexf64!("0x1.19db8426bc0bdp-289"), hexf64!("0x1.9e96fbc95e70dp-66")));
    assert_eq!(n2i(hexf64!("0x1.4e68960e85562p+117"), hexf64!("0x1.dfbf07296e0d5p+277")).sqrt(), n2i(hexf64!("0x1.9dc8b22366176p+58"), hexf64!("0x1.ef9c5cc91699ap+138")));
    assert_eq!(n2i(hexf64!("0x1.9b6f2e2aaca77p-99"), hexf64!("0x1.d0460177b1553p+204")).sqrt(), n2i(hexf64!("0x1.caf89228c2349p-50"), hexf64!("0x1.58c0892d42f3ep+102")));
    assert_eq!(n2i(hexf64!("0x1.2f500c5cdbdeap+6"), hexf64!("0x1.f4217679238f6p+595")).sqrt(), n2i(hexf64!("0x1.16a76c242f69fp+3"), hexf64!("0x1.fa07d2a534cf1p+297")));
    assert_eq!(n2i(hexf64!("0x1.4712c6d9e227cp+21"), hexf64!("0x1.8d276f38cd260p+629")).sqrt(), n2i(hexf64!("0x1.99387d9205e4cp+10"), hexf64!("0x1.c2ef801ba24d3p+314")));
    assert_eq!(n2i(hexf64!("0x1.19ca879c7c187p-175"), hexf64!("0x1.6f80e21866c06p-22")).sqrt(), n2i(hexf64!("0x1.7bd69462cdad2p-88"), hexf64!("0x1.32b9dfd924778p-11")));
    assert_eq!(n2i(hexf64!("0x1.92781ef99ab46p-559"), hexf64!("0x1.cce4ab046843dp+923")).sqrt(), n2i(hexf64!("0x1.c5f168118c2b1p-280"), hexf64!("0x1.e5c6656b17688p+461")));
    assert_eq!(n2i(hexf64!("0x1.6ab931cee2fd4p-467"), hexf64!("0x1.95cf42aa171cdp-160")).sqrt(), n2i(hexf64!("0x1.aef2280db324cp-234"), hexf64!("0x1.4250c275a7b2bp-80")));
    assert_eq!(n2i(hexf64!("0x1.1c1680a0c0b6cp-530"), hexf64!("0x1.72b9a0e0a4b86p+325")).sqrt(), n2i(hexf64!("0x1.0dadb347d442cp-265"), hexf64!("0x1.b3ac5fd5bc6d4p+162")));
    assert_eq!(n2i(hexf64!("0x1.36e874875bcc2p+146"), hexf64!("0x1.b33cc462bc32dp+197")).sqrt(), n2i(hexf64!("0x1.1a1f10af59132p+73"), hexf64!("0x1.d80f99fc38774p+98")));
    assert_eq!(n2i(hexf64!("0x1.27e9687937dd0p-326"), hexf64!("0x1.11b85141b78f6p-240")).sqrt(), n2i(hexf64!("0x1.133bbe271b45fp-163"), hexf64!("0x1.08b63617a4210p-120")));
    assert_eq!(n2i(hexf64!("0x1.419bd3b802b61p+363"), hexf64!("0x1.f33b09f0c558bp+527")).sqrt(), n2i(hexf64!("0x1.95c9a66e614f0p+181"), hexf64!("0x1.f99333020215ap+263")));
    assert_eq!(n2i(hexf64!("0x1.a211040e355aep-106"), hexf64!("0x1.9eb2e04c9c099p+137")).sqrt(), n2i(hexf64!("0x1.4725936450707p-53"), hexf64!("0x1.ccc9c68e6b873p+68")));
    assert_eq!(n2i(hexf64!("0x1.c5d872ac1544dp-354"), hexf64!("0x1.92efd09488689p-76")).sqrt(), n2i(hexf64!("0x1.54dbc003c350fp-177"), hexf64!("0x1.412c2d0ac617dp-38")));
    assert_eq!(n2i(hexf64!("0x1.88217909d04f2p-250"), hexf64!("0x1.719ba76049d42p-33")).sqrt(), n2i(hexf64!("0x1.3cd62fbb21703p-125"), hexf64!("0x1.b30435c147e51p-17")));
    assert_eq!(n2i(hexf64!("0x1.7ebe381f4672fp-501"), hexf64!("0x1.2493763542c73p+74")).sqrt(), n2i(hexf64!("0x1.baadbfcd8a3fep-251"), hexf64!("0x1.11ad7b2551299p+37")));
    assert_eq!(n2i(hexf64!("0x1.778fc6fd65a71p-213"), hexf64!("0x1.cfe27bb53debbp+192")).sqrt(), n2i(hexf64!("0x1.b6818c65b2cb8p-107"), hexf64!("0x1.589b93c7cc280p+96")));
    assert_eq!(n2i(hexf64!("0x1.e139dd116f868p-688"), hexf64!("0x1.a4a8a68dad1fap-145")).sqrt(), n2i(hexf64!("0x1.5efd65c23f515p-344"), hexf64!("0x1.d0166d0139468p-73")));
    assert_eq!(n2i(hexf64!("0x1.7064732544856p+193"), hexf64!("0x1.3544c696554d2p+261")).sqrt(), n2i(hexf64!("0x1.b24cebb3d4b84p+96"), hexf64!("0x1.8ded33641a18dp+130")));
    assert_eq!(n2i(hexf64!("0x1.671190bb5e2f2p-338"), hexf64!("0x1.18c6f488cbc30p+590")).sqrt(), n2i(hexf64!("0x1.2f2f89ad0042ep-169"), hexf64!("0x1.0c1a3e03e351bp+295")));
    assert_eq!(n2i(hexf64!("0x1.f80bd5021bfd1p-177"), hexf64!("0x1.40b109f7dabe9p-149")).sqrt(), n2i(hexf64!("0x1.fc01ee5cd905fp-89"), hexf64!("0x1.95356c567e5d8p-75")));
    assert_eq!(n2i(hexf64!("-0x1.4c8993b11d519p-149"), hexf64!("-0x1.b1d8f24f24de3p-941")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.aff52102ccd1ep-298")));
    assert_eq!(n2i(hexf64!("-0x1.9ee1a9db994f5p-436"), hexf64!("-0x1.b6451c0720bfbp-622")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.502f54272449ep-871")));
    assert_eq!(n2i(hexf64!("-0x1.59415fcfbff18p+6"), hexf64!("-0x1.1b0be7ac0af65p-959")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.d1a144efbeb44p+12")));
    assert_eq!(n2i(hexf64!("-0x1.975299ccb0e08p-372"), hexf64!("0x1.77d8fa6b68b60p-585")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.440bdfd7def8cp-743")));
    assert_eq!(n2i(hexf64!("-0x1.a8e9c46a3d769p-355"), hexf64!("-0x1.c3a9cd7025105p-564")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.60a3980b85d7cp-709")));
    assert_eq!(n2i(hexf64!("-0x1.9b66c64d19ee1p-329"), hexf64!("-0x1.0b8dbebdff270p-411")).sqr(), n2i(hexf64!("0x1.17a0fa5bfda4fp-822"), hexf64!("0x1.4a9194ff18274p-657")));
    assert_eq!(n2i(hexf64!("-0x1.32690aac2472dp-40"), hexf64!("-0x1.0e0dd7b9e7391p-789")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.6ebf489d48ca5p-80")));
    assert_eq!(n2i(hexf64!("-0x1.40516bc314cc9p-198"), hexf64!("-0x1.7ad0659980c2bp-203")).sqr(), n2i(hexf64!("0x1.18460ad749015p-405"), hexf64!("0x1.90cba74d12cf4p-396")));
    assert_eq!(n2i(hexf64!("0x1.e80ef8fd19ad4p-265"), hexf64!("0x1.ba9a1304c562dp-79")).sqr(), n2i(hexf64!("0x1.d13c8b128fdf5p-529"), hexf64!("0x1.7e9c3333ae604p-157")));
    assert_eq!(n2i(hexf64!("-0x1.764bf1b176ff7p-373"), hexf64!("-0x1.00bd1997cd82ep-915")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.11a0fe5d04eeep-745")));
    assert_eq!(n2i(hexf64!("-0x1.b28f9484ffbfcp-589"), hexf64!("0x1.c77c9276b791dp-13")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.953606a01baafp-25")));
    assert_eq!(n2i(hexf64!("-0x1.388c331648e3ep-333"), hexf64!("-0x1.65ed85df2d4b7p-576")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.7d96094e3c3d1p-666")));
    assert_eq!(n2i(hexf64!("-0x1.96433e013eda2p-935"), hexf64!("0x1.8620faa09eadbp-386")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.29443ff0807f8p-771")));
    assert_eq!(n2i(hexf64!("-0x1.ab077c8e23ef5p-491"), hexf64!("0x1.504d993745eafp-18")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.b9cbc9b69e7bfp-36")));
    assert_eq!(n2i(hexf64!("-0x1.06d7f9ae94dadp-301"), hexf64!("-0x1.9610758986a88p-835")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0ddec946af730p-602")));
    assert_eq!(n2i(hexf64!("-0x1.af3b21180e563p-498"), hexf64!("-0x1.068b13da99666p-760")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.6b34138ba2d21p-995")));
    assert_eq!(n2i(hexf64!("-0x1.2789c2d583bcdp-568"), hexf64!("-0x1.f2bd89dad0665p-780")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.9fc9d1b0afc7cp-545"), hexf64!("0x1.580844b9dc45cp-780")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8a11a53596037p-49"), hexf64!("0x1.b1e6b793078ddp-664")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2f4d28e026bf5p-97")));
    assert_eq!(n2i(hexf64!("-0x1.425eef071014fp-121"), hexf64!("-0x1.bb2efb4f70837p-547")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.95f2f4822cd2bp-242")));
    assert_eq!(n2i(hexf64!("-0x1.8e96354bf7e11p-894"), hexf64!("-0x1.039e2518cf503p-1008")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.7ba62e3fbdd83p-165"), hexf64!("0x1.069e434ee9e0fp-740")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.1982bc685f91bp-329")));
    assert_eq!(n2i(hexf64!("-0x1.e7802992ba99dp-775"), hexf64!("-0x1.7883a587654e5p-928")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("0x1.455801d3d2b63p-704"), hexf64!("0x1.d2648abc1e83dp-27")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.a8d91848c69f9p-53")));
    assert_eq!(n2i(hexf64!("-0x1.04be837a6f1f1p-375"), hexf64!("0x1.173ab0fec92afp-771")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.099388d81efaap-750")));
    assert_eq!(n2i(hexf64!("-0x1.7e13dbb66e5a3p-84"), hexf64!("-0x1.af23d175aa3d2p-538")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.1d1fa29f657dap-167")));
    assert_eq!(n2i(hexf64!("-0x1.07d4317cb3695p-274"), hexf64!("-0x1.ef8b7bcbab211p-495")).sqr(), n2i(hexf64!("0x1.df9e5a5ea5698p-989"), hexf64!("0x1.0fe5ad9038bc9p-548")));
    assert_eq!(n2i(hexf64!("-0x1.fb31317bb132bp-326"), hexf64!("-0x1.70170edbd047bp-875")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.f66df1b9f564ap-651")));
    assert_eq!(n2i(hexf64!("-0x1.8378f49913a88p-253"), hexf64!("0x1.53fab12968e9ap-607")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.253b765685531p-505")));
    assert_eq!(n2i(hexf64!("-0x1.79ca1af65e50dp-233"), hexf64!("-0x1.cd61131067370p-429")).sqr(), n2i(hexf64!("0x1.9fc361bc43cadp-857"), hexf64!("0x1.16c2717c18b6fp-465")));
}
