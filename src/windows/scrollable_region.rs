//!
//! The scrollable region window
//! 

use anyhow::Result;

use crate::color::ColorSet;

use crate::windows::Coords;

pub struct ScrollableRegion<'a> {
    cs: &'a  ColorSet,
}

impl ScrollableRegion<'_> {

    pub fn new<'a>(cs: &'a ColorSet) -> Box<ScrollableRegion> {
        Box::new(ScrollableRegion{ cs })
    }

    pub fn show(&self, size: &Coords) -> Result<()> {

        Ok(())

    }

}
