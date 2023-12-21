//!
//! Footer window
//! 

use anyhow::Result;

use crate::color::ColorSet;

use crate::windows::Coords;

pub struct Footer<'a> {
    cs: &'a  ColorSet,
}

impl Footer<'_> {

    pub fn new<'a>(cs: &'a ColorSet) -> Box<Footer> {
        Box::new(Footer{ cs })
    }

    pub fn show(&self, size: &Coords) -> Result<()> {

        Ok(())

    }

}
