#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidSeries;

impl fmt::Display for InvalidSeries {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "The arguments provided do not form a valid series.")
    }
}

impl Error for InvalidSeries {}

fn is_integer(s: String) -> Result<(), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[-]?[\d]+$").unwrap();
    }
    if RE.is_match(&s) {
        return Ok(());
    } else {
        return Err(String::from(
            "All arguments that take values must be of type isize.",
        ));
    }
}

fn steps(s: isize, e: isize, d: isize) -> Result<isize, InvalidSeries> {
    if d == 0 {
        Err(InvalidSeries)
    } else {
        if (e - s) % d != 0 {
            Err(InvalidSeries)
        } else {
            Ok((e - s) / d)
        }
    }
}

fn sum(s: isize, d: isize, t: isize) -> isize {
    return ((t + 1) * s) + ((d * t * (t + 1)) / 2);
}

fn main() {
    let matches = App::new("Calculate Reasonable Arithmetic Progression")
        .version("0.1")
        .author("Drake P. <drake.packard@zoho.com>")
        .about("Calculate arithmetic progressions with rust")
        .arg(
            Arg::with_name("start")
                .help("The starting number")
                .short("s")
                .long("start")
                .takes_value(true)
                .validator(is_integer)
                .allow_hyphen_values(true)
                .required(true)
                .requires("termination"),
        )
        .arg(
            Arg::with_name("end")
                .help("The ending number")
                .short("e")
                .long("end")
                .takes_value(true)
                .validator(is_integer)
                .allow_hyphen_values(true)
                .required(false)
                .group("termination"),
        )
        .arg(
            Arg::with_name("difference")
                .help("The difference between each term")
                .short("d")
                .long("difference")
                .takes_value(true)
                .validator(is_integer)
                .allow_hyphen_values(true)
                .required(false)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("terms")
                .short("t")
                .long("terms")
                .help("The number of terms in the progression")
                .takes_value(true)
                .validator(is_integer)
                .allow_hyphen_values(true)
                .required(false)
                .group("termination"),
        )
        .get_matches();

    let s = matches.value_of("start").unwrap();
    let d = matches.value_of("difference").unwrap();

    if matches.is_present("end") && s == matches.value_of("end").unwrap() && d == "0" {
        println!("{}", s);
        return;
    }

    let s: isize = s.parse().unwrap();
    let d: isize = d.parse().unwrap();

    let t: isize = if matches.is_present("end") {
        let e: isize = matches.value_of("end").unwrap().parse().unwrap();
        let t = match steps(s, e, d) {
            Ok(s) => s,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(0x01);
            }
        };
        t
    } else {
        matches.value_of("terms").unwrap().parse().unwrap()
    };

    println!("{}", sum(s, d, t));
    return;
}
