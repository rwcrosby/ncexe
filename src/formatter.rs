//! 
//! Format a block of memory into a window
//! 

use anyhow::{
    anyhow, 
    bail, 
    Context, 
    Result,
};
use serde::Deserialize;
use std::{
    cmp,
    collections::HashMap,
};

// ------------------------------------------------------------------------

type FmtMap = HashMap<(FieldType, FieldFormat, Option<usize>), (Box<DataToString>, Box<DataLen>)>;
type DataToString = dyn Fn(&[u8]) -> String;
type DataLen = dyn Fn(usize) -> usize;

// ------------------------------------------------------------------------
/// Global formatting information 

pub struct Formatter {

    fmt_map: Box<FmtMap>,

}

impl Formatter {

    pub fn new() -> Formatter {
        Formatter{ fmt_map: make_fmt_map()}
    }

    pub fn from_str(&self, yaml_str: &str) 
        -> Result<Box<FormatBlock>> 
    {

        let mut y_fields: Vec<Box<YamlField>> = serde_yaml::from_str(yaml_str)
            .context("Unable to parse YAML string")?;
    
        make_fmt_block(&self.fmt_map, &mut y_fields)

    }

    #[allow(dead_code)]
    pub fn from_file(&self, filename: &str) 
        -> Result<Box<FormatBlock>> 
    {
        let fd = std::fs::File::open(filename)
            .context(format!("Unable to open format file {}", filename))?;
        
        let mut y_fields: Vec<Box<YamlField>> = serde_yaml::from_reader(fd)
            .context(format!("Unable to parse YAML file {}", filename))?;

        make_fmt_block(&self.fmt_map, &mut y_fields)
    }

}

// ------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
enum FieldType {
    Be,
    Le,
    Bytes,
    Ignore,
}

// ------------------------------------------------------------------------

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
enum FieldFormat {
    Int,
    Hex,
    Ptr,
    Binary,
    Char    
}

// ------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
pub struct YamlField {
    pub size: usize,
    pub name: String,
    #[serde(rename = "type")]
    field_type: FieldType,
    #[serde(rename = "format")]
    field_fmt: FieldFormat,
}

// ------------------------------------------------------------------------

pub struct Field<'a> {
    pub y_field: Box<YamlField>,
    pub offset: isize,
    pub fmt_fn: &'a Box<DataToString>,
}

// ------------------------------------------------------------------------

pub struct FormatBlock<'a> {
    pub fields: Vec<Box<Field<'a>>>,
    pub data_len: usize,
    pub max_text_len: usize,
    pub max_value_len: usize,
}

impl<'a> FormatBlock<'a> {

    pub fn _to_string(&self, data: *const u8, offset: isize, len: usize) 
        -> Result<String>  
    {
        if offset + len as isize > self.data_len as isize {
            bail!("Data block not large enough");
        }

        let fmt_str: String = self.fields
            .iter()
            .fold(String::from(""), 
                    |fstr, field| {
                    let slice: &[u8] = unsafe {
                    std::slice::from_raw_parts( data.offset(field.offset), 
                                                field.y_field.size)
                };
            fstr + (field.fmt_fn)(slice).as_str() + "\n"
        });

        Ok(fmt_str)
    }

}

// ------------------------------------------------------------------------

pub fn center_in(width: usize, s: &str) -> (i32, String) {

    let excess = i32::try_from(width).unwrap() - i32::try_from(s.len()).unwrap();

    if excess <= 0 {
        (0, String::from(&s[0..width]))
    } else {
        (excess / 2, String::from(s))
    }

}

// ------------------------------------------------------------------------

