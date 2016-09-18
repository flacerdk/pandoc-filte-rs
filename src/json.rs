use serde_json;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::io::{Write, Read, Error};
use std::collections::BTreeMap;

use types::Pandoc;
use walk::Walkable;

impl Pandoc {
    pub fn new_from_json(meta: Value, blocks: Value) -> Self {
        let converted_meta = serde_json::from_value(meta).unwrap();
        let converted_blocks = serde_json::from_value(convert_entry(blocks)).unwrap();
        Pandoc { meta: converted_meta,
                 blocks: converted_blocks
        }
    }
}

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
    let pandoc = Pandoc::new_from_json(arr[0].clone(), arr[1].clone());
    Ok(pandoc)
}

pub fn filter<F, U: Walkable<U>>(json: String, f: &F) -> Result<String, String>
    where F: Fn(U) -> U, Pandoc: Walkable<U> {
    let value: Value = try!(serde_json::from_str(&json).map_err(|e| e.to_string()));
    let arr: &Vec<Value> = try!(value.as_array().ok_or("Not an array"));

    if arr.len() != 2 {
        return Err(String::from("Not valid Pandoc"))
    }
    let pandoc = Pandoc::new_from_json(arr[0].clone(), arr[1].clone());
    let walked = pandoc.walk(f);
    let new_json = try!(serde_json::ser::to_string(&walked).map_err(|e| e.to_string()));
    Ok(new_json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use types::{Block, Inline};
    use std::collections::BTreeMap;

    #[test]
    fn test_convert_entry_object() {
        let mut map = BTreeMap::new();
        map.insert(String::from("Str"), serde_json::Value::String(String::from("Test")));
        let expected = serde_json::Value::Object(map);

        let mut object_builder = serde_json::builder::ObjectBuilder::new();
        object_builder = object_builder.insert(String::from("t"), String::from("Str"));
        object_builder = object_builder.insert(String::from("c"), String::from("Test"));
        let object = object_builder.build();
        let converted = convert_entry(object);

        assert_eq!(converted, expected);
    }

    #[test]
    fn test_convert_entry_array() {
        let arr = vec![serde_json::Value::String(String::from("Test")),
                           serde_json::Value::String(String::from("string"))];
        let expected = serde_json::Value::Array(arr);

        let mut array_builder = serde_json::builder::ArrayBuilder::new();
        array_builder = array_builder.push(String::from("Test"));
        array_builder = array_builder.push(String::from("string"));
        let array = array_builder.build();
        let converted = convert_entry(array);

        assert_eq!(converted, expected);
    }

    #[test]
    fn test_deserialize() {
        let markdown = String::from("# Test");
        let pandoc = deserialize(markdown).unwrap();
        assert!(pandoc.meta.un_meta.is_empty());
        assert_eq!(pandoc.blocks, vec![
            Block::Header(1, (String::from("test"), vec![], vec![]),
                          vec![Inline::Str(String::from("Test"))])
        ]);
    }

    #[test]
    fn test_to_json() {
        let json = to_json(String::from("# Test")).unwrap();
        let pandoc = r#"[{"unMeta":{}},[{"t":"Header","c":[1,["test",[],[]],[{"t":"Str","c":"Test"}]]}]]"#;
        let expected: serde_json::Value = serde_json::from_str(pandoc).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, expected);
    }


}
