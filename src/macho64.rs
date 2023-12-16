#![allow(dead_code)]
//! Formatter for the MacOS Mach-O format

use std::ops::Deref;

use anyhow::Context;

use anyhow::Result;
use memmap2::Mmap;
use pancurses::COLOR_PAIR;

use crate::color::Colors;
use crate::ExeType;
use crate::FormatExe;
use crate::formatter::Formatter;
use crate::window;
use crate::main_window::MainWindow;

// ------------------------------------------------------------------------

impl Macho64Formatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn FormatExe + '_> {

        Box::new(Macho64Formatter{filename, mmap})

    }

}

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Macho64Formatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

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
        
        // Load the format specification

        let fb = fmt.from_str(HEADER)
          .context("Macho-O 64 Header")?;

        let lines = fb.fields.len();
        let cols = fb.max_text_len + 3 + fb.max_value_len;

        let color_set = colors.set("header");

        // Create the window
                    
        let w = window::ExeWindow::new(
            lines, 
            cols, 
            "Mach-O 64 Bit Header", 
            color_set,
            mw, 
        )?;

        let pw = &w.win;
        let _mpw = &mw.win;

        // Display the fields

        for (idx, fld) in fb.fields.iter().enumerate() {

            let df = &self.mmap.deref()
                [fld.offset as usize..fld.offset as usize + fld.y_field.size];

            pw.mv((idx + window::TMARGIN) as i32, 
                  window::LMARGIN as i32);

            pw.attrset(COLOR_PAIR(color_set.text as u32));
            pw.addstr(format!("{fname:>nl$.nl$} : ", 
                              nl = fb.max_text_len,
                              fname=fld.y_field.name));

            pw.attrset(COLOR_PAIR(color_set.value as u32));
            pw.addstr((fld.fmt_fn)(df));

        };

        pw.getch();

        Ok(())

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