fn derive_fmt_fn<'a>(map: &'a Box<FmtMap>,
                     y_field: &YamlField) 
    -> Result<(&'a Box<DataToString>, usize)> 
{

    // Try with length first

    match map.get(&(y_field.field_type, y_field.field_fmt, Some(y_field.size))) {

        Some((fmt_fn, len_fn)) => Ok((fmt_fn, (*len_fn)(y_field.size))),

        None => match map.get(&(y_field.field_type, y_field.field_fmt, None )) {

            Some((fmt_fn, len_fn)) => Ok((fmt_fn, (*len_fn)(y_field.size))),

            None => Err(anyhow!("No formatter for {}:  {:?} {:?} {}", 
                        y_field.name, 
                        y_field.field_type, 
                        y_field.field_fmt, 
                        y_field.size ))
        }
    }

}

// ------------------------------------------------------------------------

fn make_fmt_block<'a>(fmt_map: &'a Box<FmtMap>,
                  y_fields: &mut Vec<Box<YamlField>>) 
    -> Result<Box<FormatBlock<'a>>> 
{
    let mut fmt = Box::new(FormatBlock {
        fields: vec![],
        data_len: 0,
        max_text_len: 0,
        max_value_len: 0
    });

    for yfld in y_fields.drain(..) {
        let size = yfld.size;
        
        if yfld.field_type != FieldType::Ignore {

            fmt.max_text_len = cmp::max(yfld.name.len(), fmt.max_text_len);
            let (fmt_fn, value_len) = derive_fmt_fn(fmt_map, &yfld)?;
            fmt.fields.push(
                Box::new(
                    Field { y_field: yfld,
                            offset: fmt.data_len as isize,
                            fmt_fn,}
            ));
            fmt.max_value_len = cmp::max(value_len, fmt.max_value_len);

        }

        fmt.data_len += size;
    }

    Ok(fmt)
}

// ------------------------------------------------------------------------

