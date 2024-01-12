//!
//! Formatter for the MacOS Mach-O format
//! 
//! https://github.com/aidansteele/osx-abi-macho-file-format-reference
//! https://en.wikipedia.org/wiki/Mach-O

use anyhow::Result;
use memmap2::Mmap;
use std::ops::Deref;

use crate::{
    color::{
        Colors,
        WindowColors,
    },
    formatter::{
        self,
        FieldMap,
        FieldDef, ValEntry,
    },
    windows::{
        line::{
            Line,
            MaybeLineVec,
            PairVec,
        },
        screen::Screen
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

impl<'a> MachO64 {

    pub fn new( 
        filename: &'a str,
        mmap: Mmap,
    ) -> Result<Box<dyn Executable>> {

        Ok(Box::new(MachO64{
            filename: String::from(filename), 
            mmap, 
        }))

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
    fn header_map(&self) -> &FieldMap {
        &HEADER_MAP
    }

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }

}

// ------------------------------------------------------------------------

struct CmdLine<'a> {

    exe: &'a dyn Executable,
    val_entry: Option<&'a ValEntry>,
    fields: &'static [FieldDef],
    wc: &'a WindowColors,
    data: &'a [u8],

}

impl Line for CmdLine<'_> {
    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {

        let mut pairs = Vec::from([
            ( Some(self.wc.text), format!("{:6}", self.fields[1].to_usize(self.data) )),
            ( Some(self.wc.text), String::from(" ") ),
            ( Some(self.wc.text), self.fields[0].to_string(self.data) ),
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

    fn on_enter(&self) -> Result<MaybeLineVec> {
        
        if let Some(val_entry) = self.val_entry {

            if let Some(_detail_tbl) = val_entry.2 {

                todo!("Create the lines to add")   

            }            

        };

        Ok(None)

    }

    fn line_ind(&self) -> Option<char> {
        match self.val_entry {
            Some(_) => Some('+'),
            None => None
        }
    }

}

// ------------------------------------------------------------------------

struct CmdDetailLine<'a> {
    exe: &'a dyn Executable,

}

impl Line for CmdDetailLine<'_> {

    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {
        todo!()
    }
}

// ------------------------------------------------------------------------

fn load_commands_on_enter(
    exe: &dyn Executable, 
    colors: &Colors,
    screen: &Screen,
) -> Result<()> {

    let wsc = colors.get_window_set_colors("list")?;

    // let num_cmds = self.hdr_map.fields[4].to_usize(self.mmap());
    let num_cmds = HEADER[4].to_usize(exe.mmap());
    let cmds_len = HEADER[5].to_usize(exe.mmap());
    let mut cmd_offset = HEADER_MAP.data_len;

    let mut cmds: Vec<CmdLine> = Vec::with_capacity(num_cmds); 
    for _ in 0..num_cmds {

        let cmd_slice = &exe.mmap()[cmd_offset..cmd_offset+ CMD_HEADER_MAP.data_len];

        let cmd_len: usize = CMD_HEADER_MAP
            .fields[1]
            .to_usize(cmd_slice);

        cmds.push(CmdLine{
            exe,
            fields: CMD_HEADER,
            val_entry: CMD_HEADER[0].lookup(cmd_slice),
            wc: &wsc.scrollable_region,
            data: &exe.mmap()[cmd_offset..cmd_offset + cmd_len],
        });

        cmd_offset += cmd_len;

    }

    let mut lines = cmds
        .iter()
        .map(| f | -> &dyn Line { f })
        .collect();

    let footer = format!("Mach-O Load Commands: {} commands, {} bytes", num_cmds, cmds_len );

    details_list::show(
        &mut lines,
        "Length Command",
        &footer,
        wsc,
        screen,
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

const CMD_HEADER_MAP: FieldMap = FieldMap::new(CMD_HEADER);

const CMD_HEADER: &[FieldDef] = &[

    FieldDef::new(0, 4, "Command Type", Some(formatter::LE_32_PTR))
       .fn_usize(formatter::LE_32_USIZE)
       .val_tbl(formatter::LE_32_USIZE, CMD_TYPE),
    FieldDef::new(4, 4, "Command Length", Some(formatter::LE_32_STRING))
       .fn_usize(formatter::LE_32_USIZE),

];

const CPU_TYPE: &formatter::ValTable = &[

    (0x7, "x86", None),
    (0x01000007, "64 Bit x86", None),
    (0xC, "ARM", None),
    (0x0100000C, "64 Bit ARM", None),

];

const CMD_TYPE: &formatter::ValTable = &[

    (0x19, "Segment Load", Some(&SEGMENT_LOAD_MAP64)),
    (0x0C, "Dynamic Link Library", None),

];

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
