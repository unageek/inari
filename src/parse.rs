use crate::interval::*;
use gmp_mpfr_sys::{gmp, mpfr};
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while},
    character::complete::{char, digit0, digit1, hex_digit0, hex_digit1, one_of, space0},
    combinator::{map, opt, recognize, value},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use rug::{Float, Integer, Rational};
use std::{cmp::Ordering, convert::TryFrom, result, str::FromStr};

#[derive(Clone, Debug)]
struct ParseNumberError;

impl From<std::num::ParseIntError> for ParseNumberError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

impl From<std::num::TryFromIntError> for ParseNumberError {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InfSup {
    Inf,
    Sup,
}

impl InfSup {
    fn as_rnd_t(self) -> mpfr::rnd_t {
        match self {
            Self::Inf => mpfr::rnd_t::RNDD,
            Self::Sup => mpfr::rnd_t::RNDU,
        }
    }
}

/// An extended rational number.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Number {
    NegInfinity,
    Rational(Rational),
    Infinity,
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        use Number::*;

        match (self, other) {
            (NegInfinity, Rational(_)) | (NegInfinity, Infinity) | (Rational(_), Infinity) => {
                Ordering::Less
            }
            (Rational(_), NegInfinity) | (Infinity, NegInfinity) | (Infinity, Rational(_)) => {
                Ordering::Greater
            }
            (Rational(a), Rational(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        use Number::*;

        match self {
            NegInfinity => Infinity,
            Rational(r) => Rational(-r),
            Infinity => NegInfinity,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum NInterval {
    InfSup(Number, Number),
    Point(Number),
}

impl NInterval {
    const EMPTY: Self = Self::InfSup(Number::Infinity, Number::NegInfinity);
    const ENTIRE: Self = Self::InfSup(Number::NegInfinity, Number::Infinity);

    fn inf(&self) -> &Number {
        match self {
            Self::InfSup(x, _) => x,
            Self::Point(x) => x,
        }
    }

    fn is_common_interval(&self) -> bool {
        matches!(
            (self.inf(), self.sup()),
            (Number::Rational(_), Number::Rational(_))
        )
    }

    fn sup(&self) -> &Number {
        match self {
            Self::InfSup(_, x) => x,
            Self::Point(x) => x,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct DecNInterval {
    x: NInterval,
    d: Decoration,
}

impl DecNInterval {
    const EMPTY: Self = Self {
        x: NInterval::EMPTY,
        d: Decoration::Trv,
    };

    const NAI: Self = Self {
        x: NInterval::EMPTY,
        d: Decoration::Ill,
    };

    fn new(x: NInterval) -> Self {
        use Decoration::*;

        let d = if x == NInterval::EMPTY {
            Trv
        } else if !x.is_common_interval() {
            Dac
        } else {
            Com
        };

        Self { x, d }
    }

    fn set_dec(x: NInterval, d: Decoration) -> Self {
        use Decoration::*;

        if d == Ill {
            Self::NAI
        } else if x == NInterval::EMPTY {
            Self::EMPTY
        } else if d == Com && !x.is_common_interval() {
            Self { x, d: Dac }
        } else {
            Self { x, d }
        }
    }
}

impl From<ParseNumberError> for IntervalError<NInterval> {
    fn from(_: ParseNumberError) -> Self {
        Self {
            kind: IntervalErrorKind::PossiblyUndefinedOperation,
            value: NInterval::ENTIRE,
        }
    }
}

#[derive(Debug)]
struct NumberLiteral<'a>(char, UnsignedNumberLiteral<'a>);

#[derive(Clone, Debug)]
enum UnsignedNumberLiteral<'a> {
    DecFloat(&'a str, &'a str),
    HexFloat(&'a str, &'a str),
    Infinity,
    Rational(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UncertainDirection {
    Both,
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UncertainRadius<'a> {
    HalfUlp,
    MultipleOfUlp(&'a str),
    Unbounded,
}

fn sign(s: &str) -> IResult<&str, char> {
    map(opt(one_of("+-")), |os| os.unwrap_or('+'))(s)
}

fn integer(s: &str) -> IResult<&str, &str> {
    recognize(pair(sign, digit1))(s)
}

fn dec_significand(s: &str) -> IResult<&str, &str> {
    alt((
        // "123", "123.", "123.456"
        recognize(pair(digit1, opt(pair(char('.'), digit0)))),
        // ".456"
        recognize(pair(char('.'), digit1)),
    ))(s)
}

fn hex_significand(s: &str) -> IResult<&str, &str> {
    alt((
        recognize(pair(hex_digit1, opt(pair(char('.'), hex_digit0)))),
        recognize(pair(char('.'), hex_digit1)),
    ))(s)
}

fn dec_exponent(s: &str) -> IResult<&str, &str> {
    map(opt(preceded(tag_no_case("e"), integer)), |opt_exp| {
        opt_exp.unwrap_or("0")
    })(s)
}

fn positive_natural(s: &str) -> IResult<&str, &str> {
    recognize(tuple((
        take_while(|c| c == '0'),
        one_of("123456789"),
        digit0,
    )))(s)
}

fn unsigned_number(s: &str) -> IResult<&str, UnsignedNumberLiteral> {
    alt((
        // rational
        map(
            recognize(separated_pair(digit1, char('/'), positive_natural)),
            UnsignedNumberLiteral::Rational,
        ),
        // hexadecimal float
        map(
            pair(
                preceded(tag_no_case("0x"), hex_significand),
                preceded(tag_no_case("p"), integer),
            ),
            |(m, e)| UnsignedNumberLiteral::HexFloat(m, e),
        ),
        // decimal float
        map(pair(dec_significand, dec_exponent), |(m, e)| {
            UnsignedNumberLiteral::DecFloat(m, e)
        }),
        // infinity
        value(
            UnsignedNumberLiteral::Infinity,
            pair(tag_no_case("inf"), opt(tag_no_case("inity"))),
        ),
    ))(s)
}

fn number(s: &str) -> IResult<&str, NumberLiteral> {
    map(pair(sign, unsigned_number), |(s, n)| NumberLiteral(s, n))(s)
}

fn uncertain_radius(s: &str) -> IResult<&str, UncertainRadius> {
    map(
        opt(alt((
            map(digit1, UncertainRadius::MultipleOfUlp),
            value(UncertainRadius::Unbounded, char('?')),
        ))),
        |r| r.unwrap_or(UncertainRadius::HalfUlp),
    )(s)
}

fn uncertain_direction(s: &str) -> IResult<&str, UncertainDirection> {
    map(
        opt(alt((
            value(UncertainDirection::Down, tag_no_case("d")),
            value(UncertainDirection::Up, tag_no_case("u")),
        ))),
        |d| d.unwrap_or(UncertainDirection::Both),
    )(s)
}

fn pow(base: u32, exp: i32) -> Rational {
    let i = Integer::from(Integer::u_pow_u(base, exp.abs() as u32));
    let mut r = Rational::from(i);
    if exp < 0 {
        r.recip_mut();
    }
    r
}

fn parse_hex_float(mant: &str, exp: &str) -> result::Result<Rational, ParseNumberError> {
    let e = exp.parse::<i32>()?;
    let mut parts = mant.split('.');
    let int_part = parts.next().unwrap();
    let frac_part = match parts.next() {
        Some(s) => s,
        _ => "",
    };

    // 1 hex digit encodes 4 bin digits.
    let frac_bits = i32::try_from(frac_part.len())?
        .checked_mul(4)
        .ok_or(ParseNumberError)?;
    let log2_ulp = e.checked_sub(frac_bits).ok_or(ParseNumberError)?;
    let ulp = pow(2, log2_ulp);

    let i_str = [int_part, frac_part].concat();
    let i = Integer::parse_radix(i_str, 16).unwrap();
    Ok(Rational::from(i) * &ulp)
}

fn parse_dec_float_with_ulp(
    mant: &str,
    exp: &str,
) -> result::Result<(Rational, Rational), ParseNumberError> {
    let e = exp.parse::<i32>()?;
    let mut parts = mant.split('.');
    let int_part = parts.next().unwrap();
    let frac_part = match parts.next() {
        Some(s) => s,
        _ => "",
    };

    // 123.456e7 -> 123456e4 (ulp == 1e4)
    let frac_digits = i32::try_from(frac_part.len())?;
    let log10_ulp = e.checked_sub(frac_digits).ok_or(ParseNumberError)?;
    let ulp = pow(10, log10_ulp);

    let i_str = [int_part, frac_part].concat();
    let i = Integer::parse_radix(i_str, 10).unwrap();
    Ok((Rational::from(i) * &ulp, ulp))
}

fn parse_dec_float(mant: &str, exp: &str) -> result::Result<Rational, ParseNumberError> {
    parse_dec_float_with_ulp(mant, exp).map(|(r, _)| r)
}

fn parse_rational(s: &str) -> Rational {
    unsafe {
        let mut r = Rational::new();
        let c_string = std::ffi::CString::new(s).unwrap();
        gmp::mpq_set_str(r.as_raw_mut(), c_string.as_ptr(), 10);
        gmp::mpq_canonicalize(r.as_raw_mut());
        r
    }
}

fn parse_unsigned_number(n: UnsignedNumberLiteral) -> result::Result<Number, ParseNumberError> {
    use UnsignedNumberLiteral::*;

    Ok(match n {
        DecFloat(m, e) => Number::Rational(parse_dec_float(m, e)?),
        HexFloat(m, e) => Number::Rational(parse_hex_float(m, e)?),
        Infinity => Number::Infinity,
        Rational(s) => Number::Rational(parse_rational(s)),
    })
}

fn parse_number(NumberLiteral(s, n): NumberLiteral) -> result::Result<Number, ParseNumberError> {
    let n = parse_unsigned_number(n)?;
    Ok(if s == '-' { -n } else { n })
}

fn parse_opt_number(
    n: Option<NumberLiteral>,
    infsup: InfSup,
) -> result::Result<Number, ParseNumberError> {
    match n {
        Some(n) => parse_number(n),
        _ => match infsup {
            InfSup::Inf => Ok(Number::NegInfinity),
            InfSup::Sup => Ok(Number::Infinity),
        },
    }
}

fn infsup_interval(s: &str) -> IResult<&str, Result<NInterval>> {
    map(
        separated_pair(
            opt(number),
            delimited(space0, char(','), space0),
            opt(number),
        ),
        |(na, nb)| {
            let a = parse_opt_number(na, InfSup::Inf)?;
            let b = parse_opt_number(nb, InfSup::Sup)?;
            if a <= b && a != Number::Infinity && b != Number::NegInfinity {
                Ok(NInterval::InfSup(a, b))
            } else {
                Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: NInterval::EMPTY,
                })
            }
        },
    )(s)
}

fn point_interval(s: &str) -> IResult<&str, Result<NInterval>> {
    map(number, |n| {
        let a = parse_number(n)?;
        match a {
            Number::Rational(_) => Ok(NInterval::Point(a)),
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: NInterval::EMPTY,
            }),
        }
    })(s)
}

fn bracket_interval(s: &str) -> IResult<&str, Result<NInterval>> {
    delimited(
        pair(char('['), space0),
        map(
            opt(alt((
                map(tag_no_case("empty"), |_| Ok(NInterval::EMPTY)),
                map(tag_no_case("entire"), |_| Ok(NInterval::ENTIRE)),
                infsup_interval,
                point_interval,
            ))),
            |x| x.unwrap_or(Ok(NInterval::EMPTY)),
        ),
        pair(space0, char(']')),
    )(s)
}

fn uncertain_bound(
    center: &Rational,
    radius_or_unbounded: &Option<Rational>,
    dir: UncertainDirection,
    infsup: InfSup,
) -> Number {
    let directions_match = dir == UncertainDirection::Both
        || dir == UncertainDirection::Down && infsup == InfSup::Inf
        || dir == UncertainDirection::Up && infsup == InfSup::Sup;

    if directions_match {
        match radius_or_unbounded {
            Some(radius) => match infsup {
                InfSup::Inf => Number::Rational(Rational::from(center - radius)),
                InfSup::Sup => Number::Rational(Rational::from(center + radius)),
            },
            None => match infsup {
                InfSup::Inf => Number::NegInfinity,
                InfSup::Sup => Number::Infinity,
            },
        }
    } else {
        Number::Rational(center.clone())
    }
}

fn uncertain_interval(s: &str) -> IResult<&str, Result<NInterval>> {
    map(
        separated_pair(
            recognize(pair(sign, dec_significand)),
            char('?'),
            tuple((uncertain_radius, uncertain_direction, dec_exponent)),
        ),
        |(n, (rad, dir, exp))| {
            let (center, ulp) = parse_dec_float_with_ulp(n, exp)?;
            let radius_or_unbounded = match rad {
                UncertainRadius::HalfUlp => Some(ulp / 2),
                UncertainRadius::MultipleOfUlp(x) => {
                    let xi = Integer::parse_radix(x, 10).unwrap();
                    Some(Rational::from(xi) * ulp)
                }
                UncertainRadius::Unbounded => None,
            };
            let a = uncertain_bound(&center, &radius_or_unbounded, dir, InfSup::Inf);
            let b = uncertain_bound(&center, &radius_or_unbounded, dir, InfSup::Sup);
            Ok(NInterval::InfSup(a, b))
        },
    )(s)
}

fn interval(s: &str) -> IResult<&str, Result<NInterval>> {
    alt((bracket_interval, uncertain_interval))(s)
}

fn decoration(s: &str) -> IResult<&str, Decoration> {
    use Decoration::*;

    alt((
        value(Com, tag_no_case("com")),
        value(Dac, tag_no_case("dac")),
        value(Def, tag_no_case("def")),
        value(Trv, tag_no_case("trv")),
    ))(s)
}

fn decorated_interval(s: &str) -> IResult<&str, Result<DecNInterval>> {
    alt((
        map(
            tuple((char('['), space0, tag_no_case("nai"), space0, char(']'))),
            |_| Ok(DecNInterval::NAI),
        ),
        map(
            pair(interval, opt(preceded(char('_'), decoration))),
            |pair| match pair {
                (Ok(x), None) => Ok(DecNInterval::new(x)),
                (Ok(x), Some(d)) => {
                    let xd = DecNInterval::set_dec(x, d);
                    if xd.d == d {
                        Ok(xd)
                    } else {
                        Err(IntervalError {
                            kind: IntervalErrorKind::UndefinedOperation,
                            value: DecNInterval::NAI,
                        })
                    }
                }
                (Err(e), None) if e.kind == IntervalErrorKind::PossiblyUndefinedOperation => {
                    Err(IntervalError {
                        kind: e.kind,
                        value: DecNInterval::new(e.value),
                    })
                }
                (Err(e), Some(d)) if e.kind == IntervalErrorKind::PossiblyUndefinedOperation => {
                    Err(IntervalError {
                        kind: e.kind,
                        value: DecNInterval::set_dec(e.value, d),
                    })
                }
                _ => Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: DecNInterval::NAI,
                }),
            },
        ),
    ))(s)
}

#[derive(Debug)]
struct F64 {
    f: f64,
    inexact: bool,
    overflow: bool,
}

fn rational_to_f64(r: &Rational, infsup: InfSup) -> F64 {
    fn ternary_to_ordering(t: i32) -> Ordering {
        t.cmp(&0)
    }

    let rnd = infsup.as_rnd_t();
    let mut f = Float::new(f64::MANTISSA_DIGITS);

    unsafe {
        let orig_emin = mpfr::get_emin();
        let orig_emax = mpfr::get_emax();
        mpfr::set_emin((f64::MIN_EXP - (f64::MANTISSA_DIGITS as i32) + 1) as i64);
        mpfr::set_emax(f64::MAX_EXP as i64);
        let t = mpfr::set_q(f.as_raw_mut(), r.as_raw(), rnd);
        let t = mpfr::subnormalize(f.as_raw_mut(), t, rnd);
        let f = mpfr::get_d(f.as_raw(), rnd);
        mpfr::set_emin(orig_emin);
        mpfr::set_emax(orig_emax);
        F64 {
            f,
            inexact: ternary_to_ordering(t) != Ordering::Equal,
            overflow: f.is_infinite(),
        }
    }
}

fn number_to_f64(n: &Number, infsup: InfSup) -> F64 {
    match n {
        Number::NegInfinity => F64 {
            f: f64::NEG_INFINITY,
            inexact: false,
            overflow: false,
        },
        Number::Rational(r) => rational_to_f64(r, infsup),
        Number::Infinity => F64 {
            f: f64::INFINITY,
            inexact: false,
            overflow: false,
        },
    }
}

impl From<NInterval> for Interval {
    fn from(x: NInterval) -> Self {
        DecInterval::from(DecNInterval::new(x)).x
    }
}

impl From<DecNInterval> for DecInterval {
    fn from(DecNInterval { x, d }: DecNInterval) -> Self {
        let a = number_to_f64(x.inf(), InfSup::Inf);
        let b = number_to_f64(x.sup(), InfSup::Sup);
        // The empty interval cannot be constructed with `Interval::try_from`.
        let x = Interval::try_from((a.f, b.f)).unwrap_or(Interval::EMPTY);
        let d = if a.overflow || b.overflow {
            d.min(Decoration::Dac)
        } else {
            d
        };
        Self::set_dec(x, d)
    }
}

impl FromStr for Interval {
    type Err = IntervalError<Self>;

    fn from_str(s: &str) -> Result<Self> {
        match interval(s) {
            Ok(("", x)) => match x {
                Ok(x) => Ok(Self::from(x)),
                Err(e) => Err(IntervalError {
                    kind: e.kind,
                    value: Self::from(e.value),
                }),
            },
            // Invalid syntax.
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::EMPTY,
            }),
        }
    }
}

impl FromStr for DecInterval {
    type Err = IntervalError<Self>;

