use bitflags::*;
use core::ops::{Add, Mul, Neg, Sub};
use inari::{interval, DecoratedInterval, Decoration, Interval};
use smallvec::SmallVec;
use std::{cmp::Ordering, convert::From};

// Represents a partial function {0, ..., 31} -> {0, 1}
// which domain is the set of branch cut site ids
// and range is the set of branch indices.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct IntervalBranch {
    // A bit field that keeps track of at which sites
    // branch cut has been performed during the derivation of the interval.
    cut: u32,
    // A bit field that records the chosen branch (0 or 1)
    // at each site, when the corresponding bit of `cut` is set.
    chosen: u32,
}

impl IntervalBranch {
    fn new() -> Self {
        Self { cut: 0, chosen: 0 }
    }

    fn inserted(self, site: u8, branch: u8) -> Self {
        assert!(site <= 31 && branch <= 1 && self.cut & 1 << site == 0);
        Self {
            cut: self.cut | 1 << site,
            chosen: self.chosen | (branch as u32) << site,
        }
    }

    fn union(self, rhs: Self) -> Option<Self> {
        // Tests if Graph(self) ∪ Graph(rhs) is a valid graph of a partial function.
        let mask = self.cut & rhs.cut;
        let valid = (self.chosen & mask) == (rhs.chosen & mask);

        if valid {
            Some(Self {
                cut: self.cut | rhs.cut,
                chosen: self.chosen | rhs.chosen,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct TupperInterval {
    x: Interval,
    d: Decoration,
    g: IntervalBranch,
}

impl TupperInterval {
    fn new(x: DecoratedInterval, g: IntervalBranch) -> Self {
        let mut x = unsafe { std::mem::transmute::<DecoratedInterval, TupperInterval>(x) };
        x.g = g;
        x
    }

    fn base(self) -> DecoratedInterval {
        unsafe { std::mem::transmute(self) }
    }
}

type TupperIntervalVec = SmallVec<[TupperInterval; 2]>;

#[derive(Debug)]
pub struct TupperIntervalSet(TupperIntervalVec);

impl TupperIntervalSet {
    fn new() -> Self {
        Self(TupperIntervalVec::new())
    }

    fn insert(&mut self, x: TupperInterval) {
        if !x.base().is_empty() {
            self.0.push(x);
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn normalize(self) -> Self {
        // TODO: Merge overlapping intervals.
        self
    }
}

impl From<DecoratedInterval> for TupperIntervalSet {
    fn from(x: DecoratedInterval) -> Self {
        let mut xs = Self::new();
        xs.insert(TupperInterval::new(x, IntervalBranch::new()));
        xs
    }
}

impl Neg for &TupperIntervalSet {
    type Output = TupperIntervalSet;

    fn neg(self) -> Self::Output {
        let mut rs = Self::Output::new();
        for x in &self.0 {
            rs.insert(TupperInterval::new(-x.base(), x.g));
        }
        rs.normalize()
    }
}

macro_rules! impl_arith_op {
    ($Op:ident, $op:ident) => {
        impl<'a, 'b> $Op<&'b TupperIntervalSet> for &'a TupperIntervalSet {
            type Output = TupperIntervalSet;

            fn $op(self, rhs: &'b TupperIntervalSet) -> Self::Output {
                let mut rs = Self::Output::new();
                for x in &self.0 {
                    for y in &rhs.0 {
                        if let Some(g) = x.g.union(y.g) {
                            rs.insert(TupperInterval::new(x.base().$op(y.base()), g));
                        }
                    }
                }
                rs.normalize()
            }
        }
    };
}

impl_arith_op!(Add, add);
impl_arith_op!(Sub, sub);
impl_arith_op!(Mul, mul);

macro_rules! impl_no_cut_op {
    ($op:ident) => {
        pub fn $op(&self) -> Self {
            let mut rs = Self::new();
            for x in &self.0 {
                rs.insert(TupperInterval::new(x.base().$op(), x.g));
            }
            rs.normalize()
        }
    };
}

macro_rules! impl_no_cut_op2 {
    ($op:ident) => {
        pub fn $op(&self, rhs: &Self) -> Self {
            let mut rs = Self::new();
            for x in &self.0 {
                for y in &rhs.0 {
                    if let Some(g) = x.g.union(y.g) {
                        rs.insert(TupperInterval::new(x.base().$op(y.base()), g));
                    }
                }
            }
            rs.normalize()
        }
    };
}

macro_rules! impl_integer_op {
    ($op:ident) => {
        pub fn $op(&self, site: Option<u8>) -> Self {
            let mut rs = Self::new();
            for x in &self.0 {
                if x.base().is_empty() {
                    continue;
                }
                let y = TupperInterval::new(x.base().$op(), x.g);
                let a = y.base().inf();
                let b = y.base().sup();
                if b - a == 1.0 {
                    rs.insert(TupperInterval::new(
                        DecoratedInterval::set_dec(interval!(a, a).unwrap(), y.d),
                        match site {
                            Some(site) => y.g.inserted(site, 0),
                            _ => y.g,
                        },
                    ));
                    rs.insert(TupperInterval::new(
                        DecoratedInterval::set_dec(interval!(b, b).unwrap(), y.d),
                        match site {
                            Some(site) => y.g.inserted(site, 1),
                            _ => y.g,
                        },
                    ));
                } else {
                    rs.insert(y);
                }
            }
            rs.normalize()
        }
    };
}

impl TupperIntervalSet {
    pub fn atan2(&self, rhs: &Self, site: Option<u8>) -> Self {
        let mut rs = Self::new();
        for y in &self.0 {
            for x in &rhs.0 {
                if let Some(g) = x.g.union(y.g) {
                    let a = x.base().inf();
                    let b = x.base().sup();
                    let c = y.base().inf();
                    let d = y.base().sup();
                    if a == 0.0 && b == 0.0 && c < 0.0 && d > 0.0 {
                        rs.insert(TupperInterval::new(
                            DecoratedInterval::set_dec(-Interval::FRAC_PI_2, Decoration::Trv),
                            match site {
                                Some(site) => g.inserted(site, 0),
                                _ => g,
                            },
                        ));
                        rs.insert(TupperInterval::new(
                            DecoratedInterval::set_dec(Interval::FRAC_PI_2, Decoration::Trv),
                            match site {
                                Some(site) => g.inserted(site, 1),
                                _ => g,
                            },
                        ));
                    } else if b == 0.0 && c < 0.0 && d >= 0.0 {
                        let x0 = interval!(b, b).unwrap();
                        let y0 = interval!(c, c).unwrap();
                        rs.insert(TupperInterval::new(
                            DecoratedInterval::set_dec(
                                interval!(-Interval::PI.sup(), y0.atan2(x0).sup()).unwrap(),
                                Decoration::Trv,
                            ),
                            match site {
                                Some(site) => g.inserted(site, 0),
                                _ => g,
                            },
                        ));
                        if d == 0.0 {
                            rs.insert(TupperInterval::new(
                                DecoratedInterval::set_dec(Interval::PI, Decoration::Trv),
                                match site {
                                    Some(site) => g.inserted(site, 1),
                                    _ => g,
                                },
                            ));
                        } else {
                            let x1 = interval!(b, b).unwrap();
                            let y1 = interval!(d, d).unwrap();
                            rs.insert(TupperInterval::new(
                                DecoratedInterval::set_dec(
                                    interval!(y1.atan2(x1).inf(), Interval::PI.sup()).unwrap(),
                                    Decoration::Trv,
                                ),
                                match site {
                                    Some(site) => g.inserted(site, 1),
                                    _ => g,
                                },
                            ));
                        }
                    } else if b < 0.0 && c < 0.0 && d >= 0.0 {
                        let x0 = interval!(b, b).unwrap();
                        let y0 = interval!(c, c).unwrap();
                        rs.insert(TupperInterval::new(
                            DecoratedInterval::set_dec(
                                interval!(-Interval::PI.sup(), y0.atan2(x0).sup()).unwrap(),
                                Decoration::Def,
                            ),
                            match site {
                                Some(site) => g.inserted(site, 0),
                                _ => g,
                            },
                        ));
                        let x1 = interval!(b, b).unwrap();
                        let y1 = interval!(d, d).unwrap();
                        rs.insert(TupperInterval::new(
                            DecoratedInterval::set_dec(
                                interval!(y1.atan2(x1).inf(), Interval::PI.sup()).unwrap(),
                                Decoration::Def,
                            ),
                            match site {
                                Some(site) => g.inserted(site, 1),
                                _ => g,
                            },
                        ));
                    } else {
                        rs.insert(TupperInterval::new(y.base().atan2(x.base()), g));
                    }
                }
            }
        }
        rs.normalize()
    }

    pub fn div(&self, rhs: &Self, site: Option<u8>) -> Self {
        let mut rs = Self::new();
        for x in &self.0 {
            for y in &rhs.0 {
                if let Some(g) = x.g.union(y.g) {
                    if y.base().inf() <= 0.0 && y.base().sup() >= 0.0 {
                        let y0 = DecoratedInterval::set_dec(
                            interval!(y.base().inf(), 0.0).unwrap(),
                            y.d,
                        );
                        rs.insert(TupperInterval::new(
                            x.base() / y0,
                            match site {
                                Some(site) => g.inserted(site, 0),
                                _ => g,
                            },
                        ));
                        let y1 = DecoratedInterval::set_dec(
                            interval!(0.0, y.base().sup()).unwrap(),
                            y.d,
                        );
                        rs.insert(TupperInterval::new(
                            x.base() / y1,
                            match site {
                                Some(site) => g.inserted(site, 1),
                                _ => g,
                            },
                        ));
                    } else {
                        rs.insert(TupperInterval::new(x.base() / y.base(), g));
                    }
                }
            }
        }
        rs.normalize()
    }

    // absmax
    impl_no_cut_op!(abs);
    impl_no_cut_op2!(max);
    impl_no_cut_op2!(min);

    // basic
    impl_no_cut_op!(sqr);
    impl_no_cut_op!(sqrt);

    // elementary
    impl_no_cut_op!(acos);
    impl_no_cut_op!(acosh);
    impl_no_cut_op!(asin);
    impl_no_cut_op!(asinh);
    impl_no_cut_op!(atan);
    impl_no_cut_op!(atanh);
    impl_no_cut_op!(cos);
    impl_no_cut_op!(cosh);
    impl_no_cut_op!(exp);
    impl_no_cut_op!(exp10);
    impl_no_cut_op!(exp2);
    impl_no_cut_op!(log);
    impl_no_cut_op!(log10);
    impl_no_cut_op!(log2);
    impl_no_cut_op!(sin);
    impl_no_cut_op!(sinh);
    impl_no_cut_op!(tanh);

    // integer
    impl_integer_op!(ceil);
    impl_integer_op!(floor);
    impl_integer_op!(round_ties_to_away);
    impl_integer_op!(round_ties_to_even);
    impl_integer_op!(sign);
    impl_integer_op!(trunc);
}

// TODO: mul_add?, recip?, tan
// TODO: add_sub?

bitflags! {
    pub struct SignSet: u8 {
        const NONE = 0;
        const NEG = 1;
        const ZERO = 2;
        const POS = 4;
    }
}

pub struct EvaluationResult(pub SignSet, pub Decoration);

macro_rules! impl_rel_op {
    ($op:ident, $map_neg:expr, $map_zero:expr, $map_pos:expr) => {
        pub fn $op(&self, rhs: &Self) -> EvaluationResult {
            let xs = self - rhs;
            if xs.is_empty() {
                return EvaluationResult(SignSet::NONE, Decoration::Trv);
            }

            let mut ss = SignSet::NONE;
            let mut d = Decoration::Com;
            for x in xs.0.iter() {
                let a = x.x.inf();
                let b = x.x.sup();
                if a < 0.0 {
                    ss |= $map_neg;
                }
                if a <= 0.0 && b >= 0.0 {
                    ss |= $map_zero;
                }
                if b > 0.0 {
                    ss |= $map_pos;
                }
                d = d.min(x.d);
            }

            EvaluationResult(ss, d)
        }
    };
}

impl TupperIntervalSet {
    impl_rel_op!(eq, SignSet::NEG, SignSet::ZERO, SignSet::POS);
    impl_rel_op!(ge, SignSet::NEG, SignSet::ZERO, SignSet::ZERO);
    impl_rel_op!(gt, SignSet::NEG, SignSet::NEG, SignSet::ZERO);
    impl_rel_op!(le, SignSet::ZERO, SignSet::ZERO, SignSet::NEG);
    impl_rel_op!(lt, SignSet::ZERO, SignSet::NEG, SignSet::NEG);
}
