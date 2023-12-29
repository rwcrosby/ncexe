//! 
//! Show the file list window
//!

use anyhow::Result;
use std::rc::Rc;

use crate::{
    color::Colors,
    exe_types::{
        Executable, 
        ETYPE_LENGTH
    },
    formatter::Formatter, 
};

use super::{
    FSIZE_LENGTH,
    WindowSet,
    line::Line,
    footer::Footer,
    header::Header,
    screen::Screen,
    scrollable_region::ScrollableRegion,
};

// ------------------------------------------------------------------------

type ExeItem<'a> = Box<dyn Executable + 'a>;
type ExeList<'a> = Vec<ExeItem<'a>>;

pub type FnameFn = dyn Fn(usize, &str) -> String;

// ------------------------------------------------------------------------

pub fn show(
    executables: &mut ExeList, 
    screen: &Screen,
    fmt: &Formatter,
    colors: &Colors
) -> Result<()> {

    let wsc = colors.get_window_set_colors("file_list")?;

    // These only need to be computed once for the life of the window
    
    let mut lfn = 0usize;
    let mut sfn = std::usize::MAX;
    
    executables
        .iter()
        .for_each(| exe | {
                lfn = std::cmp::max(lfn, exe.filename().len());
                sfn = std::cmp::min(sfn, exe.filename().len());
        });
    
    let mfn = std::cmp::min(20, sfn);

    // Create header window

    let hdr = format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {filename}", 
        tl=ETYPE_LENGTH, etype="Type",
        ml=FSIZE_LENGTH, size="Size",
        filename="Name",
    );

    let hdr_fn = | sc: usize | 
        if sc > hdr.len() {
            &hdr
        }
        else {
            &hdr.as_str()[0..sc]
        };

    let mut hdr_win = Header::new(&wsc.header, &hdr_fn);

    // Create the scrollable window
        
    let fname_fn = Rc::new(move | sc: usize, filename: &str | -> String {

        let fal: i32 = sc as i32 - (3 + ETYPE_LENGTH + FSIZE_LENGTH) as i32;

        if fal  < mfn as i32 {
            format!("{filename:l$.l$}", l=mfn)
        } else if fal < lfn as i32 {
            if filename.len() <= fal as usize {
                format!("{filename:l$.l$}", l=fal as usize)
            } else {
                format!("{rtrunc:l$.l$}", 
                    l=fal as usize,
                    rtrunc=&filename[(filename.len() - fal as usize)..])
            }
        } else {
            format!("{filename:l$.l$}", l=fal as usize)
        }

    });

    let mut lines: Vec<&dyn Line> = 
        executables
            .iter_mut()
            .map(|e| {
                e.set_fname_fn(fname_fn.clone());
                e.to_line()
            })
            .collect();

    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        &mut lines,
        screen,
        fmt,
        colors,
    );

    // Create the footer window

    let mut ftr_win = Footer::new(&wsc.footer);
    
    // Create and show the set of windows

    let mut win_set = WindowSet::new(
        &screen, 
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win,
    );

    win_set.show()

}