    fn from_str(s: &str) -> Result<Self> {
        match decorated_interval(s) {
            Ok(("", x)) => match x {
                Ok(x) => Ok(Self::from(x)),
                Err(e) => Err(IntervalError {
                    kind: e.kind,
                    value: Self::from(e.value),
                }),
            },
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::NAI,
            }),
        }
    }
}

impl Interval {
    fn try_from_ninterval_exact(x: NInterval) -> Result<Self> {
        let a = number_to_f64(x.inf(), InfSup::Inf);
        let b = number_to_f64(x.sup(), InfSup::Sup);
        let x = Self::try_from((a.f, b.f)).unwrap_or(Self::EMPTY);
        if a.inexact || b.inexact {
            Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::EMPTY,
            })
        } else {
            Ok(x)
        }
    }

    #[doc(hidden)]
    pub fn _try_from_str_exact(s: &str) -> Result<Self> {
        match interval(s) {
            Ok(("", x)) => match x {
                Ok(x) => Self::try_from_ninterval_exact(x),
                Err(e) => Err(IntervalError {
                    kind: e.kind,
                    value: Self::EMPTY,
                }),
            },
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Self::EMPTY,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    #[test]
    fn parse() {
        // Integer significands without a dot are not covered by ITF1788.
        assert_eq!(interval!("[123]").unwrap(), const_interval!(123.0, 123.0));
        assert_eq!(
            interval!("[0x123p0]").unwrap(),
            const_interval!(291.0, 291.0)
        );

        // Exponent == i32::MAX + 1.
        assert_eq!(
            interval!("[123e2147483648]").unwrap_err().value(),
            I::ENTIRE
        );
        assert_eq!(
            interval!("[0x123p2147483648]").unwrap_err().value(),
            I::ENTIRE
        );

        // Exponent == i32::MIN - 1.
        assert_eq!(
            interval!("[123e-2147483649]").unwrap_err().value(),
            I::ENTIRE
        );
        assert_eq!(
            interval!("[0x123p-2147483649]").unwrap_err().value(),
            I::ENTIRE
        );

        assert_eq!(
            dec_interval!("[123e2147483648]").unwrap_err().value(),
            DI::ENTIRE
        );
        assert_eq!(
            dec_interval!("[123e2147483648]_com").unwrap_err().value(),
            DI::ENTIRE
        );
    }

    #[test]
    fn try_from_str_exact() {
        assert_eq!(interval!("[]", exact).unwrap(), I::EMPTY);
        assert_eq!(interval!("[Empty]", exact).unwrap(), I::EMPTY);
        assert_eq!(interval!("[Entire]", exact).unwrap(), I::ENTIRE);
        assert_eq!(
            interval!("[0.0, 1.0]", exact).unwrap(),
            const_interval!(0.0, 1.0)
        );
        assert_eq!(
            interval!("[0.0, 0.1]", exact).unwrap_err().value(),
            I::EMPTY
        );
        assert_eq!(
            interval!("[0.1, 1.0]", exact).unwrap_err().value(),
            I::EMPTY
        );

        // The smallest positive subnormal number.
        let f = 5e-324;
        assert_eq!(
            interval!("[0x0.0000000000001p-1022]", exact).unwrap(),
            interval!(f, f).unwrap()
        );
        assert_eq!(
            interval!("[0x0.0000000000000ffp-1022]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );
        assert_eq!(
            interval!("[0x0.000000000000101p-1022]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );
        assert_eq!(
            interval!("[0x0.0000000000001p-1023]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );

        // The largest normal number.
        let f = f64::MAX;
        assert_eq!(
            interval!("[0x1.fffffffffffffp+1023]", exact).unwrap(),
            interval!(f, f).unwrap()
        );
        assert_eq!(
            interval!("[0x1.ffffffffffffeffp+1023]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );
        assert_eq!(
            interval!("[0x1.fffffffffffff01p+1023]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );
        assert_eq!(
            interval!("[0x1.fffffffffffffp+1024]", exact)
                .unwrap_err()
                .value(),
            I::EMPTY
        );

        assert_eq!(
            interval!("[123e2147483648]", exact).unwrap_err().value(),
            I::EMPTY
        );
    }
}
