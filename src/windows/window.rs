//!
//! Structures controlling a set of windows on the screen
//! 

use anyhow::Result;
use pancurses::Input;

use crate::windows::{
    screen::Screen,
    header::Header,
    scrollable_region::ScrollableRegion,
    footer::Footer,
};

use super::Coords;

pub struct Window {

}

pub struct WindowSet<'a> {

    screen: &'a Screen,
    hdr_win: &'a  Header<'a>,
    scr_win: &'a ScrollableRegion<'a>,
    ftr_win: &'a Footer<'a>,

}

impl WindowSet<'_> {
    pub fn new<'a>(
        screen: &'a Screen,
        hdr_win: &'a Header,
        scr_win: &'a ScrollableRegion,
        ftr_win: &'a Footer,
    ) -> Box<WindowSet<'a>> 
    {
        Box::new(WindowSet{screen, hdr_win, scr_win, ftr_win})
    }

    pub fn show(&self) -> Result<()> {

        let size: Coords = self.screen.win.get_max_yx().into();

        self.hdr_win.show(&size)?;
        self.ftr_win.show(&size)?;
        self.scr_win.show(&size)?;
        
        // Loop handling keystrokes

        loop {


            let ch = self.screen.win.getch();

            match ch {

                Some(Input::Character(c)) => match c {
                    'q' | '\u{1b}' => break,
                    '\n' => (),
                    _ => ()
                    }
                _ => (),
    
            }

        }


        Ok(())

    }
}
