use types::{Inline, Block};

trait Walkable<T> {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(T) -> T;
}

impl<T> Walkable<T> for T {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Self) -> Self {
        f(self)
    }
}

impl Walkable<Inline> for Block {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        match self {
            Block::Plain(inlines) => Block::Plain(inlines.walk(f)),
            Block::Para(inlines) => Block::Plain(inlines.walk(f)),
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

impl<T> Walkable<Inline> for Vec<T>
    where T : Walkable<Inline> {
    fn walk<F>(self, f: &F) -> Self
        where F : Fn(Inline) -> Inline {
        self.into_iter().map(|i| i.walk(f)).collect()
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
