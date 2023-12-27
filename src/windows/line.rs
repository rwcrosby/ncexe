//!
//! Definition of the line trait used by the scrollable window
//! 

pub trait Line {

    fn as_line(&self, max_len: usize) -> String;

}
