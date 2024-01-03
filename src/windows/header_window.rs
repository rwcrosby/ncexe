//!
//! The executable file header window
//!

use anyhow::Result;

use crate::{
    color::{
        Colors, 
        WindowColors
    },
    exe_types::Executable,
    formatter::{
        center_in, 
        MapField
    },
};

use super::{
    footer::Footer,
    header::Header,
    line::{
        Line, 
        PairVec
    },
    screen::Screen,
    scrollable_region::ScrollableRegion,
    WindowSet,
};

// ------------------------------------------------------------------------

pub fn show<'a>(
    exe: &'a dyn Executable,
    screen: &'a Screen,
    colors: &'a Colors,
) -> Result<()> {
    let wsc = colors.get_window_set_colors("file_header")?;

    // Create header window

    let etype = exe.exe_type();

    let hdr_fn = move |sc: usize| center_in(sc, &etype.to_string());

    let hdr_win = Header::new(&wsc.header, Box::new(hdr_fn));

    // Create the scrollable window

    let hdr_map = exe.header_map();
    // let fmt_blk = fmt.from_mapfields(hdr_map)?;

    let fields: Vec<Box<HeaderLine>> = hdr_map
        .fields
        .iter()
        .map(|map_field| HeaderLine::new(
            exe, 
            map_field, 
            &wsc.scrollable_region, 
            exe.header_map().max_text_len))
        .collect();

    let mut lines = fields.iter().map(|f| -> &dyn Line { f }).collect();

    let enter_fn = Box::new(|idx: usize, _line: &dyn Line| {
        let hdr = &fields[idx];

        if let Some(efld_no) = hdr.map_field.field.enter_no {
            hdr.exe.on_enter(efld_no, colors, screen)
        } else {
            Ok(())
        }
    });

    let scr_win = ScrollableRegion::new(&wsc.scrollable_region, &mut lines, enter_fn);

    // Create the footer window

    let footer_fn = |sc: usize| center_in(sc, &format!("{}, {} bytes", exe.filename(), exe.len()));

    let ftr_win = Footer::new(&wsc.footer, Box::new(footer_fn));

    // Create and show the set of windows

    let mut win_set = WindowSet::new(&screen, hdr_win, scr_win, ftr_win);

    win_set.show()
}

// ------------------------------------------------------------------------

struct HeaderLine<'a> {
    exe: &'a dyn Executable,
    map_field: &'a MapField,
    wc: &'a WindowColors,
    max_text_len: usize,
}

impl<'a> HeaderLine<'a> {
    fn new(
        exe: &'a dyn Executable,
        map_field: &'a MapField,
        wc: &'a WindowColors,
        max_text_len: usize,
    ) -> Box<HeaderLine<'a>> {
        Box::new(HeaderLine {
            exe,
            map_field,
            wc,
            max_text_len,
        })
    }
}

impl<'a> Line for Box<HeaderLine<'a>> {
    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        let fld = self.map_field;

        Ok(Vec::from([
            (
                Some(self.wc.text),
                format!(
                    " {fld:l$.l$} :",
                    l = self.max_text_len,
                    fld = fld.field.name,
                ),
            ),
            (
                Some(self.wc.value),
                format!(" {}", (self.map_field.to_string(&self.exe.mmap()))),
            ),
        ]))
    }
}
