//! Curses based executable display
//!
//! Some playing with lifetimes
//! https://gist.github.com/rust-play/1752b69650f04d9db975db82ff348a3f
//! 
//! Keyboard mapping in tmux:
//! https://stackoverflow.com/questions/18600188/home-end-keys-do-not-work-in-tmux

//! TODO: Color support
//!       12/15/23 - Initial support in file_list_window
//! TODO: Terminal resizing
//!       12/11/23 - Setup to handle resize without file list window size change

//! FIXED: Improved error handling
//!        12/11/23 - Setup for error trait
//! FIXED: show-notexe flag
//!        12/11/23 - Will be set to true is any config item is true, false otherwise

use anyhow::Result;
use clap::Parser;
use memmap2::Mmap;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

mod color;
mod configuration;
mod elf;
mod file_list_window;
mod formatter;
mod macho32;
mod macho64;
mod main_window;
mod window;

use formatter::Formatter;
use main_window::MainWindow;

// ------------------------------------------------------------------------
/// Display executable file information

#[derive(Parser, Default, Debug)]
pub struct Arguments {
    /// Name of the executable file(s)
    #[arg(required = true)]
    exe_filename: Vec<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Show non-executable files in the list
    #[clap(short, long, action)]
    show_notexe: bool,
}

// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ExeType {
    MachO32,
    MachO64,
    ELF,
    NOPE,
    //     UNIVBIN,
    //     PE,
}

impl fmt::Display for ExeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}",
            match self {
                Self::MachO64 => "Mach-O 64 Bit",
                Self::MachO32 => "Mach-O 32 Bit",
                Self::ELF => "ELF",
                Self::NOPE => "Not Executable",
        })
    }
}

const ETYPELENGTH : usize = "Portable Executable".len();

// ------------------------------------------------------------------------

#[derive(Debug)]
struct NotExecutable<'a> {
    filename: &'a str,
    msg: String,
}

impl Formatter for NotExecutable<'_> {
    fn to_string(&self) -> String {
        format!("Not an Executable: {}: {}", self.filename, self.msg)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn filename(&self) -> &str {
        self.filename
    }

}

// ------------------------------------------------------------------------

fn new_executable(filename: &str) -> Box<dyn Formatter + '_> {
    let fd = match File::open(filename) {
        Ok(v) => v,
        Err(e) => {
            return Box::new(NotExecutable {
                filename,
                msg: e.to_string(),
            })
        }
    };

    let mmap = match unsafe { Mmap::map(&fd) } {
        Ok(v) => v,
        Err(e) => {
            return Box::new(NotExecutable {
                filename,
                msg: e.to_string(),
            })
        }
    };

    if mmap.len() < 4 {
        return Box::new(NotExecutable {
            filename,
            msg: format!("Too small: {}", mmap.len()),
        });
    };

    let raw_type = unsafe { *(mmap.as_ptr() as *const u32) };
    match raw_type {
        0xfeedface => macho32::Macho32Formatter::new(filename, mmap),
        0xfeedfacf => macho64::Macho64Formatter::new(filename, mmap),
        0x7f454c46 => elf::ELFFormatter::new(filename, mmap),
        0x464c457f => elf::ELFFormatter::new(filename, mmap),
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        v => Box::new(NotExecutable {
            filename,
            msg: format!("Invalid magic number: {:x}", v),
        }),
    }
}

// ------------------------------------------------------------------------

fn main() -> Result<()> {

    // Process the arguments
    let args: Arguments = Arguments::parse();

    // println!("Args: {:?}", args);

    // Load the configuration
    let config = configuration::Configuration::new(&args).unwrap();

    // Build the list of executable objects
    let executables: Vec<_> = args.exe_filename
        .iter()
        .map(|fname| new_executable(fname))
        .filter(|exe| config.show_notexe || exe.exe_type() != ExeType::NOPE)
        .collect();

    // Process depending on how many files are of interest
    if executables.len() == 0 || 
        (executables.len() == 1 && executables[0].exe_type() == ExeType::NOPE) {
        panic!("No executable files of interest found");
    }

    let mw = MainWindow::new();

    // Get color informatino
    let colours = color::Colors::new()?;

    if executables.len() == 1 {
        executables[0].show(&mw, &colours.set("header"))
    } else {
        file_list_window::show(&executables, &mw, &config, &colours.set("file_list"))
    }

}
