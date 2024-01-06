//! 
//! Format a block of memory into a window
//! 

use std::cmp;

// ------------------------------------------------------------------------
/// Set of mapping fields

pub struct MapSet {
    pub fields: Vec<MapField>,
    pub data_len: usize,
    pub max_text_len: usize,
}

impl<'a> MapSet {

    pub fn new(
        map_flds: &'static [FieldDef],
    ) -> Box<MapSet> {

        let mut fmt = Box::new(
            MapSet {
                fields: Vec::with_capacity(map_flds.len()),
                data_len: 0,
                max_text_len: 0,
         });

        for mfld in map_flds.iter() {

            if let Some(_) = mfld.string_fn {

                fmt.max_text_len = cmp::max(mfld.name.len(), fmt.max_text_len);

                fmt.fields.push(
                    MapField { 
                        field: mfld,
                        range: (fmt.data_len, fmt.data_len + mfld.len),
                    }
                );

            }

            fmt.data_len += mfld.len;
        }

        fmt
    }

}

// ------------------------------------------------------------------------
/// Field mapped into a data offsets

pub struct MapField {
    pub field: &'static FieldDef,
    pub range: (usize, usize),
}


impl MapField {

    pub fn to_usize(&self, data: &[u8]) -> usize {

        // Yes this could fail but panic is actuall an appropriate response
        (self.field.usize_fn.unwrap())(&data[self.range.0..self.range.1])
    }

    pub fn to_string(&self, data: &[u8]) -> String {

        // Yes this could fail but panic is actuall an appropriate response
        (self.field.string_fn.unwrap())(&data[self.range.0..self.range.1])
    }

    pub fn lookup(
        &self,
        d: &[u8], 
    ) -> Option<&'static str> {

        if let Some(vt) = self.field.val_tbl {

            let ufn = self.field.usize_fn.unwrap();
            let uv = ufn(&d[self.range.0..self.range.1]);

            if let Some((_,  s)) = vt.iter().find(| v | v.0 == uv ) {
                Some(s)
            } else {
                None
            }

        } else {
            None
        }

    }

}

// ------------------------------------------------------------------------
/// Field definition

type StringFn = dyn Fn(&[u8]) -> String;
type UsizeFn = dyn Fn(&[u8]) -> usize;

