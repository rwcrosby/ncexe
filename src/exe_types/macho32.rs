//! 
//! Formatter for the MacOS Mach-O format
//!

use anyhow::Result;
use memmap2::Mmap;
use std::ops::Deref;

use crate::{
    windows::line::{
        Line,
        PairVec,
    }, 
    formatter::FieldMap,
};

use super::{
    ExeItem, ExeType, Executable
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
    ) -> ExeItem {
        Box::new(MachO32{
            filename: String::from(filename), 
            mmap
        })
    }

}

// ------------------------------------------------------------------------

impl<'l> Line<'l> for MachO32 {
    
    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        Ok(Vec::from([(None, String::from("Mach-O 32 Not Supported Yet"))]))
    }
    
}

// ------------------------------------------------------------------------

impl<'e> Executable<'e> for MachO32 {

    fn exe_type(&self) -> ExeType {
        ExeType::MachO32
    }
    fn filename(&'e self) -> &'e str {
        &self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn mmap(&'e self) -> &'e [u8] {
        self.mmap.deref()
    }
    fn header_map(&self) -> &'e FieldMap {
        todo!("Header map not implmeneted for MachO32")
    }

}

