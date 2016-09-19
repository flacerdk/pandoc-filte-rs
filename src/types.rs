use std::collections::BTreeMap;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc(pub Meta, pub Vec<Block>);

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
#[derive(PartialEq, Debug, Deserialize)]
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

impl Serialize for Inline {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer {
        match *self {
            Inline::Str(ref s) => serializer.serialize_newtype_variant("Inline", 0, "Str", s),
            Inline::Emph(ref is) => serializer.serialize_newtype_variant("Inline", 1, "Emph", is),
            Inline::Strong(ref is) => serializer.serialize_newtype_variant("Inline", 2, "Strong", is),
            Inline::Strikeout(ref is) => serializer.serialize_newtype_variant("Inline", 3, "Strikeout", is),
            Inline::Superscript(ref is) => serializer.serialize_newtype_variant("Inline", 4, "Superscript", is),
            Inline::Subscript(ref is) => serializer.serialize_newtype_variant("Inline", 5, "Subscript", is),
            Inline::SmallCaps(ref is) => serializer.serialize_newtype_variant("Inline", 6, "SmallCaps", is),
            Inline::Quoted(ref quote_type, ref is) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 7, "Quoted", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, quote_type));
                try!(serializer.serialize_tuple_variant_elt(&mut state, is));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Cite(ref cites, ref is) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 8, "Cite", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, cites));
                try!(serializer.serialize_tuple_variant_elt(&mut state, is));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Code(ref attr, ref s) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 9, "Code", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, attr));
                try!(serializer.serialize_tuple_variant_elt(&mut state, s));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Space => {
                let mut state = try!(serializer.serialize_map(Some(1)));
                try!(serializer.serialize_map_key(&mut state, "Space"));
                let v: Vec<String> = Vec::new();
                try!(serializer.serialize_map_value(&mut state, v));
                serializer.serialize_map_end(state)
            },
            Inline::SoftBreak => {
                let mut state = try!(serializer.serialize_map(Some(1)));
                try!(serializer.serialize_map_key(&mut state, "SoftBreak"));
                let v: Vec<String> = Vec::new();
                try!(serializer.serialize_map_value(&mut state, v));
                serializer.serialize_map_end(state)
            },
            Inline::LineBreak => {
                let mut state = try!(serializer.serialize_map(Some(1)));
                try!(serializer.serialize_map_key(&mut state, "LineBreak"));
                let v: Vec<String> = Vec::new();
                try!(serializer.serialize_map_value(&mut state, v));
                serializer.serialize_map_end(state)
            },
            Inline::Math(ref math_type, ref s) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 13, "Math", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, math_type));
                try!(serializer.serialize_tuple_variant_elt(&mut state, s));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::RawInline(ref format, ref s) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 14, "RawInline", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, format));
                try!(serializer.serialize_tuple_variant_elt(&mut state, s));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Link(ref attr, ref is, ref target) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 15, "Link", 3));
                try!(serializer.serialize_tuple_variant_elt(&mut state, attr));
                try!(serializer.serialize_tuple_variant_elt(&mut state, is));
                try!(serializer.serialize_tuple_variant_elt(&mut state, target));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Image(ref attr, ref is, ref target) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 16, "Image", 3));
                try!(serializer.serialize_tuple_variant_elt(&mut state, attr));
                try!(serializer.serialize_tuple_variant_elt(&mut state, is));
                try!(serializer.serialize_tuple_variant_elt(&mut state, target));
                serializer.serialize_tuple_variant_end(state)
            },
            Inline::Span(ref attr, ref is) => {
                let mut state = try!(serializer.serialize_tuple_variant("Inline", 17, "Span", 2));
                try!(serializer.serialize_tuple_variant_elt(&mut state, attr));
                try!(serializer.serialize_tuple_variant_elt(&mut state, is));
                serializer.serialize_tuple_variant_end(state)
            },
        }
    }
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
    pub citation_id: String,
    #[serde(rename = "citationPrefix")]
    pub citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")]
    pub citation_suffix: Vec<Inline>,
    #[serde(rename = "citationmode")]
    pub citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    pub citation_note_num: u64,
    #[serde(rename = "citationHash")]
    pub citation_hash: u64
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation
}

