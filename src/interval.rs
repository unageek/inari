use crate::simd::*;
use std::{
    cmp::Ordering,
    convert::TryFrom,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    result,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalErrorKind {
    PossiblyUndefinedOperation,
    UndefinedOperation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalError {
    pub(crate) kind: IntervalErrorKind,
}

impl IntervalError {
    /// Returns the type of the error.
    pub fn kind(&self) -> IntervalErrorKind {
        self.kind
    }
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            IntervalErrorKind::PossiblyUndefinedOperation => {
                write!(f, "possibly undefined operation")
            }
            IntervalErrorKind::UndefinedOperation => write!(f, "undefined operation"),
        }
    }
}

impl Error for IntervalError {}

/// An alias for [`Result<T, E>`](`result::Result`) with [`E = IntervalError`](`IntervalError`).
pub type Result<T> = result::Result<T, IntervalError>;

/// An interval with [`f64`] bounds.
///
/// It is sometimes referred to as a *bare* interval in contrast to a decorated interval ([`DecInterval`]).
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Interval {
    // An interval is stored in a SIMD vector in the neginf-sup-nan form:
    //
    // - An nonempty interval [a, b] is stored as [-a; b].
    // - An empty interval is stored as [NaN; NaN].
    //
    // Elements of SIMD vectors are separated by a semicolon to distinguish from interval bounds.
    //
    // Representations of zeros and NaNs are arbitrary; a zero can be either +0.0 or -0.0,
    // and a NaN can be either a qNaN or a sNaN with an arbitrary payload.
    //
    // In Debug formatting, the value of `rep` is printed as either
    // `__m128d(-a, b)` (on x86-64) or `float64x2_t(-a, b)` (on AArch64).
    pub(crate) rep: F64X2,
}

impl Interval {
    pub(crate) fn inf_raw(self) -> f64 {
        -extract0(self.rep)
    }

    pub(crate) fn sup_raw(self) -> f64 {
        extract1(self.rep)
    }

    pub(crate) fn with_infsup_raw(a: f64, b: f64) -> Self {
        Self {
            rep: constant(-a, b),
        }
    }

    pub(crate) fn zero() -> Self {
        Self { rep: splat(0.0) }
    }
}

impl PartialEq for Interval {
    fn eq(&self, rhs: &Self) -> bool {
        self.both_empty(*rhs) | all(eq(self.rep, rhs.rep))
    }
}

impl Eq for Interval {}

impl Hash for Interval {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inf().to_bits().hash(state);
        self.sup().to_bits().hash(state);
    }
}

impl TryFrom<(f64, f64)> for Interval {
    type Error = IntervalError;

    fn try_from((a, b): (f64, f64)) -> Result<Self> {
        if a <= b && a != f64::INFINITY && b != f64::NEG_INFINITY {
            Ok(Self::with_infsup_raw(a, b))
        } else {
            Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
            })
        }
    }
}

/// The decoration of a [`DecInterval`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Decoration {
    /// The “ill-formed” decoration.
    Ill = 0,
    /// The “trivial” decoration.
    Trv = 4,
    /// The “defined” decoration.
    Def = 8,
    /// The “defined and continuous” decoration.
    Dac = 12,
    /// The “common” decoration.
    Com = 16,
}

impl Ord for Decoration {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (*self as u8).cmp(&(*rhs as u8))
    }
}

impl PartialOrd for Decoration {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

/// A decorated interval with [`f64`] bounds.
///
/// Note that by definition, a NaI is not equal to itself:
///
/// ```
/// use inari::*;
/// assert_ne!(DecInterval::NAI, DecInterval::NAI);
/// ```
///
/// For this reason, the traits [`Eq`] and [`Hash`] are not implemented for the type.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DecInterval {
    pub(crate) x: Interval,
    pub(crate) d: Decoration,
}

impl DecInterval {
    /// Creates a [`DecInterval`] from the given interval and the decoration below:
    ///
    /// | Interval             | Decoration          |
    /// | -------------------- | ------------------- |
    /// | Nonempty and bounded | [`Decoration::Com`] |
    /// | Unbounded            | [`Decoration::Dac`] |
    /// | Empty                | [`Decoration::Trv`] |
    pub fn new(x: Interval) -> Self {
        use Decoration::*;

        let d = if x.is_empty() {
            Trv
        } else if !x.is_common_interval() {
            Dac
        } else {
            Com
        };

        Self::new_unchecked(x, d)
    }

