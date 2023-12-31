//!
//! Formatter for the Linux ELF executable format
//! 

use anyhow::{
    Result, 
    bail,
};
use memmap2::Mmap;
use std::ops::Deref;
use std::rc::Rc;

use crate::windows::{
    FSIZE_LENGTH,
    file_list_window::FnameFn,
    line::{
        Line,
        PairVec,
    },
};

use super::{
    ETYPE_LENGTH,
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------

pub struct ELF<'a> {
    filename: &'a str,
    mmap: Mmap,
    fname_fn: Option<Rc<FnameFn>>,
}

// ------------------------------------------------------------------------

impl ELF<'_> {

    pub fn new( 
        filename : &str,
        mmap : Mmap
    ) -> Box<dyn Executable + '_> {

        Box::new(ELF{filename, mmap, fname_fn: None})

    }

}

// ------------------------------------------------------------------------

impl Line for ELF<'_> {

    fn as_executable(&self) -> &dyn Executable {
        self
    }

    fn as_pairs(&self, sc: usize) -> Result<PairVec> {

        let fname_fn = self.fname_fn.as_ref().unwrap();

        Ok(Vec::from([
            (None, 
             format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {fname}", 
                tl=ETYPE_LENGTH, etype=self.exe_type().to_string(),
                ml=FSIZE_LENGTH, size=self.mmap.len(),
                fname=fname_fn(sc, self.filename)
            ))
        ]))

    }

}

// ------------------------------------------------------------------------

