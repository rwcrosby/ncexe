//! Formatter for the MacOS Mach-O format

use anyhow::{Result, bail};
use memmap2::Mmap;
use std::ops::Deref;

use crate::ExeType;
use crate::FormatExe;

use crate::color::Colors;
use crate::formatter::Formatter;
use crate::header_window;
use crate::main_window::MainWindow;

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct ELFFormatter<'a> {
    filename: &'a str,
    mmap: Mmap,
}

// ------------------------------------------------------------------------

impl ELFFormatter<'_> {

    pub fn new( filename : &str,
            mmap : Mmap) -> Box<dyn FormatExe + '_> {

        Box::new(ELFFormatter{filename, mmap})

    }

}

// ------------------------------------------------------------------------

impl FormatExe for ELFFormatter<'_> {

    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }

    fn exe_type(&self) -> ExeType {
        ExeType::ELF
    }

    fn filename(&self) -> &str {
        self.filename
    }

    fn len(&self) -> usize {
        self.mmap.len()
    }

    fn show(
        &self, 
        mw : &MainWindow,
        fmt: &Formatter,
        colors: &Colors
    ) -> Result<()> {

        let mmap_slice = self.mmap.deref();

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

        let fmt_blk = fmt.from_str(fmt_yaml)?;

        header_window::show(
            mw, 
            colors, 
            &fmt_blk, 
            "ELF Header", 
            mmap_slice)

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
- {size: 4, format: !Hex, type: !Le, name: 'Entry Point Address'}
- {size: 4, format: !Int, type: !Le, name: 'Program Header Offset'}
- {size: 4, format: !Int, type: !Le, name: 'Segment Header Offset'}
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
- {size: 4, format: !Hex, type: !Be, name: 'Entry Point Address'}
- {size: 4, format: !Int, type: !Be, name: 'Program Header Offset'}
- {size: 4, format: !Int, type: !Be, name: 'Segment Header Offset'}
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
- {size: 8, format: !Hex, type: !Le, name: 'Entry Point Address'}
- {size: 8, format: !Int, type: !Le, name: 'Program Header Offset'}
- {size: 8, format: !Int, type: !Le, name: 'Segment Header Offset'}
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
- {size: 8, format: !Hex, type: !Be, name: 'Entry Point Address'}
- {size: 8, format: !Int, type: !Be, name: 'Program Header Offset'}
- {size: 8, format: !Int, type: !Be, name: 'Segment Header Offset'}
- {size: 4, format: !Binary, type: !Be, name: 'Flags'}
- {size: 2, format: !Int, type: !Be, name: 'Header Size'}
- {size: 2, format: !Int, type: !Be, name: 'Program Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Program Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Segment Header Size'}
- {size: 2, format: !Int, type: !Be, name: '# of Segment Headers'}
- {size: 2, format: !Int, type: !Be, name: 'Section Name Index'}

";
