//! 
//! An expandable list window
//! 

use anyhow::Result;

use crate::{
    color::WindowSetColors,
    formatter::{
        Formatter, 
        center_in,
    },
};

use super::{
    WindowSet,
    footer::Footer,
    header::Header,
    line::Line,
    screen::Screen,
    scrollable_region::ScrollableRegion,
};

// ------------------------------------------------------------------------

pub fn show<'a >(
    lines: &'a mut Vec<&'a dyn Line>,
    title: &'a str,
    trailer: &'a str,
    _fmt: &'a Formatter,
    wsc: &'a WindowSetColors,
    screen : &'a Screen,
) -> Result<()> {

    // Create header window

    let hdr_fn = move | sc: usize | center_in(sc, title );

    let hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

    // Create the scrollable window

    let enter_fn = Box::new( 
        | _idx: usize, _line: &dyn Line | { 
            Ok(())
        } 
    );

    let scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines,
        enter_fn,
    );

    // Create the footer window

    let footer_fn = | sc: usize | 
        center_in(sc, trailer );

    let ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
    );

    // Create and show the set of windows

    let mut win_set = WindowSet::new(
        screen, 
        hdr_win, 
        scr_win, 
        ftr_win,
    );

    win_set.show()

}