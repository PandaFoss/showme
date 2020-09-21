mod lib;

use std::fmt::Write;

extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ShowMe")
        .version("0.1.0")
        .author("Max Ferrer <maxi.fg13@gmail.com>")
        .about("Prints color patterns on the terminal")
        .after_help("Available patterns: arch, bars, blocks1, bloks, crunch, crunchbang, panes")
        .arg(
            Arg::with_name("patterns")
                .short("p")
                .long("patterns")
                .value_name("PATTERN1,PATTERN2,...")
                .help("Prints the specified patterns")
                .required_unless("all-patterns")
                .use_delimiter(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all-patterns")
                .long("all-patterns")
                .conflicts_with("patterns")
                .help("Print all patterns.")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no-blackwhite")
                .long("no-blackwhite")
                .help("Removes black and white colors from the color palette."),
        )
        .get_matches();

    let bw: bool = !matches.is_present("no-blackwhite");
    let mut bad_patterns: Vec<_> = Vec::new();

    if matches.is_present("patterns") {
        let patterns: Vec<_> = matches.values_of("patterns").unwrap().collect();
        for pattern in patterns {
            match pattern {
                "arch" => println!("{}", lib::arch(bw)),
                "bars" => println!("{}", lib::bars(bw)),
                "blocks1" => println!("{}", lib::blocks1(bw)),
                "bloks" => println!("{}", lib::bloks(bw)),
                "crunch" => println!("{}", lib::crunch(bw)),
                "crunchbang" => println!("{}", lib::crunchbang(bw)),
                "panes" => println!("{}", lib::panes(bw)),
                _ => bad_patterns.push(pattern),
            }
        }
    }

    if matches.is_present("all-patterns") {
        println!("Arch: \n\n{}\n", lib::arch(bw));
        println!("Bars: \n\n{}\n", lib::bars(bw));
        println!("Blocks1: \n\n{}\n", lib::blocks1(bw));
        println!("Bloks: \n\n{}\n", lib::bloks(bw));
        println!("Crunch: \n\n{}\n", lib::crunch(bw));
        println!("Crunchbang: \n\n{}\n", lib::crunchbang(bw));
        println!("Panes: \n\n{}\n", lib::panes(bw));
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
