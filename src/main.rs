//!
//! Curses based executable display
//!

mod color;
mod configuration;
mod exe_types;
mod formatter;
mod window;
mod windows;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use color::Colors;
use exe_types::ExeType;
use formatter::Formatter;
use windows::{
    screen::Screen,
    file_list_window,
};

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

fn main() -> Result<()> {

    // Process the arguments
    let args: Arguments = Arguments::parse();

    // Load the configuration
    let config = configuration::Configuration::new(&args)?;

    // Build the list of executable objects
    let executables: Vec<_> = args.exe_filename
        .iter()
        .map(|fname| exe_types::new(fname))
        .filter(|exe| config.show_notexe || exe.exe_type() != ExeType::NOPE)
        .collect();

    if executables.len() == 0 || 
        (executables.len() == 1 && executables[0].exe_type() == ExeType::NOPE) {
        panic!("No executable files of interest found");
    }

    // Initialize curses
    let screen = Screen::new();

    // Setup colors
    let colors = Colors::new()?;
    screen.win.bkgd(pancurses::COLOR_PAIR(colors.bkgr() as u32));
    screen.win.refresh();

    // Get format mapper
    let formatter = Formatter::new();

    // Display file info
    if executables.len() == 1 {
        executables[0].show(&screen, None, &formatter, &colors)
    } else {
        file_list_window::show(&executables, &screen, &formatter, &colors)
    }

}
