//!
//! Modules comprising the window manager
//! 

// #![allow(dead_code)]

pub mod details;
pub mod footer;
pub mod header;
pub mod line;
pub mod popup;
pub mod scrollable_region;

// ------------------------------------------------------------------------
/// Y/X coordinates and/or dimensions

#[derive(Debug)]
pub struct Coords {
    pub y: usize,
    pub x: usize,
}

impl From<(i32, i32)> for Coords {

    fn from(value: (i32, i32)) -> Self {
        Coords{y: value.0 as usize, x: value.1 as usize}
    }

}

pub const FSIZE_LENGTH: usize = 10;