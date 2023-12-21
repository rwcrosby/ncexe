//!
//! Modules comprising the window manager
//! 

#![allow(dead_code)]

pub mod footer;
pub mod header;
pub mod line;
pub mod screen;
pub mod scrollable_region;
pub mod window;

// ------------------------------------------------------------------------
/// Y/X coordinates and/or dimensions

#[derive(Debug)]
pub struct Coords {
    pub y: i32,
    pub x: i32,
}

impl From<(i32, i32)> for Coords {

    fn from(value: (i32, i32)) -> Self {

        Coords{y: value.0, x: value.1}

    }

}