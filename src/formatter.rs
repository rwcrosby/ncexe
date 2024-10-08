//! 
//! Definitions for the static data mappings
//! 

use anyhow::{
    anyhow,
    Context, 
    Result
};
use std::ffi::CStr;

use crate::{
    exe_types::ExeRef,
    windows::popup::error_window, 
};

// ------------------------------------------------------------------------
/// Header for a memory block mapping
pub struct FieldMap<'fd> {
    pub fields: &'fd [FieldDef<'fd>],
    pub data_len: usize,
    pub max_text_len: usize,
}

impl<'fd> FieldMap<'fd> {
    pub const fn new(fields: &'static [FieldDef]) -> Self {

        let mut fld_idx = 0;
        let mut data_len = 0;
        let mut max_text_len = 0;

        while fld_idx < fields.len() {
            let field = &fields[fld_idx];

            data_len += field.range.1 - field.range.0;
            if field.name.len() > max_text_len {
                max_text_len = field.name.len()
            }

            fld_idx += 1;
        }

        Self{fields, data_len, max_text_len}

    }
}

// ------------------------------------------------------------------------
/// Field definition

type StringFn = dyn Fn(&[u8]) -> String;
type StringFn2 = dyn Fn(&[u8]) -> Result<String>;
type UsizeFn = dyn Fn(&[u8]) -> usize;
type EnterFn = fn(ExeRef) -> Result<()>;

/// Entry in the table of values for a field
pub type ValEntry<'v> = (
    usize, 
    &'v str, 
    Option<FieldMap<'v>>
);

/// Fixed table of value entries
pub type ValTable<'vt> = [ValEntry<'vt>];

/// Map a scalar field in image
pub struct FieldDef<'fd> {
    // If the beginnning and end of the range are the same, this 
    // implies that the actual data is from start to end of segment
    pub range: (usize, usize),
    pub name: &'static str,
    pub string_fn: Option<&'fd StringFn>,
    pub string_fn2: Option<&'fd StringFn2>,
    pub usize_fn: Option<&'fd UsizeFn>,
    pub val_tbl: Option<&'fd ValTable<'fd>>,
    pub enter_fn: Option<EnterFn>,
}

impl<'fd> FieldDef<'fd> {

    pub const fn new(
        offset: usize,
        len: usize, 
        name: &'static str, 
        string_fn: Option<&'static StringFn>
    ) -> Self {
        Self {
            range: (offset, offset + len),
            name,
            string_fn,
            string_fn2: None,
            usize_fn: None,
            val_tbl: None,
            enter_fn: None,
            
        }
    }

    pub const fn new2(
        offset: usize,
        len: usize, 
        name: &'static str, 
        string_fn2: Option<&'static StringFn2>
    ) -> Self {
        Self {
            range: (offset, offset + len),
            name,
            string_fn: None,
            string_fn2,
            usize_fn: None,
            val_tbl: None,
            enter_fn: None,
            
        }
    }

    pub const fn ignore(
        offset: usize,
        len: usize
    ) -> Self {
        Self {
            range: (offset, offset+len),
            name: "",
            string_fn: None,
            string_fn2: None,
            usize_fn: None,
            val_tbl: None,
            enter_fn: None,
        }
    }

    pub const fn fn_usize(
        mut self, 
        uf: &'static UsizeFn, 
    ) -> FieldDef<'fd> {
        self.usize_fn = Some(uf);
        self
    }

    pub const fn val_tbl(
        mut self, 
        uf: &'static UsizeFn,
        vt: &'static ValTable,
    ) -> Self {
        self.usize_fn = Some(uf);
        self.val_tbl = Some(vt);
        self
    }

    pub const fn enter_fn(
        mut self, 
        enter: EnterFn,
    ) -> Self {
        self.enter_fn = Some(enter);
        self
    }

    pub fn to_usize(&self, data: &[u8]) -> usize {

        // Yes this could fail but panic is actualy an appropriate response
        (self.usize_fn.unwrap())(&data[self.range.0..self.range.1])
    }

    pub fn to_string(&self, data: &[u8]) -> Result<String> {

        Ok(match self.to_string_wrapped(data) {
            Err(err) => {
                error_window(&err);
                "???".to_string()
            },
            Ok(s) => s
        })

    }

    fn to_string_wrapped(&self, data: &[u8]) -> Result<String> {

        let data_str = if self.range.1 > self.range.0 {
                &data[self.range.0..self.range.1]
            } else {
                &data[self.range.0..]
            };

        if let Some(stringfn) = self.string_fn {
            Ok(stringfn(data_str))
        } else if let Some(string_fn) = self.string_fn2 {
            string_fn(data_str)
                .with_context(|| format!("Unable to generate string for field: {}", 
                                            self.name))
        } else {
            Err(anyhow!("Should never get here"))
        }

    }

    pub fn lookup(
        &self,
        d: &[u8], 
    ) -> Option<&ValEntry> {

        if let Some(vt) = self.val_tbl {

            let ufn = self.usize_fn.unwrap();
            let uv = ufn(&d[self.range.0..self.range.1]);

            vt.iter().find(| v | v.0 == uv )

        } else {
            None
        }

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
    fn to_string(&self) -> String;
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
    fn to_string(&self) -> String {
        let mut data: Vec<u8> = self.to_vec();
        data.append(&mut Vec::from([0]));
        CStr::from_bytes_until_nul(&data).unwrap().to_str().unwrap().into()
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

pub const BE_HEX:       &StringFn = &|d: &[u8]| d.to_hex();
pub const BE_CHAR:      &StringFn = &|d: &[u8]| d.to_string();

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
    
pub const LE_8_USIZE:   &UsizeFn = &|d: &[u8]| u8::from_le_bytes(d.try_into().unwrap())
    .into();
pub const LE_16_USIZE:  &UsizeFn = &|d: &[u8]| u16::from_le_bytes(d.try_into().unwrap())
    .into();
pub const LE_32_USIZE:  &UsizeFn = &|d: &[u8]| u32::from_le_bytes(d.try_into().unwrap())
    .try_into()
    .unwrap();
pub const LE_64_USIZE:  &UsizeFn = &|d: &[u8]| u64::from_le_bytes(d.try_into().unwrap())
    .try_into()
    .unwrap();

pub const C_STR:        &StringFn2 = &|d: &[u8]| Ok(CStr::from_bytes_until_nul(d)?
    .to_str()?
    .into());