    pub(crate) const fn new_unchecked(x: Interval, d: Decoration) -> Self {
        Self { x, d }
    }

    /// Returns the decoration part `self`.
    pub fn decoration(self) -> Decoration {
        self.d
    }

    /// Returns the interval part of `self` if it is not NaI; otherwise, [`None`].
    pub fn interval(self) -> Option<Interval> {
        if self.is_nai() {
            return None;
        }

        Some(self.x)
    }

    /// Creates a [`DecInterval`] from the given interval and decoration.
    /// If the decoration is invalid for the interval, the first one in the list is used:
    ///
    /// | Interval             | Valid decorations                                                                                       |
    /// | -------------------- | ------------------------------------------------------------------------------------------------------- |
    /// | Nonempty and bounded | [`Decoration::Com`], [`Decoration::Dac`], [`Decoration::Def`], [`Decoration::Trv`], [`Decoration::Ill`] |
    /// | Unbounded            | [`Decoration::Dac`], [`Decoration::Def`], [`Decoration::Trv`], [`Decoration::Ill`]                      |
    /// | Empty                | [`Decoration::Trv`], [`Decoration::Ill`]                                                                |
    pub fn set_dec(x: Interval, d: Decoration) -> Self {
        use Decoration::*;

        if d == Ill {
            Self::NAI
        } else if x.is_empty() {
            Self::EMPTY
        } else if d == Com && !x.is_common_interval() {
            Self::new_unchecked(x, Dac)
        } else {
            Self::new_unchecked(x, d)
        }
    }
}

impl PartialEq for DecInterval {
    fn eq(&self, rhs: &Self) -> bool {
        if self.is_nai() || rhs.is_nai() {
            return false;
        }

        self.x == rhs.x
    }
}

impl TryFrom<(f64, f64)> for DecInterval {
    type Error = IntervalError;

    fn try_from(x: (f64, f64)) -> Result<Self> {
        match Interval::try_from(x) {
            Ok(x) => Ok(Self::new(x)),
            _ => Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
            }),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _interval {
    ($a:expr, $b:expr) => {{
        use ::std::{convert::TryFrom, primitive::*};
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

/// Creates an [`Interval`] from [`f64`] bounds or from a bare interval literal.
///
/// In case of a failure, it returns a [`Result<Interval>`] instead of an [`Interval`].
///
/// If the construction is invalid,
/// an [`Err`] with [`IntervalErrorKind::UndefinedOperation`] is returned.
/// If it cannot determine whether the construction is valid or not,
/// [`IntervalErrorKind::PossiblyUndefinedOperation`] is returned.
///
/// - `interval!(a, b)`
///
///   Creates an interval $\[a, b\]$, where `a` and `b` are [`f64`] values.
///
///   The conditions $a ≤ b$, $a < +∞$ and $b > -∞$ must be held.
///
/// - `interval!(s)`
///
///   Creates an interval from a bare interval literal. `s` must be a string slice ([`&str`]).
///
/// - `interval!(s, exact)`
///
///   Creates an interval $𝒙$ from the bare interval literal obtained by `format!("{:x}", x)`.
///   `s` must be a string slice ([`&str`]).
///
/// For creating a constant, the macro [`const_interval!`] should be preferred over this one.
#[cfg(feature = "gmp")]
#[macro_export]
macro_rules! interval {
    ($s:expr) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($s);
        $s.parse::<$crate::Interval>()
    }};

    ($s:expr, exact) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($s);
        $crate::Interval::_try_from_str_exact($s)
    }};

    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _dec_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{convert::TryFrom, primitive::*};
        fn is_f64(_: f64) {}
        is_f64($a);
        is_f64($b);
        $crate::DecInterval::try_from(($a, $b))
    }};
}

#[cfg(not(feature = "gmp"))]
#[macro_export]
macro_rules! dec_interval {
    ($a:expr, $b:expr) => {
        $crate::_dec_interval!($a, $b)
    };
}

