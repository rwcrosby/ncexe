#![allow(dead_code)]
//! Formatter for the MacOS Mach-O format

use anyhow::Result;
use memmap2::Mmap;
use std::ops::Deref;

use crate::ExeType;
use crate::FormatExe;

use crate::color::Colors;
use crate::formatter::Formatter;
use crate::header_window;
use crate::main_window::MainWindow;

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Macho64Formatter<'a> {
  filename: &'a str,
  mmap: Mmap,
}

// ------------------------------------------------------------------------

impl Macho64Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn FormatExe + '_> {

        Box::new(Macho64Formatter{filename, mmap})

    }

}

// ------------------------------------------------------------------------

impl FormatExe for Macho64Formatter<'_> {

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }

    fn exe_type(&self) -> ExeType {
        ExeType::MachO64
    }

    fn filename(&self) -> &str {
        self.filename
    }

    fn len(&self) -> usize {
        self.mmap.len()
    }

    fn show(
        &self, 
        mw : &MainWindow,
        fmt: &Formatter,
        colors: &Colors
    ) -> Result<()> {

        let fmt_blk = fmt.from_str(HEADER)?;

        header_window::show(
            mw, 
            colors, 
            &fmt_blk, 
            "Macho-O 64 Header", 
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
  format: !Int
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