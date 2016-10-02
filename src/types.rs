use std::collections::BTreeMap;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc(pub Meta, pub Vec<Block>);

// TODO: add tests
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
// TODO: add tests
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

// TODO: add tests
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

// TODO: add tests
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

#[cfg(test)]
mod tests {
    use serde_json::ser::to_string;
    use types::*;
    use std::collections::BTreeMap;

    fn inline_base_val() -> Inline {
        Inline::Str(String::from("test"))
    }

    fn meta_base_val() -> MetaValue {
        MetaValue::MetaString(String::from("test"))
    }

    fn block_base_val() -> Block {
        Block::Plain(vec![inline_base_val()])
    }

    fn attr_base_val() -> Attr {
        (String::from("test"), vec![String::from("test")],
         vec![(String::from("test"), String::from("test"))])
    }

    fn list_attributes_base_val() -> ListAttributes {
        (0, ListNumberStyle::DefaultStyle, ListNumberDelim::DefaultDelim)
    }

    #[test]
    fn serialize_meta_value() {
        let mut map = BTreeMap::new();
        map.insert(String::from("test"), meta_base_val());
        assert_eq!(to_string(&MetaValue::MetaMap(map)).unwrap(),
                   r#"{"MetaMap":{"test":{"MetaString":"test"}}}"#);
        assert_eq!(to_string(&MetaValue::MetaList(vec![meta_base_val()])).unwrap(),
                   r#"{"MetaList":[{"MetaString":"test"}]}"#);
        assert_eq!(to_string(&MetaValue::MetaBool(true)).unwrap(),
                   r#"{"MetaBool":true}"#);
        assert_eq!(to_string(&meta_base_val()).unwrap(),
                   r#"{"MetaString":"test"}"#);
        assert_eq!(to_string(&MetaValue::MetaInlines(vec![inline_base_val()])).unwrap(),
                   r#"{"MetaInlines":[{"Str":"test"}]}"#);
        assert_eq!(to_string(&MetaValue::MetaBlocks(vec![block_base_val()])).unwrap(),
                   r#"{"MetaBlocks":[{"Plain":[{"Str":"test"}]}]}"#);
    }

    #[test]
    fn serialize_block() {
        assert_eq!(to_string(&block_base_val()).unwrap(),
                   r#"{"Plain":[{"Str":"test"}]}"#);
        assert_eq!(to_string(&Block::Para(vec![inline_base_val()])).unwrap(),
                   r#"{"Para":[{"Str":"test"}]}"#);
        assert_eq!(to_string(&Block::CodeBlock(attr_base_val(), String::from("test"))).unwrap(),
                   r#"{"CodeBlock":[["test",["test"],[["test","test"]]],"test"]}"#);
        assert_eq!(to_string(&Block::RawBlock(String::from("test"), String::from("test"))).unwrap(),
                   r#"{"RawBlock":["test","test"]}"#);
        assert_eq!(to_string(&Block::BlockQuote(vec![block_base_val()])).unwrap(),
                   r#"{"BlockQuote":[{"Plain":[{"Str":"test"}]}]}"#);
        assert_eq!(to_string(&Block::OrderedList(list_attributes_base_val(),
                                                 vec![vec![block_base_val()]])).unwrap(),
                   r#"{"OrderedList":[[0,{"DefaultStyle":[]},{"DefaultDelim":[]}],[[{"Plain":[{"Str":"test"}]}]]]}"#);
        assert_eq!(to_string(&Block::BulletList(vec![vec![block_base_val()]])).unwrap(),
                   r#"{"BulletList":[[{"Plain":[{"Str":"test"}]}]]}"#);
        assert_eq!(to_string(&Block::DefinitionList(vec![(vec![inline_base_val()],
                                                          vec![vec![block_base_val()]])])).unwrap(),
                   r#"{"DefinitionList":[[[{"Str":"test"}],[[{"Plain":[{"Str":"test"}]}]]]]}"#);
        assert_eq!(to_string(&Block::Header(0, attr_base_val(), vec![inline_base_val()])).unwrap(),
                   r#"{"Header":[0,["test",["test"],[["test","test"]]],[{"Str":"test"}]]}"#);
        assert_eq!(to_string(&Block::HorizontalRule).unwrap(),
                   "\"HorizontalRule\"");
        assert_eq!(to_string(&Block::Table(vec![inline_base_val()], vec![Alignment::AlignLeft],
                                           vec![0.0], vec![vec![block_base_val()]],
                                           vec![vec![vec![block_base_val()]]])).unwrap(),
                   r#"{"Table":[[{"Str":"test"}],[{"AlignLeft":[]}],[0.0],[[{"Plain":[{"Str":"test"}]}]],[[[{"Plain":[{"Str":"test"}]}]]]]}"#);
        assert_eq!(to_string(&Block::Div(attr_base_val(), vec![block_base_val()])).unwrap(),
                   r#"{"Div":[["test",["test"],[["test","test"]]],[{"Plain":[{"Str":"test"}]}]]}"#);
        assert_eq!(to_string(&Block::Null).unwrap(),
                   "\"Null\"");
    }

    #[test]
    fn serialize_citation_mode() {
        assert_eq!(to_string(&CitationMode::AuthorInText).unwrap(), "\"AuthorInText\"");
        assert_eq!(to_string(&CitationMode::SuppressAuthor).unwrap(), "\"SuppressAuthor\"");
        assert_eq!(to_string(&CitationMode::NormalCitation).unwrap(), "\"NormalCitation\"");
    }

    #[test]
    fn serialize_mathtype() {
        assert_eq!(to_string(&MathType::DisplayMath).unwrap(), r#"{"DisplayMath":[]}"#);
        assert_eq!(to_string(&MathType::InlineMath).unwrap(), r#"{"InlineMath":[]}"#);
    }

    #[test]
    fn serialize_quotetype() {
        assert_eq!(to_string(&QuoteType::SingleQuote).unwrap(), r#"{"SingleQuote":[]}"#);
        assert_eq!(to_string(&QuoteType::DoubleQuote).unwrap(), r#"{"DoubleQuote":[]}"#);
    }

    #[test]
    fn serialize_alignment() {
        assert_eq!(to_string(&Alignment::AlignLeft).unwrap(), r#"{"AlignLeft":[]}"#);
        assert_eq!(to_string(&Alignment::AlignRight).unwrap(), r#"{"AlignRight":[]}"#);
        assert_eq!(to_string(&Alignment::AlignCenter).unwrap(), r#"{"AlignCenter":[]}"#);
        assert_eq!(to_string(&Alignment::AlignDefault).unwrap(), r#"{"AlignDefault":[]}"#);
    }

    #[test]
    fn serialize_list_number_delim() {
        assert_eq!(to_string(&ListNumberDelim::DefaultDelim).unwrap(), r#"{"DefaultDelim":[]}"#);
        assert_eq!(to_string(&ListNumberDelim::Period).unwrap(), r#"{"Period":[]}"#);
        assert_eq!(to_string(&ListNumberDelim::OneParen).unwrap(), r#"{"OneParen":[]}"#);
        assert_eq!(to_string(&ListNumberDelim::TwoParens).unwrap(), r#"{"TwoParens":[]}"#);
    }

    #[test]
    fn serialize_list_number_style() {
        assert_eq!(to_string(&ListNumberStyle::DefaultStyle).unwrap(), r#"{"DefaultStyle":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::Example).unwrap(), r#"{"Example":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::Decimal).unwrap(), r#"{"Decimal":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::LowerRoman).unwrap(), r#"{"LowerRoman":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::UpperRoman).unwrap(), r#"{"UpperRoman":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::LowerAlpha).unwrap(), r#"{"LowerAlpha":[]}"#);
        assert_eq!(to_string(&ListNumberStyle::UpperAlpha).unwrap(), r#"{"UpperAlpha":[]}"#);
    }
}
