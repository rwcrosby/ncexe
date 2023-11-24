use clap::Parser;

#[derive(Parser,Default,Debug)]
struct Arguments {
    exe_filename: String
}

#[derive(Debug)]
struct Mapfile<'a> {
    filename: String,
    addr: Option<&'a u8>,
    len: u64,
}

impl<'a> Mapfile<'a> {
    fn new(filename: String) -> Result<Self, &'static str> {
        Ok(Self{filename: filename, 
            addr: None,
            len: 0})
        }
    }
    
    #[derive(Debug)]
    struct Configuration {

}

impl Configuration {
    fn new(args : & Arguments) -> Result<Self, &'static str> {
        Ok(Self{})
    }
}

fn main() {

    // Process the arguments

    let args = Arguments::parse();
    println!("{:?}", args);
    
    // Setup the configuration
    
    let config = Configuration::new(&args).unwrap();
    println!("{:?}", config);

    // Map the executable
    
    let mapped_file = Mapfile::new(args.exe_filename).unwrap();
    println!("{:?}", mapped_file);

    // Dump the executable

}
