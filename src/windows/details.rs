//!
//! Produce a set of detail lines for a field memory block
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
        FieldMap, 
    }, 
};

use super::{
    line::{
        Line,
        LineVec, 
        PairVec
    }, 
    screen::Screen,
};

// ------------------------------------------------------------------------

pub fn to_lines<'a>(
    exe: Rc<dyn Executable>, 
    data: (usize, usize),
    map: &FieldMap,
    wc: WindowColors,
) -> Result<LineVec<'a>> {

    Ok(map.fields
        .iter()
        .filter(| f | f.string_fn.is_some() )
        .map(|map_field| -> Box<dyn Line> {
            Box::new(DetailLine{
                exe: exe.clone(),
                data,
                field_def: map_field,
                wc: wc.clone(),
                max_text_len: map.max_text_len,
        })})
        .collect())

}

// ------------------------------------------------------------------------

struct DetailLine<'a> {
    exe: Rc<dyn Executable>, 
    data: (usize, usize),
    field_def: &'a FieldDef,
    wc: WindowColors,
    max_text_len: usize,
}

impl<'a> Line for DetailLine<'a> {

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        let fld = self.field_def;
        let data_slice = &self.exe.mmap()[self.data.0..self.data.1];

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
                format!(" {}", (self.field_def.to_string(data_slice))),
            ),
        ]);

        if let Some(desc) = fld.lookup(data_slice) {
            pairs.push(
                (
                    Some(self.wc.value),
                    format!(" ({})",desc.1 ),
                )
            );
        };

        Ok(pairs)

    }

    fn new_window(&self) -> bool {
        self.field_def.enter_fn.is_some()
    }

    fn new_window_fn<'b>(
        &'b self,
        screen: &'b Screen,
        colors: &'b Colors,
    ) -> Result<()> {
        
        if let Some(efn) = self.field_def.enter_fn {
            efn(self.exe.clone(), colors, screen)
        } else {
            Ok(())
        }

    }


}
