// Formats and parses an exact interval literal.

use inari::{interval, Interval, IntervalError};

fn main() -> Result<(), IntervalError<Interval>> {
    let x = interval!("[0.1]")?;
    let s = format!("{:x}", x);
    let y = interval!(&s, exact)?;
    assert_eq!(x, y);

    Ok(())
}
