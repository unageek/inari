use crate::interval::*;
use std::convert::TryFrom;

macro_rules! impl_to_bytes {
    ($to_xe_bytes:ident) => {
        pub fn $to_xe_bytes(self) -> [u8; 16] {
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&f64::$to_xe_bytes(self.inf()));
            bytes[8..16].copy_from_slice(&f64::$to_xe_bytes(self.sup()));
            bytes
        }
    };
}

macro_rules! impl_try_from_bytes {
    ($try_from_xe_bytes:ident, $from_xe_bytes:ident) => {
        pub fn $try_from_xe_bytes(bytes: [u8; 16]) -> Result<Self> {
            let a = f64::$from_xe_bytes(<[u8; 8]>::try_from(&bytes[..8]).unwrap());
            let b = f64::$from_xe_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
            if a <= b
                && a != f64::INFINITY
                && b != f64::NEG_INFINITY
                && (a != 0.0 || a.is_sign_negative())
                && (b != 0.0 || b.is_sign_positive())
            {
                Ok(Self::with_infsup_raw(a, b))
            } else {
                Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: Self::EMPTY,
                })
            }
        }
    };
}

impl Interval {
    impl_to_bytes!(to_be_bytes);
    impl_to_bytes!(to_le_bytes);
    impl_to_bytes!(to_ne_bytes);

    impl_try_from_bytes!(try_from_be_bytes, from_be_bytes);
    impl_try_from_bytes!(try_from_le_bytes, from_le_bytes);
    impl_try_from_bytes!(try_from_ne_bytes, from_ne_bytes);
}

macro_rules! impl_to_bytes {
    ($to_xe_bytes:ident) => {
        pub fn $to_xe_bytes(self) -> [u8; 17] {
            let mut bytes = [0u8; 17];
            bytes[..8].copy_from_slice(&f64::$to_xe_bytes(self.inf()));
            bytes[8..16].copy_from_slice(&f64::$to_xe_bytes(self.sup()));
            bytes[16] = self.d as u8;
            bytes
        }
    };
}

macro_rules! impl_try_from_bytes {
    ($try_from_xe_bytes:ident, $from_xe_bytes:ident) => {
        pub fn $try_from_xe_bytes(bytes: [u8; 17]) -> Result<Self> {
            let a = f64::$from_xe_bytes(<[u8; 8]>::try_from(&bytes[..8]).unwrap());
            let b = f64::$from_xe_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());

            if a.is_nan() && b.is_nan() && bytes[16] == 0 {
                return Ok(Self::NAI);
            }

            if let (true, Some(d)) = (
                a <= b
                    && a != f64::INFINITY
                    && b != f64::NEG_INFINITY
                    && (a != 0.0 || a.is_sign_negative())
                    && (b != 0.0 || b.is_sign_positive()),
                match bytes[16] {
                    4 => Some(Decoration::Trv),
                    8 => Some(Decoration::Def),
                    12 => Some(Decoration::Dac),
                    16 if a != f64::NEG_INFINITY && b != f64::INFINITY => Some(Decoration::Com),
                    _ => None,
                },
            ) {
                Ok(Self::new_unchecked(Interval::with_infsup_raw(a, b), d))
            } else {
                Err(IntervalError {
                    kind: IntervalErrorKind::UndefinedOperation,
                    value: Self::NAI,
                })
            }
        }
    };
}

impl DecoratedInterval {
    impl_to_bytes!(to_be_bytes);
    impl_to_bytes!(to_le_bytes);
    impl_to_bytes!(to_ne_bytes);

