#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use serde_json::Value;

use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::io::{Write, Read, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc {
    pub meta: Meta,
    pub blocks: Vec<Block>
}

impl Pandoc {
    pub fn new(meta: Value, blocks: Value) -> Self {
        let converted_meta = serde_json::from_value(meta).unwrap();
        let converted_blocks = serde_json::from_value(convert_entry(blocks)).unwrap();
        Pandoc { meta: converted_meta,
                 blocks: converted_blocks
        }
    }
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
// TODO: ordered list, table
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Block {
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    BlockQuote(Vec<Block>),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header(u64, Attr, Vec<Inline>),
    HorizontalRule,
    Div(Attr, Vec<Block>),
    Null
}

// http://hackage.haskell.org/package/pandoc-types-1.16.1.1/docs/Text-Pandoc-Definition.html#t:Inline
// TODO: add cite, note
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

pub fn convert_entry(entry: Value) -> Value {
    match entry {
        Value::Object(obj) => {
            let mut new_entry = BTreeMap::new();
            let t = String::from(obj.get("t").unwrap().as_str().unwrap());
            let c = obj.get("c").unwrap().clone();
            let new_c = convert_entry(c);
            new_entry.insert(t, new_c);
            Value::Object(new_entry)
        },
        Value::Array(arr) => {
            let mut array = Vec::new();
            for item in arr {
                array.push(convert_entry(item));
            }
            Value::Array(array)
        },
        _ => entry
    }
}

pub fn to_json(markdown: String) -> Result<String, Error> {
    let process = try!(
        Command::new("pandoc")
            .args(&["-t", "json", "--mathjax"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
    );
    try!(process.stdin.unwrap().write_all(markdown.as_bytes()));

    let mut s = String::new();
    try!(process.stdout.unwrap().read_to_string(&mut s));
    Ok(s)
}

pub fn deserialize(markdown: String) -> Result<Pandoc, String> {
    let json = try!(to_json(markdown).map_err(|e| e.to_string()));
    let value: Value = try!(serde_json::from_str(&json).map_err(|e| e.to_string()));
    let arr: &Vec<Value> = try!(value.as_array().ok_or("Not an array"));

    if arr.len() != 2 {
        return Err(String::from("Not valid Pandoc"))
    }
    let pandoc = Pandoc::new(arr[0].clone(), arr[1].clone());
    Ok(pandoc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let markdown = String::from("# Test");
        let pandoc = deserialize(markdown).unwrap();
        assert!(pandoc.meta.un_meta.is_empty());
        assert_eq!(pandoc.blocks, vec![Block::Header(1, (String::from("test"), vec![], vec![]), vec![Inline::Str(String::from("Test"))])]);
    }
}
