//! 
//! An expandable list window
//! 

use anyhow::Result;

use crate::{
    color::WindowSetColors,
    formatter::center_in,
    screens,
    windows::{
        footer::Footer,
        header::Header,
        line::LineVec,
        scrollable_region::ScrollableRegion,
    },
};

// ------------------------------------------------------------------------

pub fn show<'s>(
    lines: LineVec<'s>,
    title: &str,
    trailer: &str,
    wsc: &'s WindowSetColors,
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
    );

    // Create the footer window

    let footer_fn = | sc: usize | center_in(sc, trailer );
    let mut ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
    );

    // Create and show the set of windows

    screens::show(
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win,
    )

}