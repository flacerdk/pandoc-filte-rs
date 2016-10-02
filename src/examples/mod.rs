use types::{Inline, Block};

pub fn to_upper(inline: Inline) -> Inline {
    match inline {
        Inline::Str(s) => Inline::Str(s.to_uppercase()),
        e => e
    }
}

pub fn behead(header: Block) -> Block {
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
