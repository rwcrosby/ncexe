//!
//! Curses based executable file dumper
//!

mod configuration;

use anyhow::Result;
use clap::Parser;
use std::{
    path::PathBuf, 
    rc::Rc, 
};

use ncexe::{
    color::Colors,
    exe_types,
    exe_types::ExeType,
    screens::{
        file_list,
        file_header,

    },
    windows::screen::Screen,
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

    // Setup the list of executable objects
    let mut executables: Vec<_> = args.exe_filename
        .iter()
        .map(|fname| 
            match exe_types::new(fname) {
                Ok(exe) => exe,
                Err(err) => Rc::new(exe_types::NotExecutable {
                    filename: String::from(fname),
                    msg: err.to_string(),
                })
            }
        )
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
        file_header::show(
            executables[0].clone(), 
            &screen, 
            &colors,
        )
    } else {
        file_list::show(
            &mut executables, 
            &screen, 
            &colors,
        )
    }

}