use std::fs::File;
use std::env;
use std::io::prelude::*;

mod lib;
use lib::*;

// TODO usage and help instructions
// TODO different options
fn main() {
    let filename = env::args().nth(1).unwrap_or("src/main.rs".to_string());
    print!("{}:", filename);

    let mut f = File::open(&filename).expect("Unable to open file");
    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Unable to read data");
    print_hexdump(&data);
}