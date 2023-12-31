//! 
//! Container for the line trait
//! 

use anyhow::Result;
use pancurses::chtype;

use crate::exe_types::Executable;

// ------------------------------------------------------------------------
/// Definition of the line trait used by the scrollable window

pub trait Line {

    /// Return the Executable trait for this line
    fn as_executable(&self) -> &dyn Executable;

    /// Return a set of attr/string pairs
    fn as_pairs(&self, max_len: usize) -> Result<PairVec>;

}

// ------------------------------------------------------------------------
/// The actual line is a vector of tuples(attribute,string)

pub type Pair = (Option<chtype>, String);
pub type PairVec = Vec<Pair>;
