use std::collections::BTreeMap;
use types::{Inline, Block, Citation, Pandoc, Meta, MetaValue};

pub trait Walkable<T> {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(T) -> T;
}

impl<U, T> Walkable<U> for Vec<T>
    where T : Walkable<U> {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(U) -> U {
        self.into_iter().map(|i| i.walk(f)).collect()
    }
}

impl<U, V> Walkable<U> for BTreeMap<String, V>
    where V : Walkable<U> {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(U) -> U {
        self.into_iter().map(|(k,v)| (k,v.walk(f))).collect()
    }
}

impl Walkable<Pandoc> for Pandoc {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Pandoc) -> Pandoc {
        f(self)
    }
}

impl Walkable<Block> for Pandoc {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Block) -> Block {
        Pandoc(self.0.walk(f), self.1.walk(f))
    }
}

impl Walkable<Inline> for Pandoc {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        Pandoc(self.0.walk(f), self.1.walk(f))
    }
}

impl Walkable<Meta> for Meta {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Meta) -> Meta {
        f(self)
    }
}

impl Walkable<Inline> for Meta {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        Meta { un_meta: self.un_meta.walk(f) }
    }
}

impl Walkable<Block> for Meta {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Block) -> Block {
        Meta { un_meta: self.un_meta.walk(f) }
    }
}

impl Walkable<MetaValue> for MetaValue {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(MetaValue) -> MetaValue {
        match self {
            MetaValue::MetaMap(map) => MetaValue::MetaMap(map.walk(f)),
            MetaValue::MetaList(values) => MetaValue::MetaList(values.walk(f)),
            e => e
        }
    }
}

impl Walkable<Block> for MetaValue {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Block) -> Block {
        match self {
            MetaValue::MetaMap(map) => MetaValue::MetaMap(map.walk(f)),
            MetaValue::MetaList(values) => MetaValue::MetaList(values.walk(f)),
            MetaValue::MetaBlocks(blocks) => MetaValue::MetaBlocks(blocks.walk(f)),
            e => e
        }
    }
}

impl Walkable<Inline> for MetaValue {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        match self {
            MetaValue::MetaMap(map) => MetaValue::MetaMap(map.walk(f)),
            MetaValue::MetaList(values) => MetaValue::MetaList(values.walk(f)),
            MetaValue::MetaInlines(inlines) => MetaValue::MetaInlines(inlines.walk(f)),
            MetaValue::MetaBlocks(blocks) => MetaValue::MetaBlocks(blocks.walk(f)),
            e => e
        }
    }
}


impl Walkable<Block> for Block {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Block) -> Block {
        match self {
            Block::BlockQuote(blocks) =>
                Block::BlockQuote(blocks.walk(f)),
            Block::OrderedList(list, vecs_blocks) =>
                Block::OrderedList(list, vecs_blocks.walk(f)),
            Block::BulletList(vecs_blocks) =>
                Block::BulletList(vecs_blocks.walk(f)),
            Block::DefinitionList(inlines_and_blocks) => {
                Block::DefinitionList(inlines_and_blocks
                                      .into_iter()
                                      .map(|(is, vbs)| (is, vbs.walk(f)))
                                      .collect())
            },
            Block::Table(inlines, alignment, width, headers, rows) => {
                Block::Table(inlines, alignment, width,
                             headers.walk(f),
                             rows.walk(f))
            },
            Block::Div(attr, blocks) => Block::Div(attr, blocks.walk(f)),
            e => e
        }
    }
}

impl Walkable<Inline> for Block {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        match self {
            Block::Plain(inlines) => Block::Plain(inlines.walk(f)),
            Block::Para(inlines) => Block::Para(inlines.walk(f)),
            Block::BlockQuote(blocks) =>
                Block::BlockQuote(blocks.walk(f)),
            Block::OrderedList(list, vecs_blocks) =>
                Block::OrderedList(list, vecs_blocks.walk(f)),
            Block::BulletList(vecs_blocks) =>
                Block::BulletList(vecs_blocks.walk(f)),
            Block::DefinitionList(inlines_and_blocks) => {
                Block::DefinitionList(inlines_and_blocks
                                      .into_iter()
                                      .map(|(is, vbs)| (is.walk(f), vbs.walk(f)))
                                      .collect())
            },
            Block::Header(i, attr, inlines) =>
                Block::Header(i, attr, inlines.walk(f)),
            Block::Table(inlines, alignment, width, headers, rows) => {
                Block::Table(inlines.walk(f), alignment, width,
                             headers.walk(f),
                             rows.walk(f))
            },
            Block::Div(attr, blocks) =>
                Block::Div(attr, blocks.walk(f)),
            b => b
        }
    }
}

impl Walkable<Inline> for Inline {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        match self {
            Inline::Emph(inlines) => Inline::Emph(inlines.walk(f)),
            Inline::Strong(inlines) => Inline::Strong(inlines.walk(f)),
            Inline::Strikeout(inlines) => Inline::Strikeout(inlines.walk(f)),
            Inline::Superscript(inlines) => Inline::Superscript(inlines.walk(f)),
            Inline::Subscript(inlines) => Inline::Subscript(inlines.walk(f)),
            Inline::SmallCaps(inlines) => Inline::SmallCaps(inlines.walk(f)),
            Inline::Quoted(quote_type, inlines) => Inline::Quoted(quote_type, inlines.walk(f)),
            Inline::Cite(citations, inlines) => Inline::Cite(citations.walk(f), inlines.walk(f)),
            Inline::Link(attr, inlines, target) => Inline::Link(attr, inlines.walk(f), target),
            Inline::Image(attr, inlines, target) => Inline::Image(attr, inlines.walk(f), target),
            Inline::Span(attr, inlines) => Inline::Span(attr, inlines.walk(f)),
            e => f(e)
        }
    }
}

impl Walkable<Inline> for Citation {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        Citation { citation_id: self.citation_id,
                   citation_prefix: self.citation_prefix.walk(f),
                   citation_suffix: self.citation_suffix.walk(f),
                   citation_mode: self.citation_mode,
                   citation_note_num: self.citation_note_num,
                   citation_hash: self.citation_hash
        }
    }
}

#[cfg(test)]
mod tests {
    use walk::Walkable;
    use types::Inline;

    #[test]
    fn test_walk_inline() {
        let inline_str = Inline::Str(String::from("a"));
        let emph = inline_str.walk(&(|s| Inline::Emph(vec![s])));
        let expected = Inline::Emph(vec![Inline::Str(String::from("a"))]);
        assert_eq!(emph, expected);
    }
}
