//! 
//! Container for the line trait
//! 

use pancurses::chtype;

use crate::exe_types::Executable;

// ------------------------------------------------------------------------
/// Definition of the line trait used by the scrollable window

pub trait Line {

    /// Return a string representation of the line
    fn as_line(&self, max_len: usize) -> LineVec;

    /// Return the Executable trait for this line
    fn to_executable(&self) -> &dyn Executable;

}

// ------------------------------------------------------------------------
/// The actual line is a vector of tuples(attribute,string)

pub type LineItem = (Option<chtype>, String);
pub type LineVec = Vec<LineItem>;
