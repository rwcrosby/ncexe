//!
//! Produce a set of detail lines for a field memory block
//!

use anyhow::Result;
use crate::{color, exe_types, formatter};
use super::line;

// ------------------------------------------------------------------------

pub fn to_lines<'l>(
    exe: exe_types::ExeRef<'l>,
    data: (usize, usize),
    map: &formatter::FieldMap<'l>,
    wc: color::WindowColors,
) -> line::LineVec<'l> {
    map.fields
        .iter()
        .filter(|f| f.string_fn.is_some() || f.string_fn2.is_some())
        .map(|field_def| -> line::LineItem<'l> {
            Box::new(DetailLine {
                exe,
                data,
                field_def,
                wc,
                max_text_len: map.max_text_len,
                action: if field_def.enter_fn.is_some() {
                    Some(line::ActionType::NewWindow(Box::new(|| field_def.enter_fn.unwrap()(exe))))
                } else {
                    None
                },
            })
        })
        .collect()
}

// ------------------------------------------------------------------------

struct DetailLine<'dl> {
    exe: exe_types::ExeRef<'dl>,
    data: (usize, usize),
    field_def: &'dl formatter::FieldDef<'dl>,
    wc: color::WindowColors,
    max_text_len: usize,
    action: Option<line::ActionType<'dl>>,
}

impl<'l> line::Line<'l> for DetailLine<'l> {
    fn as_pairs(&self, _max_len: usize) -> Result<line::PairVec> {
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

    fn enter_fn(&self) -> Option<line::EnterFn<'l>> {
        match self.field_def.enter_fn {
            Some(_) => Some(Box::new(|_sr| self.field_def.enter_fn.unwrap()(self.exe))),
            None => None,
        }
    }

    fn action_type(&self) -> Option<&line::ActionType<'l>> {
        if let Some(ref at) = self.action {
            Some(at)
        } else {
            None
        }
    }

    fn action_type_mut(&mut self) -> Option<&mut line::ActionType<'l>> {
        if let Some(ref mut at) = self.action {
            Some(at)
        } else {
            None
        }
    }

}
