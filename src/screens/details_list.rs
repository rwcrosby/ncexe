//! 
//! An expandable list window
//! 

use anyhow::Result;

use crate::{
    color::{WindowSetColors, Colors},
    formatter::center_in,
    windows::{
        WindowSet,
        footer::Footer,
        header::Header,
        line::Line,
        screen::Screen,
        scrollable_region::ScrollableRegion,
            
    },
};

// ------------------------------------------------------------------------

pub fn show<'a >(
    lines: Vec<Box<dyn Line>>,
    title: &'a str,
    trailer: &'a str,
    wsc: &'a WindowSetColors,
    screen : &'a Screen,
    colors : &'a Colors,
) -> Result<()> {

    // Create header window

    let hdr_fn = move | _sc: usize | (1, title.into());
    let hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

    // Create the scrollable window

    let scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines,
        screen,
        colors,
    );

    // Create the footer window

    let footer_fn = | sc: usize | center_in(sc, trailer );
    let ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
    );

    // Create and show the set of windows

    WindowSet::new(
        screen, 
        hdr_win, 
        scr_win, 
        ftr_win,
    ).show()

}