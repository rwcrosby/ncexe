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
        FieldDef
    },
    windows::{
        footer::Footer,
        header::Header,
        line::{
            Line, 
            PairVec
        },
        screen::Screen,
        scrollable_region::ScrollableRegion,
        WindowSet,
    
    }
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

    let fields: Vec<Box<HeaderLine>> = hdr_map
        .fields
        .iter()
        .filter(| f | f.string_fn.is_some() )
        .map(|map_field| HeaderLine::new(
            exe, 
            map_field, 
            &wsc.scrollable_region, 
            exe.header_map().max_text_len,
            colors,
            screen,
        ))
        .collect();

    let mut lines = fields.iter().map(|f| -> &dyn Line { f.as_ref() }).collect();

    let scr_win = ScrollableRegion::new(&wsc.scrollable_region, &mut lines);

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
    field_def: &'a FieldDef,
    wc: &'a WindowColors,
    max_text_len: usize,
    colors: &'a Colors,
    screen: &'a Screen,
}

impl<'a> HeaderLine<'a> {
    fn new(
        exe: &'a dyn Executable,
        field_def: &'a FieldDef,
        wc: &'a WindowColors,
        max_text_len: usize,
        colors: &'a Colors,
        screen: &'a Screen,
    ) -> Box<Self> {
        Box::new(Self{
            exe,
            field_def,
            wc,
            max_text_len,
            colors,
            screen,
        })
    }
}

impl<'a> Line for HeaderLine<'a> {
    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

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
                    format!(" ({})",desc ),
                )
            );
        };

        Ok(pairs)

    }

    fn on_enter(&self) -> Result<()> {
        if let Some(efn) = self.field_def.enter_fn {
            efn(
                self.exe,
                self.colors,
                self.screen,
            )
        } else {
            Ok(())
        }
    }

    fn line_ind(&self) -> Option<char> {
        match self.field_def.enter_fn {
            Some(_) => Some('='),
            None => None
        }
    }


}