impl Executable for ELF<'_> {

    fn exe_type(&self) -> ExeType {
        ExeType::ELF
    }
    fn filename(&self) -> &str {
        self.filename
    }
    fn len(&self) -> usize {
        self.mmap.len()
    }
    fn mmap(&self) -> &[u8] {
        self.mmap.deref()
    }
    fn fmt_yaml(&self) -> Result<&str> {

        let mmap_slice = self.mmap();

        let fmt_yaml = match mmap_slice[4] {
            1 => match mmap_slice[5] {
                1 => HDR_32_LE,
                2 => HDR_32_BE,
                v => bail!("Invalid ELF endianness {:02x}", v)
            }
            2 => match mmap_slice[5] {
                1 => HDR_64_LE,
                2 => HDR_64_BE,
                v => bail!("Invalid ELF endianness {:02x}", v)
            }
            v => bail!("Invalid ELF bit length {:02x}", v)
        };

        Ok(fmt_yaml)

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

const HDR_32_LE: &str = "
---
- {size: 4, format: !Hex, type: !Be, name: 'Magic Number'}
- {size: 1, format: !Hex, type: !Le, name: 'Bit Length'}
- {size: 1, format: !Hex, type: !Le, name: 'Endianness'}
- {size: 1, format: !Int, type: !Le, name: 'ELF Version'}
- {size: 1, format: !Hex, type: !Le, name: 'Operating System ABI'}
- {size: 1, format: !Hex, type: !Le, name: 'ABI Version'}
- {size: 7, format: !Char, type: !Ignore, name: 'Reserved'}
- {size: 2, format: !Hex, type: !Le, name: 'Object File Type'}
- {size: 2, format: !Hex, type: !Le, name: 'Instruction Set Architecture'}
- {size: 4, format: !Int, type: !Le, name: 'ELF Version'}
- {size: 4, format: !Ptr, type: !Le, name: 'Entry Point Address'}
- {size: 4, format: !Ptr, type: !Le, name: 'Program Header Offset'}
- {size: 4, format: !Ptr, type: !Le, name: 'Segment Header Offset'}
- {size: 4, format: !Binary, type: !Le, name: 'Flags'}
- {size: 2, format: !Int, type: !Le, name: 'Header Size'}
- {size: 2, format: !Int, type: !Le, name: 'Program Header Size'}
- {size: 2, format: !Int, type: !Le, name: '# of Program Headers'}
- {size: 2, format: !Int, type: !Le, name: 'Segment Header Size'}
- {size: 2, format: !Int, type: !Le, name: '# of Segment Headers'}
- {size: 2, format: !Int, type: !Le, name: 'Section Name Index'}
";

const HDR_32_BE: &str = "
---
- {size: 4, format: !Hex, type: !Be, name: 'Magic Number'}
- {size: 1, format: !Hex, type: !Be, name: 'Bit Length'}
- {size: 1, format: !Hex, type: !Be, name: 'Endianness'}
- {size: 1, format: !Int, type: !Be, name: 'ELF Version'}
- {size: 1, format: !Hex, type: !Be, name: 'Operating System ABI'}
- {size: 1, format: !Hex, type: !Be, name: 'ABI Version'}
- {size: 7, format: !Char, type: !Ignore, name: 'Reserved'}
- {size: 2, format: !Hex, type: !Be, name: 'Object File Type'}
- {size: 2, format: !Hex, type: !Be, name: 'Instruction Set Architecture'}
- {size: 4, format: !Int, type: !Be, name: 'ELF Version'}
- {size: 4, format: !Ptr, type: !Be, name: 'Entry Point Address'}
- {size: 4, format: !Ptr, type: !Be, name: 'Program Header Offset'}
- {size: 4, format: !Pre, type: !Be, name: 'Segment Header Offset'}
- {size: 4, format: !Binary, type: !Be, name: 'Flags'}
- {size: 2, format: !Int, type: !Be, name: 'Header Size'}
- {size: 2, format: !Int, type: !Be, name: 'Program Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Program Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Segment Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Segment Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Section Name Index'}

";
const HDR_64_LE: &str = "
---
- {size: 4, format: !Hex, type: !Be, name: 'Magic Number'}
- {size: 1, format: !Hex, type: !Le, name: 'Bit Length'}
- {size: 1, format: !Hex, type: !Le, name: 'Endianness'}
- {size: 1, format: !Int, type: !Le, name: 'ELF Version'}
- {size: 1, format: !Hex, type: !Le, name: 'Operating System ABI'}
- {size: 1, format: !Hex, type: !Le, name: 'ABI Version'}
- {size: 7, format: !Char, type: !Ignore, name: 'Reserved'}
- {size: 2, format: !Hex, type: !Le, name: 'Object File Type'}
- {size: 2, format: !Hex, type: !Le, name: 'Instruction Set Architecture'}
- {size: 4, format: !Int, type: !Le, name: 'ELF Version'}
- {size: 8, format: !Ptr, type: !Le, name: 'Entry Point Address'}
- {size: 8, format: !Ptr, type: !Le, name: 'Program Header Offset'}
- {size: 8, format: !Ptr, type: !Le, name: 'Segment Header Offset'}
- {size: 4, format: !Binary, type: !Le, name: 'Flags'}
- {size: 2, format: !Int, type: !Le, name: 'Header Size'}
- {size: 2, format: !Int, type: !Le, name: 'Program Header Size'}
- {size: 2, format: !Int, type: !Le, name: '# of Program Headers'}
- {size: 2, format: !Int, type: !Le, name: 'Segment Header Size'}
- {size: 2, format: !Int, type: !Le, name: '# of Segment Headers'}
- {size: 2, format: !Int, type: !Le, name: 'Section Name Index'}
";
const HDR_64_BE: &str = "
---
- {size: 4, format: !Hex, type: !Be, name: 'Magic Number'}
- {size: 1, format: !Hex, type: !Be, name: 'Bit Length'}
- {size: 1, format: !Hex, type: !Be, name: 'Endianness'}
- {size: 1, format: !Int, type: !Be, name: 'ELF Version'}
- {size: 1, format: !Hex, type: !Be, name: 'Operating System ABI'}
- {size: 1, format: !Hex, type: !Be, name: 'ABI Version'}
- {size: 7, format: !Char, type: !Ignore, name: 'Reserved'}
- {size: 2, format: !Hex, type: !Be, name: 'Object File Type'}
- {size: 2, format: !Hex, type: !Be, name: 'Instruction Set Architecture'}
- {size: 4, format: !Int, type: !Be, name: 'ELF Version'}
- {size: 8, format: !Ptr, type: !Be, name: 'Entry Point Address'}
- {size: 8, format: !Ptr, type: !Be, name: 'Program Header Offset'}
- {size: 8, format: !Ptr, type: !Be, name: 'Segment Header Offset'}
- {size: 4, format: !Binary, type: !Be, name: 'Flags'}
- {size: 2, format: !Int, type: !Be, name: 'Header Size'}
- {size: 2, format: !Int, type: !Be, name: 'Program Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Program Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Segment Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Segment Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Section Name Index'}

";
