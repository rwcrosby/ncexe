//!
//! Header window
//! 

use anyhow::Result;

use crate::color::ColorSet;

use crate::windows::Coords;

pub struct Header<'a> {
    cs: &'a  ColorSet,
}

impl Header<'_> {

    pub fn new<'a>(cs: &'a ColorSet) -> Box<Header> {
        Box::new(Header{ cs })
    }

    pub fn show(&self, size: &Coords) -> Result<()> {

        Ok(())

    }

}
