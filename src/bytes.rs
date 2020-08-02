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
            if let (true, Some(d)) = (
                a <= b
                    && a != f64::INFINITY
                    && b != f64::NEG_INFINITY
                    && (a != 0.0 || a.is_sign_negative())
                    && (b != 0.0 || b.is_sign_positive()),
                match bytes[16] {
                    0 if a.is_nan() && b.is_nan() => Some(Decoration::Ill),
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
