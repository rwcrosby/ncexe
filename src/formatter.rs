#![allow(dead_code)]
/// Format a block of memory into a window
use serde::Deserialize;

// ------------------------------------------------------------------------

struct Formatter {
    fields: Vec<Box<Field>>,
    len: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
struct YamlField {
    size: usize,
    name: String,
    #[serde(rename = "type")]
    field_type: FieldType,
}

struct Field {
    y_field: Box<YamlField>,
    offset: isize,
    fmt_fn: DataToString,
}

#[derive(Debug, Deserialize, PartialEq)]
enum FieldType {
    BeInt,
    LeInt,
    Hex,
    Binary,
    Char,
    Ignore,
}

// ------------------------------------------------------------------------

type DataToString = fn(data: &[u8]) -> String;

fn u8_2_string(data: &[u8]) -> String {
    data[0].to_string()
}

fn be_u16_2_string(data: &[u8]) -> String {
    u16::from_be_bytes(data.try_into().unwrap()).to_string()
}

fn be_u32_2_string(data: &[u8]) -> String {
    u32::from_be_bytes(data.try_into().unwrap()).to_string()
}

fn be_u64_2_string(data: &[u8]) -> String {
    u64::from_be_bytes(data.try_into().unwrap()).to_string()
}

fn le_u16_2_string(data: &[u8]) -> String {
    u16::from_le_bytes(data.try_into().unwrap()).to_string()
}

fn le_u32_2_string(data: &[u8]) -> String {
    u32::from_le_bytes(data.try_into().unwrap()).to_string()
}

fn le_u64_2_string(data: &[u8]) -> String {
    u64::from_le_bytes(data.try_into().unwrap()).to_string()
}

fn binary_2_string(data: &[u8]) -> String {
    data.iter()
        .map(|byte| -> String { format!("{:08b}", byte) })
        .collect::<Vec<_>>()
        .join(" ")
}

fn hex_2_string(data: &[u8]) -> String {
    data.iter()
        .map(|byte| -> String { format!("{:02x}", byte) })
        .collect::<Vec<_>>()
        .join(" ")
}

fn char_2_string(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(v) => format!("\"{}\"", v),
        Err(e) => e.to_string(),
    }
}

fn ignore_2_string(_data: &[u8]) -> String {
    String::from("")
}

// ------------------------------------------------------------------------

fn derive_fmt_fn(y_field: &YamlField) -> Result<DataToString, String> {
    match y_field.field_type {
        FieldType::BeInt => match y_field.size {
            1 => Ok(u8_2_string),
            2 => Ok(be_u16_2_string),
            4 => Ok(be_u32_2_string),
            8 => Ok(be_u64_2_string),
            s => Err(format!("Bad integer length: {}", s)),
        },
        FieldType::LeInt => match y_field.size {
            1 => Ok(u8_2_string),
            2 => Ok(le_u16_2_string),
            4 => Ok(le_u32_2_string),
            8 => Ok(le_u64_2_string),
            s => Err(format!("Bad integer length: {}", s)),
        },
        FieldType::Binary => Ok(binary_2_string),
        FieldType::Hex => Ok(hex_2_string),
        FieldType::Char => Ok(char_2_string),
        FieldType::Ignore => Ok(ignore_2_string),
    }
}

// ------------------------------------------------------------------------

impl Formatter {

    fn make_formatter(y_fields: &mut Vec<Box<YamlField>>) -> Result<Box<Formatter>, String> {

        let mut fmt: Box<Formatter> = Box::new(Formatter {
            fields: vec![],
            len: 0,
        });

        for yfld in y_fields.drain(..) {
            let size = yfld.size;
            let fmt_fn = derive_fmt_fn(&yfld)?;
            fmt.fields.push(Box::new(Field {y_field: yfld,
                                            offset: fmt.len as isize,
                                            fmt_fn,
            }));
            fmt.len += size;
        }

        Ok(fmt)
    }

    pub fn from_str(yaml_str: &str) -> Result<Box<Formatter>, String> {
        let mut y_fields: Vec<Box<YamlField>> = serde_yaml::from_str(yaml_str)
            .or_else(|e| return Err(e.to_string()))
            .unwrap();

        Formatter::make_formatter(&mut y_fields)
    }

    pub fn from_file(filename: &str) -> Result<Box<Formatter>, String> {
        let fd = std::fs::File::open(filename)
            .or_else(|e| return Err(e.to_string()))
            .unwrap();

        let mut y_fields: Vec<Box<YamlField>> = serde_yaml::from_reader(fd)
            .or_else(|e| return Err(e.to_string()))
            .unwrap();


        Formatter::make_formatter(&mut y_fields)
    }

    pub fn to_string(&self, data: *const u8, offset: isize, len: usize) -> Result<String, String> {
        if offset + len as isize > self.len as isize {
            return Err(String::from("Data block not large enough"));
        }

        let fmt_str: String = self.fields.iter().fold("".to_string(), |fstr, field| {
            let slice: &[u8] = unsafe {
                std::slice::from_raw_parts(data.offset(field.offset), field.y_field.size)
            };
            fstr + (field.fmt_fn)(slice).as_str() + "\n"
        });

        Ok(fmt_str)
    }
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
        let f = Formatter::from_file("tests/SampleFormat.yaml").unwrap();

        assert!(f.len == 9);
    }

    #[test]
    fn ints_from_file() {
        let f = Formatter::from_file("tests/Ints.yaml").unwrap();
        let fstr = f.to_string(&INTS as *const u8, 0, INTS.len()).unwrap();
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
  type: !Binary 
  name: Binary bytes
- size: 6
  type: !Hex 
  name: Hex Bytes
- size: 10
  type: !Char 
  name: Character string
";

    #[test]
    fn strs_from_str() {
        let f = Formatter::from_str(YAMLSTRING).unwrap();
        let fstr = f.to_string(&STR as *const u8, 0, STR.len()).unwrap();
        println!("{}", fstr);
        assert!(
            fstr == "00000000 11111111 01000101
cf fc 32 23 00 ff
\"HelloWorld\"
"
        );
    }

    /*
        #[test]
        fn formatter_from_string() {

            let f = Formatter::new("tests/SampleFormat.yaml").unwrap();

            let ptr : [u8; 9] = hex!("31 32 33 34 35 0f 00 00 00");

            let fstr = f.format(&ptr as *const u8);

            println!("{}", fstr);

            // assert!(f.format(&ptr as *const u8) == "Blah\nBlah\n");
            assert!(fstr == "00110001 00110010 00110011 00110100 00110101\n15\n");
        }

        #[test]
        #[should_panic(expected="No such file or directory (os error 2)")]
        fn missing_fmt_file() {

            let _f = Formatter::new("missingfile.yaml").unwrap();

        }
    */
}
