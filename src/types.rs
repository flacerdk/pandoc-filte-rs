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

macro_rules! serialize_enum {
    ($name:ident,
     units = { $( $unit:ident ),* },
     newtypes = { $( $newtype:ident[$val_ident:ident, $newtype_val:ty] ),* },
     tuples = { $( $tuple:ident[$( $el_ident:ident=$tuple_el:ty ),*] ),* }) => {
        #[derive(PartialEq, Debug, Deserialize)]
        pub enum $name {
            $( $unit, )*
            $( $newtype($newtype_val), )*
            $( $tuple($( $tuple_el ),*), )*
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: Serializer {
                match *self {
                    $(
                        $name::$unit => {
                            let mut state = try!(serializer.serialize_map(Some(1)));
                            try!(serializer.serialize_map_key(&mut state, stringify!($unit)));
                            let v: Vec<String> = Vec::new();
                            try!(serializer.serialize_map_value(&mut state, v));
                            serializer.serialize_map_end(state)
                        },
                    )*
                    $(
                        $name::$newtype(ref $val_ident) =>
                            serializer.serialize_newtype_variant(stringify!($name), 0,
                                                                 stringify!($newtype), $val_ident),
                    )*
                    $(
                        $name::$tuple( $( ref $el_ident ),* ) => {
                            let mut state = try!(serializer.serialize_tuple_variant(stringify!($name), 0,
                                                                                    stringify!($tuple), 2));
                            $(
                                try!(serializer.serialize_tuple_variant_elt(&mut state, $el_ident));
                            )*
                                serializer.serialize_tuple_variant_end(state)
                        },
                    )*
                }
            }
        }
    }
}

serialize_enum!(
    ListNumberStyle,
    units = {
        DefaultStyle,
        Example,
        Decimal,
        LowerRoman,
        UpperRoman,
        LowerAlpha,
        UpperAlpha
    },
    newtypes = {}, tuples = {}
);

serialize_enum!(
    ListNumberDelim,
    units = {
        DefaultDelim,
        Period,
        OneParen,
        TwoParens
    },
    newtypes = {}, tuples = {}
);

serialize_enum!(
    Alignment,
    units = {
        AlignLeft,
        AlignRight,
        AlignCenter,
        AlignDefault
    },
    newtypes = {}, tuples = {}
);

type TableCell = Vec<Block>;

// http://hackage.haskell.org/package/pandoc-types-1.16.1.1/docs/Text-Pandoc-Definition.html#t:Inline
serialize_enum!(
    Inline,
    units = {
        Space,
        SoftBreak,
        LineBreak
    },
    newtypes = {
        Str[s, String],
        Emph[v, Vec<Inline>],
        Strong[v, Vec<Inline>],
        Strikeout[v, Vec<Inline>],
        Superscript[v, Vec<Inline>],
        Subscript[v, Vec<Inline>],
        SmallCaps[v, Vec<Inline>]
    },
    tuples = {
        Quoted[q = QuoteType, v = Vec<Inline>],
        Cite[c = Vec<Citation>, v = Vec<Inline>],
        Code[a=Attr, s=String],
        Math[t=MathType, s=String],
        RawInline[f=Format, s=String],
        Link[a=Attr, v=Vec<Inline>, t=Target],
        Image[a=Attr, v=Vec<Inline>, t=Target],
        Span[a=Attr, v=Vec<Inline>]
    }
);

serialize_enum!(
    QuoteType,
    units = {
        SingleQuote,
        DoubleQuote
    },
    newtypes = {}, tuples = {}
);

serialize_enum!(
    MathType,
    units = {
        DisplayMath,
        InlineMath
    },
    newtypes = {}, tuples = {}
);

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

