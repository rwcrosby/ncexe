/// Format a block of memory into a window

use serde::Deserialize;

// ------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct Formatter {

    fields : Vec<Box<Field>>

}

#[derive(Debug, Deserialize, PartialEq)]
struct Field {

    size : usize,
    name : String,
    #[serde(rename = "type")]
    field_type : FieldType,
    #[serde(skip)]
    offset : isize,

}

#[derive(Debug, Deserialize, PartialEq)]
enum FieldType {
    Int,
    Hex,
    Binary,
    Char,
    Ignore
}

// ------------------------------------------------------------------------

#[allow(dead_code)]
impl Formatter {

    pub fn new(filename : &str) -> Result<Box<Formatter>, String> {

        let fd = std::fs::File::open(filename)
                    .or_else(| e | return Err(e.to_string()))
                    .unwrap();

        let mut formatter : Box<Formatter> = serde_yaml::from_reader(fd)
                    .or_else(| e | return Err(e.to_string()))
                    .unwrap();

        let mut start = 0;
        formatter.fields.iter_mut().for_each(| f | {
            f.offset = start;
            start += f.size as isize;
        } );

        Ok(formatter)
        
    }

    pub fn format(&self, data : *const u8) -> String {

        let fmt_str : String = self.fields
            .iter()
            .fold("".to_string(), |fstr, field| {
                let ptr = unsafe{ data.offset(field.offset) };
                fstr + field.format(ptr).as_str() + "\n"
                });

        fmt_str

    }
    
}

// ------------------------------------------------------------------------

impl Field {

    // TODO Check data bounds
    // TODO Handle endian-ness

    fn format(&self, data : *const u8) -> String {
        
        match self.field_type {

            FieldType::Int => {
                match self.size {
                    1 => {
                        let value = unsafe{&*data};
                        value.to_string()
                    },
                    2 => {
                        let mut val : u16 = 0;
                        unsafe {
                            std::ptr::copy_nonoverlapping(data as *const u16, &mut val, 1);
                        };
                        val.to_string()
                    },
                    4 => {
                        let mut val : u32 = 0;
                        unsafe {
                            std::ptr::copy_nonoverlapping(data as *const u32, &mut val, 1);
                        };
                        val.to_string()
                    },
                    8 => {
                        let mut val : u64 = 0;
                        unsafe {
                            std::ptr::copy_nonoverlapping(data as *const u64, &mut val, 1);
                        };
                        val.to_string()
                    },
                    _ => panic!("Invalid length {} for int", self.size)
                }
            },

            // FieldType::Hex =>

            FieldType::Binary => {

                let u8data = unsafe{ std::slice::from_raw_parts(data, self.size) };

                u8data
                    .iter()
                    .map(|byte| -> String { format!("{:08b}", byte) } )
                    .collect::<Vec<_>>()
                    .join(" ")

            
            },

            // FieldType::Char =>
            
            _ => panic!("Unsupported type"),
        }
        
        // String::from("Blah")

    }

}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    use hex_literal::hex;

    const INTS: [u8; 29] = hex!("
    01
    0002 0200 
    00000003 03000000 
    00000000 00000004  04000000 00000000");

    #[test]
    fn good_fmt_file() {

        let f = Formatter::new("tests/SampleFormat.yaml").unwrap();
        
        let v = Formatter{ fields: vec!( 
            Box::new(Field{size: 5, name: "Bilbo".to_string(), field_type: FieldType::Binary, offset: 0}),
            Box::new(Field{size: 4, name: "Frodo".to_string(), field_type: FieldType::Int, offset: 5}),
        )};

        println!("{:?}",f);

        assert!(*f == v);

    }

    #[test]
    fn formatter_from_ints() {

        let f = Formatter::new("tests/Ints.yaml").unwrap();
        
        let fstr = f.format(&INTS as *const u8);

        println!("{}", fstr);

        // assert!(f.format(&ptr as *const u8) == "Blah\nBlah\n");
        assert!(fstr == 
"1
512
2
50331648
3
288230376151711744
4
");
    }

    #[test]
    fn formatter_from_file() {

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

}