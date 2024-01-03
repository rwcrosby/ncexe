#![allow(unused)]
extern crate ncexe;

type StringFn = dyn Fn(&[u8]) -> String;
type UsizeFn = dyn Fn(&[u8]) -> usize;

type ValEntry = (usize, &'static str);
type ValTable = [ValEntry];

const STRING_BE_8:  &StringFn = &|d: &[u8]| u8::from_be_bytes(d.try_into().unwrap()).to_string();
const STRING_BE_16: &StringFn = &|d: &[u8]| u16::from_be_bytes(d.try_into().unwrap()).to_string();
const STRING_BE_32: &StringFn = &|d: &[u8]| u32::from_be_bytes(d.try_into().unwrap()).to_string();
const STRING_BE_64: &StringFn = &|d: &[u8]| u64::from_be_bytes(d.try_into().unwrap()).to_string();

const USIZE_BE_8:  &UsizeFn = &|d: &[u8]| u8::from_be_bytes(d.try_into().unwrap()).into();
const USIZE_BE_16: &UsizeFn = &|d: &[u8]| u16::from_be_bytes(d.try_into().unwrap()).into();
const USIZE_BE_32: &UsizeFn = &|d: &[u8]| u32::from_be_bytes(d.try_into().unwrap())
        .try_into().unwrap();
const USIZE_BE_64: &UsizeFn = &|d: &[u8]| u64::from_be_bytes(d.try_into().unwrap())
        .try_into().unwrap();

struct Info {
    len: usize,
    name: &'static str,
    string_fn: Option<&'static StringFn>,
    val_fn: Option<(&'static UsizeFn, &'static ValTable) >,
    enter_no: Option<usize>,
}

impl Info {
    const fn new(len: usize, name: &'static str, string_fn: Option<&'static StringFn>) -> Info {
        Info {
            len,
            name,
            string_fn,
            enter_no: None,
            val_fn: None,
        }
    }
    const fn fn_val(
        mut self: Info, 
        vf: &'static UsizeFn, 
        vt: &'static ValTable
    ) -> Info {
        self.val_fn = Some((vf, vt));
        self
    }
    const fn enter_no(
        mut self: Info, 
        enter: usize
    ) -> Info {
        self.enter_no = Some(enter);
        self
    }
    fn lookup(
        d: &[u8], 
        ufn: &UsizeFn, 
        table: &ValTable
    ) -> Option<&'static str> {
        let val = ufn(d);
        if let Some((_,  s)) = table.iter().find(| v | v.0 == val ) {
            Some(s)
        } else {
            None
        }
    }
}

const TABLE1: &ValTable = &[
    (0x1, "val1"), 
    (0x2, "val2"),
    (0x3, "val3"),
    (0x4, "val4"),
];

const MAP3: &[Info] = &[
    Info::new(1, "Blah8", Some(STRING_BE_8)),
    Info::new(2, "Blah16", Some(STRING_BE_16)).enter_no(3),
    Info::new(4, "Blah32", Some(STRING_BE_32))
        .fn_val(USIZE_BE_32, TABLE1)
        .enter_no(2),
    Info::new(8, "Blah64", Some(STRING_BE_64)),
];

// -----------------------------------------------------------------

fn main() {
    let val: &[u8] = &[
        0x1, 
        0x0, 0x2,
        0x0, 0x0, 0x0, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4
    ];

    let mut offset = 0;
    for info in MAP3 {
        let val = &val[offset..offset+info.len];
        println!(
            "Len:{}/Name:{}/String:{}/Enter:{}/Value:{}",
            info.len,
            info.name,
            if let Some(sfn) = info.string_fn {
                sfn(val)
            } else {
                String::from("Ignore")
            },
            if let Some(v) = info.enter_no {
                format!("({})", v)
            } else {
                String::from("None")
            },
            if let Some((tfn, tbl)) = info.val_fn {
                if let Some(s) = Info::lookup(val, tfn, tbl ) {
                    s
                } else {
                    "Not Found"
                }
            } else {
                "No lookup"
            },
        );
        offset += info.len;
    }

}

