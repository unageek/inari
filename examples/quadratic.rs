// Solves the quadratic equation a x^2 + b x + c == 0.

use inari::{interval, IntervalError};

fn main() -> Result<(), IntervalError> {
    let a = interval!("[1]")?;
    let b = interval!("[1e15]")?;
    let c = interval!("[1e14]")?;
    let two = interval!("[2]")?;
    let four = interval!("[4]")?;

    let (x1, x2) = if b.mid() >= 0.0 {
        (
            (two * c) / (-b - (b.sqr() - four * a * c).sqrt()),
            (-b - (b.sqr() - four * a * c).sqrt()) / (two * a),
        )
    } else {
        (
            (-b + (b.sqr() - four * a * c).sqrt()) / (two * a),
            (two * c) / (-b + (b.sqr() - four * a * c).sqrt()),
        )
    };

    println!("x1 = {:.15}", x1);
    println!("x2 = {:.15}", x2);

    Ok(())
}
