// An interval extension of exp(x), exp2(x) and exp10(x). Not tightest, but simple.

#![allow(clippy::approx_constant)]

use inari::*;

// To compute exp(x), change the base to 2 using the relation:
//
//   exp(x) = 2^(lg(e) x).
//
// Now consider computing 2^x. Let D = [-1/2, 1/2].
// Decompose x into an integer and a fractional parts:
//
//   x = a + b,
//
// where a ∈ ℤ, b ∈ D. Therefore, 2^x = 2^a 2^b.
// From the Taylor polynomial of 2^b at b = 0 and its Lagrange remainder, we have:
//
//           N-1 ln(2)^n       ln(2)^N
//   2^b ∈ {  ∑  ------- b^n + ------- b^N 2^β ∣ β ∈ D}, ∀b ∈ D.
//           n=0   n!            N!
//
// From the following relations, N = 14 would be sufficient:
//
//         | ln(2)^N         |   ln(2)^N
//    max  | ------- b^N 2^β | = -------- (1/2)^N √2,
//   b,β∈D |   N!            |     N!
//
//   ln(2)^N            |
//   ------- (1/2)^N √2 |     ≤ 2^-53.
//     N!               |N=14
//
// References:
//
// - Oishi, S., & Kashiwagi M. (2018). 数学関数の精度保証. In S. Oishi (Ed.), 精度保証付き数値計算の基礎 (pp. 91-107). コロナ社.

fn exp2_point_reduced(x: f64) -> Interval {
    let x = interval!(x, x).unwrap();
    assert!(x.subset(const_interval!(-0.5, 0.5)));

    const N: usize = 14;
    // C[n] = ln(2)^n / n! for n = 0, …, N - 1.
    const C: [Interval; N] = [
        const_interval!(1.0, 1.0),
        const_interval!(0.6931471805599453, 0.6931471805599454),
        const_interval!(0.2402265069591007, 0.24022650695910072),
        const_interval!(5.5504108664821576e-2, 5.550410866482158e-2),
        const_interval!(9.618129107628477e-3, 9.618129107628479e-3),
        const_interval!(1.3333558146428443e-3, 1.3333558146428445e-3),
        const_interval!(1.540353039338161e-4, 1.5403530393381611e-4),
        const_interval!(1.525273380405984e-5, 1.5252733804059841e-5),
        const_interval!(1.3215486790144307e-6, 1.321548679014431e-6),
        const_interval!(1.0178086009239699e-7, 1.01780860092397e-7),
        const_interval!(7.0549116208011225e-9, 7.054911620801123e-9),
        const_interval!(4.445538271870811e-10, 4.4455382718708116e-10),
        const_interval!(2.5678435993488202e-11, 2.5678435993488206e-11),
        const_interval!(1.3691488853904128e-12, 1.369148885390413e-12),
    ];

    // Initially, y = 2^D ln(2)^N / N!.
    let mut y = const_interval!(4.7932833733029885e-14, 9.586566746605978e-14);
    for n in (0..N).rev() {
        y = y.mul_add(x, C[n]);
    }
    y
}

fn exp2_point(x: f64) -> Interval {
    assert!(x.is_finite());

    let mut a = x.floor();
    let mut b = x - a;
    if b > 0.5 {
        a += 1.0;
        b -= 1.0;
    }

    let exp2_a = if a < -1074.0 {
        const_interval!(0.0, 5e-324)
    } else if a > 1023.0 {
        const_interval!(f64::MAX, f64::INFINITY)
    } else {
        let exp2_a = libm::ldexp(1.0, a as i32);
        interval!(exp2_a, exp2_a).unwrap()
    };

    let exp2_b = exp2_point_reduced(b);

    exp2_a * exp2_b
}

fn exp2(x: Interval) -> Interval {
    if x.is_empty() {
        return x;
    }

    let inf = if x.inf() == f64::NEG_INFINITY {
        0.0
    } else {
        exp2_point(x.inf()).inf()
    };
    let sup = if x.sup() == f64::INFINITY {
        f64::INFINITY
    } else {
        exp2_point(x.sup()).sup()
    };
    interval!(inf, sup).unwrap()
}

fn exp(x: Interval) -> Interval {
    exp2(Interval::LOG2_E * x)
}

fn exp10(x: Interval) -> Interval {
    exp2(Interval::LOG2_10 * x)
}

fn main() {
    let x = interval!("[1.234567]").unwrap();

    println!("2^x ⊆ {:.15e}", exp2(x));
    println!("2^x ⊆ {:.15e} (MPFR)", x.exp2());
    assert!(x.exp2().subset(exp2(x)));

    println!("e^x ⊆ {:.15e}", exp(x));
    println!("e^x ⊆ {:.15e} (MPFR)", x.exp());
    assert!(x.exp().subset(exp(x)));

    println!("10^x ⊆ {:.15e}", x.exp10());
    println!("10^x ⊆ {:.15e} (MPFR)", exp10(x));
    assert!(x.exp10().subset(exp10(x)));
}
