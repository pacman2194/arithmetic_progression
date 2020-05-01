#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fmt;

// Struct to handle errors related to inputs providing invalid series
#[derive(Debug)]
pub struct InvalidSeries;

impl fmt::Display for InvalidSeries {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "The arguments provided do not form a valid series.")
    }
}

impl Error for InvalidSeries {}

// Validation check that an input appears in the form of an integer
fn is_integer(s: String) -> Result<(), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[-]?[\d]+$").unwrap();
    }
    if RE.is_match(&s) {
        Ok(())
    } else {
        Err(String::from(
            "All arguments that take values must be integers.",
        ))
    }
}

// Calculate the number of terms in the series
fn term_count(start: isize, end: isize, difference: isize) -> Result<isize, InvalidSeries> {
    if difference == 0 || (end - start) % difference != 0 {
        // Difference of 0 means infinite loop since check is performed
        // outside this function if start and end are equal. If remainder
        // from dividing the difference of the start and end term by the
        // difference between each term then this is an invalid series.
        Err(InvalidSeries)
    } else {
        let terms = (end - start) / difference + 1;
        if terms > 0 {
            Ok(terms)
        } else {
            // If the number of terms is less than 1, then the value given
            // for the difference between each term moves the starting term
            // further from the end term
            Err(InvalidSeries)
        }
    }
}

// Calculate the sum of the series
fn sum(start: isize, difference: isize, terms: isize) -> isize {
    (terms * start) + ((difference * (terms - 1) * (terms)) / 2)
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

    // Fetch the starting term and difference
    let start = matches.value_of("start").unwrap();
    let difference = matches.value_of("difference").unwrap();

    // Check for valid single term series
    if matches.is_present("end") && start == matches.value_of("end").unwrap() && difference == "0" {
        println!("{}", start);
        return;
    }

    // Shadow start and difference to integers
    let start: isize = start.parse().unwrap();
    let difference: isize = difference.parse().unwrap();

    // Calculate terms if necessary, otherwise fetch the number of terms from args.
    let terms: isize = if matches.is_present("end") {
        let end: isize = matches.value_of("end").unwrap().parse().unwrap();
        match term_count(start, end, difference) {
            Ok(s) => s,
            Err(error) => {
                // InvalidSeries error, exit not so gracefully
                eprintln!("{}", error);
                std::process::exit(0x01);
            }
        }
    } else {
        // Number of terms was given instead of end term
        matches.value_of("terms").unwrap().parse().unwrap()
    };

    // Return the calculated sum of the series
    println!("{}", sum(start, difference, terms));
}
