extern crate serde;
extern crate serde_json;
extern crate pandoc_filters;
use pandoc_filters::json::filter;
use pandoc_filters::types::{Inline, Block};

use std::io::{self, Read};

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json).unwrap();
    let new_json = filter(json, &behead).unwrap();
    println!("{}", new_json);
}

fn to_upper(inline: Inline) -> Inline {
    match inline {
        Inline::Str(s) => Inline::Str(s.to_uppercase()),
        e => e
    }
}

fn behead(header: Block) -> Block {
    match header {
        Block::Header(level, attrs, inlines) => {
            if level >= 2 {
                Block::Para(vec![Inline::Emph(inlines)])
            } else {
                Block::Header(level, attrs, inlines)
            }
        },
        e => e
    }
}
