use crate::interval::*;
use core::cmp::Ordering;
use gmp_mpfr_sys::{gmp, mpfr};
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while},
    character::complete::{char, digit0, digit1, hex_digit0, hex_digit1, one_of, space0},
    combinator::{map, opt, recognize, value},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use rug::{float::Round, Float, Integer, Rational};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct ParseNumberError;

impl From<std::num::ParseIntError> for ParseNumberError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseNumberError
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InfSup {
    Inf,
    Sup,
}

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
    type Output = Number;

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
struct NInterval(Number, Number);

impl NInterval {
    fn empty() -> Self {
        Self(Number::Infinity, Number::NegInfinity)
    }

    fn entire() -> Self {
        Self(Number::NegInfinity, Number::Infinity)
    }
}

type NIntervalResult = Result<NInterval, IntervalError<NInterval>>;

#[derive(Debug, Eq, PartialEq)]
struct DNInterval {
    x: NInterval,
    d: Decoration,
}

impl DNInterval {
    fn new(x: NInterval) -> Self {
        use Decoration::*;

        let d = if x == NInterval::empty() {
            Trv
        } else if x.0 == Number::NegInfinity || x.1 == Number::Infinity {
            Dac
        } else {
            Com
        };

        Self { x, d }
    }

    fn empty() -> Self {
        Self {
            x: NInterval::empty(),
            d: Decoration::Trv,
        }
    }

    fn nai() -> Self {
        Self {
            x: NInterval::empty(),
            d: Decoration::Ill,
        }
    }

    fn set_dec(x: NInterval, d: Decoration) -> Self {
        use Decoration::*;

        if d == Ill {
            return Self::nai();
        }
        if x == NInterval::empty() {
            return Self::empty();
        }
        if d == Com && x.0 < Number::Infinity && x.1 > Number::NegInfinity {
            return Self { x, d: Dac };
        }

        Self { x, d }
    }
}

type DNIntervalResult = Result<DNInterval, IntervalError<DNInterval>>;

