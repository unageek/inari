// Formats and parses an exact interval literal.

use inari::{interval, IntervalError};

fn main() -> Result<(), IntervalError> {
    let x = interval!("[0.1]")?;
    let s = format!("{:x}", x);
    let y = interval!(&s, exact)?;
    assert_eq!(x, y);

    Ok(())
}