fn make_fmt_map() 
    -> Box<FmtMap>
{

    let map : FmtMap = HashMap::from([
        ((FieldType::Be, FieldFormat::Int, Some(1)), 
            (
                Box::new(| d: &[u8]| d[0].to_string() ) as Box<DataToString>, 
                Box::new(| _d | 3usize) as Box<DataLen>
        )),
        ((FieldType::Be, FieldFormat::Int, Some(2)), 
            (
                Box::new(| d: &[u8]| u16::from_be_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 5usize)
        )),
        ((FieldType::Be, FieldFormat::Int, Some(4)), 
            (
                Box::new(| d: &[u8]| u32::from_be_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 10usize)
        )),
        ((FieldType::Be, FieldFormat::Int, Some(8)), 
            (
                Box::new(| d: &[u8]| u64::from_be_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 12usize)
        )),
        ((FieldType::Be, FieldFormat::Ptr, Some(4)), 
            (
                Box::new(| d: &[u8]| 
                    format!("{:p}", u32::from_be_bytes(d.try_into().unwrap()) as *const u32) ), 
                Box::new(| _d | 10usize)
        )),
        ((FieldType::Be, FieldFormat::Ptr, Some(8)), 
            (
                Box::new(| d: &[u8]| 
                    format!("{:p}", u64::from_be_bytes(d.try_into().unwrap()) as *const u64) ), 
                Box::new(| _d | 18usize)
        )),
        ((FieldType::Be, FieldFormat::Hex, None), 
            (
                Box::new(| d: &[u8]| to_hex(d)), 
                Box::new(| d | d * 3 - 1)
        )),
        ((FieldType::Be, FieldFormat::Binary, None), 
            (
                Box::new(| d: &[u8]| to_binary(d)), 
                Box::new(| d | d * 9 - 1 )
        )),
        ((FieldType::Le, FieldFormat::Int, Some(1)), 
            (
                Box::new(| d: &[u8]| d[0].to_string() ) as Box<DataToString>, 
                Box::new(| _d | 3usize) as Box<DataLen>
        )),
        ((FieldType::Le, FieldFormat::Int, Some(2)), 
            (
                Box::new(| d: &[u8]| u16::from_le_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 5usize)
        )),
        ((FieldType::Le, FieldFormat::Int, Some(4)), 
            (
                Box::new(| d: &[u8]| u32::from_le_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 10usize)
        )),
        ((FieldType::Le, FieldFormat::Int, Some(8)), 
            (
                Box::new(| d: &[u8]| u64::from_le_bytes(d.try_into().unwrap()).to_string() ), 
                Box::new(| _d | 12usize)
        )),
        ((FieldType::Le, FieldFormat::Ptr, Some(4)), 
            (
                Box::new(| d: &[u8]| 
                    format!("{:p}", u32::from_le_bytes(d.try_into().unwrap()) as *const u32) ), 
                Box::new(| _d | 10usize)
        )),
        ((FieldType::Le, FieldFormat::Ptr, Some(8)), 
            (
                Box::new(| d: &[u8]| 
                    format!("{:p}", u64::from_le_bytes(d.try_into().unwrap()) as *const u64) ), 
                Box::new(| _d | 18usize)
        )),
        ((FieldType::Le, FieldFormat::Hex, Some(2)), 
            (
                Box::new(| d: &[u8]| 
                    to_hex(
                        u16::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 3 - 1)
        )),
        ((FieldType::Le, FieldFormat::Hex, Some(4)), 
            (
                Box::new(| d: &[u8]| 
                    to_hex(
                        u32::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 3 - 1)
        )),
        ((FieldType::Le, FieldFormat::Hex, Some(8)), 
            (
                Box::new(| d: &[u8]| 
                    to_hex(
                        u64::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 3 - 1)
        )),
        ((FieldType::Le, FieldFormat::Hex, None), 
            (
                Box::new(| d: &[u8]| to_hex(d)), 
                Box::new(| d | d * 3 - 1)
        )),
        ((FieldType::Le, FieldFormat::Binary, Some(2)), 
            (
                Box::new(| d: &[u8]| 
                    to_binary(
                        u16::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 9 - 1)
        )),
        ((FieldType::Le, FieldFormat::Binary, Some(4)), 
            (
                Box::new(| d: &[u8]| 
                    to_binary(
                        u32::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 9 - 1)
        )),
        ((FieldType::Le, FieldFormat::Binary, Some(8)), 
            (
                Box::new(| d: &[u8]| 
                    to_binary(
                        u64::from_le_bytes(d.try_into().unwrap()).to_be_bytes()
                                .as_slice())), 
                Box::new(| d | d * 9 - 1)
        )),
        ((FieldType::Le, FieldFormat::Binary, None), 
            (
                Box::new(| d: &[u8]| to_binary(d)), 
                Box::new(| d | d * 9 - 1 )
        )),
        ((FieldType::Bytes, FieldFormat::Binary, None), 
            (
                Box::new(| d: &[u8]| to_binary(d)), 
                Box::new(| d | d * 9 - 1 )
        )),
        ((FieldType::Bytes, FieldFormat::Hex, None), 
            (
                Box::new(| d: &[u8]| to_hex(d)), 
                Box::new(| d | d * 3 - 1 )
        )),
        ((FieldType::Bytes, FieldFormat::Char, None), 
            (
                Box::new(| d: &[u8]|     
                    match std::str::from_utf8(d) {
                        Ok(v) => format!("\"{}\"", v),
                        Err(e) => e.to_string(),
                    }), 
                Box::new(| d | d )
        )),
    ]);

    Box::new(map)

}

fn to_hex(d: &[u8]) -> String {
    d.iter()
        .map(|byte| -> String { format!("{:02x}", byte) })
        .collect::<Vec<_>>()
        .join(" ") 
}

fn to_binary(d: &[u8]) -> String {
    d.iter()
        .map(|byte| -> String { format!("{:08b}", byte) })
        .collect::<Vec<_>>()
        .join(" ") 
}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    use hex_literal::hex;

    const INTS: [u8; 29] = hex!(
        "
    01
    0002 0200 
    00000003 03000000 
    00000000 00000004  04000000 00000000"
    );

    const STR: [u8; 19] = hex!(
        "
    00 ff 45
    cf fc 32 23 00 ff
    48656c6c6f576f726c64
    "
    );

    #[test]
    fn good_fmt_file() {
        let fmt = Formatter::new();
        let f = fmt.from_file("tests/SampleFormat.yaml").unwrap();

        assert!(f.data_len == 9);
    }

    #[test]
    fn ints_from_file() {
        let fmt = Formatter::new();
        let f = fmt.from_file("tests/Ints.yaml").unwrap();
        let fstr = f._to_string(&INTS as *const u8, 0, INTS.len()).unwrap();
        println!("{}", fstr);
        assert!(
            fstr == "1
2
2
3
3
4
4
"
        );
    }

    const YAMLSTRING : &str = "
---
- size: 3
  name: Binary bytes
  type: !Bytes
  format: !Binary 
- size: 6
  name: Hex Bytes
  type: !Bytes
  format: !Hex
- size: 10
  name: Character string
  type: !Bytes
  format: Char
";

    #[test]
    fn strs_from_str() {
        let fmt = Formatter::new();
        let f = fmt.from_str(YAMLSTRING).unwrap();
        let fstr = f._to_string(&STR as *const u8, 0, STR.len()).unwrap();
        println!("{}", fstr);
        assert!(
            fstr == "00000000 11111111 01000101
cf fc 32 23 00 ff
\"HelloWorld\"
"
        );
    }

    #[test]
    #[should_panic(expected="No such file or directory (os error 2)")]
    fn missing_fmt_file() {
        let fmt = Formatter::new();
        let _f = fmt.from_file("missingfile.yaml").unwrap();
    }

}
