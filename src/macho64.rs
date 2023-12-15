#![allow(dead_code)]
//! Formatter for the MacOS Mach-O format

use memmap2::Mmap;

use crate::color::ColorSet;
use crate::ExeType;
use crate::Formatter;
use crate::window;

// ------------------------------------------------------------------------

impl Macho64Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn Formatter + '_> {

        Box::new(Macho64Formatter{filename, mmap})

    }

}

// ------------------------------------------------------------------------

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

    fn show(
        &self, 
        mw : &crate::main_window::MainWindow,
        colors: &ColorSet
    ) -> Result<(), Box<dyn std::error::Error>> {
        
        // Load the format specification

        let lines = 1;
        let cols = 1;

        // Create the window
                    
        let w = window::ExeWindow::new(
            lines, 
            cols, 
            "Mach-O 64 Bit Header", 
            colors,
            mw, 
        )?;

        let _pw = &w.win;
        let _mpw = &mw.win;


        Ok(())

    }

}
