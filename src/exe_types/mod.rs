//!
//! Executable file typing
//! 

pub mod elf;
pub mod macho32;
pub mod macho64;

use anyhow::{
    Result,
    bail,
};
use memmap2::Mmap;
use std::{
    fmt,
    fs::File,
    rc::Rc,
};

use crate::{windows::{
    file_list_window::FnameFn,
    line::{
        Line,
        PairVec,
    }, screen::Screen,
}, formatter::Formatter, color::Colors};

use macho32::Macho32Formatter;
use macho64::MachO64;
use elf::ELF;

// ------------------------------------------------------------------------
/// Trait to be implemented by the various executable handlers
pub trait Executable {

    fn exe_type(&self) -> ExeType
    {
            ExeType::NOPE
    }
    fn len(&self) -> usize 
    { 
        0 
    }
    fn filename(&self) -> &str 
    {
        ""
    }
    fn fmt_yaml(&self) -> Result<&str>
    {
        bail!("Default trait method called")
    }
    fn mmap(&self) -> &[u8] {
        &[]
    }

    fn on_enter(
        &self, 
        _efld_no: usize,
        _fmt: &Formatter,
        _colors: &Colors,
        _screen: &Screen,
    ) -> Result<()>
    {
        Ok(())
    }

    fn to_line(&self) -> &dyn Line;
    fn to_string(&self) -> String 
    { 
        String::from("") 
    }
    
    fn set_fname_fn(&mut self, _fname_dn: Rc<FnameFn>) {}

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

impl Executable for NotExecutable<'_> {
    fn to_string(&self) -> String {
        format!("Not an Executable: {}: {}", self.filename, self.msg)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn filename(&self) -> &str {
        self.filename
    }
    fn to_line(&self) -> &dyn Line {
        self
    }
}

impl Line for NotExecutable<'_> {

    fn as_executable(&self) -> &dyn Executable {
        self
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        Ok(Vec::from([(None, String::from(" Not Executable"))]))
    }

}

// ------------------------------------------------------------------------
pub fn new<'a>(
    filename: &'a str,
    fmt: &'a Formatter,
) -> Result<Box<dyn Executable + 'a>> 
{
    let fd = File::open(filename)?;

    let mmap = unsafe { Mmap::map(&fd) }?;

    if mmap.len() < 4 {
        return Ok(Box::new(NotExecutable {
            filename,
            msg: format!("Too small: {}", mmap.len()),
        }));
    };

    let raw_type = unsafe { *(mmap.as_ptr() as *const u32) };

    match raw_type {
        0xfeedface => Macho32Formatter::new(filename, mmap),
        0xfeedfacf => MachO64::new(filename, mmap, fmt),
        0x7f454c46 => ELF::new(filename, mmap),
        0x464c457f => ELF::new(filename, mmap),
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        v => Ok(Box::new(NotExecutable {
            filename,
            msg: format!("Invalid magic number: {:x}", v),
        })),
    }
}

