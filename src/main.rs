use clap::Parser;
use memmap2::Mmap;
use std::io::{Error, Result};
use std::fs::File;

/// Display executable file information
#[derive(Parser,Default,Debug)]
struct Arguments {

    /// Name of the executable file(s)
    #[arg(required=true)]
    exe_filename: Vec<String>
}

#[derive(Debug)]
struct Mapfile<'a> {
    filename: &'a str,
    mmap: Mmap,
}

impl<'a> Mapfile<'a> {
    fn new(filename: &'a str) -> Result<Self> {


        let file = File::open(filename)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self{filename: filename, 
                mmap: mmap
                }
            )
        }
    }
    
#[derive(Debug)]
enum ExeType {
    MachO32,
    MachO64,   
    UNIVBIN,   
    ELF,   
}

impl ExeType {
    fn set(mmap: &Mmap) -> Result<Self> {

        if mmap.len() < 4 {
            return Err(Error::other(format!("File too short: {}", mmap.len())))
        }

        let raw_type = unsafe{ *(mmap.as_ptr() as *const u32) };
        match raw_type {
            0xfeedfacf => Ok(ExeType::MachO64),
            0xfeedface => Ok(ExeType::MachO32),
            0xcafebabe => Ok(ExeType::UNIVBIN),
            0x7f454c46 => Ok(ExeType::ELF),
            v => Err(Error::other(format!("Invalid magic number: {:x}", v))),
        }

    }
}

fn main() {

    // Process the arguments
    let args = Arguments::parse();
    
    // Map the executable
    
    // let mapped_file = Mapfile::new(&args.exe_filename).expect("Unable to map file: ");
    let mapped_file = match Mapfile::new(&args.exe_filename[0]) {
        Ok(v) => v,
        Err(e) => panic!("Unable to map file: {}", e.to_string())
    };
    println!("{:?}", mapped_file);

    // Dump the executable

    println!("{} {:?}", mapped_file.filename, mapped_file.mmap);

    // Get the executable type

    let exe_type = match ExeType::set(&mapped_file.mmap) {
        Ok(v) => v,
        Err(e) => panic!("Bad magic number: {}", e.to_string())
    };

    println!("{:?}", exe_type);

}
