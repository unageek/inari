use crate::interval::*;

macro_rules! impl_to_bytes {
    ($(#[$meta:meta])* $to_bytes:ident) => {
        $(#[$meta])*
        pub fn $to_bytes(self) -> [u8; 16] {
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&f64::$to_bytes(self.inf()));
            bytes[8..16].copy_from_slice(&f64::$to_bytes(self.sup()));
            bytes
        }
    };
}

macro_rules! impl_try_from_bytes {
    ($(#[$meta:meta])* $try_from_bytes:ident, $from_bytes:ident) => {
        $(#[$meta])*
        pub fn $try_from_bytes(bytes: [u8; 16]) -> Option<Self> {
            let a = f64::$from_bytes(<[u8; 8]>::try_from(&bytes[..8]).unwrap());
            let b = f64::$from_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());

            if a <= b
                && a != f64::INFINITY
                && b != f64::NEG_INFINITY
                && (a != 0.0 || a.is_sign_negative())
                && (b != 0.0 || b.is_sign_positive())
            {
                Some(Self::with_infsup_raw(a, b))
            } else if a == f64::INFINITY && b == f64::NEG_INFINITY {
                Some(Self::EMPTY)
            } else {
                None
            }
        }
    };
}

impl Interval {
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the big-endian byte order.
        to_be_bytes
    );
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the little-endian byte order.
        to_le_bytes
    );
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the native byte order of the target platform.
        to_ne_bytes
    );

    impl_try_from_bytes!(
        /// Creates an [`Interval`] from its interchange representation in the big-endian byte order.
        try_from_be_bytes,
        from_be_bytes
    );
    impl_try_from_bytes!(
        /// Creates an [`Interval`] from its interchange representation in the little-endian byte order.
        try_from_le_bytes,
        from_le_bytes
    );
    impl_try_from_bytes!(
        /// Creates an [`Interval`] from its interchange representation in the native byte order of the target platform.
        try_from_ne_bytes,
        from_ne_bytes
    );
}

