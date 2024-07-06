//! 
//! Formatter for the MacOS Mach-O format
//!

use anyhow::Result;
use memmap2::Mmap;
use std::{
    rc::Rc, 
    ops::Deref
};

use crate::{
    windows::line::{
        Line,
        PairVec,
    }, 
    formatter::FieldMap,
};

use super::{
    ExeType,
    Executable,
};

// ------------------------------------------------------------------------

pub struct MachO32 {
    filename: String,
    mmap: Mmap,
}

impl MachO32 {

    pub fn new( 
        filename : &str,
        mmap : Mmap,
    ) -> Rc<MachO32> {
        Rc::new(MachO32{
            filename: String::from(filename), 
            mmap
        })
    }

}

// ------------------------------------------------------------------------

impl Line for MachO32 {
    
    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        Ok(Vec::from([(None, String::from("Mach-O 32 Not Supported Yet"))]))
    }
    
}

// ------------------------------------------------------------------------

impl Executable for MachO32 {

    fn exe_type(&self) -> ExeType {
        ExeType::MachO32
    }
    fn filename(&self) -> &str {
        &self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn mmap(&self) -> &[u8] {
        self.mmap.deref()
    }
    fn header_map(&self) -> &FieldMap {
        todo!("Header map not implmeneted for MachO32")
    }

}

