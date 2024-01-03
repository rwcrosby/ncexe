//!
//! Formatter for the MacOS Mach-O format
//! 
//! https://github.com/aidansteele/osx-abi-macho-file-format-reference
//! https://en.wikipedia.org/wiki/Mach-O

use anyhow::Result;
use memmap2::Mmap;
use std::{
    rc::Rc, 
    ops::Deref
};

use crate::{
    color::{
        Colors,
        WindowColors,
    },
    formatter::{
        self,
        FieldDef,
        MapSet,
        MapField,
    },
    windows::{
        FSIZE_LENGTH,
        file_list_window::FnameFn,
        list_window,
        line::{
            Line,
            PairVec,
        },
        screen::Screen
    },
};

use super::{
    ETYPE_LENGTH,
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------

pub struct MachO64 {
    filename: String,
    mmap: Mmap,
    fname_fn: Option<Rc<FnameFn>>,
    hdr_map: Box<MapSet>,
    cmd_hdr_map: Box<MapSet>,
}

// ------------------------------------------------------------------------

impl<'a> MachO64 {

    pub fn new( 
        filename: &'a str,
        mmap: Mmap,
        // fmt: &'a Formatter,
    ) -> Result<Box<dyn Executable>> {

        Ok(Box::new(MachO64{
            filename: String::from(filename), 
            mmap, 
            fname_fn: None,
            hdr_map: MapSet::new(HEADER),
            cmd_hdr_map: MapSet::new(CMD_HEADER),
        }))

    }

}

// ------------------------------------------------------------------------

// Return a file list line 

impl<'a> Line for MachO64 {

    fn as_executable(&self) -> &dyn Executable {
        self
    }

    fn as_pairs(&self, sc: usize) -> Result<PairVec> {

        let fname_fn = self.fname_fn.as_ref().unwrap();

        Ok(Vec::from([
            (   None,
                format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {fname}", 
                    tl=ETYPE_LENGTH, etype=self.exe_type().to_string(),
                    ml=FSIZE_LENGTH, size=self.mmap.len(),
                    fname=fname_fn(sc, &self.filename)
            ))
        ]))

    }

}

// ------------------------------------------------------------------------

impl<'a> Executable for MachO64 {

    fn exe_type(&self) -> super::ExeType {
        ExeType::MachO64
    }
    fn filename(&self) -> &str {
        &self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn mmap(&self) -> &[u8] {
        self.mmap.deref()
    }
    fn header_map(&self) -> &MapSet {
        &self.hdr_map
    }

    fn on_enter(
        &self,
        _efld_no: usize,
        colors: & Colors,
        screen: & Screen,
    ) -> Result<()> {

        let wsc = colors.get_window_set_colors("list")?;

        let num_cmds = self.hdr_map.fields[4].to_usize(self.mmap());
        let mut cmd_offset = self.hdr_map.data_len;

        let mut cmds: Vec<Box<CmdBlock>> = Vec::with_capacity(num_cmds); 
        for _ in 0..num_cmds {

            let cmd_slice = &self.mmap()
                [cmd_offset..cmd_offset+self.cmd_hdr_map.data_len];

            let cmd_len: usize = self.cmd_hdr_map
                .fields[1]
                .to_usize(cmd_slice);

            let cb = Box::new(CmdBlock{
                exe: self,
                fields: &self.cmd_hdr_map.fields,
                wc: &wsc.scrollable_region,
                data: &self.mmap[cmd_offset..cmd_offset + cmd_len],
            });

            cmds.push(cb);
            cmd_offset += cmd_len;

        }

        let mut lines = cmds
            .iter()
            .map(|f| -> &dyn Line {f})
            .collect();

        list_window::show(
            &mut lines,
            "Mach-O Load Commands",
            "Say something pithy, # commands, total len",
            wsc,
            screen,
        )

    }

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }
    fn to_line(&self) -> &dyn Line {
        self
    }

    fn set_fname_fn(&mut self, fname_fn: Rc<FnameFn>) {
        self.fname_fn = Some(fname_fn);
    }

}

// ------------------------------------------------------------------------

struct CmdBlock<'a> {

    exe: &'a dyn Executable,
    fields: &'a Vec<MapField>,
    wc: &'a WindowColors,
    data: &'a [u8],

}

impl<'a> Line for Box<CmdBlock<'a>> {
    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {

        Ok(Vec::from([
            ( Some(self.wc.text), String::from(" ") ),
            ( Some(self.wc.text), self.fields[0].to_string(self.data) ),
            ( Some(self.wc.text), String::from(" ") ),
            ( Some(self.wc.text), self.fields[1].to_string(self.data) ),
        ]))

    }

}

// ------------------------------------------------------------------------

const HEADER: &[FieldDef] = &[

    FieldDef::new(4, "Magic Number", Some(formatter::LE_32_HEX)),
    FieldDef::new(4, "CPU Type", Some(formatter::LE_32_HEX) ),
    FieldDef::new(4, "CPU Sub-Type", Some(formatter::LE_32_HEX)),
    FieldDef::new(4, "File Type", Some(formatter::LE_32_HEX)),
    FieldDef::new(4, "Load Commands", Some(formatter::LE_32_STRING))
        .enter_no(0)
        .fn_usize(formatter::LE_32_USIZE),
    FieldDef::new(4, "Load Command Length", Some(formatter::LE_32_PTR)),
    FieldDef::new(4, "Flags", Some(formatter::BIN_STRING)),
    FieldDef::ignore(4),

];

const CMD_HEADER: &[FieldDef] = &[
    FieldDef::new(4, "Command Type", Some(formatter::LE_32_PTR))
       .fn_usize(formatter::LE_32_USIZE),
    FieldDef::new(4, "Command Length", Some(formatter::LE_32_HEX))
       .fn_usize(formatter::LE_32_USIZE),
];
