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
        Quoted[q=QuoteType, v=Vec<Inline>],
        Cite[c=Vec<Citation>, v=Vec<Inline>],
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
    #[serde(rename = "citationMode")]
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

    macro_rules! test_serialize {
        ($( $value:expr => $expected:expr ),*) => {
            $(
                assert_eq!(to_string(&$value).unwrap(), $expected);
            )*
        }
    }

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

    fn citation_base_val() -> Citation {
        Citation {
            citation_id: String::from("test"),
            citation_prefix: vec![inline_base_val()],
            citation_suffix: vec![inline_base_val()],
            citation_mode: CitationMode::AuthorInText,
            citation_note_num: 0,
            citation_hash: 0
        }
    }

    #[test]
    fn serialize_meta_value() {
        let mut map = BTreeMap::new();
        map.insert(String::from("test"), meta_base_val());
        test_serialize!(
            MetaValue::MetaMap(map) => r#"{"MetaMap":{"test":{"MetaString":"test"}}}"#,
            MetaValue::MetaList(vec![meta_base_val()]) => r#"{"MetaList":[{"MetaString":"test"}]}"#,
            MetaValue::MetaBool(true) => r#"{"MetaBool":true}"#,
            meta_base_val() => r#"{"MetaString":"test"}"#,
            MetaValue::MetaInlines(vec![inline_base_val()]) => r#"{"MetaInlines":[{"Str":"test"}]}"#,
            MetaValue::MetaBlocks(vec![block_base_val()]) => r#"{"MetaBlocks":[{"Plain":[{"Str":"test"}]}]}"#
        );
    }

    #[test]
    fn serialize_block() {
        test_serialize!(
            block_base_val() => r#"{"Plain":[{"Str":"test"}]}"#,
            Block::Para(vec![inline_base_val()]) => r#"{"Para":[{"Str":"test"}]}"#,
            Block::CodeBlock(attr_base_val(), String::from("test")) => 
                   r#"{"CodeBlock":[["test",["test"],[["test","test"]]],"test"]}"#,
            Block::RawBlock(String::from("test"), String::from("test")) =>
                r#"{"RawBlock":["test","test"]}"#,
            Block::BlockQuote(vec![block_base_val()]) =>
                r#"{"BlockQuote":[{"Plain":[{"Str":"test"}]}]}"#,
            Block::OrderedList(list_attributes_base_val(), vec![vec![block_base_val()]]) =>
                r#"{"OrderedList":[[0,{"DefaultStyle":[]},{"DefaultDelim":[]}],[[{"Plain":[{"Str":"test"}]}]]]}"#,
            Block::BulletList(vec![vec![block_base_val()]]) =>
                r#"{"BulletList":[[{"Plain":[{"Str":"test"}]}]]}"#,
            Block::DefinitionList(vec![(vec![inline_base_val()], vec![vec![block_base_val()]])]) =>
                r#"{"DefinitionList":[[[{"Str":"test"}],[[{"Plain":[{"Str":"test"}]}]]]]}"#,
            Block::Header(0, attr_base_val(), vec![inline_base_val()]) =>
                r#"{"Header":[0,["test",["test"],[["test","test"]]],[{"Str":"test"}]]}"#,
            Block::HorizontalRule => "\"HorizontalRule\"",
            Block::Table(vec![inline_base_val()], vec![Alignment::AlignLeft],
                         vec![0.0], vec![vec![block_base_val()]],
                         vec![vec![vec![block_base_val()]]]) =>
                r#"{"Table":[[{"Str":"test"}],[{"AlignLeft":[]}],[0.0],[[{"Plain":[{"Str":"test"}]}]],[[[{"Plain":[{"Str":"test"}]}]]]]}"#,
            Block::Div(attr_base_val(), vec![block_base_val()]) =>
                r#"{"Div":[["test",["test"],[["test","test"]]],[{"Plain":[{"Str":"test"}]}]]}"#,
            Block::Null => "\"Null\""
        );
    }

    #[test]
    fn serialize_inline() {
        test_serialize!(
            Inline::Space => r#"{"Space":[]}"#,
            Inline::SoftBreak => r#"{"SoftBreak":[]}"#,
            Inline::LineBreak => r#"{"LineBreak":[]}"#,
            inline_base_val() => r#"{"Str":"test"}"#,
            Inline::Emph(vec![inline_base_val()]) => r#"{"Emph":[{"Str":"test"}]}"#,
            Inline::Strong(vec![inline_base_val()]) => r#"{"Strong":[{"Str":"test"}]}"#,
            Inline::Strikeout(vec![inline_base_val()]) => r#"{"Strikeout":[{"Str":"test"}]}"#,
            Inline::Superscript(vec![inline_base_val()]) => r#"{"Superscript":[{"Str":"test"}]}"#,
            Inline::Subscript(vec![inline_base_val()]) => r#"{"Subscript":[{"Str":"test"}]}"#,
            Inline::SmallCaps(vec![inline_base_val()]) => r#"{"SmallCaps":[{"Str":"test"}]}"#,
            Inline::Quoted(QuoteType::SingleQuote, vec![inline_base_val()]) =>
                r#"{"Quoted":[{"SingleQuote":[]},[{"Str":"test"}]]}"#,
            Inline::Cite(vec![citation_base_val()], vec![inline_base_val()]) =>
                r#"{"Cite":[[{"citationId":"test","citationPrefix":[{"Str":"test"}],"citationSuffix":[{"Str":"test"}],"citationMode":"AuthorInText","citationNoteNum":0,"citationHash":0}],[{"Str":"test"}]]}"#,
            Inline::Code(attr_base_val(), String::from("test")) =>
                r#"{"Code":[["test",["test"],[["test","test"]]],"test"]}"#,
            Inline::Math(MathType::DisplayMath, String::from("test")) =>
                r#"{"Math":[{"DisplayMath":[]},"test"]}"#,
            Inline::RawInline(String::from("test"), String::from("test")) =>
                r#"{"RawInline":["test","test"]}"#,
            Inline::Link(attr_base_val(), vec![inline_base_val()],
                         (String::from("test"), String::from("test"))) =>
                r#"{"Link":[["test",["test"],[["test","test"]]],[{"Str":"test"}],["test","test"]]}"#,
            Inline::Image(attr_base_val(), vec![inline_base_val()],
                         (String::from("test"), String::from("test"))) =>
                r#"{"Image":[["test",["test"],[["test","test"]]],[{"Str":"test"}],["test","test"]]}"#,
            Inline::Span(attr_base_val(), vec![inline_base_val()]) =>
                r#"{"Span":[["test",["test"],[["test","test"]]],[{"Str":"test"}]]}"#
        );
    }

    #[test]
    fn serialize_quotetype() {
        QuoteType::SingleQuote => r#"{"SingleQuote":[]}"#,
        QuoteType::DoubleQuote => r#"{"DoubleQuote":[]}"#
    }

    #[test]
    fn serialize_mathtype() {
        MathType::DisplayMath => r#"{"DisplayMath":[]}"#,
        MathType::InlineMath => r#"{"InlineMath":[]}"#
    }

    #[test]
    fn serialize_citation_mode() {
        test_serialize!(
            CitationMode::AuthorInText => "\"AuthorInText\"",
            CitationMode::SuppressAuthor => "\"SuppressAuthor\"",
            CitationMode::NormalCitation => "\"NormalCitation\""
        );
    }

    #[test]
    fn serialize_alignment() {
        Alignment::AlignLeft => r#"{"AlignLeft":[]}"#,
        Alignment::AlignRight => r#"{"AlignRight":[]}"#,
        Alignment::AlignCenter => r#"{"AlignCenter":[]}"#,
        Alignment::AlignDefault => r#"{"AlignDefault":[]}"#,
    }

    #[test]
    fn serialize_list_number_delim() {
        ListNumberDelim::DefaultDelim => r#"{"DefaultDelim":[]}"#,
        ListNumberDelim::Period => r#"{"Period":[]}"#,
        ListNumberDelim::OneParen => r#"{"OneParen":[]}"#,
        ListNumberDelim::TwoParens => r#"{"TwoParens":[]}"#
    }

    #[test]
    fn serialize_list_number_style() {
        ListNumberStyle::DefaultStyle => r#"{"DefaultStyle":[]}"#,
        ListNumberStyle::Example => r#"{"Example":[]}"#,
        ListNumberStyle::Decimal => r#"{"Decimal":[]}"#,
        ListNumberStyle::LowerRoman => r#"{"LowerRoman":[]}"#,
        ListNumberStyle::UpperRoman => r#"{"UpperRoman":[]}"#,
        ListNumberStyle::LowerAlpha => r#"{"LowerAlpha":[]}"#,
        ListNumberStyle::UpperAlpha => r#"{"UpperAlpha":[]}"#
    }
}