pub type ValEntry = (usize, &'static str);
pub type ValTable = [ValEntry];

pub struct FieldDef {
    pub len: usize,
    pub name: &'static str,
    pub string_fn: Option<&'static StringFn>,
    pub usize_fn: Option<&'static UsizeFn>,
    pub val_tbl: Option<&'static ValTable>,
    pub enter_no: Option<usize>,
}

impl FieldDef {

    pub const fn new(
        len: usize, 
        name: &'static str, 
        string_fn: Option<&'static StringFn>
    ) -> FieldDef {
        FieldDef {
            len,
            name,
            string_fn,
            enter_no: None,
            usize_fn: None,
            val_tbl: None,
        }
    }

    pub const fn ignore(
        len: usize
    ) -> FieldDef {
        FieldDef {
            len,
            name: "",
            string_fn: None,
            enter_no: None,
            usize_fn: None,
            val_tbl: None,
        }
    }

    pub const fn fn_usize(
        mut self: FieldDef, 
        uf: &'static UsizeFn, 
    ) -> FieldDef {
        self.usize_fn = Some(uf);
        self
    }

    pub const fn val_tbl(
        mut self: FieldDef, 
        uf: &'static UsizeFn,
        vt: &'static ValTable,
    ) -> FieldDef {
        self.usize_fn = Some(uf);
        self.val_tbl = Some(vt);
        self
    }

    pub const fn enter_no(
        mut self: FieldDef, 
        enter: usize
    ) -> FieldDef {
        self.enter_no = Some(enter);
        self
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

trait Converters {
    fn to_hex(&self) -> String;
    fn to_bits(&self) -> String;
}

impl Converters for &[u8] {
    fn to_hex(&self) -> String {
        self.iter()
            .map(|byte| -> String { format!("{:02x}", byte) })
            .collect::<Vec<_>>()
            .join(" ") 
    }
    fn to_bits(&self) -> String {
        self.iter()
            .map(|byte| -> String { format!("{:08b}", byte) })
            .collect::<Vec<_>>()
            .join(" ") 
    }
}
// ------------------------------------------------------------------------
/// Formatting closures

pub const BE_8_STRING:  &StringFn = &|d: &[u8]| u8::from_be_bytes(d.try_into().unwrap()).to_string();
pub const BE_16_STRING: &StringFn = &|d: &[u8]| u16::from_be_bytes(d.try_into().unwrap()).to_string();
pub const BE_32_STRING: &StringFn = &|d: &[u8]| u32::from_be_bytes(d.try_into().unwrap()).to_string();
pub const BE_64_STRING: &StringFn = &|d: &[u8]| u64::from_be_bytes(d.try_into().unwrap()).to_string();

pub const BE_8_USIZE:   &UsizeFn = &|d: &[u8]| u8::from_be_bytes(d.try_into().unwrap()).into();
pub const BE_16_USIZE:  &UsizeFn = &|d: &[u8]| u16::from_be_bytes(d.try_into().unwrap()).into();
pub const BE_32_USIZE:  &UsizeFn = &|d: &[u8]| u32::from_be_bytes(d.try_into().unwrap())
        .try_into().unwrap();
pub const BE_64_USIZE:  &UsizeFn = &|d: &[u8]| u64::from_be_bytes(d.try_into().unwrap())
        .try_into().unwrap();

pub const BE_HEX:    &StringFn = &|d: &[u8]| d.to_hex();

pub const BE_32_PTR:    &StringFn = &|d: &[u8]| format!("{:010p}", 
    u32::from_be_bytes(d.try_into().unwrap()) as *const u32);
pub const BE_64_PTR:    &StringFn = &|d: &[u8]| format!("{:018p}", 
    u64::from_be_bytes(d.try_into().unwrap()) as *const u64);

pub const LE_8_STRING:  &StringFn = &|d: &[u8]| u8::from_le_bytes(d.try_into().unwrap()).to_string();
pub const LE_16_STRING: &StringFn = &|d: &[u8]| u16::from_le_bytes(d.try_into().unwrap()).to_string();
pub const LE_32_STRING: &StringFn = &|d: &[u8]| u32::from_le_bytes(d.try_into().unwrap()).to_string();
pub const LE_64_STRING: &StringFn = &|d: &[u8]| u64::from_le_bytes(d.try_into().unwrap()).to_string();

pub const LE_8_HEX:     &StringFn = &|d: &[u8]| d.to_hex();
pub const LE_16_HEX:    &StringFn = &|d: &[u8]| u16::from_le_bytes(d.try_into().unwrap())
    .to_be_bytes()
    .as_slice()
    .to_hex();
pub const LE_32_HEX:    &StringFn = &|d: &[u8]| u32::from_le_bytes(d.try_into().unwrap())
    .to_be_bytes()
    .as_slice()
    .to_hex();

pub const LE_32_PTR:    &StringFn = &|d: &[u8]| format!("{:010p}", 
    u32::from_le_bytes(d.try_into().unwrap()) as *const u32);
pub const LE_64_PTR:    &StringFn = &|d: &[u8]| format!("{:018p}", 
    u64::from_le_bytes(d.try_into().unwrap()) as *const u64);
pub const BIN_STRING:   &StringFn = &|d: &[u8]| d.to_bits();
    
pub const LE_8_USIZE:   &UsizeFn = &|d: &[u8]| u8::from_le_bytes(d.try_into().unwrap()).into();
pub const LE_16_USIZE:  &UsizeFn = &|d: &[u8]| u16::from_le_bytes(d.try_into().unwrap()).into();
pub const LE_32_USIZE:  &UsizeFn = &|d: &[u8]| u32::from_le_bytes(d.try_into().unwrap())
        .try_into().unwrap();
pub const LE_64_USIZE:  &UsizeFn = &|d: &[u8]| u64::from_le_bytes(d.try_into().unwrap())
        .try_into().unwrap();
