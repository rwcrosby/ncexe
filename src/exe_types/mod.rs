//!
//! Executable file typing
//!

pub mod elf;
pub mod macho32;
pub mod macho64;
pub mod notexe;

use memmap2::Mmap;
use std::{
    fmt, 
    fs::File,
};

use crate::formatter::FieldMap;

use elf::ELF;
use macho32::MachO32;
use macho64::MachO64;
use notexe::NotExecutable;

// ------------------------------------------------------------------------
/// Trait to be implemented by the various executable handlers

pub trait Executable<'e>: fmt::Display + fmt::Debug {

    fn filename(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {self.len() == 0 }
    fn mmap(&self) -> &[u8];
    fn header_map(&'e self) -> &'e FieldMap;

}

// Convenience types for the executable trait

pub type ExeItem<'e> = Box<dyn Executable<'e> + 'e>;
pub type ExeRef<'e> = &'e dyn Executable<'e>;
pub type ExeList<'e> = Vec<ExeItem<'e>>;

// ------------------------------------------------------------------------
// Constructor for an executable object

pub fn new_exe(
    filename: &str
) -> ExeItem<'_> {

    let fd = match File::open(filename) {
        Ok(f) => f,
        Err(msg) => 
        return Box::new(NotExecutable::new(
            filename,
            msg.to_string(),
            ))
    };

    let mmap = match unsafe { Mmap::map(&fd) } {
        Ok(mmap) => mmap,
        Err(m) => 
            return Box::new(NotExecutable::new(
                filename,
                m.to_string(),
                ))
    };

    if mmap.len() < 4 {

        return Box::new(NotExecutable::new(
            filename,
            format!("Too small: {}", mmap.len()),
            ))
        ;

    };

    let raw_type = unsafe { *(mmap.as_ptr() as *const u32) };

    match raw_type {
        0xfeedface => Box::new(MachO32::new(filename, mmap)),
        0xfeedfacf => Box::new(MachO64::new(filename, mmap)),
        0x7f454c46 | 0x464c457f => match ELF::new(filename, mmap) {
            Ok(elf) => Box::new(elf),
            Err(msg)=> Box::new(NotExecutable::new(filename, msg.to_string()))
        },
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        v => Box::new(NotExecutable::new(filename, 
            format!("Invalid magic number: {:x}", v))),

    }

}

// ------------------------------------------------------------------------

pub const ETYPE_LENGTH: usize = "Portable Executable".len();
