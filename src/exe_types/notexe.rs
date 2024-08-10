//!
//! Not Executable file typing
//!

use std::fmt;

use crate::formatter::FieldMap;

use super::Executable;

/// Simple not executable file

pub struct NotExecutable {
    pub filename: String,
    pub msg: String,
}

impl NotExecutable {
    pub fn new(filename: &str, msg: String) -> Self {
        Self {
            filename: filename.to_string(),
            msg,
        }
    }
}

// ------------------------------------------------------------------------
/// Basic trait implementation for a non-executable file
///

impl Executable for NotExecutable {
    fn filename(&self) -> &str {
        &self.filename
    }
    fn len(&self) -> usize {
        0
    }
    fn is_empty(&self) -> bool {self.len() == 0 }

    fn mmap(&self) -> &[u8] {
        panic!("Mmap called on non-executable")
    }
    fn header_map(&self) -> &FieldMap {
        panic!("Header map called on non-executable")
    }
}

impl fmt::Display for NotExecutable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Not Executable")
    }
}

impl fmt::Debug for NotExecutable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Not Executable filename:{}, msg: {}",
            self.filename, self.msg
        )
    }
}
