//!
//! Formatter for the MacOS Mach-O format
//! 
//! - <https://github.com/aidansteele/osx-abi-macho-file-format-reference>
//! - <https://en.wikipedia.org/wiki/Mach-O>

use anyhow::Result;
use memmap2::Mmap;
use std::{
    ops::Deref, 
    rc::Rc
};

use crate::{
    color::{
        Colors,
        WindowColors,
    },
    formatter::{
        self,
        FieldMap,
        FieldDef, 
        ValEntry,
    },
    windows::{
        line::{
            Line,
            MaybeLineVec,
            PairVec, 
        },
        screen::Screen, details
    },
    screens::details_list,
};

use super::{
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------

pub struct MachO64 {
    filename: String,
    mmap: Mmap,
}

// ------------------------------------------------------------------------

impl MachO64 {

    pub fn new( 
        filename: &str,
        mmap: Mmap,
    ) -> Rc<MachO64> {

        Rc::new(
            MachO64{
                filename: String::from(filename), 
                mmap, 
        })

    }

}

// ------------------------------------------------------------------------

impl Executable for MachO64 {

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
    fn header_map(&self) -> &FieldMap {
        &HEADER_MAP
    }

}

// ------------------------------------------------------------------------
/// Load commands line -> new window listing the load commands

struct CmdLine<'a> {

    exe: Rc<dyn Executable>,
    data: (usize, usize),
    val_entry: Option<&'a ValEntry>,
    fields: &'static [FieldDef],
    wc: WindowColors,
    range: (usize, usize),

}

const DTL_INDENT: usize = 7;

impl Line for CmdLine<'_> {

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {

        let data = &self.exe.mmap()[self.range.0..self.range.1];

        let mut pairs = Vec::from([
            ( Some(self.wc.text), format!("{:6}", self.fields[1].to_usize(data) )),
            ( Some(self.wc.text), String::from(" ") ),
            ( Some(self.wc.text), self.fields[0].to_string(data) ),
        ]);

        if let Some(desc) = self.val_entry {
            pairs.push(
                (
                    Some(self.wc.value),
                    format!(" ({})",desc.1 ),
                )
            );
        };

        Ok(pairs)

    }

    fn expand(&self) -> Option<usize> {
        if let Some(val_entry) = self.val_entry {
            if val_entry.2.is_some() {
                return Some(DTL_INDENT);
            }
        }
        None
    }

    fn expand_fn(&self) -> Result<MaybeLineVec> {

        let mut rc = None;

        if let Some(val_entry) = self.val_entry {
            if let Some(detail_map) = val_entry.2 {

                let cmds = details::to_lines(
                    self.exe.clone(), 
                    self.data,
                    detail_map, 
                    self.wc
                );
                rc = Some(cmds);

            }
        }

        Ok(rc)

    }

}

// ------------------------------------------------------------------------

fn load_commands_on_enter(
    exe: Rc<dyn Executable>, 
    colors: &Colors,
    screen: &Screen,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("list")?;

    let num_cmds = HEADER[4].to_usize(exe.mmap());
    let cmds_len = HEADER[5].to_usize(exe.mmap());
    let mut cmd_offset = HEADER_MAP.data_len;

    let mut lines: Vec<Box<dyn Line>> = Vec::with_capacity(num_cmds); 
    for _ in 0..num_cmds {

        let cmd_slice = &exe.mmap()[cmd_offset..cmd_offset+ CMD_HEADER_MAP.data_len];
        let cmd_len: usize = CMD_HEADER[1].to_usize(cmd_slice);

        lines.push(
            Box::new(
                CmdLine{
                    exe: exe.clone(),
                    data: (cmd_offset, cmd_offset+cmd_len),
                    fields: CMD_HEADER,
                    val_entry: CMD_HEADER[0].lookup(cmd_slice),
                    wc: wsc.scrollable_region,
                    range: (cmd_offset, cmd_offset + cmd_len),
                }
            )
        );

        cmd_offset += cmd_len;

    }

    let footer = format!("Mach-O Load Commands: {} commands, {} bytes", 
        num_cmds, 
        cmds_len);

    details_list::show(
        lines,
        "Length Command",
        &footer,
        wsc,
        screen,
        colors,
    )

}

