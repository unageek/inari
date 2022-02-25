use inari::*;

#[allow(
    unused_attributes,
    unused_imports,
    clippy::approx_constant,
    clippy::eq_op,
    clippy::excessive_precision,
    clippy::float_cmp
)]
mod itf1788_tests {
    //mod abs_rev;
    mod atan2;
    mod c_xsc;
    mod fi_lib;
    mod ieee1788_constructors;
    mod ieee1788_exceptions;
    mod libieeep1788_bool;
    //mod libieeep1788_cancel;
    mod libieeep1788_class;
    mod libieeep1788_elem;
    //mod libieeep1788_mul_rev;
    mod libieeep1788_num;
    mod libieeep1788_overlap;
    mod libieeep1788_rec_bool;
    //mod libieeep1788_reduction;
    //mod libieeep1788_rev;
    mod libieeep1788_set;
    mod mpfi;
    //mod pow_rev;
}

pub fn n2i(a: f64, b: f64) -> Interval {
    match interval!(a, b) {
        Ok(x) => x,
        Err(e) if e.kind() == IntervalErrorKind::UndefinedOperation => Interval::EMPTY,
        _ => panic!(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2i(s: &str) -> Interval {
    match interval!(s) {
        Ok(x) => x,
        Err(e) if e.kind() == IntervalErrorKind::UndefinedOperation => Interval::EMPTY,
        _ => panic!(),
    }
}

pub fn n2di(a: f64, b: f64) -> DecInterval {
    match dec_interval!(a, b) {
        Ok(x) => x,
        Err(e) if e.kind() == IntervalErrorKind::UndefinedOperation => DecInterval::NAI,
        _ => panic!(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2di(s: &str) -> DecInterval {
    match dec_interval!(s) {
        Ok(x) => x,
        Err(e) if e.kind() == IntervalErrorKind::UndefinedOperation => DecInterval::NAI,
        _ => panic!(),
    }
}

pub fn nd2di(a: f64, b: f64, d: Decoration) -> DecInterval {
    DecInterval::set_dec(interval!(a, b).unwrap(), d)
}

pub trait Eq2: PartialEq {
    fn eq2(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl Eq2 for bool {}
impl Eq2 for Decoration {}
impl Eq2 for Interval {}
impl Eq2 for Overlap {}

impl Eq2 for f64 {
    #[allow(clippy::float_cmp)]
    fn eq2(&self, rhs: &Self) -> bool {
        self.is_nan() && rhs.is_nan() || self == rhs
    }
}

impl Eq2 for DecInterval {
    fn eq2(&self, rhs: &Self) -> bool {
        self.is_nai() && rhs.is_nai() || self == rhs && self.decoration() == rhs.decoration()
    }
}

impl<T: Eq2> Eq2 for &T {
    fn eq2(&self, other: &Self) -> bool {
        Eq2::eq2(*self, *other)
    }
}

// Copied from `assert_eq`.
#[macro_export]
macro_rules! assert_eq2 {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !Eq2::eq2(left_val, right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(
                        r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}
