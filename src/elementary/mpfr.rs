use gmp_mpfr_sys::mpfr;
use rug::Float;

macro_rules! mpfr_fn {
    ($mpfr_f:ident, $f_rd:ident, $f_ru:ident) => {
        pub fn $f_rd(x: f64) -> f64 {
            mpfr_fn!($mpfr_f(x, RNDD))
        }

        pub fn $f_ru(x: f64) -> f64 {
            mpfr_fn!($mpfr_f(x, RNDU))
        }
    };

    ($mpfr_f:ident($x:ident, $rnd:ident)) => {{
        let mut x = Float::with_val(f64::MANTISSA_DIGITS, $x);
        let rnd = mpfr::rnd_t::$rnd;
        unsafe {
            mpfr::$mpfr_f(x.as_raw_mut(), x.as_raw(), rnd);
            mpfr::get_d(x.as_raw(), rnd)
        }
    }};
}

macro_rules! mpfr_fn2 {
    ($mpfr_f:ident, $f_rd:ident, $f_ru:ident) => {
        pub fn $f_rd(x: f64, y: f64) -> f64 {
            mpfr_fn2!($mpfr_f(x, y, RNDD))
        }

        pub fn $f_ru(x: f64, y: f64) -> f64 {
            mpfr_fn2!($mpfr_f(x, y, RNDU))
        }
    };

    ($mpfr_f:ident($x:ident, $y:ident, $rnd:ident)) => {{
        let mut x = Float::with_val(f64::MANTISSA_DIGITS, $x);
        let y = Float::with_val(f64::MANTISSA_DIGITS, $y);
        let rnd = mpfr::rnd_t::$rnd;
        unsafe {
            mpfr::$mpfr_f(x.as_raw_mut(), x.as_raw(), y.as_raw(), rnd);
            mpfr::get_d(x.as_raw(), rnd)
        }
    }};
}

macro_rules! mpfr_fn_si {
    ($mpfr_f:ident, $f_rd:ident, $f_ru:ident) => {
        pub fn $f_rd(x: f64, y: i32) -> f64 {
            mpfr_fn_si!($mpfr_f(x, y, RNDD))
        }

        pub fn $f_ru(x: f64, y: i32) -> f64 {
            mpfr_fn_si!($mpfr_f(x, y, RNDU))
        }
    };

    ($mpfr_f:ident($x:ident, $y:ident, $rnd:ident)) => {{
        let mut x = Float::with_val(f64::MANTISSA_DIGITS, $x);
        let rnd = mpfr::rnd_t::$rnd;
        unsafe {
            mpfr::$mpfr_f(x.as_raw_mut(), x.as_raw(), $y.into(), rnd);
            mpfr::get_d(x.as_raw(), rnd)
        }
    }};
}

mpfr_fn!(acos, acos_rd, acos_ru);
mpfr_fn!(acosh, acosh_rd, acosh_ru);
mpfr_fn!(asin, asin_rd, asin_ru);
mpfr_fn!(asinh, asinh_rd, asinh_ru);
mpfr_fn!(atan, atan_rd, atan_ru);
mpfr_fn2!(atan2, atan2_rd, atan2_ru);
mpfr_fn!(atanh, atanh_rd, atanh_ru);
mpfr_fn!(cos, cos_rd, cos_ru);
mpfr_fn!(cosh, cosh_rd, cosh_ru);
mpfr_fn!(exp, exp_rd, exp_ru);
mpfr_fn!(exp10, exp10_rd, exp10_ru);
mpfr_fn!(exp2, exp2_rd, exp2_ru);
mpfr_fn!(log, ln_rd, ln_ru);
mpfr_fn!(log10, log10_rd, log10_ru);
mpfr_fn!(log2, log2_rd, log2_ru);
mpfr_fn2!(pow, pow_rd, pow_ru);
mpfr_fn_si!(pow_si, pown_rd, pown_ru);
mpfr_fn!(sin, sin_rd, sin_ru);
mpfr_fn!(sinh, sinh_rd, sinh_ru);
mpfr_fn!(tan, tan_rd, tan_ru);
mpfr_fn!(tanh, tanh_rd, tanh_ru);
