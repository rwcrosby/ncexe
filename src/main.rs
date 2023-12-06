//! Curses based executable display
//!
//! Some playing with lifetimes
//! https://gist.github.com/rust-play/1752b69650f04d9db975db82ff348a3f

use clap::Parser;
use memmap2::Mmap;
use std::fs::File;
use std::path::PathBuf;

mod configuration;
mod elf;
mod file_list_window;
mod formatter;
mod macho32;
mod macho64;
mod main_window;

use formatter::Formatter;
use main_window::MainWindow;

// ------------------------------------------------------------------------
/// Display executable file information

#[derive(Parser, Default, Debug)]
struct Arguments {
    /// Name of the executable file(s)
    #[arg(required = true)]
    exe_filename: Vec<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Show non-executable files in the list
    #[arg(short, long)]
    show_notexe: Option<bool>,
}

// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum ExeType {
    MachO32,
    MachO64,
    ELF,
    NOPE,
    //     UNIVBIN,
}

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

fn main() {
    // Process the arguments
    let args: Arguments = Arguments::parse();

    // println!("Args: {:?}", args);

    // Load the configuration
    let config = configuration::Configuration::new(&args).unwrap();

    // Build the list of executable objects
    let exe_vec: Vec<_> = args.exe_filename
        .iter()
        .map(|fname| new_executable(fname))
        .filter(|exe| config.show_notexe || exe.exe_type() != ExeType::NOPE)
        .collect();

    MainWindow::new().show(&exe_vec);
}
