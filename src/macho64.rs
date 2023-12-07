#![allow(dead_code)]
//! Formatter for the MacOS Mach-O format

use memmap2::Mmap;

use crate::ExeType;
use crate::Formatter;
use crate::MainWindow;

#[derive(Debug)]
pub struct Macho64Formatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

impl Formatter for Macho64Formatter<'_> {

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

    fn show(&self, _mw : &MainWindow) {}


}

impl Macho64Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn Formatter + '_> {

        Box::new(Macho64Formatter{filename, mmap})

    }

}