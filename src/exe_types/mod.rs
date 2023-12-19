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

use crate::{
    color::Colors,
    formatter::Formatter,
    window::ExeWindow,
    windows::screen::Screen,
};

use macho32::Macho32Formatter;
use macho64::Macho64Formatter;
use elf::ELFFormatter;

// ------------------------------------------------------------------------
/// Trait to be implemented by the various executable handlers
pub trait ExeFormat: std::fmt::Debug {

    fn to_string(&self) -> String { String::from("") }
    fn exe_type(&self) -> ExeType;
    fn len(&self) -> usize { 0 }
    fn filename(&self) -> &str {""}

    fn show(&self, 
            _mw : &Screen,
            _parent: Option<&ExeWindow>,
            _fmt: &Formatter,
            _colors: &Colors)
        -> Result<()>
    { 
        Ok(())
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
        write!(f, "{}",
            match self {
                Self::MachO64 => "Mach-O 64 Bit",
                Self::MachO32 => "Mach-O 32 Bit",
                Self::ELF => "ELF",
                Self::NOPE => "Not Executable",
        })
    }
}

// ------------------------------------------------------------------------

pub const ETYPE_LENGTH : usize = "Portable Executable".len();

// ------------------------------------------------------------------------
/// Basic trait implementation for a non-executable file
#[derive(Debug)]
pub struct NotExecutable<'a> {
    pub filename: &'a str,
    pub msg: String,
}

impl ExeFormat for NotExecutable<'_> {
    fn to_string(&self) -> String {
        format!("Not an Executable: {}: {}", self.filename, self.msg)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn filename(&self) -> &str {
        self.filename
    }

}

// ------------------------------------------------------------------------
pub fn new(filename: &str) 
    -> Box<dyn ExeFormat + '_> 
{
    let fd = match File::open(filename) {
        Ok(v) => v,
        Err(e) => {
            return Box::new(NotExecutable {
                filename,
                msg: e.to_string(),
            })
        }
    };

    let mmap = match unsafe { Mmap::map(&fd) } {
        Ok(v) => v,
        Err(e) => {
            return Box::new(NotExecutable {
                filename,
                msg: e.to_string(),
            })
        }
    };

    if mmap.len() < 4 {
        return Box::new(NotExecutable {
            filename,
            msg: format!("Too small: {}", mmap.len()),
        });
    };

    let raw_type = unsafe { *(mmap.as_ptr() as *const u32) };
    match raw_type {
        0xfeedface => Macho32Formatter::new(filename, mmap),
        0xfeedfacf => Macho64Formatter::new(filename, mmap),
        0x7f454c46 => ELFFormatter::new(filename, mmap),
        0x464c457f => ELFFormatter::new(filename, mmap),
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        v => Box::new(NotExecutable {
            filename,
            msg: format!("Invalid magic number: {:x}", v),
        }),
    }
}

