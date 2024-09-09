//!
//! Produce a set of detail lines for a field memory block
//!

use anyhow::Result;

use crate::{
    color::WindowColors,
    exe_types::ExeRef,
    formatter::{FieldDef, FieldMap},
};

use super::line::{Line, LineItem, LineVec, PairVec};

// ------------------------------------------------------------------------

pub fn to_lines<'l>(
    exe: ExeRef<'l>,
    data: (usize, usize),
    map: &FieldMap<'l>,
    wc: WindowColors,
) -> LineVec<'l> {
    map.fields
        .iter()
        .filter(|f| f.string_fn.is_some() || f.string_fn2.is_some())
        .map(|map_field| -> LineItem<'l> {
            Box::new(DetailLine {
                exe,
                data,
                field_def: map_field,
                wc,
                max_text_len: map.max_text_len,
            })
        })
        .collect()
}

// ------------------------------------------------------------------------

struct DetailLine<'dl> {
    exe: ExeRef<'dl>,
    data: (usize, usize),
    field_def: &'dl FieldDef<'dl>,
    wc: WindowColors,
    max_text_len: usize,
}

impl<'l> Line<'l> for DetailLine<'l> {
    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        let fld = self.field_def;
        // let data_slice = &[];
        let data_slice = &self.exe.mmap()[self.data.0..self.data.1];

        let mut pairs = Vec::from([
            (
                Some(self.wc.text),
                format!("{fld:l$.l$} :", l = self.max_text_len, fld = fld.name,),
            ),
            (
                Some(self.wc.value),
                format!(" {}", (self.field_def.to_string(data_slice)?)),
            ),
        ]);

        if let Some(desc) = fld.lookup(data_slice) {
            pairs.push((Some(self.wc.value), format!(" ({})", desc.1)));
        };

        Ok(pairs)
    }

    fn new_window(&self) -> bool {
        self.field_def.enter_fn.is_some()
    }

    fn new_window_fn(&self) -> Result<()> {
        if let Some(efn) = self.field_def.enter_fn {
            efn(self.exe)
        } else {
            Ok(())
        }
    }
}