macro_rules! impl_to_bytes {
    ($(#[$meta:meta])* $to_bytes:ident) => {
        $(#[$meta])*
        pub fn $to_bytes(self) -> [u8; 17] {
            let mut bytes = [0u8; 17];
            bytes[..8].copy_from_slice(&f64::$to_bytes(self.inf()));
            bytes[8..16].copy_from_slice(&f64::$to_bytes(self.sup()));
            bytes[16] = self.d as u8;
            bytes
        }
    };
}

macro_rules! impl_try_from_bytes {
    ($(#[$meta:meta])* $try_from_bytes:ident, $from_bytes:ident) => {
        $(#[$meta])*
        pub fn $try_from_bytes(bytes: [u8; 17]) -> Option<Self> {
            use Decoration::*;
            let a = f64::$from_bytes(<[u8; 8]>::try_from(&bytes[..8]).unwrap());
            let b = f64::$from_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
            let dec = match bytes[16] {
                0 => Ill,
                4 => Trv,
                8 => Def,
                12 => Dac,
                16 => Com,
                _ => return None,
            };

            if a <= b
                && a != f64::INFINITY
                && b != f64::NEG_INFINITY
                && (a != 0.0 || a.is_sign_negative())
                && (b != 0.0 || b.is_sign_positive())
                && (dec != Com || a != f64::NEG_INFINITY && b != f64::INFINITY)
            {
                Some(Self::new_unchecked(Interval::with_infsup_raw(a, b), dec))
            } else if a == f64::INFINITY && b == f64::NEG_INFINITY && dec == Trv {
                Some(Self::EMPTY)
            } else if a.is_nan() && b.is_nan() && dec == Ill {
                Some(Self::NAI)
            } else {
                None
            }
        }
    };
}

impl DecInterval {
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the big-endian byte order.
        to_be_bytes
    );
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the little-endian byte order.
        to_le_bytes
    );
    impl_to_bytes!(
        /// Returns the interchange representation of `self` in the native byte order of the target platform.
        to_ne_bytes
    );

    impl_try_from_bytes!(
        /// Creates a [`DecInterval`] from its interchange representation in the big-endian byte order.
        try_from_be_bytes,
        from_be_bytes
    );
    impl_try_from_bytes!(
        /// Creates a [`DecInterval`] from its interchange representation in the little-endian byte order.
        try_from_le_bytes,
        from_le_bytes
    );
    impl_try_from_bytes!(
        /// Creates a [`DecInterval`] from its interchange representation in the native byte order of the target platform.
        try_from_ne_bytes,
        from_ne_bytes
    );
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Decoration::*;
    use Interval as I;

    #[test]
    fn interval() {
        fn test_roundtrip(x: I) {
            #[allow(clippy::type_complexity)]
            let fs: [(fn(_) -> _, fn(_) -> _); 3] = [
                (I::try_from_be_bytes, I::to_be_bytes),
                (I::try_from_le_bytes, I::to_le_bytes),
                (I::try_from_ne_bytes, I::to_ne_bytes),
            ];
            for (try_from_bytes, to_bytes) in fs {
                let y = try_from_bytes(to_bytes(x)).unwrap();
                assert_eq!(x, y);
            }
        }

        test_roundtrip(const_interval!(0.0, 0.0));
        test_roundtrip(const_interval!(-2.0, 3.0));
        test_roundtrip(const_interval!(f64::NEG_INFINITY, 3.0));
        test_roundtrip(const_interval!(-2.0, f64::INFINITY));
        test_roundtrip(Interval::EMPTY);
        test_roundtrip(Interval::ENTIRE);

        fn make_bytes(a: f64, b: f64) -> [u8; 16] {
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&f64::to_ne_bytes(a));
            bytes[8..16].copy_from_slice(&f64::to_ne_bytes(b));
            bytes
        }

        assert!(I::try_from_ne_bytes(make_bytes(3.0, -2.0)).is_none());
        assert!(I::try_from_ne_bytes(make_bytes(f64::INFINITY, f64::INFINITY)).is_none());
        assert!(I::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, f64::NEG_INFINITY)).is_none());
        assert!(I::try_from_ne_bytes(make_bytes(0.0, 0.0)).is_none());
        assert!(I::try_from_ne_bytes(make_bytes(-0.0, -0.0)).is_none());
    }

    #[test]
    fn dec_interval() {
        fn test_roundtrip(x: DI) {
            #[allow(clippy::type_complexity)]
            let fs: [(fn(_) -> _, fn(_) -> _); 3] = [
                (DI::try_from_be_bytes, DI::to_be_bytes),
                (DI::try_from_le_bytes, DI::to_le_bytes),
                (DI::try_from_ne_bytes, DI::to_ne_bytes),
            ];
            for (try_from_bytes, to_bytes) in fs {
                if x.is_nai() {
                    let y = try_from_bytes(to_bytes(x)).unwrap();
                    assert!(y.is_nai());
                } else {
                    for dec in [Com, Dac, Def, Trv] {
                        let x = DecInterval::set_dec(x.interval().unwrap(), dec);
                        let y = try_from_bytes(to_bytes(x)).unwrap();
                        assert_eq!(x, y);
                        assert_eq!(x.decoration(), y.decoration());
                    }
                }
            }
        }

        test_roundtrip(const_dec_interval!(-0.0, 0.0));
        test_roundtrip(const_dec_interval!(-2.0, 3.0));
        test_roundtrip(const_dec_interval!(f64::NEG_INFINITY, 3.0));
        test_roundtrip(const_dec_interval!(-2.0, f64::INFINITY));
        test_roundtrip(DI::EMPTY);
        test_roundtrip(DI::ENTIRE);
        test_roundtrip(DI::NAI);

        fn make_bytes(a: f64, b: f64, dec: Decoration) -> [u8; 17] {
            let mut bytes = [0u8; 17];
            bytes[..8].copy_from_slice(&f64::to_ne_bytes(a));
            bytes[8..16].copy_from_slice(&f64::to_ne_bytes(b));
            bytes[16] = dec as u8;
            bytes
        }

        assert!(DI::try_from_ne_bytes(make_bytes(3.0, -2.0, Trv)).is_none());
        assert!(DI::try_from_ne_bytes(make_bytes(f64::INFINITY, f64::INFINITY, Trv)).is_none());
        assert!(
            DI::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, f64::NEG_INFINITY, Trv)).is_none()
        );
        assert!(DI::try_from_ne_bytes(make_bytes(0.0, 0.0, Trv)).is_none());
        assert!(DI::try_from_ne_bytes(make_bytes(-0.0, -0.0, Trv)).is_none());

        assert!(DI::try_from_ne_bytes(make_bytes(-2.0, f64::NAN, Ill)).is_none());
        assert!(DI::try_from_ne_bytes(make_bytes(f64::NAN, 3.0, Ill)).is_none());

        assert!(DI::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, 3.0, Com)).is_none());
        assert!(DI::try_from_ne_bytes(make_bytes(-2.0, f64::INFINITY, Com)).is_none());
    }
}
