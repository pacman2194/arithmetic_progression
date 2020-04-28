use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DivisionByZero;

impl fmt::Display for DivisionByZero {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Division by zero is not allowed!")
    }
}

impl Error for DivisionByZero {}

fn steps(n: i32, m: i32, d: i32) -> Result<i32, DivisionByZero> {
    if d == 0 {
        return Err(DivisionByZero);
    } else {
        return Ok((m - n) / d);
    }
}

fn sum(n: i32, d: i32, s: i32) -> i32 {
    return ( (s + 1) * n ) + ((d * s * (s + 1)) / 2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (a, z, d): (i32, i32, i32) = match args.len() {
        3 => (args[1].parse().unwrap(), args[2].parse().unwrap(), 1),
        4 => (args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
        _ => panic!("This program requires 2 or 3 arguments."),
    };

    if a == z {
        println!("{}", a);
        return;
    }

    let step = match steps(a,z,d) {
        Ok(s) => s,
        Err(error) => panic!(error),
    };

    let sumb = sum(a,d,step);

    println!("{}", sumb);
    return;
}
