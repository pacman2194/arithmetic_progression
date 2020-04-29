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

fn term_count(start: isize, end: isize, difference: isize) -> Result<isize, InvalidSeries> {
    if difference == 0 {
        Err(InvalidSeries)
    } else {
        if (end - start) % difference != 0 {
            Err(InvalidSeries)
        } else {
            Ok((end - start) / difference + 1)
        }
    }
}

fn sum(start: isize, difference: isize, terms: isize) -> isize {
    return (terms * start) + ((difference * (terms - 1) * (terms)) / 2);
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

    let start = matches.value_of("start").unwrap();
    let difference = matches.value_of("difference").unwrap();

    if matches.is_present("end") && start == matches.value_of("end").unwrap() && difference == "0" {
        println!("{}", start);
        return;
    }

    let start: isize = start.parse().unwrap();
    let difference: isize = difference.parse().unwrap();

    let terms: isize = if matches.is_present("end") {
        let end: isize = matches.value_of("end").unwrap().parse().unwrap();
        match term_count(start, end, difference) {
            Ok(s) => s,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(0x01);
            }
        }
    } else {
        matches.value_of("terms").unwrap().parse().unwrap()
    };

    println!("{}", sum(start, difference, terms));
    return;
}
