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
use std::{convert::TryInto, str::FromStr};

#[derive(Clone, Debug)]
struct ParseNumberError;

impl From<std::num::ParseIntError> for ParseNumberError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseNumberError
    }
}

type IntervalResult = Result<Interval, IntervalError<Interval>>;

impl From<ParseNumberError> for IntervalError<Interval> {
    fn from(_: ParseNumberError) -> Self {
        Self {
            kind: IntervalErrorKind::PossiblyUndefinedOperation,
            value: Interval::entire(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InfSup {
    Inf,
    Sup,
}

#[derive(Debug, Eq, PartialEq)]
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
    Ok(match n {
        UnsignedNumberLiteral::DecFloat(m, e) => Number::Rational(parse_dec_float(m, e)?),
        UnsignedNumberLiteral::HexFloat(m, e) => Number::Rational(parse_hex_float(m, e)?),
        UnsignedNumberLiteral::Infinity => Number::Infinity,
        UnsignedNumberLiteral::Rational(s) => Number::Rational(parse_rational(s)?),
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

fn rational_to_f64(r: &Rational, infsup: InfSup) -> f64 {
    let mut f = Float::new(f64::MANTISSA_DIGITS);
    unsafe { mpfr::set_q(f.as_raw_mut(), r.as_raw(), to_rnd_t(infsup)) };
    let rnd = to_round(infsup);
    f.subnormalize_ieee_round(Ordering::Equal, rnd);
    f.to_f64_round(rnd)
}

fn number_to_f64(n: &Number, infsup: InfSup) -> f64 {
    use Number::*;

    match n {
        NegInfinity => f64::NEG_INFINITY,
        Rational(r) => rational_to_f64(r, infsup),
        Infinity => f64::INFINITY,
    }
}

fn infsup(s: &str) -> IResult<&str, IntervalResult> {
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
                let fa = number_to_f64(&a, InfSup::Inf);
                let fb = number_to_f64(&b, InfSup::Sup);
                Ok((fa, fb).try_into().unwrap())
            } else {
                Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: Interval::empty(),
                })
            }
        },
    )(s)
}

fn point(s: &str) -> IResult<&str, IntervalResult> {
    map(number, |n| {
        let a = parse_number(n)?;
        match a {
            Number::Rational(_) => {
                let fa = number_to_f64(&a, InfSup::Inf);
                let fb = number_to_f64(&a, InfSup::Sup);
                Ok((fa, fb).try_into().unwrap())
            }
            _ => Err(IntervalError {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Interval::empty(),
            }),
        }
    })(s)
}

fn bracket(s: &str) -> IResult<&str, IntervalResult> {
    delimited(
        pair(char('['), space0),
        map(
            opt(alt((
                value(Ok(Interval::empty()), tag_no_case("empty")),
                value(Ok(Interval::entire()), tag_no_case("entire")),
                infsup,
                point,
            ))),
            |x| x.unwrap_or_else(|| Ok(Interval::empty())),
        ),
        pair(space0, char(']')),
    )(s)
}

fn uncertain_to_f64(
    center: &Rational,
    radius_or_unbounded: &Option<Rational>,
    dir: UncertainDirection,
    infsup: InfSup,
) -> f64 {
    let directions_match = dir == UncertainDirection::Both
        || dir == UncertainDirection::Down && infsup == InfSup::Inf
        || dir == UncertainDirection::Up && infsup == InfSup::Sup;

    if directions_match {
        match radius_or_unbounded {
            Some(radius) => {
                let r = match infsup {
                    InfSup::Inf => Rational::from(center - radius),
                    InfSup::Sup => Rational::from(center + radius),
                };
                rational_to_f64(&r, infsup)
            }
            None => match infsup {
                InfSup::Inf => f64::NEG_INFINITY,
                InfSup::Sup => f64::INFINITY,
            },
        }
    } else {
        rational_to_f64(center, infsup)
    }
}

fn uncertain(s: &str) -> IResult<&str, IntervalResult> {
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
            let fa = uncertain_to_f64(&center, &radius_or_unbounded, dir, InfSup::Inf);
            let fb = uncertain_to_f64(&center, &radius_or_unbounded, dir, InfSup::Sup);
            Ok((fa, fb).try_into().unwrap())
        },
    )(s)
}

impl FromStr for Interval {
    type Err = IntervalError<Interval>;

    fn from_str(s: &str) -> IntervalResult {
        match alt((bracket, uncertain))(s) {
            Ok(("", x)) => x,
            _ => Err(Self::Err {
                kind: IntervalErrorKind::UndefinedOperation,
                value: Interval::empty(),
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