/// Creates a [`DecInterval`] from [`f64`] bounds or from a decorated interval literal.
///
/// In case of a failure, it returns a [`Result<DecInterval>`] instead of a [`DecInterval`].
///
/// If the construction is invalid,
/// an [`Err`] with [`IntervalErrorKind::UndefinedOperation`] is returned.
/// If it cannot determine whether the construction is valid or not,
/// [`IntervalErrorKind::PossiblyUndefinedOperation`] is returned.
///
/// - `dec_interval!(a, b)`
///
///   Creates a decorated interval $\[a, b\]$ with the strongest decoration,
///   where `a` and `b` are [`f64`] values.
///
///   The conditions $a ≤ b$, $a < +∞$ and $b > -∞$ must be held.
///
/// - `dec_interval!(s)`
///
///   Creates a decorated interval from a decorated interval literal.
///   `s` must be a string slice ([`&str`]).
///
/// For creating a constant, the macro [`const_dec_interval!`] should be preferred over this one.
#[cfg(feature = "gmp")]
#[macro_export]
macro_rules! dec_interval {
    ($s:expr) => {{
        use ::std::primitive::*;
        fn is_str(_: &str) {}
        is_str($s);
        $s.parse::<$crate::DecInterval>()
    }};

    ($a:expr, $b:expr) => {
        $crate::_dec_interval!($a, $b)
    };
}

/// Creates an [`Interval`] from [`f64`] bounds.
///
/// It can be used in [constant expressions](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
///
/// The usage is almost the same as the macro [`interval!(a, b)`](`interval!`),
/// except that `const_interval!(a, b)` returns an [`Interval`]
/// or results in a compilation error if the construction is invalid.
#[macro_export]
macro_rules! const_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{mem::transmute, primitive::*};

        ::static_assertions::const_assert!(
            $a <= $b && $a != f64::INFINITY && $b != f64::NEG_INFINITY
        );

        #[allow(unused_unsafe)]
        unsafe {
            // Parentheses are used to avoid `clippy::double_neg`.
            transmute::<_, $crate::Interval>([-($a), $b])
        }
    }};
}

/// Creates a [`DecInterval`] from [`f64`] bounds.
///
/// It can be used in [constant expressions](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
///
/// The usage is almost the same as the macro [`dec_interval!(a, b)`](`dec_interval!`),
/// except that `const_dec_interval!(a, b)` returns a [`DecInterval`]
/// or results in a compilation error if the construction is invalid.
#[macro_export]
macro_rules! const_dec_interval {
    ($a:expr, $b:expr) => {{
        use ::std::{mem::transmute, primitive::*};

        #[repr(C)]
        struct _DecInterval {
            x: $crate::Interval,
            d: $crate::Decoration,
        }

        #[allow(unused_unsafe)]
        unsafe {
            transmute::<_, $crate::DecInterval>(_DecInterval {
                x: $crate::const_interval!($a, $b),
                d: if $a == f64::NEG_INFINITY || $b == f64::INFINITY {
                    $crate::Decoration::Dac
                } else {
                    $crate::Decoration::Com
                },
            })
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn decoration_order() {
        use Decoration::*;
        assert!(Ill < Trv);
        assert!(Trv < Def);
        assert!(Def < Dac);
        assert!(Dac < Com);
    }

    #[test]
    fn macros() {
        // Check that these macros are usable for constants.
        const _I: Interval = const_interval!(1.0, 2.0);
        const _DI: DecInterval = const_dec_interval!(1.0, 2.0);

        // Check that type inference works.
        let _i = const_interval!(1.0, 2.0);
        let _di = const_dec_interval!(1.0, 2.0);

        assert_eq!(interval!(1.0, 1.0).unwrap(), const_interval!(1.0, 1.0));
        assert_eq!(interval!(1.0, 2.0).unwrap(), const_interval!(1.0, 2.0));
        assert_eq!(
            interval!(f64::NEG_INFINITY, 1.0).unwrap(),
            const_interval!(f64::NEG_INFINITY, 1.0)
        );
        assert_eq!(
            interval!(1.0, f64::INFINITY).unwrap(),
            const_interval!(1.0, f64::INFINITY)
        );

        assert_eq!(
            dec_interval!(1.0, 1.0).unwrap(),
            const_dec_interval!(1.0, 1.0)
        );
        assert_eq!(
            dec_interval!(1.0, 2.0).unwrap(),
            const_dec_interval!(1.0, 2.0)
        );
        assert_eq!(
            dec_interval!(f64::NEG_INFINITY, 1.0).unwrap(),
            const_dec_interval!(f64::NEG_INFINITY, 1.0)
        );
        assert_eq!(
            dec_interval!(1.0, f64::INFINITY).unwrap(),
            const_dec_interval!(1.0, f64::INFINITY)
        );
    }
}
