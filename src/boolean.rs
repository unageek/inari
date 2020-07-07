use crate::interval::*;
use core::arch::x86_64::*;

// NOTE: `eq` is implemented in interval.rs

impl Interval {
    pub fn disjoint(self, rhs: Self) -> bool {
        self.is_empty()
            || rhs.is_empty()
            || self.sup_raw() < rhs.inf_raw()
            || rhs.sup_raw() < self.inf_raw()
    }

    pub fn interior(self, rhs: Self) -> bool {
        let l = self.is_empty()
            || self.sup_raw() < rhs.sup_raw()
            || self.sup_raw() == f64::INFINITY && rhs.sup_raw() == f64::INFINITY;
        let r = self.is_empty()
            || rhs.inf_raw() < self.inf_raw()
            || rhs.inf_raw() == f64::NEG_INFINITY && self.inf_raw() == f64::NEG_INFINITY;
        l && r
    }

    pub fn is_common_interval(self) -> bool {
        unsafe { _mm_movemask_pd(_mm_cmplt_pd(self.rep, _mm_set1_pd(f64::INFINITY))) == 3 }
    }

    pub fn is_empty(self) -> bool {
        unsafe { _mm_movemask_pd(_mm_cmpunord_pd(self.rep, self.rep)) == 3 }
    }

    pub fn is_entire(self) -> bool {
        unsafe { _mm_movemask_pd(_mm_cmpeq_pd(self.rep, _mm_set1_pd(f64::INFINITY))) == 3 }
    }

    pub fn is_member(f: f64, rhs: Self) -> bool {
        f.is_finite() && rhs.inf_raw() <= f && f <= rhs.sup_raw()
    }

    pub fn is_singleton(self) -> bool {
        self.inf_raw() == self.sup_raw()
    }

    pub fn less(self, rhs: Self) -> bool {
        let l = self.is_empty() || self.sup_raw() <= rhs.sup_raw();
        let r = rhs.is_empty() || self.inf_raw() <= rhs.inf_raw();
        l && r
    }

    pub fn precedes(self, rhs: Self) -> bool {
        self.is_empty() || rhs.is_empty() || self.sup_raw() <= rhs.inf_raw()
    }

    pub fn strict_less(self, rhs: Self) -> bool {
        let l = self.is_empty()
            || self.sup_raw() < rhs.sup_raw()
            || self.sup_raw() == f64::INFINITY && rhs.sup_raw() == f64::INFINITY;
        let r = rhs.is_empty()
            || self.inf_raw() < rhs.inf_raw()
            || self.inf_raw() == f64::NEG_INFINITY && rhs.inf_raw() == f64::NEG_INFINITY;
        l && r
    }

    pub fn strict_precedes(self, rhs: Self) -> bool {
        self.is_empty() || rhs.is_empty() || self.sup_raw() < rhs.inf_raw()
    }

    pub fn subset(self, rhs: Self) -> bool {
        self.is_empty() || rhs.inf_raw() <= self.inf_raw() && self.sup_raw() <= rhs.sup_raw()
    }
}

macro_rules! impl_dec {
    ($f:ident, 1) => {
        pub fn $f(self) -> bool {
            if self.is_nai() {
                return false;
            }

            self.x.$f()
        }
    };

    ($f:ident, 2) => {
        pub fn $f(self, rhs: Self) -> bool {
            if self.is_nai() || rhs.is_nai() {
                return false;
            }

            self.x.$f(rhs.x)
        }
    };
}

impl DecoratedInterval {
    impl_dec!(disjoint, 2);
    impl_dec!(interior, 2);
    impl_dec!(is_common_interval, 1);
    impl_dec!(is_empty, 1);
    impl_dec!(is_entire, 1);

    pub fn is_member(f: f64, rhs: Self) -> bool {
        if rhs.is_nai() {
            return false;
        }

        Interval::is_member(f, rhs.x)
    }

    pub fn is_nai(self) -> bool {
        self.d == Decoration::Ill
    }

    impl_dec!(is_singleton, 1);
    impl_dec!(less, 2);
    impl_dec!(precedes, 2);
    impl_dec!(strict_less, 2);
    impl_dec!(strict_precedes, 2);
    impl_dec!(subset, 2);
}
