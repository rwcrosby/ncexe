//!
//! Modules comprising the window manager
//! 

#![allow(dead_code)]

pub mod footer;
pub mod header;
pub mod line;
pub mod screen;
pub mod scrollable_region;

use anyhow::Result;
use pancurses::Input;

use crate::windows::{
    screen::Screen,
    header::Header,
    scrollable_region::ScrollableRegion,
    footer::Footer,
};

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

// ------------------------------------------------------------------------
/// The set of widnows (header, scrollable region, footer)
pub struct WindowSet<'a> {

    screen: &'a Screen,
    hdr_win: &'a mut Header<'a>,
    scr_win: &'a mut ScrollableRegion<'a>,
    ftr_win: &'a mut Footer<'a>,

}

impl WindowSet<'_> {

    pub fn new<'a>(
        screen: &'a Screen,
        hdr_win: &'a mut Header<'a>,
        scr_win: &'a mut ScrollableRegion<'a>,
        ftr_win: &'a mut Footer<'a>,
    ) -> Box<WindowSet<'a>> 
    {
        Box::new(WindowSet{screen, hdr_win, scr_win, ftr_win})
    }

    pub fn show(& mut self) -> Result<()> {

        let size: Coords = self.screen.win.get_max_yx().into();

        self.hdr_win.show(&size)?;
        self.scr_win.show(&size)?;
        self.ftr_win.show(&size)?;
        
        // Loop handling keystrokes

        loop {

            let ch = self.scr_win.pwin.getch();

            match ch {

                Some(Input::KeyResize) => self.key_resize_handler()?,

                Some(Input::Character(c)) => match c {
                    'q' | '\u{1b}' => break,
                    '\n' => (),
                    _ => (),
                    },

                _ => (),
    
            };

        }

        Ok(())

    }

    fn key_resize_handler(&mut  self) -> Result<()> {

        let new_size: Coords = self.screen.win.get_max_yx().into();

        self.hdr_win.resize(&new_size)?;
        self.scr_win.resize(&new_size)?;
        self.ftr_win.resize(&new_size)?;
    
        pancurses::doupdate();
    
        Ok(())
    
    }

}
