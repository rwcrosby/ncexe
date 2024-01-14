//!
//! The executable file header window
//!


use anyhow::Result;
use std::rc::Rc;

use crate::{
    color::{
        Colors, 
        WindowColors,
    },
    exe_types::Executable,
    formatter::{
        FieldDef,
        center_in, 
    },
    windows::{
        footer::Footer,
        header::Header,
        line::{
            Line, 
            MaybeLineVec,
            PairVec, 
        },
        screen::Screen,
        scrollable_region::ScrollableRegion,
        WindowSet,
    },
};

// ------------------------------------------------------------------------

pub fn show(
    exe: Rc<dyn Executable>,
    screen: &Screen,
    colors: &Colors,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("file_header")?;

    // Create header window

    let etype = exe.exe_type();

    let hdr_fn = move |sc: usize| center_in(sc, &etype.to_string());

    let hdr_win = Header::new(&wsc.header, Box::new(hdr_fn));

    // Create the scrollable window

    let hdr_map = exe.header_map();

    let wc = wsc.scrollable_region;

    let lines: Vec<Box<dyn Line>> = hdr_map
        .fields
        .iter()
        .filter(| f | f.string_fn.is_some() )
        .map(|map_field| -> Box<dyn Line> {
            Box::new(HeaderLine{
                exe: exe.clone(),
                field_def: map_field,
                wc: wc.clone(),
                max_text_len: exe.header_map().max_text_len,
        })})
        .collect();

    let scr_win = ScrollableRegion::new(
        &wsc.scrollable_region, 
        lines, 
        screen, 
        colors
    );

    // Create the footer window

    let footer_fn = |sc: usize| 
        center_in(sc, &format!("{}, {} bytes", exe.filename(), exe.len()));

    let ftr_win = Footer::new(&wsc.footer, Box::new(footer_fn));

    // Create and show the set of windows

    let mut win_set = WindowSet::new(
        &screen, 
        hdr_win, 
        scr_win, 
        ftr_win
    );

    win_set.show()

}

// ------------------------------------------------------------------------

struct HeaderLine<'a> {
    exe: Rc<dyn Executable>,
    field_def: &'a FieldDef,
    wc: WindowColors,
    max_text_len: usize,
}

impl<'a> Line for HeaderLine<'a> {

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        let fld = self.field_def;

        let mut pairs = Vec::from([
            (
                Some(self.wc.text),
                format!(
                    "{fld:l$.l$} :",
                    l = self.max_text_len,
                    fld = fld.name,
                ),
            ),
            (
                Some(self.wc.value),
                format!(" {}", (self.field_def.to_string(&self.exe.mmap()))),
            ),
        ]);

        if let Some(desc) = fld.lookup(&self.exe.mmap()) {
            pairs.push(
                (
                    Some(self.wc.value),
                    format!(" ({})",desc.1 ),
                )
            );
        };

        Ok(pairs)

    }

    fn on_enter(
        &self, 
        screen: &Screen, 
        colors: &Colors,
    ) -> Result<MaybeLineVec> {
        if let Some(efn) = self.field_def.enter_fn {
            efn(
                self.exe.clone(),
                colors,
                screen,
            )?;
        }
        Ok(None)
    }

    fn line_ind(&self) -> Option<char> {
        match self.field_def.enter_fn {
            Some(_) => Some('='),
            None => None
        }
    }


}
