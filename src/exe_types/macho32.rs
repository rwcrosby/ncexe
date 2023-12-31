//! 
//! Formatter for the MacOS Mach-O format
//!

use anyhow::Result;
use memmap2::Mmap;

use crate::windows::line::{
    Line,
    PairVec,
};

use super::{
    ExeType,
    Executable,
};

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Macho32Formatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

// ------------------------------------------------------------------------

impl Line for Macho32Formatter<'_> {

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        Ok(Vec::from([(None, String::from("Mach-O 32 Not Supported Yet"))]))
    }

    fn as_executable(&self) -> &dyn Executable {
        self
    }

}

impl Executable for Macho32Formatter<'_> {

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

    fn to_line(&self) -> &dyn Line {
        self
    }

}

impl Macho32Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn Executable + '_> {

        Box::new(Macho32Formatter{filename, mmap})

    }

}