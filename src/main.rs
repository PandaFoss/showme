mod lib;

use std::fmt::Write;

extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ShowMe")
        .version("1.0")
        .author("Max Ferrer <maxi.fg13@gmail.com>")
        .about("Prints color patterns on the terminal")
        .after_help("Available patterns: bars, blocks1, bloks, crunch, panes")
        .arg(
            Arg::with_name("patterns")
                .short("p")
                .long("patterns")
                .value_name("PATTERN1,PATTERN2,...")
                .help("Prints the specified patterns")
                .required(true)
                .use_delimiter(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-blackwhite")
                .long("no-blackwhite")
                .help("Removes black and white colors from the color palette."),
        )
        .get_matches();

    let patterns: Vec<_> = matches.values_of("patterns").unwrap().collect();
    let bw: bool = !matches.is_present("no-blackwhite");
    let mut bad_patterns: Vec<_> = Vec::new();

    for pattern in patterns {
        match pattern {
            "bars" => println!("{}", lib::bars(bw)),
            "blocks1" => println!("{}", lib::blocks1(bw)),
            "bloks" => println!("{}", lib::bloks(bw)),
            "crunch" => println!("{}", lib::crunch(bw)),
            "panes" => println!("{}", lib::panes(bw)),
            _ => bad_patterns.push(pattern),
        }
    }

    if !bad_patterns.is_empty() {
        let mut s = String::new();
        for bad in bad_patterns {
            write!(s, " {}", bad)?;
        }
        write!(s, ": Unrecognized patterns.")?;
        println!("{}", s);
    }

    Ok(())
}