    impl_try_from_bytes!(try_from_be_bytes, from_be_bytes);
    impl_try_from_bytes!(try_from_le_bytes, from_le_bytes);
    impl_try_from_bytes!(try_from_ne_bytes, from_ne_bytes);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_dec_interval, const_interval};
    type D = Decoration;
    type DI = DecoratedInterval;
    type I = Interval;

    #[test]
    fn interval() {
        fn test_roundtrip(x: I) {
            assert_eq!(I::try_from_be_bytes(x.to_be_bytes()).unwrap(), x);
            assert_eq!(I::try_from_le_bytes(x.to_le_bytes()).unwrap(), x);
            assert_eq!(I::try_from_ne_bytes(x.to_ne_bytes()).unwrap(), x);
        }

        test_roundtrip(const_interval!(-0.0, 0.0));
        test_roundtrip(const_interval!(-1.0, 3.0));
        test_roundtrip(const_interval!(f64::NEG_INFINITY, f64::INFINITY));

        fn make_bytes(a: f64, b: f64) -> [u8; 16] {
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&f64::to_ne_bytes(a));
            bytes[8..16].copy_from_slice(&f64::to_ne_bytes(b));
            bytes
        }

        assert!(I::try_from_ne_bytes(make_bytes(3.0, -1.0)).is_err());
        assert!(I::try_from_ne_bytes(make_bytes(f64::INFINITY, f64::INFINITY)).is_err());
        assert!(I::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, f64::NEG_INFINITY)).is_err());
        assert!(I::try_from_ne_bytes(make_bytes(0.0, 0.0)).is_err());
        assert!(I::try_from_ne_bytes(make_bytes(-0.0, -0.0)).is_err());
    }

    #[test]
    fn dec_interval() {
        fn test_roundtrip(x: DI) {
            let y = DI::try_from_be_bytes(x.to_be_bytes()).unwrap();
            assert!(x.is_nai() && y.is_nai() || x == y);
            let y = DI::try_from_le_bytes(x.to_le_bytes()).unwrap();
            assert!(x.is_nai() && y.is_nai() || x == y);
            let y = DI::try_from_ne_bytes(x.to_ne_bytes()).unwrap();
            assert!(x.is_nai() && y.is_nai() || x == y);
        }

        test_roundtrip(const_dec_interval!(-0.0, 0.0));
        test_roundtrip(const_dec_interval!(-1.0, 3.0));
        test_roundtrip(DI::set_dec(
            const_interval!(f64::NEG_INFINITY, f64::INFINITY),
            D::Trv,
        ));
        test_roundtrip(DI::set_dec(
            const_interval!(f64::NEG_INFINITY, f64::INFINITY),
            D::Def,
        ));
        test_roundtrip(DI::set_dec(
            const_interval!(f64::NEG_INFINITY, f64::INFINITY),
            D::Dac,
        ));
        test_roundtrip(DI::NAI);

        fn make_bytes(a: f64, b: f64, d: D) -> [u8; 17] {
            let mut bytes = [0u8; 17];
            bytes[..8].copy_from_slice(&f64::to_ne_bytes(a));
            bytes[8..16].copy_from_slice(&f64::to_ne_bytes(b));
            bytes[16] = d as u8;
            bytes
        }

        assert!(DI::try_from_ne_bytes(make_bytes(3.0, -1.0, D::Trv)).is_err());
        assert!(DI::try_from_ne_bytes(make_bytes(f64::INFINITY, f64::INFINITY, D::Trv)).is_err());
        assert!(
            DI::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, f64::NEG_INFINITY, D::Trv))
                .is_err()
        );
        assert!(DI::try_from_ne_bytes(make_bytes(0.0, 0.0, D::Trv)).is_err());
        assert!(DI::try_from_ne_bytes(make_bytes(-0.0, -0.0, D::Trv)).is_err());

        assert!(DI::try_from_ne_bytes(make_bytes(-1.0, f64::NAN, D::Ill)).is_err());
        assert!(DI::try_from_ne_bytes(make_bytes(f64::NAN, 3.0, D::Ill)).is_err());

        assert!(DI::try_from_ne_bytes(make_bytes(f64::NEG_INFINITY, 3.0, D::Com)).is_err());
        assert!(DI::try_from_ne_bytes(make_bytes(-1.0, f64::INFINITY, D::Com)).is_err());
    }
}
