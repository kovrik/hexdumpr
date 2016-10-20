extern crate getopts;
use getopts::Options;

use std::fs::File;
use std::env;
use std::io::prelude::*;

mod lib;
use lib::*;

fn main() {

    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print help");
    opts.optopt("s", "offset", "", "Skip offset bytes from the beginning of the input");
    opts.optopt("n", "length", "", "Interpret only length bytes of input");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage();
        return;
    }

    // offset in bytes
    let offset: usize = if matches.opt_present("s") {
        match matches.opt_str("s") {
            Some(s) => {
                match s.parse::<usize>() {
                    Ok(s)  => s,
                    Err(s) => panic!("hexdumpr: failed to parse offset: {}", s),
                }
            },
            None => panic!("hexdumpr: failed to parse offset!"),
        }
    } else {
        0
    };

    // get filename
    let filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage();
        return;
    };
    print!("{}:", filename);

    let mut f = File::open(&filename).expect("Unable to open file");
    let mut data = Vec::new();
    // TODO optimize
    f.read_to_end(&mut data).expect("Unable to read data");

    if offset > data.len() {
        return;
    }

    // length in bytes
    let mut end: usize = data.len();
    if matches.opt_present("n") {
        let length = match matches.opt_str("n") {
            Some(s) => {
                match s.parse::<usize>() {
                    Ok(s)  => s,
                    Err(s) => panic!("hexdumpr: failed to parse length: {}", s),
                }
            },
            None => panic!("hexdumpr: failed to parse length!"),
        };
        if length < data.len() - offset {
            end = offset + length;
        }
    }
    if end == 0 {
        return;
    }

    print_hexdump(&data, offset, end);
}

fn print_usage() {
    println!("Usage: hexdumpr [-s offset][-n length] file ...");
}