impl From<ParseNumberError> for IntervalError<NInterval> {
    fn from(_: ParseNumberError) -> Self {
        Self {
            kind: IntervalErrorKind::PossiblyUndefinedOperation,
            value: NInterval::entire(),
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

fn uc_radius(s: &str) -> IResult<&str, UncertainRadius> {
    map(
        opt(alt((
            map(digit1, UncertainRadius::MultipleOfUlp),
            value(UncertainRadius::Unbounded, char('?')),
        ))),
        |r| r.unwrap_or(UncertainRadius::HalfUlp),
    )(s)
}

fn uc_direction(s: &str) -> IResult<&str, UncertainDirection> {
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

fn parse_hex_float(mant: &str, exp: &str) -> Result<Rational, ParseNumberError> {
    let e = exp.parse::<i32>()?;
    let splitted = mant.split('.').collect::<Vec<_>>();
    let int_part = splitted[0];
    let frac_part = match splitted.get(1) {
        Some(s) => *s,
        _ => "",
    };

    // 1 hex digit encodes 4 bin digits.
    let lg_ulp = e
        .checked_sub(4 * frac_part.len() as i32)
        .ok_or(ParseNumberError)?;
    let ulp = pow(2, lg_ulp);

    let i_str = [int_part, frac_part].concat();
    let i = Integer::parse_radix(i_str, 16).unwrap();
    Ok(Rational::from(i) * &ulp)
}

fn parse_dec_float_with_ulp(
    mant: &str,
    exp: &str,
) -> Result<(Rational, Rational), ParseNumberError> {
    let e = exp.parse::<i32>()?;
    let splitted = mant.split('.').collect::<Vec<_>>();
    let int_part = splitted[0];
    let frac_part = match splitted.get(1) {
        Some(s) => *s,
        _ => "",
    };

    // 123.456e7 -> 123456e4 (ulp == 1e4)
    let log_ulp = e
        .checked_sub(frac_part.len() as i32)
        .ok_or(ParseNumberError)?;
    let ulp = pow(10, log_ulp);

    let i_str = [int_part, frac_part].concat();
    let i = Integer::parse_radix(i_str, 10).unwrap();
    Ok((Rational::from(i) * &ulp, ulp))
}

fn parse_dec_float(mant: &str, exp: &str) -> Result<Rational, ParseNumberError> {
    parse_dec_float_with_ulp(mant, exp).map(|(r, _)| r)
}

fn parse_rational(s: &str) -> Result<Rational, ParseNumberError> {
    unsafe {
        let mut r = Rational::new();
        let c_string = std::ffi::CString::new(s).unwrap();
        gmp::mpq_set_str(r.as_raw_mut(), c_string.as_ptr(), 10);
        gmp::mpq_canonicalize(r.as_raw_mut());
        Ok(r)
    }
}

fn to_rnd_t(infsup: InfSup) -> mpfr::rnd_t {
    match infsup {
        InfSup::Inf => mpfr::rnd_t::RNDD,
        InfSup::Sup => mpfr::rnd_t::RNDU,
    }
}

fn to_round(infsup: InfSup) -> Round {
    match infsup {
        InfSup::Inf => Round::Down,
        InfSup::Sup => Round::Up,
    }
}

fn parse_unsigned_number(n: UnsignedNumberLiteral) -> Result<Number, ParseNumberError> {
    use UnsignedNumberLiteral::*;

    Ok(match n {
        DecFloat(m, e) => Number::Rational(parse_dec_float(m, e)?),
        HexFloat(m, e) => Number::Rational(parse_hex_float(m, e)?),
        Infinity => Number::Infinity,
        Rational(s) => Number::Rational(parse_rational(s)?),
    })
}

fn parse_number(NumberLiteral(s, n): NumberLiteral) -> Result<Number, ParseNumberError> {
    let n = parse_unsigned_number(n)?;
    Ok(if s == '-' { -n } else { n })
}

fn parse_opt_number(n: Option<NumberLiteral>, infsup: InfSup) -> Result<Number, ParseNumberError> {
    match n {
        Some(n) => parse_number(n),
        _ => match infsup {
            InfSup::Inf => Ok(Number::NegInfinity),
            InfSup::Sup => Ok(Number::Infinity),
        },
    }
}

fn infsup(s: &str) -> IResult<&str, NIntervalResult> {
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
                Ok(NInterval(a, b))
            } else {
                Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: NInterval::empty(),
                })
            }
        },
    )(s)
}

fn point(s: &str) -> IResult<&str, NIntervalResult> {
    map(number, |n| {
        let a = parse_number(n)?;
        match a {
            Number::Rational(_) => {
                let b = a.clone();
                Ok(NInterval(a, b))
            }
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: NInterval::empty(),
            }),
        }
    })(s)
}

