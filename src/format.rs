use crate::interval::*;
use gmp_mpfr_sys::mpfr;
use rug::Float;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;

fn mpfr_printf(template: &str, f: &Float) -> String {
    assert!(!f.is_nan());

    // https://github.com/rust-lang/rust/blob/229e5b2640fc5715e77607a989748be588d983f2/src/libcore/num/dec2flt/mod.rs#L118
    if f.is_infinite() {
        return if f.is_sign_negative() {
            String::from("-inf")
        } else {
            String::from("inf")
        };
    }

    let mut p: *mut c_char = std::ptr::null_mut();
    let c_template = CString::new(template).unwrap();

    unsafe {
        let len = mpfr::asprintf(&mut p, c_template.as_ptr(), f.as_raw());
        assert!(len >= 0);
        let c_str = CStr::from_ptr(p);
        let s = c_str.to_str().unwrap().to_owned();
        mpfr::free_str(p);
        s
    }
}

fn fmt_impl(x: Interval, d: Option<Decoration>, f: &mut fmt::Formatter, conv: char) -> fmt::Result {
    let width = match f.width() {
        Some(w) => w,
        None => 0,
    };
    let str_width = 2 * width + 1;
    if d == Some(Decoration::Ill) {
        return write!(f, "[{:^w$}]", "nai", w = str_width);
    }
    let sd = match d {
        Some(Decoration::Com) => "_com",
        Some(Decoration::Dac) => "_dac",
        Some(Decoration::Def) => "_def",
        Some(Decoration::Trv) => "_trv",
        _ => "",
    };
    if x.is_empty() {
        write!(f, "[{:^w$}]{}", "empty", sd, w = str_width)
    } else if x.is_entire() {
        write!(f, "[{:^w$}]{}", "entire", sd, w = str_width)
    } else {
        let prec = match f.precision() {
            Some(p) => format!(".{}", p),
            None => String::new(),
        };
        let fa = Float::with_val(f64::MANTISSA_DIGITS, x.inf());
        let fb = Float::with_val(f64::MANTISSA_DIGITS, x.sup());
        let sa = mpfr_printf(&format!("%{}RD{}", prec, conv), &fa);
        let sb = mpfr_printf(&format!("%{}RU{}", prec, conv), &fb);
        write!(f, "[{:>w$},{:>w$}]{}", sa, sb, sd, w = width)
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(*self, None, f, 'f')
    }
}

impl fmt::LowerExp for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(*self, None, f, 'e')
    }
}

impl fmt::LowerHex for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(*self, None, f, 'a')
    }
}

impl fmt::Display for DecoratedInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(self.x, Some(self.d), f, 'f')
    }
}

impl fmt::LowerExp for DecoratedInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(self.x, Some(self.d), f, 'e')
    }
}

impl fmt::LowerHex for DecoratedInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_impl(self.x, Some(self.d), f, 'a')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval;
    type D = Decoration;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn format() {
        assert_eq!(
            format!("{}", interval!(f64::NEG_INFINITY, 0.0).unwrap()),
            "[-inf,0]"
        );
        assert_eq!(
            format!("{:e}", interval!(f64::NEG_INFINITY, 0.0).unwrap()),
            "[-inf,0e+00]"
        );
        assert_eq!(
            format!("{:x}", interval!(f64::NEG_INFINITY, 0.0).unwrap()),
            "[-inf,0x0p+0]"
        );

        assert_eq!(
            format!("{}", interval!(0.0, f64::INFINITY).unwrap()),
            "[-0,inf]"
        );
        assert_eq!(
            format!("{:e}", interval!(0.0, f64::INFINITY).unwrap()),
            "[-0e+00,inf]"
        );
        assert_eq!(
            format!("{:x}", interval!(0.0, f64::INFINITY).unwrap()),
            "[-0x0p+0,inf]"
        );

        assert_eq!(format!("{:.6}", I::PI), "[3.141592,3.141593]");
        assert_eq!(format!("{:.6e}", I::PI), "[3.141592e+00,3.141593e+00]");
        assert_eq!(format!("{:.6x}", I::PI), "[0x3.243f6ap+0,0x3.243f6bp+0]");
        assert_eq!(
            format!("{:.6}", DI::set_dec(I::PI, D::Com)),
            "[3.141592,3.141593]_com"
        );
        assert_eq!(
            format!("{:.6}", DI::set_dec(I::PI, D::Dac)),
            "[3.141592,3.141593]_dac"
        );
        assert_eq!(
            format!("{:.6}", DI::set_dec(I::PI, D::Def)),
            "[3.141592,3.141593]_def"
        );
        assert_eq!(
            format!("{:.6}", DI::set_dec(I::PI, D::Trv)),
            "[3.141592,3.141593]_trv"
        );

        assert_eq!(
            format!("{:15.6}", I::PI),
            "[       3.141592,       3.141593]"
        );
        assert_eq!(
            format!("{:15.6e}", I::PI),
            "[   3.141592e+00,   3.141593e+00]"
        );
        assert_eq!(
            format!("{:15.6x}", I::PI),
            "[  0x3.243f6ap+0,  0x3.243f6bp+0]"
        );

        macro_rules! check {
            ($($f:literal),*) => {$(
                assert_eq!(format!($f, I::empty()), "[empty]");
                assert_eq!(format!($f, I::entire()), "[entire]");
                assert_eq!(format!($f, DI::empty()), "[empty]_trv");
                assert_eq!(format!($f, DI::entire()), "[entire]_dac");
                assert_eq!(format!($f, DI::nai()), "[nai]");
            )*};
        }
        check!("{}", "{:e}", "{:x}");

        macro_rules! check {
            ($($f:literal),*) => {$(
                assert_eq!(format!($f, I::empty()), "[   empty   ]");
                assert_eq!(format!($f, I::entire()), "[  entire   ]");
                assert_eq!(format!($f, DI::empty()), "[   empty   ]_trv");
                assert_eq!(format!($f, DI::entire()), "[  entire   ]_dac");
                assert_eq!(format!($f, DI::nai()), "[    nai    ]");
            )*};
        }
        check!("{:5}", "{:5e}", "{:5x}");
    }
}
