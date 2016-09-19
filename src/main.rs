extern crate serde;
extern crate serde_json;
extern crate pandoc_filters;
use pandoc_filters::json::filter;
use pandoc_filters::types::Inline;

use std::io::{self, Read};

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json).unwrap();
    let new_json = filter(json, &to_upper).unwrap();
    println!("{}", new_json);
}

fn to_upper(inline: Inline) -> Inline {
    match inline {
        Inline::Str(s) => Inline::Str(s.to_uppercase()),
        e => e
    }
}
