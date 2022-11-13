#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    #[allow(dead_code)]
    if args().len() != 3 {
        eprintln!("Usage: `source`, `target`");
    }

    let mut input = BufReader::new(File::open(args().nth(1)).unwrap());

}
