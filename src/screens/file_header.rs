//!
//! The executable file header window
//!


use anyhow::Result;
use std::rc::Rc;

use crate::{
    color::Colors, 
    exe_types::Executable,
    formatter::center_in, 
    screens,
    windows::{
        details, 
        footer::Footer, 
        header::Header, 
        scrollable_region::ScrollableRegion,
    },
};

// ------------------------------------------------------------------------

pub fn show(
    exe: Rc<dyn Executable>
) -> Result<()> {

    let wsc = Colors::global().get_window_set_colors("file_header")?;

    // Create header window

    let etype = exe.exe_type();
    let hdr_fn = move |sc: usize| center_in(sc, &etype.to_string());
    let mut hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn)
    );

    // Create the scrollable window

    let lines = details::to_lines(
        exe.clone(), 
        (0, exe.mmap().len()),
        exe.header_map(), 
        wsc.scrollable_region);
    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines 
    );

    // Create the footer window

    let footer_fn = |sc: usize| 
        center_in(sc, &format!("{}, {} bytes", exe.filename(), exe.len()));
    let mut ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
    );

    // Create and show the set of windows

    screens::show(
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win
    )

}
