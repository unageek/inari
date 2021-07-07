// An interval extension of ln(x), log2(x) and log10(x). Not tightest, but simple.

#![allow(clippy::approx_constant)]

use inari::*;

// To compute ln(x), change the basis to 2 using the relation:
//
//   ln(x) = ln(2) lg(x).
//
// Now consider computing lg(x). Let D = [bl, bu] be such that:
//
//   bu / bl = √2,  1 - bl = bu - 1.
//
// Thus, D = [2 √2 - 2, 4 - 2 √2]. Decompose x as follows:
//
//   x = 2^a b,
//
// where 2a ∈ Z, b ∈ D. Therefore, lg(x) = a + lg(b).
// From the Taylor polynomial of lg(b) at b = 1 and its Lagrange remainder, we have:
//
//             N-1 (-1)^(n + 1)             (-1)^(N + 1)
//   lg(b) ∈ {  ∑  ------------ (b - 1)^n + ------------ (b - 1)^N β^-N | β ∈ D}, ∀b ∈ D.
//             n=0   n ln(2)                  N ln(2)
//
// From the following relations, N = 22 would be sufficient.
//
//         | (-1)^(n + 1)                |      1     √2 - 1
//    max  | ------------ (b - 1)^N β^-N | = ------- (------)^N,
//   b,β∈D |   N ln(2)                   |   N ln(2)    2
//
//      1     √2 - 1    |
//   ------- (------)^N |     ≤ 2^-53.
//   N ln(2)    2       |N=22
//
// References:
//
// - Oishi, S., & Kashiwagi M. (2018). 数学関数の精度保証. In S. Oishi (Ed.), 精度保証付き数値計算の基礎 (pp. 91-107). コロナ社.

fn log2_point_reduced(x: f64) -> Interval {
    let x = interval!(x, x).unwrap();
    assert!(x.subset(const_interval!(0.8284271247461901, 1.17157287525381)));

    const N: usize = 22;

    /// C[0] = 0,
    /// C[n] = (-1)^(n + 1) / (n ln(2)) for n = 1, …, N - 1.
    const C: [Interval; N] = [
        const_interval!(0.0, 0.0),
        const_interval!(1.4426950408889634, 1.4426950408889636),
        const_interval!(-0.7213475204444818, -0.7213475204444817),
        const_interval!(0.4808983469629878, 0.48089834696298783),
        const_interval!(-0.3606737602222409, -0.36067376022224085),
        const_interval!(0.28853900817779266, 0.2885390081777927),
        const_interval!(-0.24044917348149392, -0.2404491734814939),
        const_interval!(0.2060992915555662, 0.20609929155556622),
        const_interval!(-0.18033688011112045, -0.18033688011112042),
        const_interval!(0.16029944898766257, 0.1602994489876626),
        const_interval!(-0.14426950408889636, -0.14426950408889633),
        const_interval!(0.1311540946262694, 0.13115409462626942),
        const_interval!(-0.12022458674074696, -0.12022458674074694),
        const_interval!(0.11097654160684334, 0.11097654160684335),
        const_interval!(-0.10304964577778311, -0.1030496457777831),
        const_interval!(9.617966939259755e-2, 9.617966939259756e-2),
        const_interval!(-9.016844005556023e-2, -9.016844005556021e-2),
        const_interval!(8.486441416993902e-2, 8.486441416993903e-2),
        const_interval!(-8.01497244938313e-2, -8.014972449383129e-2),
        const_interval!(7.593131794152438e-2, 7.59313179415244e-2),
        const_interval!(-7.213475204444818e-2, -7.213475204444816e-2),
        const_interval!(6.869976385185539e-2, 6.86997638518554e-2),
    ];

    let xm1 = x - const_interval!(1.0, 1.0);
    // Initially, y = (-1)^(N + 1) / (N ln(2)) D^-N.
    let mut y = const_interval!(-4.122465510826449, -2.0129226127082265e-3);
    for n in (0..N).rev() {
        y = y.mul_add(xm1, C[n]);
    }
    y
}

enum Round {
    Down,
    Up,
}

fn log2_point(x: f64, rnd: Round) -> Interval {
    assert!(x > 0.0 && x < f64::INFINITY);

    let (mut b, a) = libm::frexp(x);
    let mut a = a as f64;

    // 0.5 ≤ b < 1.

    const BL_RD: f64 = 0.8284271247461901; // 𝔽⁻(bl)
    const BL2_RD: f64 = 0.5857864376269049; // 𝔽⁻(1/√2 bl)
    if b < BL2_RD {
        a -= 1.0;
        // ∀b ∈ 𝔽 : (0.5 ≤ b < 𝔽⁻(1/√2 bl) ⟹ 2b ≤ 𝔽⁺(bu)).
        b *= 2.0;
    } else if b < BL_RD {
        a -= 0.5;
        // ∀b ∈ 𝔽 : (0.5 ≤ b < 𝔽⁻(bl) ⟹ 𝔽⁺(𝔽⁺(√2) b) ≤ 𝔽⁺(bu)).
        b = {
            let b = Interval::SQRT_2 * interval!(b, b).unwrap();
            match rnd {
                Round::Down => b.inf(),
                Round::Up => b.sup(),
            }
        };
    }

    let a = interval!(a, a).unwrap();
    a + log2_point_reduced(b)
}

fn log2(x: Interval) -> Interval {
    if x.is_empty() {
        return x;
    }

    let inf = if x.inf() <= 0.0 {
        f64::NEG_INFINITY
    } else {
        log2_point(x.inf(), Round::Down).inf()
    };
    let sup = if x.sup() == f64::INFINITY {
        f64::INFINITY
    } else {
        log2_point(x.sup(), Round::Up).sup()
    };
    interval!(inf, sup).unwrap()
}

fn ln(x: Interval) -> Interval {
    Interval::LN_2 * log2(x)
}

fn log10(x: Interval) -> Interval {
    Interval::LOG10_2 * log2(x)
}

fn main() {
    let x = interval!("[1.234567]").unwrap();

    println!("log2(x) ⊆ {:.15e}", log2(x));
    println!("log2(x) ⊆ {:.15e} (MPFR)", x.log2());
    assert!(x.log2().subset(log2(x)));

    println!("ln(x) ⊆ {:.15e}", ln(x));
    println!("ln(x) ⊆ {:.15e} (MPFR)", x.ln());
    assert!(x.ln().subset(ln(x)));

    println!("log10(x) ⊆ {:.15e}", log10(x));
    println!("log10(x) ⊆ {:.15e} (MPFR)", x.log10());
    assert!(x.log10().subset(log10(x)));
}
