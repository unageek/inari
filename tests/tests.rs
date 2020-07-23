#![allow(clippy::approx_constant, clippy::eq_op, clippy::float_cmp)]

mod itf1788_tests {
    //mod abs_rev;
    //mod atan2;
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

mod util;

// Just for checking that build succeeds.
#[test]
fn macros() {
    use inari::{const_dec_interval, const_interval, dec_interval, interval};

    assert_eq!(interval!(1.0, 2.0).unwrap(), const_interval!(1.0, 2.0));
    assert_eq!(
        dec_interval!(1.0, 2.0).unwrap(),
        const_dec_interval!(1.0, 2.0)
    );
}
