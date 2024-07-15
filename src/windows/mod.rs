//!
//! Modules comprising the window manager
//! 

// #![allow(dead_code)]

pub mod details;
pub mod footer;
pub mod header;
pub mod line;
pub mod popup;
pub mod screen;
pub mod scrollable_region;

use anyhow::Result;
use pancurses::Input;

use crate::windows::{
    screen::SCREEN,
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

pub fn show<'a>(
    hdr_win: &mut Header<'a>,
    scr_win: &mut ScrollableRegion,
    ftr_win: &mut Footer,
) -> Result<()> {

    let size: Coords = SCREEN.win.get_max_yx().into();

    hdr_win.show(&size)?;
    scr_win.show(&size)?;
    ftr_win.show(&size)?;
    pancurses::doupdate();
    
    // Loop handling keystrokes

    loop {

        let ch = scr_win.pwin.getch();

        match ch {

            Some(Input::KeyResize) =>
            {
                let new_size: Coords = SCREEN.win.get_max_yx().into();
        
                hdr_win.resize(&new_size)?;
                scr_win.resize(&new_size)?;
                ftr_win.resize(&new_size)?;
                
                pancurses::doupdate();
        
            }

            Some(Input::Character(c)) => match c {
                'q' | '\u{1b}' => break,
                '\n' => {
                    scr_win.handle_key(Input::KeyEnter)?;

                    let new_size: Coords = SCREEN.win.get_max_yx().into();

                    hdr_win.pwin.touch();
                    hdr_win.resize(&new_size)?;
                    scr_win.pwin.touch();
                    scr_win.resize(&new_size)?;
                    ftr_win.pwin.touch();
                    ftr_win.resize(&new_size)?;
            
                    pancurses::doupdate();

                }
                _ => (),
                },

            Some(c) => scr_win.handle_key(c)?,
            None => (),

        };

    }

    Ok(())
}
