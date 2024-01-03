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
    formatter::center_in, 
};

use super::{
    FSIZE_LENGTH,
    WindowSet,
    footer::Footer,
    header::Header,
    line::Line,
    header_window,
    screen::Screen,
    scrollable_region::ScrollableRegion,
};

// ------------------------------------------------------------------------

type ExeItem<'a> = Box<dyn Executable>;
type ExeList<'a> = Vec<ExeItem<'a>>;

pub type FnameFn = dyn Fn(usize, &str) -> String;

// ------------------------------------------------------------------------

pub fn show<'a>(
    executables: &'a mut ExeList, 
    screen: &'a Screen,
    colors: &'a Colors,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("file_list")?;

    // These only need to be computed once for the life of the window
    
    let mut lfn = 0usize;
    let mut sfn = std::usize::MAX;
    let num_exe = executables.len();
    
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

    let hdr_fn = | _sc: usize | (0, hdr.clone());
    
    let hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

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

    let enter_fn = | _idx: usize,  line: & dyn Line |

        header_window::show(
            line.as_executable(), 
            screen, 
            colors,

    );

    let mut total_len = 0;
    let mut lines = executables
        .iter_mut()
        .map(|e| {
            e.set_fname_fn(fname_fn.clone());
            total_len += e.len();
            e.to_line()
        })
        .collect();

    let scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        &mut lines,
        Box::new(enter_fn),
    );

    // Create the footer window

    let footer_fn = | sc: usize | {

        let txt = format!("{} Files, {} Bytes",
            num_exe,
            total_len );

        center_in(sc, &txt)

    };

    let ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn),
    );
    
    // Create and show the set of windows

    let mut win_set = WindowSet::new(
        &screen, 
        hdr_win, 
        scr_win, 
        ftr_win,
    );

    win_set.show()

}
