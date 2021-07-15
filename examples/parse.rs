// Parses and displays interval literals.

use inari::*;

fn main() -> Result<()> {
    println!("{}", interval!("[  ]")?);
    println!("{}", interval!("[  empty  ]")?);
    println!("{}", interval!("[  1.0  ]")?);
    println!("{}", interval!("[  0x1.23p0  ]")?);
    println!("{}", interval!("[  entire  ]")?);
    println!("{}", interval!("[  ,  ]")?);
    println!("{}", interval!("[  ,  2.0  ]")?);
    println!("{}", interval!("[  1.0  ,  ]")?);
    println!("{}", interval!("[  1.0  ,  2.0  ]")?);
    println!("{}", interval!("[  0.1  ,  0.1  ]")?);
    println!("{}", interval!("[  1/10  ]")?);
    println!("{}", interval!("[  0.1,  INFINITY  ]")?);
    println!("{}", interval!("[  0.1,  inf  ]")?);
    println!("{}", interval!("[  -INFINITY,  inf  ]")?);
    println!("{}", interval!("3.56?1")?);
    println!("{}", interval!("3.56?1e2")?);
    println!("{}", interval!("3.560?2")?);
    println!("{}", interval!("3.56?")?);
    println!("{}", interval!("3.560?2u")?);
    println!("{}", interval!("-10?")?);
    println!("{}", interval!("-10?u")?);
    println!("{}", interval!("-10?12")?);

    Ok(())
}
