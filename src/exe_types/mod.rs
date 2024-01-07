//!
//! Executable file typing
//!

pub mod elf;
pub mod macho32;
pub mod macho64;

use anyhow::Result;
use memmap2::Mmap;
use std::{
    fmt, 
    fs::File, 
};

use crate::formatter::FieldMap;

use elf::ELF;
use macho32::Macho32Formatter;
use macho64::MachO64;

// ------------------------------------------------------------------------
/// Trait to be implemented by the various executable handlers

pub trait Executable {
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn len(&self) -> usize {
        0
    }
    fn filename(&self) -> &str {
        ""
    }
    fn header_map(&self) -> &FieldMap {
        todo!("Default trait method called")
    }
    fn mmap(&self) -> &[u8] {
        &[]
    }

    fn to_string(&self) -> String {
        String::from("")
    }

}

// ------------------------------------------------------------------------
/// The types of executable files supported

#[derive(Debug, PartialEq)]
pub enum ExeType {
    MachO32,
    MachO64,
    ELF,
    NOPE,
    //     UNIVBIN,
    //     PE,
}

impl fmt::Display for ExeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::MachO64 => "Mach-O 64 Bit",
                Self::MachO32 => "Mach-O 32 Bit",
                Self::ELF => "ELF",
                Self::NOPE => "Not Executable",
            }
        )
    }
}

// ------------------------------------------------------------------------
// Constructor for an executable object

pub fn new(
    filename: &str
) -> Result<Box<dyn Executable>> {
    let fd = File::open(&filename)?;

    let mmap = unsafe { Mmap::map(&fd) }?;

    if mmap.len() < 4 {
        return Ok(Box::new(NotExecutable {
            filename: String::from(filename),
            msg: format!("Too small: {}", mmap.len()),
        }));
    };

    let raw_type = unsafe { *(mmap.as_ptr() as *const u32) };

    match raw_type {
        0xfeedface => Macho32Formatter::new(filename, mmap),
        0xfeedfacf => MachO64::new(filename, mmap),
        0x7f454c46 => ELF::new(filename, mmap),
        0x464c457f => ELF::new(filename, mmap),
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        v => Ok(Box::new(NotExecutable {
            filename: String::from(filename),
            msg: format!("Invalid magic number: {:x}", v),
        })),
    }
}
// ------------------------------------------------------------------------

pub const ETYPE_LENGTH: usize = "Portable Executable".len();

// ------------------------------------------------------------------------
/// Basic trait implementation for a non-executable file
///

#[derive(Debug)]
pub struct NotExecutable {
    pub filename: String,
    pub msg: String,
}

impl Executable for NotExecutable {
    fn to_string(&self) -> String {
        format!("Not an Executable: {}: {}", self.filename, self.msg)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn filename(&self) -> &str {
        &self.filename
    }
}

