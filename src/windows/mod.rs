//!
//! Modules comprising the window manager
//! 

// #![allow(dead_code)]

pub mod details;
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
    pub y: usize,
    pub x: usize,
}

impl From<(i32, i32)> for Coords {

    fn from(value: (i32, i32)) -> Self {
        Coords{y: value.0 as usize, x: value.1 as usize}
    }

}

pub const FSIZE_LENGTH: usize = 10;

// ------------------------------------------------------------------------
/// The set of widnows (header, scrollable region, footer)

pub struct WindowSet<'a> {

    screen: &'a Screen,
    hdr_win: Box<Header<'a>>,
    scr_win: Box<ScrollableRegion<'a>>,
    ftr_win: Box<Footer<'a>>,

}

impl WindowSet<'_> {

    pub fn new<'a>(
        screen: &'a Screen,
        hdr_win: Box<Header<'a>>,
        scr_win: Box<ScrollableRegion<'a>>,
        ftr_win: Box<Footer<'a>>,
    ) -> Box<WindowSet<'a>> 
    {
        Box::new(WindowSet{screen, hdr_win, scr_win, ftr_win})
    }

    // --------------------------------------------------------------------

    pub fn show(&mut self) -> Result<()> {

        let size: Coords = self.screen.win.get_max_yx().into();

        self.hdr_win.show(&size)?;
        self.scr_win.show(&size)?;
        self.ftr_win.show(&size)?;
        pancurses::doupdate();
        
        // Loop handling keystrokes

        loop {

            let ch = self.scr_win.pwin.getch();

            match ch {

                Some(Input::KeyResize) => self.key_resize_handler()?,

                Some(Input::Character(c)) => match c {
                    'q' | '\u{1b}' => break,
                    '\n' => {
                        self.scr_win.handle_key(Input::KeyEnter)?;
                        self.repaint()?;
                    }
                    _ => (),
                    },

                Some(c) => self.scr_win.handle_key(c)?,
                None => (),
    
            };

        }

        Ok(())

    }

    // --------------------------------------------------------------------
    
    fn key_resize_handler(&mut  self) -> Result<()> {
        
        let new_size: Coords = self.screen.win.get_max_yx().into();
        
        self.hdr_win.resize(&new_size)?;
        self.scr_win.resize(&new_size)?;
        self.ftr_win.resize(&new_size)?;
        
        pancurses::doupdate();
        
        Ok(())
        
    }

    // --------------------------------------------------------------------

    fn repaint(&mut self) -> Result<()> {

        let size: Coords = self.screen.win.get_max_yx().into();

        self.hdr_win.pwin.touch();
        self.hdr_win.resize(&size)?;
        self.scr_win.pwin.touch();
        self.scr_win.resize(&size)?;
        self.ftr_win.pwin.touch();
        self.ftr_win.resize(&size)?;

        pancurses::doupdate();
        
        Ok(())
    }

}
