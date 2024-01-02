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

use crate::color::WindowColors;
use crate::formatter::Field;
use crate::{
    color::Colors,
    formatter::{
        Formatter, 
        FormatBlock,
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
// #[derive(Debug)]
pub struct MachO64<'a> {
    filename: &'a str,
    mmap: Mmap,
    fname_fn: Option<Rc<FnameFn>>,
    hdr_fmt: Box<FormatBlock<'a>>,
    cmd_hdr_fmt: Box<FormatBlock<'a>>,
}

// ------------------------------------------------------------------------

impl<'a> MachO64<'a> {

    pub fn new( 
        filename: &'a str,
        mmap: Mmap,
        fmt: &'a Formatter,
    ) -> Result<Box<dyn Executable + 'a>> {

        Ok(Box::new(MachO64{
            filename, 
            mmap, 
            fname_fn: None,
            hdr_fmt: fmt.from_str(HEADER)?,
            cmd_hdr_fmt: fmt.from_str(CMD_HEADER)?,
        }))

    }

}

// ------------------------------------------------------------------------
// Return a file list line 

impl Line for MachO64<'_> {

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
                    fname=fname_fn(sc, self.filename)
            ))
        ]))

    }

}

// ------------------------------------------------------------------------

impl Executable for MachO64<'_> {

    fn exe_type(&self) -> super::ExeType {
        ExeType::MachO64
    }
    fn filename(&self) -> &str {
        self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn fmt_yaml(&self) -> Result<&str> { 
        Ok(HEADER)
    }
    fn mmap(&self) -> &[u8] {
        self.mmap.deref()
    }

    fn on_enter<'a>(
        &self,
        _efld_no: usize,
        fmt: &'a Formatter,
        colors: &'a Colors,
        screen: &'a Screen,
    ) -> Result<()> {

        let wsc = colors.get_window_set_colors("list")?;

        let num_cmds = self.hdr_fmt.fields[4].try_le_usize(self.mmap())?;
        let mut cmd_offset = self.hdr_fmt.data_len;

        let mut cmds: Vec<Box<CmdBlock>> = Vec::with_capacity(num_cmds); 
        for _ in 0..num_cmds {

            let cmd_hdr = &self.mmap()
                [cmd_offset..cmd_offset+self.cmd_hdr_fmt.data_len];
            let cmd_len: usize = self.cmd_hdr_fmt
                .fields[1]
                .try_le_usize(cmd_hdr)?;
            let cb = Box::new(CmdBlock{
                exe: self,
                wc: &wsc.scrollable_region,
                data: &self.mmap[cmd_offset..cmd_offset + cmd_len],
                fmt_blk: &self.cmd_hdr_fmt,
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
            fmt,
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
    fmt_blk: &'a FormatBlock<'a>,
    wc: &'a WindowColors,
    data: &'a [u8],

}

impl Line for Box<CmdBlock<'_>> {
    fn as_executable(&self) -> &dyn Executable {
        self.exe
    }

    fn as_pairs(&self, _max_len: usize) -> Result<PairVec> {

        Ok(Vec::from([

            (
                Some(self.wc.text),
                String::from(" "),
            ),
            (
                Some(self.wc.text),
                self.to_string(&self.fmt_blk.fields[0]),
            ),
            (
                Some(self.wc.text),
                String::from(" "),
            ),
            (
                Some(self.wc.text),
                format!("{:>9.9}", self.to_string(&self.fmt_blk.fields[1])),
            ),
        ]))

    }

}

impl CmdBlock<'_> {

    fn to_string(&self, fld: &Field) -> String {

        let df = &self.data
            [fld.offset as usize..fld.offset as usize + fld.y_field.size];

        (fld.fmt_fn)(df)

    }

}

// ------------------------------------------------------------------------

const HEADER: &str = "
---

- {size: 4, format: !Hex, type: !Le, name: 'Magic Number'}
- {size: 4, format: !Hex, type: !Le, name: 'CPU Type '}
- {size: 4, format: !Hex, type: !Le, name: 'CPU Sub-Type '}
- {size: 4, format: !Hex, type: !Le, name: 'File Type'}
- {size: 4, format: !Int, type: !Le, name: 'Load Commands', on_enter: 0}
- {size: 4, format: !Ptr, type: !Le, name: 'Load Command Length'}
- {size: 4, format: !Binary, type: !Le, name: 'Flags'}
- {size: 4, format: !Char, type: !Ignore, name: 'Reserved'}

";

const CMD_HEADER: &str = "
---

- {size: 4, format: !Ptr, type: !Le, name: 'Command Type'}
- {size: 4, format: !Int, type: !Le, name: 'Command Length'}

";