//! 
//! Formatter for the MacOS Mach-O format - 64 bit
//! 

use anyhow::Result;
use memmap2::Mmap;
use std::ops::Deref;

use crate::{
  color::Colors,
  formatter::Formatter,
  window::ExeWindow,
};
use super::ExeFormat;

// ------------------------------------------------------------------------
#[derive(Debug)]
pub struct Mach_O_64<'a> {
  filename: &'a str,
  mmap: Mmap,
}

// ------------------------------------------------------------------------

impl Mach_O_64<'_> {

    pub fn new( filename : &str,
                mmap : Mmap) -> Box<dyn ExeFormat + '_> {

        Box::new(Macho64Formatter{filename, mmap})

    }

}

// ------------------------------------------------------------------------

impl Executable for Mach_O_64<'_> {

    // fn to_string(&self) -> String {
    //     format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    // }

    fn exe_type(&self) -> super::ExeType {
        super::ExeType::MachO64
    }

    fn filename(&self) -> &str {
        self.filename
    }

    fn len(&self) -> usize {
        self.mmap.len()
    }

    fn show(
        &self, 
        screen : &Screen,
        parent : Option<&WindowSet>,
        fmt: &Formatter,
        colors: &Colors
    ) -> Result<()> {

        let fmt_blk = fmt.from_str(HEADER)?;

        header_window::show(
            screen,
            parent, 
            colors, 
            "Macho-O 64 Header", 
            &fmt_blk, 
            self.mmap.deref())

    }

}

// ------------------------------------------------------------------------

const HEADER: &str = "
---

- name: Magic Number 
  format: !Hex
  type: !Le
  size: 4
- name: CPU Type 
  format: !Hex
  type: !Le
  size: 4
- name: CPU Sub-Type 
  format: !Hex
  type: !Le
  size: 4
- name: File Type 
  type: !Le
  format: !Hex
  size: 4
- name: Load Commands
  type: !Le
  format: !Int
  size: 4
- name: Load Command Length
  type: !Le
  format: !Ptr
  size: 4
- name: Flags
  type: !Le
  format: !Binary
  size: 4
- name: Reserved
  type: !Ignore
  format: !Char
  size: 4

";