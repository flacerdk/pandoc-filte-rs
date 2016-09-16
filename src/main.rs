extern crate serde;
extern crate serde_json;
extern crate pandoc_filters;
use pandoc_filters::deserialize;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut fh;
    if let Some(arg) = env::args().skip(1).next() {
        fh = match File::open(arg) {
            Ok(f) => f,
            Err(e) => panic!("Couldn't read file: {}", e)
        };
    } else {
        panic!("Please provide a file to read");
    };

    let mut markdown = String::new();
    fh.read_to_string(&mut markdown).unwrap();
    let pandoc = deserialize(markdown).unwrap();
    println!("{:?}", pandoc);
}

