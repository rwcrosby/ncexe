//!
//! Formatter for the MacOS Mach-O format
//! 
//! - <https://github.com/aidansteele/osx-abi-macho-file-format-reference>
//! - <https://en.wikipedia.org/wiki/Mach-O>

use anyhow::Result;
use memmap2::Mmap;
use std::{fmt, ops::Deref};

use crate::{
    color::{
        Colors,
        WindowColors,
    }, formatter::{
        self, FieldDef, FieldMap, ValEntry
    }, screens::details_list, windows::{
        details, line::{
            Line, LineVec, MaybeLineVec, PairVec 
        }
    }
};

use super::{
    ExeRef, Executable
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
    ) -> Self {

        Self{
                filename: String::from(filename), 
                mmap, 
        }

    }

}

// ------------------------------------------------------------------------

impl Executable for MachO64 {

    fn filename(&self) -> &str {
        &self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn mmap(&self) -> &[u8] {
        self.mmap.deref()
    }
    fn header_map<'e >(&'e self) -> &'e FieldMap {
        &HEADER_MAP
    }

}

impl fmt::Display for MachO64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Mach-O 64 Bit")
    }
}

impl fmt::Debug for MachO64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Mach-O 64 Bit: {}: {:p}/{}",
            self.filename,
            self.mmap.as_ptr(),
            self.len(),
        )
    }
}

// ------------------------------------------------------------------------
/// Load commands line -> new window listing the load commands

struct CmdLine<'e> {
    exe: ExeRef<'e>,
    data: (usize, usize),
    val_entry: Option<&'e ValEntry<'e>>,
    fields: &'e [FieldDef<'e>],
    wc: WindowColors,
}

const DTL_INDENT: usize = 7;

impl<'l> Line<'l> for CmdLine<'l> {

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {

        let data = &self.exe.mmap()[self.data.0..self.data.1];

        let mut pairs = Vec::from([
            ( Some(self.wc.text), format!("{:6}", self.fields[1].to_usize(data) )),
            ( Some(self.wc.text), String::from(" ") ),
            ( Some(self.wc.text), self.fields[0].to_string(data)?),
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

    fn expand_fn(&self) -> Result<MaybeLineVec<'l>> {

        let mut rc = None;

        if let Some(val_entry) = self.val_entry {
            if let Some(detail_map) = &val_entry.2 {

                let cmds = details::to_lines(
                    self.exe, 
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

fn load_commands_on_enter<'lce>(
    exe: ExeRef<'lce>, 
) -> Result<()> {

    let wsc = Colors::global().get_window_set_colors("list")?;

    let num_cmds = HEADER[4].to_usize(exe.mmap());
    let cmds_len = HEADER[5].to_usize(exe.mmap());
    let mut cmd_offset = HEADER_MAP.data_len;

    let mut lines: LineVec<'lce> = Vec::with_capacity(num_cmds); 
    for _ in 0..num_cmds {

        let cmd_slice = &exe.mmap()[cmd_offset..cmd_offset+ CMD_HEADER_MAP.data_len];
        let cmd_len: usize = CMD_HEADER[1].to_usize(cmd_slice);

        lines.push(
            Box::new(
                CmdLine{
                    exe,
                    data: (cmd_offset, cmd_offset+cmd_len),
                    fields: CMD_HEADER,
                    val_entry: CMD_HEADER[0].lookup(cmd_slice),
                    wc: wsc.scrollable_region,
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

    (0x19, "Segment Load", Some(SEGMENT_LOAD_MAP64)),
    (0x0C, "Dynamic Link Library - Full Path", Some(DLL_FULL_PATH_MAP)),

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
    FieldDef::new2(24, 0, "Library Name",Some(formatter::C_STR)),

];
