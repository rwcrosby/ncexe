//!
//! Formatter for the MacOS Mach-O format
//!

use anyhow::Result;
use memmap2::Mmap;
use std::{fmt, ops::Deref};

use crate::{
    formatter::FieldMap,
    windows::line::{Line, PairVec},
};

use super::Executable;

// ------------------------------------------------------------------------

pub struct MachO32 {
    filename: String,
    mmap: Mmap,
}

impl MachO32 {
    pub fn new(filename: &str, mmap: Mmap) -> Self {
        MachO32 {
            filename: String::from(filename),
            mmap,
        }
    }
}

// ------------------------------------------------------------------------

impl<'l> Line<'l> for MachO32 {
    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        Ok(Vec::from([(
            None,
            String::from("Mach-O 32 Not Supported Yet"),
        )]))
    }

}

// ------------------------------------------------------------------------

impl Executable for MachO32 {
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

impl fmt::Display for MachO32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Mach-O 32 Bit")
    }
}

impl fmt::Debug for MachO32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Mach-O 32 Bit: {}: {:p}/{}",
            self.filename,
            self.mmap.as_ptr(),
            self.len(),
        )
    }
}
