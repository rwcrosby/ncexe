//!
//! Formatter for the Linux ELF executable format
//! 

use anyhow::{
    Result, 
    bail,
};
use memmap2::Mmap;
use std::{ops::Deref, rc::Rc};

use crate::formatter::{
    self,
    FieldDef, 
    FieldMap,
};
use super::{
    Executable,
    ExeType,
};

// ------------------------------------------------------------------------

pub struct ELF {
    filename: String,
    mmap: Mmap,
    hdr_map: &'static FieldMap,
}

// ------------------------------------------------------------------------

impl ELF {

    pub fn new( 
        filename : &str,
        mmap : Mmap,
    // ) -> Result<Rc<dyn Executable>> {
    ) -> Result<Rc<ELF>> {

        let mmap_slice = mmap.deref();

        let hdr_map = match mmap_slice[4] {
            1 => match mmap_slice[5] {
                1 => &HEADER_MAP_32_LE,
                2 => &HEADER_MAP_32_BE,
                v => bail!("Invalid ELF endianness {:02x}", v)
            }
            2 => match mmap_slice[5] {
                1 => &HEADER_MAP_64_LE,
                2 => &HEADER_MAP_64_BE,
                v => bail!("Invalid ELF endianness {:02x}", v)
            }
            v => bail!("Invalid ELF bit length {:02x}", v)
        };

        Ok(Rc::new(ELF{
            filename: String::from(filename), 
            mmap, 
            // fname_fn: None,
            hdr_map
        }))

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
    fn header_map(&self) -> &FieldMap {
        self.hdr_map
    }

}

// ------------------------------------------------------------------------

const HEADER_MAP_32_LE: FieldMap = FieldMap::new(HDR_32_LE);
const HEADER_MAP_32_BE: FieldMap = FieldMap::new(HDR_32_BE);
const HEADER_MAP_64_LE: FieldMap = FieldMap::new(HDR_64_LE);
const HEADER_MAP_64_BE: FieldMap = FieldMap::new(HDR_64_BE);

const HDR_32_LE: & [FieldDef] = &[
	FieldDef::new(0, 4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(4, 1, "Bit Length", Some(formatter::LE_8_HEX)),
	FieldDef::new(5, 1, "Endianness", Some(formatter::LE_8_HEX)),
	FieldDef::new(6, 1, "ELF Version", Some(formatter::LE_8_STRING)),
	FieldDef::new(7, 1, "Operating System ABI", Some(formatter::LE_8_HEX)),
	FieldDef::new(8, 1, "ABI Version", Some(formatter::LE_8_HEX)),
	FieldDef::ignore(9, 7),
	FieldDef::new(16, 2, "Object File Type", Some(formatter::LE_16_HEX)),
	FieldDef::new(18, 2, "Instruction Set Architecture", Some(formatter::LE_16_HEX)),
	FieldDef::new(20, 4, "ELF Version", Some(formatter::LE_32_STRING)),
	FieldDef::new(24, 4, "Entry Point Address", Some(formatter::LE_32_PTR)),
	FieldDef::new(38, 4, "Program Header Offset", Some(formatter::LE_32_PTR)),
	FieldDef::new(32, 4, "Segment Header Offset", Some(formatter::LE_32_PTR)),
	FieldDef::new(36, 4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(40, 2, "Header Size", Some(formatter::LE_32_STRING)),
	FieldDef::new(42, 2, "Program Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(44, 2, "# of Program Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(46, 2, "Segment Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(48, 2, "# of Segment Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(50, 2, "Section Name Index", Some(formatter::LE_16_STRING)),
];

const HDR_32_BE: &[FieldDef] = &[
	FieldDef::new(0, 4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(4, 1, "Bit Length", Some(formatter::BE_HEX)),
	FieldDef::new(5, 1, "Endianness", Some(formatter::BE_HEX)),
	FieldDef::new(6, 1, "ELF Version", Some(formatter::BE_8_STRING)),
	FieldDef::new(7, 1, "Operating System ABI", Some(formatter::BE_HEX)),
	FieldDef::new(8, 1, "ABI Version", Some(formatter::BE_HEX)),
	FieldDef::ignore(9, 7),
	FieldDef::new(16, 2, "Object File Type", Some(formatter::BE_HEX)),
	FieldDef::new(18, 2, "Instruction Set Architecture", Some(formatter::BE_HEX)),
	FieldDef::new(20, 4, "ELF Version", Some(formatter::BE_32_STRING)),
	FieldDef::new(24, 4, "Entry Point Address", Some(formatter::BE_32_PTR)),
	FieldDef::new(28, 4, "Program Header Offset", Some(formatter::BE_32_PTR)),
	FieldDef::new(32, 4, "Segment Header Offset", Some(formatter::BE_32_PTR)),
	FieldDef::new(36, 4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(40, 2, "Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(42, 2, "Program Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(44, 2, "# of Program Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(46, 2, "Segment Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(48, 2, "# of Segment Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(50, 2, "Section Name Index", Some(formatter::BE_16_STRING)),
];

const HDR_64_LE: &[FieldDef] = &[
	FieldDef::new(0, 4, "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(4, 1, "Bit Length", Some(formatter::LE_8_HEX)),
	FieldDef::new(5, 1, "Endianness", Some(formatter::LE_8_HEX)),
	FieldDef::new(6, 1, "ELF Version", Some(formatter::LE_8_STRING)),
	FieldDef::new(7, 1, "Operating System ABI", Some(formatter::LE_8_HEX)),
	FieldDef::new(8, 1, "ABI Version", Some(formatter::LE_8_HEX)),
	FieldDef::ignore(9, 7),
	FieldDef::new(16, 2, "Object File Type", Some(formatter::LE_16_HEX)),
	FieldDef::new(18, 2, "Instruction Set Architecture", Some(formatter::LE_16_HEX)),
	FieldDef::new(20, 4, "ELF Version", Some(formatter::LE_32_STRING)),
	FieldDef::new(24, 8, "Entry Point Address", Some(formatter::LE_64_PTR)),
	FieldDef::new(32, 8, "Program Header Offset", Some(formatter::LE_64_PTR)),
	FieldDef::new(40, 8, "Segment Header Offset", Some(formatter::LE_64_PTR)),
	FieldDef::new(48, 4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(50, 2, "Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(52, 2, "Program Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(54, 2, "# of Program Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(56, 2, "Segment Header Size", Some(formatter::LE_16_STRING)),
	FieldDef::new(58, 2, "# of Segment Headers", Some(formatter::LE_16_STRING)),
	FieldDef::new(60, 2, "Section Name Index", Some(formatter::LE_16_STRING)),
];

const HDR_64_BE: &[FieldDef] = &[
	FieldDef::new(0, 4,  "Magic Number", Some(formatter::BE_HEX)),
	FieldDef::new(4, 1,  "Bit Length", Some(formatter::BE_HEX)),
	FieldDef::new(5, 1,  "Endianness", Some(formatter::BE_HEX)),
	FieldDef::new(6, 1,  "ELF Version", Some(formatter::BE_8_STRING)),
	FieldDef::new(7, 1,  "Operating System ABI", Some(formatter::BE_HEX)),
	FieldDef::new(8, 1,  "ABI Version", Some(formatter::BE_HEX)),
	FieldDef::ignore(9, 7),
	FieldDef::new(16, 2, "Object File Type", Some(formatter::BE_HEX)),
	FieldDef::new(18, 2, "Instruction Set Architecture", Some(formatter::BE_HEX)),
	FieldDef::new(20, 4, "ELF Version", Some(formatter::BE_32_STRING)),
	FieldDef::new(24, 8, "Entry Point Address", Some(formatter::BE_64_PTR)),
	FieldDef::new(32, 8, "Program Header Offset", Some(formatter::BE_64_PTR)),
	FieldDef::new(40, 8, "Segment Header Offset", Some(formatter::BE_64_PTR)),
	FieldDef::new(48, 4, "Flags", Some(formatter::BIN_STRING)),
	FieldDef::new(50, 2, "Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(52, 2, "Program Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(54, 2, "# of Program Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(56, 2, "Segment Header Size", Some(formatter::BE_16_STRING)),
	FieldDef::new(58, 2, "# of Segment Headers", Some(formatter::BE_16_STRING)),
	FieldDef::new(60, 2, "Section Name Index", Some(formatter::BE_16_STRING)),
];
