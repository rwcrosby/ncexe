//! Formatter for the MacOS Mach-O format

use memmap2::Mmap;

use crate::ExeType;
use crate::FormatExe;

#[derive(Debug)]
pub struct ELFFormatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

impl FormatExe for ELFFormatter<'_> {

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }

    fn exe_type(&self) -> ExeType {
        ExeType::ELF
    }

    fn filename(&self) -> &str {
        self.filename
    }

    fn len(&self) -> usize {
        self.mmap.len()
    }

}

impl ELFFormatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn FormatExe + '_> {

        Box::new(ELFFormatter{filename, mmap})

    }

}