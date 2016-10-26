extern crate getopts;
use getopts::Options;

use std::fs::File;
use std::env;
use std::io::prelude::*;

mod lib;
use lib::*;

fn print_usage() {
    println!("Usage: hexdumpr [-bcCdox][-s offset][-n length] file ...");
}

fn main() {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "offset", "", "Skip offset bytes from the beginning of the input");
    opts.optopt("n", "length", "", "Interpret only length bytes of input");
    opts.optflag("h", "help", "Print help");
    opts.optflag("b", "one-byte-octal", "One-byte octal display");
    opts.optflag("c", "one-byte-char",  "One-byte character display");
    opts.optflag("C", "canonical-hex",  "Canonical hex display");
    opts.optflag("x", "two-byte-hex",   "Two-byte hexadecimal display");
    opts.optflag("d", "two-byte-dec",   "Two-byte decimal display");
    opts.optflag("o", "two-byte-octal", "Two-byte octal display");
    let args = opts.parse(&args[1..]).unwrap_or_else(|e| panic!(e.to_string()));
    if args.opt_present("h") {
        print_usage();
        return;
    }

    // offset in bytes
    let offset: usize = match args.opt_present("s") {
        true => args.opt_str("s").unwrap().parse::<usize>().unwrap_or_else(|e| panic!("hexdumpr: failed to parse offset: {}", e)),
        _    => 0,
    };

    // get filename
    let filename = if !args.free.is_empty() {
        args.free[0].clone()
    } else {
        print_usage();
        return;
    };
    print!("{}:", filename);
    let mut f = File::open(&filename).expect("Unable to open file");
    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Unable to read data");
    if data.len() < offset { return }

    // length in bytes
    let mut end = data.len();
    if args.opt_present("n") {
        let length = args.opt_str("n").unwrap().parse::<usize>().unwrap_or_else(|e| panic!("hexdumpr: failed to parse length: {}", e));
        if length < data.len() - offset {
            end = offset + length;
        }
    }
    if end == 0 { return }

    // display mode
    let bytes;
    let display;
    match () {
        _ if args.opt_present("b") => { display = 'b'; bytes = 1; }
        _ if args.opt_present("c") => { display = 'c'; bytes = 1; }
        _ if args.opt_present("C") => { display = 'C'; bytes = 1; }
        _ if args.opt_present("d") => { display = 'd'; bytes = 2; }
        _ if args.opt_present("o") => { display = 'o'; bytes = 2; }
        _                          => { display = 'x'; bytes = 2; }
    }
    print_hexdump(&data[offset..end], offset, display, bytes);
}