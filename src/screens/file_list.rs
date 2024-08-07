//! 
//! Show the file list window
//!

use anyhow::Result;

use crate::{
    color::Colors, 
    exe_types::{
        ExeList, 
        ExeRef, 
        ExeType, 
        ETYPE_LENGTH
    }, 
    formatter::center_in, 
    screens, 
    windows::{
        footer::Footer,
         header::Header, 
         line::{
            Line, LineVec, PairVec 
        }, 
        scrollable_region::ScrollableRegion, 
        FSIZE_LENGTH 
    }
};

use super::file_header;

// ------------------------------------------------------------------------

pub fn show<'s>(
    executables: &'s ExeList<'s>, 
) -> Result<()> {

    let wsc = Colors::global().get_window_set_colors("file_list")?;

    // These only need to be computed once for the life of the window
    
    let num_exe = executables.len();
    
    let (_sfn, _lfn) = executables
        .iter()
        .fold((usize::MAX,0 ), | m, e | (std::cmp::min(m.0, e.filename().len()),
                                         std::cmp::max(m.1, e.filename().len())));

    // Create header window

    let hdr = format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {filename}", 
        tl=ETYPE_LENGTH, etype="Type",
        ml=FSIZE_LENGTH, size="Size",
        filename="Name",
    );

    let hdr_fn = | _sc: usize | (0, hdr.clone());
    
    let mut hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

    // Create the scrollable window
        
    let mut total_len = 0;
    let lines: LineVec<'s> = executables
        .iter()
        .map(| exe | -> Box<dyn Line<'s> + 's> {
            total_len += exe.len();
            Box::new(FileLine{
                // exe: exe.as_ref(),
                exe: (*exe).as_ref(),
        })})
        .collect();

    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines,
    );

    // Create the footer window

    let footer_fn = | sc: usize | {

        let txt = format!("{} Files, {} Bytes",
            num_exe,
            total_len );

        center_in(sc, &txt)

    };

    let mut ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn),
    );
    
    // Create and show the set of windows

    screens::show(
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win
    )

}

// ------------------------------------------------------------------------
/// Line in the file list

struct FileLine<'e> {
    exe: ExeRef<'e>,
}

impl<'l> Line<'l> for FileLine<'l> {

    fn as_pairs(&self, width: usize) -> Result<PairVec> {

        let max_fname = width as isize - (ETYPE_LENGTH + FSIZE_LENGTH + 2) as isize;
        let fname = self.exe.filename();

        let first_part = format!("{etype:<tl$.tl$} {size:>ml$.ml$} ", 
            tl=ETYPE_LENGTH, etype=self.exe.exe_type().to_string(),
            ml=FSIZE_LENGTH, size=self.exe.len());

        let line =  &(first_part + if max_fname < FSIZE_LENGTH as isize {

            &fname[(fname.len() - width)..]

        }
        else {

            let start = max_fname - fname.len() as isize;

            if start < 0 {
                &fname[(-start as usize)..]
            } else {
                fname
            }

        });

        let line = if width < line.len() {
            &line[..width]
        } else {
            line
        };


        Ok(Vec::from([
            (   None,
                line.into(),
            )
        ]))

    }

    fn new_window(&self) -> bool {
        self.exe.exe_type() != ExeType::NOPE
    }

    fn new_window_fn<'a>(
        &self,
    ) -> Result<()> {
    
        file_header::show(self.exe)

    }
    
}
