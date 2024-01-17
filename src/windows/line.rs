//! 
//! Container for the line trait
//! 


use anyhow::Result;
use pancurses::chtype;

use crate::color::Colors;

use super::{
    Coords, 
    screen::Screen
};

// ------------------------------------------------------------------------

pub type LineVec<'a> = Vec<Box<dyn Line>>;
pub type MaybeLineVec<'a> = Option<LineVec<'a>>;

// ------------------------------------------------------------------------
/// Definition of the line trait used by the scrollable window

pub trait Line {

    /// Return a set of attr/string pairs
    /// The total length is guaranteed not to exceed the specified value
    fn as_pairs(&self, max_len: usize) -> Result<PairVec>;

    /// Handle hitting enter on the line
    fn on_enter(
        &self,
        _screen: &Screen,
        _colors: &Colors,
    ) -> Result<MaybeLineVec> { Ok(None) }

    /// Open a new window?
    fn new_window(&self) -> bool { false }

    /// Function to open a full new window
    fn new_window_fn<'a>(
        &'a self,
        _screen: &'a Screen,
        _colors: &'a Colors,
    ) -> Result<()> { Ok(()) }

    /// Expand in-line?? Return the indention amount
    fn expand(&self) -> Option<usize> { None }

    /// Function to expand 
    fn expand_fn<'a>(
        &'a self,
        _screen: &'a Screen,
        _colors: &'a Colors,
    ) -> Result<MaybeLineVec> { Ok(None) }
    
}

// ------------------------------------------------------------------------
/// The generated line to be displayed is a vector of tuples(attribute,string)

pub type Pair = (Option<chtype>, String);
pub type PairVec = Vec<Pair>;

// ------------------------------------------------------------------------
/// Just write a set of pairs, without any bound checking to the window
/// starting at the specified coordinates

pub trait ToScreen {
    fn show(
        &self, 
        pwin: &pancurses::Window, 
        start_pt: Coords, 
    );
}

impl ToScreen for PairVec {

    fn show(
        &self, 
        pwin: &pancurses::Window, 
        start_pt: Coords, 
    ) {

        pwin.mv(start_pt.y as i32, start_pt.x as i32);
        
        self.iter()
                .for_each(| lp | {

            if let Some(attr) = lp.0 {
                pwin.attrset(attr);
            }

            pwin.printw(&lp.1);

        })

    }
}
