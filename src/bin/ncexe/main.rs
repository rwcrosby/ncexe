//!
//! Curses based executable file dumper
//!

mod configuration;

use anyhow::Result;
use clap::Parser;
use std::{
    ops::Deref,
    path::PathBuf, 
};

use ncexe::{
    color::Colors,
    exe_types,
    exe_types::ExeType,
    formatter::Formatter,
    windows::{
        file_list_window,
        header_window,
        screen::Screen,
    },
};

// ------------------------------------------------------------------------
/// Command line argument format

#[derive(Parser, Default, Debug)]
pub struct Arguments {
    /// Name of the executable file(s)
    #[arg(required = true)]
    exe_filename: Vec<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Show non-executable files in the list
    #[arg(short, long, action)]
    show_notexe: bool,

    /// Theme to use 
    #[arg(short, long, default_value="dark")]
    theme: String,
    
}

// ------------------------------------------------------------------------

fn main() -> Result<()> {

    // Build the configuration
    let args: Arguments = Arguments::parse();
    let config = configuration::Configuration::new(&args)?;

    let fmt = Formatter::new();

    // Setup the list of executable objects
    let mut executables: Vec<_> = args.exe_filename
        .iter()
        .map(|fname| exe_types::new(fname, &fmt).unwrap())
        .filter(|exe| config.show_notexe || exe.exe_type() != ExeType::NOPE)
        .collect();

    if executables.len() == 0 || 
        (executables.len() == 1 && executables[0].exe_type() == ExeType::NOPE) {
        panic!("No executable files of interest found");
    }

    // Setup principal objects
    let screen = Screen::new();
    let colors = Colors::new(&config.theme)?;

    // Initialize screen
    screen.win.bkgd(colors.bkgr()?);
    screen.win.refresh();

    // Display file info
    if executables.len() == 1 {
        header_window::show(
            executables[0].deref(), 
            &screen, 
            &fmt, 
            &colors
        )
    } else {
        file_list_window::show(
            &mut executables, 
            &screen, 
            &fmt, 
            &colors
        )
    }

}