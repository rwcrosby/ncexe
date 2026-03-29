//!
//! Container for the line trait
//!

use anyhow::Result;
use ratatui::style::Style;

use super::scrollable_region::ScrollableRegion;

// ------------------------------------------------------------------------

pub type LineItem<'l> = Box<dyn Line<'l> + 'l>;
pub type LineVec<'l> = Vec<LineItem<'l>>;
pub type LineRef<'l> = &'l dyn Line<'l>;

pub type EnterFn<'l> = Box<dyn Fn(&mut ScrollableRegion) -> Result<()> + 'l>;
pub type NewWindowFn<'l> = Box<dyn Fn() -> Result<()> + 'l>;
pub type ExpandLinesFn<'l> = Box<dyn Fn() -> LineVec<'l> + 'l>;

// ------------------------------------------------------------------------
/// Definition of the line trait used by the scrollable window

pub trait Line<'l> {

    /// Return a set of style/string pairs.
    /// The total length is guaranteed not to exceed the specified value.
    fn as_pairs(&self, max_len: usize) -> Result<PairVec>;

    fn action_type(&self) -> Option<&ActionType<'l>> { None }
    fn action_type_mut(&mut self) -> Option<&mut ActionType<'l>> { None }

    /// Expand in-line? Return the indentation amount
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
    Expandable(ExpandLinesFn<'at>, usize, usize),
}

// ------------------------------------------------------------------------
/// The generated line to be displayed is a vector of tuples (style, string)

pub type Pair = (Option<Style>, String);
pub type PairVec = Vec<Pair>;
