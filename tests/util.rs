use inari::{dec_interval, interval, DecoratedInterval, Decoration, Interval, OverlappingState};

pub fn n2i(a: f64, b: f64) -> Interval {
    match interval!(a, b) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2i(s: &str) -> Interval {
    match interval!(s) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

pub fn n2di(a: f64, b: f64) -> DecoratedInterval {
    match dec_interval!(a, b) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2di(s: &str) -> DecoratedInterval {
    match dec_interval!(s) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

pub fn nd2di(a: f64, b: f64, d: Decoration) -> DecoratedInterval {
    DecoratedInterval::set_dec(interval!(a, b).unwrap(), d)
}

pub fn interval_part(x: DecoratedInterval) -> Interval {
    match x.interval_part() {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

pub trait Eq2: PartialEq {
    fn eq2(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl Eq2 for bool {}
impl Eq2 for Decoration {}
impl Eq2 for Interval {}
impl Eq2 for OverlappingState {}

impl Eq2 for f64 {
    fn eq2(&self, rhs: &Self) -> bool {
        self.is_nan() && rhs.is_nan() || self == rhs
    }
}

impl Eq2 for DecoratedInterval {
    fn eq2(&self, rhs: &Self) -> bool {
        self.is_nai() && rhs.is_nai()
            || self == rhs && self.decoration_part() == rhs.decoration_part()
    }
}

impl<T: Eq2> Eq2 for &T {
    fn eq2(&self, other: &Self) -> bool {
        Eq2::eq2(*self, *other)
    }
}

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
