mod configuration;
mod scrollwindow;
mod curses;

use clap::Parser;
use memmap2::Mmap;
use std::fs::File;

/// Display executable file information
#[derive(Parser,Default,Debug)]
struct Arguments {

    /// Name of the executable file(s)
    #[arg(required=true)]
    exe_filename: Vec<String>
}

#[derive(Debug)]
enum ExeType {
    MachO32,
    MachO64,
//     UNIVBIN,   
//     ELF,
    NOPE,
}

// trait ValRequireTrait<T: ValTrait = Self>: ValTrait<T> {}

trait ExeFormat : std::fmt::Debug
{
    fn format(&self);
    fn to_string(&self) -> String;
    fn exe_type(&self) -> ExeType;
    fn filename(&self) -> &str;
}

#[derive(Debug)]
struct NotExecutable {
    filename: String,
    msg: String,
}

impl ExeFormat for NotExecutable{
    fn format(&self) {
    }
    fn to_string(&self) -> String {
        format!("Not an Executable: {}: {}", self.filename, self.msg)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::NOPE
    }
    fn filename(&self) -> &str {
        &self.filename
    }
}


#[derive(Debug)]
struct ExecutableMach32 {
    filename: String,
    mmap: Mmap,
}

impl ExeFormat for ExecutableMach32 {
    fn format(&self) {
    }
    fn to_string(&self) -> String {
        format!("Mach-O 32: {:30} {:?}", self.filename, self.mmap)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::MachO32
    }
    fn filename(&self) -> &str {
        &self.filename
    }
}

#[derive(Debug)]
struct ExecutableMach64 {
    filename: String,
    mmap: Mmap,
}

impl ExeFormat for ExecutableMach64 {
    fn format(&self) {
    }
    fn to_string(&self) -> String {
        format!("Mach-O 64: {:30} {:?}", self.filename, self.mmap)
    }
    fn exe_type(&self) -> ExeType {
        ExeType::MachO64
    }
    fn filename(&self) -> &str {
        &self.filename
    }
}

fn new_executable(fname: & str) -> Box<dyn ExeFormat> {

    let fd = match File::open(fname) {
        Ok(v) => v,
        Err(e) => return Box::new(NotExecutable{filename: fname.to_string(),
                                                msg: e.to_string()})
    };

    let mmap = match unsafe { Mmap::map(&fd) } {
        Ok(v) => v,
        Err(e) => return Box::new(NotExecutable{filename:  fname.to_string(),
                                                msg: e.to_string()})
    };

    if mmap.len() < 4 {
        return Box::new(NotExecutable{filename: fname.to_string(),
                                      msg: format!("Too small: {}", mmap.len())})
    };

    let raw_type = unsafe{ *(mmap.as_ptr() as *const u32) };
    match raw_type {
        0xfeedface => Box::new(ExecutableMach32{filename: fname.to_string(), mmap}),
        0xfeedfacf => Box::new(ExecutableMach64{filename: fname.to_string(), mmap}),
        // 0xcafebabe => ExeType::UNIVBIN,
        // 0xbebafeca => ExeType::UNIVBIN,
        // 0x7f454c46 => ExeType::ELF,
        v => Box::new(NotExecutable{filename: fname.to_string(),
                                    msg: format!("Invalid magic number: {:x}", v)}),
    }

}

fn main() {

    // Process the arguments
    let args : Arguments = Arguments::parse();

    // Load the configuration
    let config = configuration::Configuration::new();
    println!("{:?}", config);

    // Map all executables

    let exe_vec : Vec<_>= args.exe_filename
                                .iter()
                                .map(|fname| new_executable(fname))
                                .collect();

    exe_vec.iter().for_each(|e| println!("{}", e.to_string()));

    let exe_win = curses::ExeWin::new();

    exe_win.show(&exe_vec);
    // curses::testwin();

}
