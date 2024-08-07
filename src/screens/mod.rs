//!
//! Modules for the various screens in the program
//! 

pub mod details_list;
pub mod file_header;
pub mod file_list;
pub mod terminal;

use anyhow::Result;
use pancurses::Input;

use crate::{
    screens::terminal::TERMWIN,
    windows::{
        header::Header,
        scrollable_region::ScrollableRegion,
        footer::Footer,
        Coords,
}
};

// ------------------------------------------------------------------------

pub fn show(
    hdr_win: &mut Header,
    scr_win: &mut ScrollableRegion,
    ftr_win: &mut Footer,
) -> Result<()> {

    let size: Coords = TERMWIN.win.get_max_yx().into();

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
                let new_size: Coords = TERMWIN.win.get_max_yx().into();
        
                hdr_win.resize(&new_size)?;
                scr_win.resize(&new_size)?;
                ftr_win.resize(&new_size)?;
                
                pancurses::doupdate();
        
            }

            Some(Input::Character(c)) => match c {
                'q' | '\u{1b}' => break,
                '\n' => {
                    scr_win.handle_key(Input::KeyEnter)?;

                    let new_size: Coords = TERMWIN.win.get_max_yx().into();

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
