//! 
//! An expandable list window
//! 

use anyhow::Result;

use crate::{
    color::{WindowSetColors, Colors},
    formatter::center_in,
    windows::{
        footer::Footer,
        header::Header,
        line::Line,
        scrollable_region::ScrollableRegion,
        self,
    },
};

// ------------------------------------------------------------------------

pub fn show<'a >(
    lines: Vec<Box<dyn Line>>,
    title: &'a str,
    trailer: &'a str,
    wsc: &'a WindowSetColors,
    colors : &'a Colors,
) -> Result<()> {

    // Create header window

    let hdr_fn = move | _sc: usize | (1, title.into());
    let mut hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

    // Create the scrollable window

    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines,
        colors,
    );

    // Create the footer window

    let footer_fn = | sc: usize | center_in(sc, trailer );
    let mut ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
    );

    // Create and show the set of windows

    windows::show(
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win,
    )

}