// ------------------------------------------------------------------------

const HEADER_MAP: FieldMap = FieldMap::new(HEADER);

const HEADER: &[FieldDef] =  &[

    FieldDef::new(0, 4, "Magic Number", Some(formatter::LE_32_HEX)),
    FieldDef::new(4, 4, "CPU Type", Some(formatter::LE_32_HEX) )
        .val_tbl(formatter::LE_32_USIZE, CPU_TYPE),
    FieldDef::new(8, 4, "CPU Sub-Type", Some(formatter::LE_32_HEX)),
    FieldDef::new(12, 4, "File Type", Some(formatter::LE_32_HEX)),
    FieldDef::new(16, 4, "Load Commands", Some(formatter::LE_32_STRING))
        .enter_fn(load_commands_on_enter)
        .fn_usize(formatter::LE_32_USIZE),
    FieldDef::new(20, 4, "Load Command Length", Some(formatter::LE_32_PTR))
        .fn_usize(formatter::LE_32_USIZE),
    FieldDef::new(24, 4, "Flags", Some(formatter::BIN_STRING)),
    FieldDef::ignore(28, 4),

];

const CPU_TYPE: &formatter::ValTable = &[

    (0x7, "x86", None),
    (0x01000007, "64 Bit x86", None),
    (0xC, "ARM", None),
    (0x0100000C, "64 Bit ARM", None),

];

// ------------------------------------------------------------------------

const CMD_HEADER_MAP: FieldMap = FieldMap::new(CMD_HEADER);

const CMD_HEADER: &[FieldDef] = &[

    FieldDef::new(0, 4, "Command Type", Some(formatter::LE_32_PTR))
       .fn_usize(formatter::LE_32_USIZE)
       .val_tbl(formatter::LE_32_USIZE, CMD_TYPE),
    FieldDef::new(4, 4, "Command Length", Some(formatter::LE_32_STRING))
       .fn_usize(formatter::LE_32_USIZE),

];

const CMD_TYPE: &formatter::ValTable = &[

    (0x19, "Segment Load", Some(&SEGMENT_LOAD_MAP64)),
    (0x0C, "Dynamic Link Library - Full Path", Some(&DLL_FULL_PATH_MAP)),

];

// ------------------------------------------------------------------------

const SEGMENT_LOAD_MAP64: FieldMap = FieldMap::new(SEGMENT_LOAD64);

const SEGMENT_LOAD64: &[FieldDef] = &[

    FieldDef::ignore(0, 4),
    FieldDef::ignore(4, 4),
    FieldDef::new(8, 16, "Segment Name", Some(formatter::BE_CHAR)),
    FieldDef::new(24, 8, "Address", Some(formatter::LE_64_PTR)),
    FieldDef::new(32, 8, "Address Size", Some(formatter::LE_64_PTR)),
    FieldDef::new(40, 8, "File Offset", Some(formatter::LE_64_PTR)),
    FieldDef::new(48, 8, "Size", Some(formatter::LE_64_PTR)),
    FieldDef::new(56, 4, "Maximum Memory Protections", Some(formatter::LE_32_PTR)),
    FieldDef::new(60, 4, "Initial Memory Protections", Some(formatter::LE_32_PTR)),
    FieldDef::new(64, 4, "Number of Sections", Some(formatter::LE_32_STRING)),
    FieldDef::new(68, 4, "Flags", Some(formatter::BIN_STRING)),

];

// ------------------------------------------------------------------------

const DLL_FULL_PATH_MAP: FieldMap = FieldMap::new(DLL_FULL_PATH);

const DLL_FULL_PATH: &[FieldDef] = &[

    FieldDef::ignore(0, 4),
    FieldDef::ignore(4, 4),
    FieldDef::ignore(8, 4),
    FieldDef::new(12, 4, "Timestamp", Some(formatter::LE_32_HEX)),
    FieldDef::new(16, 4, "Current Version", Some(formatter::LE_32_HEX)),
    FieldDef::new(20, 4, "Compatable Version", Some(formatter::LE_32_HEX)),
    FieldDef::new(24, 0, "Library Name",Some(formatter::C_STR)),

];
