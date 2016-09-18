extern crate serde;
extern crate serde_json;
extern crate pandoc_filters;
use pandoc_filters::json::filter;
use pandoc_filters::types::Inline;

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

    let mut json = String::new();
    fh.read_to_string(&mut json).unwrap();
    let new_json = filter(json, &f).unwrap();
    println!("{}", new_json);
}

fn f(inline: Inline) -> Inline {
    match inline {
        Inline::Str(s) => Inline::Emph(vec![Inline::Str(s)]),
        e => e
    }
}
