// Demonstrates loss of significance.

use inari::{interval, Interval, IntervalError};

fn acc(x: Interval) -> f64 {
    -x.rad().log10()
}

fn prec(x: Interval) -> f64 {
    -(x.rad() / x.mid().abs()).log10()
}

fn main() -> Result<(), IntervalError> {
    let two = interval!("[2]")?;
    let eleven = interval!("[11]")?;
    let mut x = interval!("[0.2]")?;

    for _ in 0..20 {
        println!("x = {}", x);
        println!("prec(x) = {}", prec(x));
        println!("acc(x) = {}", acc(x));
        x = eleven * x - two;
    }

    Ok(())
}
