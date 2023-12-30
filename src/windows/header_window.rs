#![allow(dead_code, unused)]

//! 
//! The executable file header window
//! 

use anyhow::Result;

use crate::{
    color::Colors,
    exe_types::Executable,
    formatter::Formatter,
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

pub fn show(
    exe : &dyn Executable,
    screen : &Screen,
    fmt: &Formatter,
    colors: &Colors,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("file_header")?;

    // Create header window

    let hdr_fn = | sc: usize | String::from("Blah");

    let mut hdr_win = Header::new(&wsc.header, &hdr_fn);

    // Create the scrollable window
        
    let mut lines: Vec<&dyn Line> = Vec::from([]);

    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        &mut lines,
        screen,
        fmt,
        colors,
    );

    // Create the footer window

    let footer_fn = | sc: usize | 
        { (10, String::from("Blah")) };

    let mut ftr_win = Footer::new(&wsc.footer, &footer_fn);

    // Create and show the set of windows

    let mut win_set = WindowSet::new(
        &screen, 
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win,
    );

    win_set.show()

}
