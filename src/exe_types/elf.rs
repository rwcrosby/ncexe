//!
//! Formatter for the Linux ELF executable format
//! 

use anyhow::{
    Result, 
    bail,
};
use memmap2::Mmap;
use std::{
    ops::Deref,
    rc::Rc
};

use crate::{
    windows::{
        FSIZE_LENGTH,
        file_list_window::FnameFn,
        line::{
            Line,
            PairVec,
        },
    },
    formatter::{
        self,
        FieldDef, 
        MapSet,
    },
};
use super::{
    ETYPE_LENGTH,
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------

pub struct ELF {
    filename: String,
    mmap: Mmap,
    fname_fn: Option<Rc<FnameFn>>,
    hdr_map: Box<MapSet>,
}

// ------------------------------------------------------------------------

impl<'a> ELF {

    pub fn new( 
        filename : &'a str,
        mmap : Mmap,
    ) -> Result<Box<dyn Executable>> {

        let mmap_slice = mmap.deref();

        let hdr_map = MapSet::new(match mmap_slice[4] {
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
        });

        Ok(Box::new(ELF{
            filename: String::from(filename), 
            mmap, 
            fname_fn: None,
            hdr_map
        }))

    }

}

// ------------------------------------------------------------------------

impl<'a> Line for ELF {

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
                fname=fname_fn(sc, &self.filename)
            ))
        ]))

    }

}

// ------------------------------------------------------------------------

impl Executable for ELF {

    fn exe_type(&self) -> ExeType {
        ExeType::ELF
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

const HDR_32_LE: &[FieldDef] = &[
	FieldDef::new(4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Bit Length", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "Endianness", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "ELF Version", Some(formatter::LE_8_STRING)),
	FieldDef::new(1, "Operating System ABI", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "ABI Version", Some(formatter::LE_8_HEX)),
	FieldDef::ignore(7),
	FieldDef::new(2, "Object File Type", Some(formatter::LE_16_HEX)),
	FieldDef::new(2, "Instruction Set Architecture", Some(formatter::LE_16_HEX)),
	FieldDef::new(4, "ELF Version", Some(formatter::LE_32_STRING)),
	FieldDef::new(4, "Entry Point Address", Some(formatter::LE_32_PTR)),
	FieldDef::new(4, "Program Header Offset", Some(formatter::LE_32_PTR)),
	FieldDef::new(4, "Segment Header Offset", Some(formatter::LE_32_PTR)),
	FieldDef::new(4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(2, "Header Size", Some(formatter::LE_32_STRING)),
	FieldDef::new(2, "Program Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "# of Program Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "Segment Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "# of Segment Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "Section Name Index", Some(formatter::LE_16_STRING)),
];

const HDR_32_BE: &[FieldDef] = &[
	FieldDef::new(4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Bit Length", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Endianness", Some(formatter::BE_HEX)),
	FieldDef::new(1, "ELF Version", Some(formatter::BE_8_STRING)),
	FieldDef::new(1, "Operating System ABI", Some(formatter::BE_HEX)),
	FieldDef::new(1, "ABI Version", Some(formatter::BE_HEX)),
	FieldDef::ignore(7),
	FieldDef::new(2, "Object File Type", Some(formatter::BE_HEX)),
	FieldDef::new(2, "Instruction Set Architecture", Some(formatter::BE_HEX)),
	FieldDef::new(4, "ELF Version", Some(formatter::BE_32_STRING)),
	FieldDef::new(4, "Entry Point Address", Some(formatter::BE_32_PTR)),
	FieldDef::new(4, "Program Header Offset", Some(formatter::BE_32_PTR)),
	FieldDef::new(4, "Segment Header Offset", Some(formatter::BE_32_PTR)),
	FieldDef::new(4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(2, "Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Program Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "# of Program Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Segment Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "# of Segment Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Section Name Index", Some(formatter::BE_16_STRING)),
];

const HDR_64_LE: &[FieldDef] = &[
	FieldDef::new(4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Bit Length", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "Endianness", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "ELF Version", Some(formatter::LE_8_STRING)),
	FieldDef::new(1, "Operating System ABI", Some(formatter::LE_8_HEX)),
	FieldDef::new(1, "ABI Version", Some(formatter::LE_8_HEX)),
	FieldDef::ignore(7),
	FieldDef::new(2, "Object File Type", Some(formatter::LE_16_HEX)),
	FieldDef::new(2, "Instruction Set Architecture", Some(formatter::LE_16_HEX)),
	FieldDef::new(4, "ELF Version", Some(formatter::LE_32_STRING)),
	FieldDef::new(8, "Entry Point Address", Some(formatter::LE_64_PTR)),
	FieldDef::new(8, "Program Header Offset", Some(formatter::LE_64_PTR)),
	FieldDef::new(8, "Segment Header Offset", Some(formatter::LE_64_PTR)),
	FieldDef::new(4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(2, "Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "Program Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "# of Program Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "Segment Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "# of Segment Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(2, "Section Name Index", Some(formatter::LE_16_STRING)),
];

const HDR_64_BE: &[FieldDef] = &[
	FieldDef::new(4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Bit Length", Some(formatter::BE_HEX)),
	FieldDef::new(1, "Endianness", Some(formatter::BE_HEX)),
	FieldDef::new(1, "ELF Version", Some(formatter::BE_8_STRING)),
	FieldDef::new(1, "Operating System ABI", Some(formatter::BE_HEX)),
	FieldDef::new(1, "ABI Version", Some(formatter::BE_HEX)),
	FieldDef::ignore(7),
	FieldDef::new(2, "Object File Type", Some(formatter::BE_HEX)),
	FieldDef::new(2, "Instruction Set Architecture", Some(formatter::BE_HEX)),
	FieldDef::new(4, "ELF Version", Some(formatter::BE_32_STRING)),
	FieldDef::new(8, "Entry Point Address", Some(formatter::BE_64_PTR)),
	FieldDef::new(8, "Program Header Offset", Some(formatter::BE_64_PTR)),
	FieldDef::new(8, "Segment Header Offset", Some(formatter::BE_64_PTR)),
	FieldDef::new(4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(2, "Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Program Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "# of Program Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Segment Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "# of Segment Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(2, "Section Name Index", Some(formatter::BE_16_STRING)),
];
