extern crate serde;
extern crate serde_json;
extern crate pandoc_filters;
use pandoc_filters::json::filter;
use pandoc_filters::examples::behead;

use std::io::{self, Read};

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json).unwrap();
    let new_json = filter(json, &behead).unwrap();
    println!("{}", new_json);
}

