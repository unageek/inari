use core::arch::x86_64::*;
use std::{convert::TryFrom, error::Error, fmt};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Interval {
    // The neginf-sup-nan form is used to represent an interval.
    // 0 is stored as either 0.0 or -0.0.
    // NaN can be stored in any representation.
    //
    // An interval [a, b] is stored as [b; -a]:
    //   +----------------+----------------+
    //   |        b       |       -a       |
    //   +----------------+----------------+
    //   |127           64|63             0|
    //
    // The value of `rep` is Formatted as `__m128d(-a, b)` in Debug formatting.
    pub(crate) rep: __m128d,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            rep: unsafe { _mm_set1_pd(f64::NAN) },
        }
    }

    pub fn entire() -> Self {
        Self {
            rep: unsafe { _mm_set1_pd(f64::INFINITY) },
        }
    }

    pub(crate) fn inf_raw(self) -> f64 {
        unsafe { -_mm_cvtsd_f64(self.rep) }
    }

    pub(crate) fn sup_raw(self) -> f64 {
        unsafe { _mm_cvtsd_f64(_mm_unpackhi_pd(self.rep, self.rep)) }
    }

    pub(crate) fn with_infsup_raw(a: f64, b: f64) -> Self {
        Self {
            rep: unsafe { _mm_set_pd(b, -a) },
        }
    }

    pub(crate) fn zero() -> Self {
        Self {
            rep: unsafe { _mm_setzero_pd() },
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, rhs: &Self) -> bool {
        self.is_empty() && rhs.is_empty()
            || unsafe { _mm_movemask_pd(_mm_cmpeq_pd(self.rep, rhs.rep)) == 3 }
    }
}

impl Eq for Interval {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalErrorKind {
    PossiblyUndefinedOperation,
    UndefinedOperation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalError<T: Copy + fmt::Debug> {
    pub(crate) kind: IntervalErrorKind,
    pub(crate) value: T,
}

impl<T> IntervalError<T>
where
    T: Copy + fmt::Debug,
{
    pub fn kind(&self) -> IntervalErrorKind {
        self.kind
    }

    pub fn value(&self) -> T {
        self.value
    }
}

impl<T> fmt::Display for IntervalError<T>
where
    T: Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            IntervalErrorKind::PossiblyUndefinedOperation => {
                write!(f, "possibly undefined operation")
            }
            IntervalErrorKind::UndefinedOperation => write!(f, "undefined operation"),
        }
    }
}

impl<T> Error for IntervalError<T> where T: Copy + fmt::Debug {}

impl TryFrom<(f64, f64)> for Interval {
    type Error = IntervalError<Interval>;

    fn try_from((a, b): (f64, f64)) -> Result<Self, Self::Error> {
        if a <= b && a != f64::INFINITY && b != f64::NEG_INFINITY {
            Ok(Self::with_infsup_raw(a, b))
        } else {
            Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Interval::empty(),
            })
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _interval {
    ($a:expr, $b:expr) => {{
        use std::convert::TryFrom;
        fn is_f64(_: f64) {}
        is_f64($a);
        is_f64($b);
        $crate::Interval::try_from(($a, $b))
    }};
}

#[cfg(not(feature = "gmp"))]
#[macro_export]
macro_rules! interval {
    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };
}

#[cfg(feature = "gmp")]
#[macro_export]
macro_rules! interval {
    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };

    ($text:expr) => {{
        fn is_str(_: &str) {}
        is_str($text);
        $text.parse::<$crate::Interval>()
    }};
}
