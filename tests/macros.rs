#![allow(dead_code, non_camel_case_types)]

// Check that macros are defined using fully qualified paths.

// Shadow primitive types.
type bool = ();
type char = ();
type f32 = ();
type f64 = ();
type i128 = ();
type i16 = ();
type i32 = ();
type i64 = ();
type i8 = ();
type isize = ();
type str = ();
type u128 = ();
type u16 = ();
type u32 = ();
type u64 = ();
type u8 = ();
type usize = ();

#[test]
fn macros() {
    assert_eq!(
        inari::interval!(1.0, 2.0).unwrap(),
        inari::const_interval!(1.0, 2.0)
    );
    assert_eq!(
        inari::dec_interval!(1.0, 2.0).unwrap(),
        inari::const_dec_interval!(1.0, 2.0)
    );
}

#[cfg(feature = "gmp")]
#[test]
fn macros_gmp() {
    assert_eq!(
        inari::interval!("[1.0, 2.0]").unwrap(),
        inari::const_interval!(1.0, 2.0)
    );
    assert_eq!(
        inari::interval_exact!("[1.0, 2.0]").unwrap(),
        inari::const_interval!(1.0, 2.0)
    );
    assert_eq!(
        inari::dec_interval!("[1.0, 2.0]").unwrap(),
        inari::const_dec_interval!(1.0, 2.0)
    );
}
