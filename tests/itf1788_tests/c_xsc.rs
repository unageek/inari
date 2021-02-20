/*
 *
 * Unit tests from C-XSC version 2.5.4
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 1990-2000 Institut fuer Angewandte Mathematik,
 *                     Universitaet Karlsruhe, Germany
 * Copyright 2000-2014 Wiss. Rechnen/Softwaretechnologie
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

//Test library imports

//Arithmetic library imports

//Preamble
use crate::*;
use inari::{DecInterval as DI, Decoration as D, Interval as I, Overlap as O};

// Tests A+B, B+A, A-B, B-A, -A, +A
#[test]
fn cxsc_intervaladdsub() {
    assert_eq2!(n2i(10.0, 20.0) + n2i(13.0, 17.0), n2i(23.0, 37.0));
    assert_eq2!(n2i(13.0, 17.0) + n2i(10.0, 20.0), n2i(23.0, 37.0));
    assert_eq2!(n2i(10.0, 20.0) - n2i(13.0, 16.0), n2i(-6.0, 7.0));
    assert_eq2!(n2i(13.0, 16.0) - n2i(10.0, 20.0), n2i(-7.0, 6.0));
    assert_eq2!(-n2i(10.0, 20.0), n2i(-20.0, -10.0));
}

// Tests A*B, B*A, A/B, B/A
#[test]
fn cxsc_intervalmuldiv() {
    assert_eq2!(n2i(1.0, 2.0) * n2i(3.0, 4.0), n2i(3.0, 8.0));
    assert_eq2!(n2i(-1.0, 2.0) * n2i(3.0, 4.0), n2i(-4.0, 8.0));
    assert_eq2!(n2i(-2.0, 1.0) * n2i(3.0, 4.0), n2i(-8.0, 4.0));
    assert_eq2!(n2i(-2.0, -1.0) * n2i(3.0, 4.0), n2i(-8.0, -3.0));
    assert_eq2!(n2i(1.0, 2.0) * n2i(-3.0, 4.0), n2i(-6.0, 8.0));
    assert_eq2!(n2i(-1.0, 2.0) * n2i(-3.0, 4.0), n2i(-6.0, 8.0));
    assert_eq2!(n2i(-2.0, 1.0) * n2i(-3.0, 4.0), n2i(-8.0, 6.0));
    assert_eq2!(n2i(-2.0, -1.0) * n2i(-3.0, 4.0), n2i(-8.0, 6.0));
    assert_eq2!(n2i(1.0, 2.0) * n2i(-4.0, 3.0), n2i(-8.0, 6.0));
    assert_eq2!(n2i(-1.0, 2.0) * n2i(-4.0, 3.0), n2i(-8.0, 6.0));
    assert_eq2!(n2i(-2.0, 1.0) * n2i(-4.0, 3.0), n2i(-6.0, 8.0));
    assert_eq2!(n2i(-2.0, -1.0) * n2i(-4.0, 3.0), n2i(-6.0, 8.0));
    assert_eq2!(n2i(1.0, 2.0) * n2i(-4.0, -3.0), n2i(-8.0, -3.0));
    assert_eq2!(n2i(-1.0, 2.0) * n2i(-4.0, -3.0), n2i(-8.0, 4.0));
    assert_eq2!(n2i(-2.0, -1.0) * n2i(-4.0, -3.0), n2i(3.0, 8.0));
    assert_eq2!(n2i(1.0, 2.0) / n2i(4.0, 8.0), n2i(0.125, 0.5));
    assert_eq2!(n2i(-1.0, 2.0) / n2i(4.0, 8.0), n2i(-0.25, 0.5));
    assert_eq2!(n2i(-2.0, 1.0) / n2i(4.0, 8.0), n2i(-0.5, 0.25));
    assert_eq2!(n2i(-2.0, -1.0) / n2i(4.0, 8.0), n2i(-0.5, -0.125));
    assert_eq2!(n2i(1.0, 2.0) / n2i(-4.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 2.0) / n2i(-4.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.0) / n2i(-4.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(-2.0, -1.0) / n2i(-4.0, 8.0), I::ENTIRE);
    assert_eq2!(n2i(1.0, 2.0) / n2i(-8.0, 4.0), I::ENTIRE);
    assert_eq2!(n2i(-1.0, 2.0) / n2i(-8.0, 4.0), I::ENTIRE);
    assert_eq2!(n2i(-2.0, 1.0) / n2i(-8.0, 4.0), I::ENTIRE);
    assert_eq2!(n2i(-2.0, -1.0) / n2i(-8.0, 4.0), I::ENTIRE);
    assert_eq2!(n2i(1.0, 2.0) / n2i(-8.0, -4.0), n2i(-0.5, -0.125));
    assert_eq2!(n2i(-1.0, 2.0) / n2i(-8.0, -4.0), n2i(-0.5, 0.25));
    assert_eq2!(n2i(-2.0, 1.0) / n2i(-8.0, -4.0), n2i(-0.25, 0.5));
    assert_eq2!(n2i(-2.0, -1.0) / n2i(-8.0, -4.0), n2i(0.125, 0.5));
}

// Tests A|B, B|A, A&B, B&A
#[test]
fn cxsc_intervalsetops() {
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(-4.0, -3.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(-4.0, -1.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(-4.0, 4.0)), n2i(-4.0, 4.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(-1.0, 1.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(1.0, 4.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(3.0, 4.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(-4.0, -3.0).convex_hull(n2i(-2.0, 2.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(-4.0, -1.0).convex_hull(n2i(-2.0, 2.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(-4.0, 4.0).convex_hull(n2i(-2.0, 2.0)), n2i(-4.0, 4.0));
    assert_eq2!(n2i(-1.0, 1.0).convex_hull(n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(1.0, 4.0).convex_hull(n2i(-2.0, 2.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(3.0, 4.0).convex_hull(n2i(-2.0, 2.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(-4.0, -3.0)), I::EMPTY);
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(-4.0, -1.0)), n2i(-2.0, -1.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(-4.0, 4.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(-1.0, 1.0)), n2i(-1.0, 1.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(1.0, 4.0)), n2i(1.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(3.0, 4.0)), I::EMPTY);
    assert_eq2!(n2i(-4.0, -3.0).intersection(n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(-4.0, -1.0).intersection(n2i(-2.0, 2.0)), n2i(-2.0, -1.0));
    assert_eq2!(n2i(-4.0, 4.0).intersection(n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-1.0, 1.0).intersection(n2i(-2.0, 2.0)), n2i(-1.0, 1.0));
    assert_eq2!(n2i(1.0, 4.0).intersection(n2i(-2.0, 2.0)), n2i(1.0, 2.0));
    assert_eq2!(n2i(3.0, 4.0).intersection(n2i(-2.0, 2.0)), I::EMPTY);
}

// Tests A|B, B|A, A&B, B&A, B is scalar-type
#[test]
fn cxsc_intervalmixsetops() {
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(-4.0, -4.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(1.0, 1.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-2.0, 2.0).convex_hull(n2i(4.0, 4.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(-4.0, -4.0).convex_hull(n2i(-2.0, 2.0)), n2i(-4.0, 2.0));
    assert_eq2!(n2i(1.0, 1.0).convex_hull(n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(4.0, 4.0).convex_hull(n2i(-2.0, 2.0)), n2i(-2.0, 4.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(-4.0, -4.0)), I::EMPTY);
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(1.0, 1.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(-2.0, 2.0).intersection(n2i(4.0, 4.0)), I::EMPTY);
    assert_eq2!(n2i(-4.0, -4.0).intersection(n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq2!(n2i(1.0, 1.0).intersection(n2i(-2.0, 2.0)), n2i(1.0, 1.0));
    assert_eq2!(n2i(4.0, 4.0).intersection(n2i(-2.0, 2.0)), I::EMPTY);
}

// Tests A|B, B|A, A and B are scalar-type
#[test]
fn cxsc_scalarmixsetops() {
    assert_eq2!(n2i(-2.0, -2.0).convex_hull(n2i(-4.0, -4.0)), n2i(-4.0, -2.0));
    assert_eq2!(n2i(-2.0, -2.0).convex_hull(n2i(-2.0, -2.0)), n2i(-2.0, -2.0));
    assert_eq2!(n2i(-2.0, -2.0).convex_hull(n2i(2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq2!(n2i(-4.0, -4.0).convex_hull(n2i(-2.0, -2.0)), n2i(-4.0, -2.0));
    assert_eq2!(n2i(-2.0, -2.0).convex_hull(n2i(-2.0, -2.0)), n2i(-2.0, -2.0));
    assert_eq2!(n2i(2.0, 2.0).convex_hull(n2i(-2.0, -2.0)), n2i(-2.0, 2.0));
}

// Tests A<B, A>B, A<=B, A>=B, A==B
#[test]
fn cxsc_intervalsetcompops() {
    assert_eq2!(n2i(-1.0, 2.0).interior(n2i(-1.0, 2.0)), false);
    assert!(n2i(-2.0, 1.0).interior(n2i(-3.0, 2.0)));
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-1.0, 1.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-1.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-2.0, 1.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-2.0, 3.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-3.0, 2.0)), false);
    assert_eq2!(n2i(-1.0, 2.0).interior(n2i(-1.0, 2.0)), false);
    assert_eq2!(n2i(-3.0, 2.0).interior(n2i(-2.0, 1.0)), false);
    assert!(n2i(-1.0, 1.0).interior(n2i(-2.0, 2.0)));
    assert_eq2!(n2i(-1.0, 2.0).interior(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 1.0).interior(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 3.0).interior(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(-3.0, 2.0).interior(n2i(-2.0, 2.0)), false);
    assert!(n2i(-1.0, 2.0).subset(n2i(-1.0, 2.0)));
    assert!(n2i(-2.0, 1.0).subset(n2i(-3.0, 2.0)));
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(-1.0, 1.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(-1.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(-2.0, 1.0)), false);
    assert!(n2i(-2.0, 2.0).subset(n2i(-2.0, 3.0)));
    assert!(n2i(-2.0, 2.0).subset(n2i(-3.0, 2.0)));
    assert_eq2!(n2i(-3.0, 2.0).subset(n2i(-2.0, 1.0)), false);
    assert!(n2i(-1.0, 1.0).subset(n2i(-2.0, 2.0)));
    assert!(n2i(-1.0, 2.0).subset(n2i(-2.0, 2.0)));
    assert!(n2i(-2.0, 1.0).subset(n2i(-2.0, 2.0)));
    assert_eq2!(n2i(-2.0, 3.0).subset(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(-3.0, 2.0).subset(n2i(-2.0, 2.0)), false);
    assert!(n2i(-1.0, 2.0) == n2i(-1.0, 2.0));
    assert_eq2!(n2i(-2.0, 1.0) == n2i(-3.0, 2.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-1.0, 1.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-1.0, 2.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-2.0, 1.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-2.0, 3.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-3.0, 2.0), false);
}

// Tests A<B, A>B, A<=B, A>=B, A==B, B<A, B>A, B<=A, B>=A, B==A, where B is scalar
#[test]
fn cxsc_intervalscalarsetcompops() {
    assert_eq2!(n2i(-1.0, 2.0).interior(n2i(-2.0, -2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(-2.0, -2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(2.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).interior(n2i(3.0, 3.0)), false);
    assert_eq2!(n2i(-1.0, -1.0).interior(n2i(1.0, 1.0)), false);
    assert_eq2!(n2i(-1.0, -1.0).interior(n2i(-1.0, -1.0)), false);
    assert_eq2!(n2i(-2.0, -2.0).interior(n2i(-1.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, -2.0).interior(n2i(-2.0, 2.0)), false);
    assert!(n2i(0.0, 0.0).interior(n2i(-2.0, 2.0)));
    assert_eq2!(n2i(2.0, 2.0).interior(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(3.0, 3.0).interior(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(1.0, 1.0).interior(n2i(-1.0, -1.0)), false);
    assert_eq2!(n2i(-1.0, -1.0).interior(n2i(-1.0, -1.0)), false);
    assert_eq2!(n2i(-1.0, 2.0).subset(n2i(-2.0, -2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(-2.0, -2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(0.0, 0.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(2.0, 2.0)), false);
    assert_eq2!(n2i(-2.0, 2.0).subset(n2i(3.0, 3.0)), false);
    assert_eq2!(n2i(-1.0, -1.0).subset(n2i(1.0, 1.0)), false);
    assert!(n2i(-1.0, -1.0).subset(n2i(-1.0, -1.0)));
    assert_eq2!(n2i(-2.0, -2.0).subset(n2i(-1.0, 2.0)), false);
    assert!(n2i(-2.0, -2.0).subset(n2i(-2.0, 2.0)));
    assert!(n2i(0.0, 0.0).subset(n2i(-2.0, 2.0)));
    assert!(n2i(2.0, 2.0).subset(n2i(-2.0, 2.0)));
    assert_eq2!(n2i(3.0, 3.0).subset(n2i(-2.0, 2.0)), false);
    assert_eq2!(n2i(1.0, 1.0).subset(n2i(-1.0, -1.0)), false);
    assert!(n2i(-1.0, -1.0).subset(n2i(-1.0, -1.0)));
    assert_eq2!(n2i(-1.0, 2.0) == n2i(-2.0, -2.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(-2.0, -2.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(0.0, 0.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(2.0, 2.0), false);
    assert_eq2!(n2i(-2.0, 2.0) == n2i(3.0, 3.0), false);
    assert_eq2!(n2i(-1.0, -1.0) == n2i(1.0, 1.0), false);
    assert!(n2i(-1.0, -1.0) == n2i(-1.0, -1.0));
}

#[test]
fn cxsc_intervalstdfunc() {
    assert_eq2!(n2i(11.0, 11.0).sqr(), n2i(121.0, 121.0));
    assert_eq2!(n2i(0.0, 0.0).sqr(), n2i(0.0, 0.0));
    assert_eq2!(n2i(-9.0, -9.0).sqr(), n2i(81.0, 81.0));
    assert_eq2!(n2i(121.0, 121.0).sqrt(), n2i(11.0, 11.0));
    assert_eq2!(n2i(0.0, 0.0).sqrt(), n2i(0.0, 0.0));
    assert_eq2!(n2i(81.0, 81.0).sqrt(), n2i(9.0, 9.0));
}

#[cfg(feature = "gmp")]
#[test]
fn cxsc_intervalstdfunc_gmp() {
    assert_eq2!(n2i(2.0, 2.0).pow(n2i(2.0, 2.0)), n2i(4.0, 4.0));
    assert_eq2!(n2i(4.0, 4.0).pow(n2i(5.0, 5.0)), n2i(1024.0, 1024.0));
    // Negativ geht noch nicht
    assert_eq2!(n2i(2.0, 2.0).pow(n2i(3.0, 3.0)), n2i(8.0, 8.0));
}
