//!
//! Curses based executable file dumper
//!

mod configuration;

use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;
use std::path::PathBuf;

use ncexe::{
    color::{self, Colors},
    exe_types::{self, ExeVec},
    screens::{
        file_header,
        file_list,
        terminal::TERMWIN,
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
    #[arg(short, long, default_value = "dark")]
    theme: String,
}

// ------------------------------------------------------------------------

fn main() -> Result<()> {
    // Build the configuration
    let args: Arguments = Arguments::parse();
    let config = configuration::Configuration::new(&args)?;

    // Setup the list of executable objects
    let executables: ExeVec = args
        .exe_filename
        .iter()
        .map(|fname| exe_types::new(fname))
        .filter(|exe| config.show_notexe || !exe.is_empty())
        .collect();

    if executables.is_empty() || (executables.len() == 1 && executables[0].is_empty()) {
        panic!("No executable files of interest found");
    }

    // Setup principal objects, colors must follow screen
    // Force screen lazy initialization
    Lazy::force(&TERMWIN);
    color::init(&config.theme);

    // Initialize screen
    TERMWIN.win.bkgd(Colors::global().bkgr()?);
    TERMWIN.win.refresh();

    // Display file info
    let rc = if executables.len() == 1 {
        file_header::show(executables[0].as_ref())
    } else {
        file_list::show(&executables)
    };

    TERMWIN.term();

    rc
}
