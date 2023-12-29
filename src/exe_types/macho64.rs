//! 
//! Formatter for the MacOS Mach-O format
//! 

use anyhow::Result;
use memmap2::Mmap;
use std::rc::Rc;

use crate::{
    color::Colors,
    formatter::Formatter,
    windows::{
        FSIZE_LENGTH,
        file_list_window::FnameFn,
        line::{
            Line,
            LineVec,
        },
        screen::Screen,
    },
};

use super::{
    ETYPE_LENGTH,
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------
// #[derive(Debug)]
pub struct MachO64<'a> {
    filename: &'a str,
    mmap: Mmap,
    fname_fn: Option<Rc<FnameFn>>,
}

// ------------------------------------------------------------------------

impl<'a> MachO64<'a> {

    pub fn new( 
        filename: &'a str,
        mmap: Mmap,
    ) -> Box<dyn Executable + 'a> {

        Box::new(MachO64{filename, mmap, fname_fn: None})

    }

}

// ------------------------------------------------------------------------
// Return a file list line 

impl Line for MachO64<'_> {

    fn as_line(&self, sc: usize) -> LineVec {

        let fname_fn = self.fname_fn.as_ref().unwrap();

        Vec::from([
            (   None,
                format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {fname}", 
                    tl=ETYPE_LENGTH, etype=self.exe_type().to_string(),
                    ml=FSIZE_LENGTH, size=self.mmap.len(),
                    fname=fname_fn(sc, self.filename)
                )
    
            )
        ])

    }

    fn to_executable(&self) -> &dyn Executable {
        self
    }

}

// ------------------------------------------------------------------------

impl Executable for MachO64<'_> {

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }

    fn exe_type(&self) -> super::ExeType {
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
        _screen : &Screen,
        _fmt: &Formatter,
        _colors: &Colors
    ) -> Result<()> {

      Ok(())

    }

    fn to_line(&self) -> &dyn Line {
        self
    }

    fn set_fname_fn(&mut self, fname_fn: Rc<FnameFn>) {
        self.fname_fn = Some(fname_fn);
    }

}

// ------------------------------------------------------------------------

const _HEADER: &str = "
---

-   name: Magic Number 
    format: !Hex
    type: !Le
    size: 4
-   name: CPU Type 
    format: !Hex
    type: !Le
    size: 4
-   name: CPU Sub-Type 
    format: !Hex
    type: !Le
    size: 4
-   name: File Type 
    type: !Le
    format: !Hex
    size: 4
-   name: Load Commands
    type: !Le
    format: !Int
    size: 4
-   name: Load Command Length
    type: !Le
    format: !Ptr
    size: 4
-   name: Flags
    type: !Le
    format: !Binary
    size: 4
-   name: Reserved
    type: !Ignore
    format: !Char
    size: 4

";