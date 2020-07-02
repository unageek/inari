use inari::{interval, Interval};

pub fn n2i(a: f64, b: f64) -> Interval {
    match interval!(a, b) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2i(s: &str) -> Interval {
    match interval!(s) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}
