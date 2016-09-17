use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc {
    pub meta: Meta,
    pub blocks: Vec<Block>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "unMeta")]
    pub un_meta: BTreeMap<String, MetaValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MetaValue {
    MetaMap(BTreeMap<String, MetaValue>),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>)
}

// http://hackage.haskell.org/package/pandoc-types-1.16.1.1/docs/Text-Pandoc-Definition.html#t:Block
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Block {
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    BlockQuote(Vec<Block>),
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header(u64, Attr, Vec<Inline>),
    HorizontalRule,
    Table(Vec<Inline>, Vec<Alignment>, Vec<f64>, Vec<TableCell>, Vec<Vec<TableCell>>),
    Div(Attr, Vec<Block>),
    Null
}

pub type ListAttributes = (u64, ListNumberStyle, ListNumberDelim);
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault
}

type TableCell = Vec<Block>;

// http://hackage.haskell.org/package/pandoc-types-1.16.1.1/docs/Text-Pandoc-Definition.html#t:Inline
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Inline {
    Str(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    Quoted(QuoteType, Vec<Inline>),
    Cite(Vec<Citation>, Vec<Inline>),
    Code(Attr, String),
    Space,
    SoftBreak,
    LineBreak,
    Math(MathType, String),
    RawInline(Format, String),
    Link(Attr, Vec<Inline>, Target),
    Image(Attr, Vec<Inline>, Target),
    Span(Attr, Vec<Inline>)
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum MathType {
    DisplayMath,
    InlineMath
}

pub type Format = String;
pub type Attr = (String, Vec<String>, Vec<(String, String)>);
pub type Target = (String, String);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Citation {
    #[serde(rename = "citationId")]
    citation_id: String,
    #[serde(rename = "citationPrefix")]
    citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")]
    citation_suffix: Vec<Inline>,
    #[serde(rename = "citationmode")]
    citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    citation_note_num: u64,
    #[serde(rename = "citationHash")]
    citation_hash: u64
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation
}