fn bracket(s: &str) -> IResult<&str, NIntervalResult> {
    delimited(
        pair(char('['), space0),
        map(
            opt(alt((
                map(tag_no_case("empty"), |_| Ok(NInterval::empty())),
                map(tag_no_case("entire"), |_| Ok(NInterval::entire())),
                infsup,
                point,
            ))),
            |x| x.unwrap_or_else(|| Ok(NInterval::empty())),
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

fn uncertain(s: &str) -> IResult<&str, NIntervalResult> {
    map(
        separated_pair(
            recognize(pair(sign, dec_significand)),
            char('?'),
            tuple((uc_radius, uc_direction, dec_exponent)),
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
            Ok(NInterval(a, b))
        },
    )(s)
}

fn interval(s: &str) -> IResult<&str, NIntervalResult> {
    alt((bracket, uncertain))(s)
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

fn decorated_interval(s: &str) -> IResult<&str, DNIntervalResult> {
    alt((
        map(
            tuple((char('['), space0, tag_no_case("nai"), space0, char(']'))),
            |_| Ok(DNInterval::nai()),
        ),
        map(
            pair(interval, opt(preceded(char('_'), decoration))),
            |pair| match pair {
                (Ok(x), None) => Ok(DNInterval::new(x)),
                (Ok(x), Some(d)) => {
                    let dec_x = DNInterval::new(x);
                    if d <= dec_x.d {
                        Ok(DNInterval::set_dec(dec_x.x, d))
                    } else {
                        Err(IntervalError {
                            kind: IntervalErrorKind::UndefinedOperation,
                            value: DNInterval::nai(),
                        })
                    }
                }
                (Err(e), None) if e.kind == IntervalErrorKind::PossiblyUndefinedOperation => {
                    Err(IntervalError {
                        kind: e.kind,
                        value: DNInterval::new(e.value),
                    })
                }
                (Err(e), Some(d)) if e.kind == IntervalErrorKind::PossiblyUndefinedOperation => {
                    Err(IntervalError {
                        kind: e.kind,
                        value: DNInterval::set_dec(e.value, d),
                    })
                }
                _ => Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: DNInterval::nai(),
                }),
            },
        ),
    ))(s)
}

#[derive(Debug)]
struct F64 {
    f: f64,
    overflow: bool,
}

fn rational_to_f64(r: &Rational, infsup: InfSup) -> F64 {
    let mut f = Float::new(f64::MANTISSA_DIGITS);
    unsafe { mpfr::set_q(f.as_raw_mut(), r.as_raw(), to_rnd_t(infsup)) };
    let rnd = to_round(infsup);
    f.subnormalize_ieee_round(Ordering::Equal, rnd);
    let f = f.to_f64_round(rnd);
    let overflow = f.is_infinite();
    F64 { f, overflow }
}

fn number_to_f64(n: &Number, infsup: InfSup) -> F64 {
    match n {
        Number::NegInfinity => F64 {
            f: f64::NEG_INFINITY,
            overflow: false,
        },
        Number::Rational(r) => rational_to_f64(r, infsup),
        Number::Infinity => F64 {
            f: f64::INFINITY,
            overflow: false,
        },
    }
}

impl From<NInterval> for Interval {
    fn from(x: NInterval) -> Self {
        DecoratedInterval::from(DNInterval::set_dec(x, Decoration::Trv)).x
    }
}

impl From<DNInterval> for DecoratedInterval {
    fn from(DNInterval { x, d }: DNInterval) -> Self {
        let a = number_to_f64(&x.0, InfSup::Inf);
        let b = number_to_f64(&x.1, InfSup::Sup);
        let x = Interval::try_from((a.f, b.f)).unwrap_or_else(|_| Interval::empty());
        let d = if a.overflow || b.overflow {
            d.min(Decoration::Dac)
        } else {
            d
        };
        Self::set_dec(x, d)
    }
}

impl FromStr for Interval {
    type Err = IntervalError<Interval>;

    fn from_str(s: &str) -> Result<Interval, IntervalError<Interval>> {
        match interval(s) {
            Ok(("", x)) => match x {
                Ok(x) => Ok(Self::from(x)),
                Err(e) => Err(IntervalError {
                    kind: e.kind,
                    value: Self::from(e.value),
                }),
            },
            _ => Err(Self::Err {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Interval::empty(),
            }),
        }
    }
}

impl FromStr for DecoratedInterval {
    type Err = IntervalError<DecoratedInterval>;

    fn from_str(s: &str) -> Result<DecoratedInterval, IntervalError<DecoratedInterval>> {
        match decorated_interval(s) {
            Ok(("", x)) => match x {
                Ok(x) => Ok(Self::from(x)),
                Err(e) => Err(IntervalError {
                    kind: e.kind,
                    value: Self::from(e.value),
                }),
            },
            _ => Err(Self::Err {
                kind: IntervalErrorKind::UndefinedOperation,
                value: DecoratedInterval::nai(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval;
    type I = Interval;

    #[test]
    fn parse() {
        // Test cases not covered by ITF1788.
        assert_eq!(
            "[123]".parse::<I>().unwrap(),
            interval!(123.0, 123.0).unwrap()
        );
        assert_eq!(
            "[0x123p0]".parse::<I>().unwrap(),
            interval!(291.0, 291.0).unwrap()
        );
        assert_eq!(
            "[123e10000000000]".parse::<I>().unwrap_err().value(),
            I::entire()
        );
        assert_eq!(
            "[0x123p10000000000]".parse::<I>().unwrap_err().value(),
            I::entire()
        );
    }
}
