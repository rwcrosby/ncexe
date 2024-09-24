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
        .map(|field_def| -> LineItem<'l> {
            Box::new(DetailLine {
                exe,
                data,
                field_def,
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
        let data_slice = &self.exe.mmap()[self.data.0..self.data.1];

        let mut pairs = Vec::from([
            (
                Some(self.wc.text),
                format!(
                    "{fld:l$.l$} :",
                    l = self.max_text_len,
                    fld = self.field_def.name,
                ),
            ),
            (
                Some(self.wc.value),
                format!(" {}", (self.field_def.to_string(data_slice)?)),
            ),
        ]);

        if let Some(desc) = self.field_def.lookup(data_slice) {
            pairs.push((Some(self.wc.value), format!(" ({})", desc.1)));
        };

        Ok(pairs)
    }

    fn enter_fn(&self) -> Option<Box<dyn Fn() -> Result<()> + 'l>> {
        match self.field_def.enter_fn {
            Some(_) => Some(Box::new(|| self.field_def.enter_fn.unwrap()(self.exe))),
            None => None,
        }
    }
}
