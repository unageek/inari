use inari::{dec_interval, interval, DecoratedInterval, Decoration, Interval};

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

pub fn n2di(a: f64, b: f64) -> DecoratedInterval {
    match dec_interval!(a, b) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

#[cfg(feature = "gmp")]
pub fn t2di(s: &str) -> DecoratedInterval {
    match dec_interval!(s) {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}

pub fn nd2di(a: f64, b: f64, d: Decoration) -> DecoratedInterval {
    DecoratedInterval::set_dec(interval!(a, b).unwrap(), d)
}

pub fn interval_part(x: DecoratedInterval) -> Interval {
    match x.interval_part() {
        Ok(x) => x,
        Err(x) => x.value(),
    }
}
