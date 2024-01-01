#![allow(dead_code, unused)]

//! 
//! The executable file header window
//! 

use std::{ops::Deref, fmt::Pointer};

use anyhow::Result;

use crate::{
    color::{Colors, WindowColors},
    exe_types::Executable,
    formatter::{
        Formatter, 
        Field, 
        FormatBlock,
        center_in,
    },
};

use super::{
    WindowSet,
    footer::Footer,
    header::{Header, self},
    line::{
        Line,
        PairVec
    },
    screen::Screen,
    scrollable_region::ScrollableRegion,
};

// ------------------------------------------------------------------------

pub fn show<'a >(
    exe : &'a dyn Executable,
    screen : &'a Screen,
    fmt: &'a Formatter,
    colors: &'a Colors,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("file_header")?;

    // Create header window

    let etype = exe.exe_type();

    let hdr_fn = move | sc: usize | center_in(sc, &etype.to_string() );

    let mut hdr_win = Header::new(
        &wsc.header, 
        Box::new(hdr_fn),
    );

    // Create the scrollable window

    let enter_fn = move | idx: usize, line: &dyn Line | -> Result<()> {
        Ok(())
    };

    let fmt_yaml = exe.fmt_yaml()?;
    let fmt_blk = fmt.from_str(fmt_yaml)?;

    let mut fields: Vec<Box<HeaderLine>> = fmt_blk.fields
        .iter()
        .map(| fmt_field | HeaderLine::new(
            exe, 
            fmt_field, 
            &wsc.scrollable_region, 
            &fmt_blk)
        )
        .collect();

    let mut lines = fields
        .iter()
        .map(|f| -> &dyn Line {f})
        .collect();

    let enter_fn = Box::new( 
        | idx: usize, _line: &dyn Line | { 

            let hdr = &fields[idx];

            if let Some(efld_no) =  hdr.fmt_field.y_field.on_enter {
                hdr.exe.on_enter(efld_no)
            } else {
                Ok(())
            }

        } 
    );

    let mut scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        &mut lines,
        enter_fn,
    );

    // Create the footer window

    let filename = String::from(exe.filename());
    let file_len = exe.len();

    // let footer_fn = move | sc: usize | 
    //     center_in(sc, &format!("{}, {} bytes", filename, file_len) );

    let footer_fn = | sc: usize | 
        center_in(sc, &format!("{}, {} bytes", exe.filename(), file_len) );

    let mut ftr_win = Footer::new(
        &wsc.footer, 
        Box::new(footer_fn)
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

// ------------------------------------------------------------------------

struct HeaderLine<'a> {
    exe: &'a dyn Executable,
    fmt_field: &'a Field<'a>,
    wc: &'a WindowColors,
    fmt_blk: &'a FormatBlock<'a>,
}

impl<'a> HeaderLine<'a> {
    fn new(
        exe: &'a dyn Executable, 
        fmt_field: &'a Field<'a>,
        wc: &'a WindowColors,
        fmt_blk: &'a FormatBlock,
    ) -> Box<HeaderLine<'a>> {
        Box::new(HeaderLine{exe, fmt_field, wc, fmt_blk})
    }
}

impl Line for Box<HeaderLine<'_>> {

    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, max_len: usize) -> Result<PairVec> {

        let fld = self.fmt_field;

        let df = &self.exe.mmap()
            [fld.offset as usize..fld.offset as usize + fld.y_field.size];

        Ok(Vec::from([
            
            (
                Some(self.wc.text), 
                format!(" {fld:l$.l$} :", 
                    l=self.fmt_blk.max_text_len,
                    fld=fld.y_field.name,
                )
            ),
            (
                Some(self.wc.value), 
                format!(" {}", (self.fmt_field.fmt_fn)(df) )
            ),

        ]))

    }

}