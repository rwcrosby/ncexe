//! 
//! Formatter for the MacOS Mach-O format
//!

use memmap2::Mmap;

use super::{
    ExeType,
    ExeFormat,
};

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Macho32Formatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

impl ExeFormat for Macho32Formatter<'_> {

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
            mmap : Mmap) -> Box<dyn ExeFormat + '_> {

        Box::new(Macho32Formatter{filename, mmap})

    }

}