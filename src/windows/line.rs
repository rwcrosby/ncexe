//! 
//! Container for the line trait
//! 

use anyhow::Result;
use pancurses::chtype;

use super::{scrollable_region::ScrollableRegion, Coords};

// ------------------------------------------------------------------------

pub type LineItem<'l> = Box<dyn Line<'l> + 'l>;
pub type LineVec<'l> = Vec<LineItem<'l>>;
pub type LineRef<'l> = &'l dyn Line<'l>;

pub type EnterFn<'l> = Box<dyn Fn(&mut ScrollableRegion) -> Result<()> + 'l>;
pub type NewWindowFn<'l> = Box<dyn Fn(&mut ScrollableRegion) -> Result<()> + 'l>;
pub type ExpandLinesFn<'l> = Box<dyn Fn(&mut ScrollableRegion) -> LineVec<'l> + 'l>;

// ------------------------------------------------------------------------
/// Definition of the line trait used by the scrollable window

pub trait Line<'l> {

    /// Return a set of attr/string pairs
    /// The total length is guaranteed not to exceed the specified value
    fn as_pairs(&self, max_len: usize) -> Result<PairVec>;

    fn action_type(&self) -> &'l ActionType { &ActionType::None }

    /// Expand in-line?? Return the indention amount
    fn expand(&self) -> Option<usize> { None }

    /// Function to expand 
    fn expand_fn(&self) -> Result<Option<LineVec<'l>>> { Ok(None) }
    
    // Function to call when enter is hit on the line
    fn enter_fn(&self) -> Option<EnterFn<'l>> { None }

}

// --------------------------------------------------------------------

pub enum ActionType<'at> {
    /// Open a new window on enter
    NewWindow(NewWindowFn<'at>),
    /// Tuple is expansion fn, number of expanded lines, amount to indent
    Expandable((ExpandLinesFn<'at >, usize, usize)),
    None,
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
