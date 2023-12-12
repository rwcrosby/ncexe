#![allow(dead_code)]
//! Formatter for the MacOS Mach-O format

use memmap2::Mmap;

use crate::ExeType;
use crate::Formatter;

#[derive(Debug)]
pub struct Macho32Formatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

impl Formatter for Macho32Formatter<'_> {

    fn to_string(&self) -> String {
        format!("Mach-O32: {:30} {:?}", self.filename, self.mmap)
    }

    fn exe_type(&self) -> ExeType {
        ExeType::MachO32
    }

    fn filename(&self) -> &str {
        self.filename
    }

    fn len(&self) -> usize {
        self.mmap.len()
    }

}

impl Macho32Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn Formatter + '_> {

        Box::new(Macho32Formatter{filename, mmap})

    }

}