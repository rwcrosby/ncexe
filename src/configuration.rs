//! Configuration object creation
//! 
//! The configuration object is used throughout and is intended to be the 
//! single source of configuration information

use serde::Deserialize;
use std::{env, path::PathBuf};

use crate::Arguments;

// ------------------------------------------------------------------------
/// Configuration items

#[derive(Debug, PartialEq)]
pub struct Configuration {
    /// Theme to apply to screen, not yet implemented
    theme: String,
    /// Show non-executable files is the file list window?
    pub show_notexe: bool,
}

// ------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
struct YamlConfig {
    show_notexe: bool,
    theme: String,
}

// ------------------------------------------------------------------------
/// - Priority for configuration file:
///     1. Standard location `~/.config/ncexe.yaml`
///     2. File pointed to by the `NCEXE_CONFIG` environment variable
///     3. File pointed to by the `--config` command line argument

impl<'a> Configuration {

    /// Create a new configuratio object from
    /// - The default configuration file at ~/.config/ncexe.yaml
    pub fn new(args: &Arguments) -> Result<Box<Configuration>, String> {
        
        // Select a configuration file
        
        let cfile : PathBuf;
        

        cfile  = match &args.config {
            Some(v) => v.to_path_buf(),
            None => {
                match env::var("NCEXE_CONFIG") {
                    Ok(v) => PathBuf::from(v),
                    Err(_) => {
                        let mut home = dirs::home_dir().unwrap();
                        home.push(".config/ncexe.yaml");
                        home
                    }
                }
            }
        };
        
        let mut config = match std::fs::symlink_metadata(&cfile) {
            Ok(_) => load_config_file(&cfile)?,
            Err(_) => {
                eprintln!("Config file <{}> not found", cfile.display().to_string());
                Box::new(Configuration{theme: "No config file".to_string(),
                                          show_notexe : false})
            }
        };


        if args.show_notexe {
                config.show_notexe = true;
        };
 
        Ok(config)

    }
}

// ------------------------------------------------------------------------

fn load_config_file(cfile: &PathBuf) -> Result<Box<Configuration>, String> {

    let fd = match std::fs::File::open(cfile) {
        Ok(fd) => fd,
        Err(e) => return Err(e.to_string()),
    };

    // TODO handle the result
    let config_from_yaml : YamlConfig = match serde_yaml::from_reader(fd) {
        Ok(cy) => cy,
        Err(e) => return Err(e.to_string()),
    };

    Ok(Box::new(Configuration { theme: config_from_yaml.theme, 
                                show_notexe: config_from_yaml.show_notexe }))
}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use std::env;
    use super::*;
    use crate::Arguments;

    // Need to guarantee that the tests run sequentially because of the environment variable usage

    #[test]
    fn tests() {
        test_1();
        test_2();
        test_3();
    }

    fn test_1() {

        env::set_var("NCEXE_CONFIG", "tests/goodconfig.yaml");
        assert!(env::var("NCEXE_CONFIG") == Ok("tests/goodconfig.yaml".to_string()));

        let arg = Arguments{exe_filename: vec!("blah".to_string()), config: None, show_notexe: false}; 
        let cfg = Configuration::new(&arg).unwrap();

        println!("{:?}", cfg);
        env::remove_var("NCEXE_CONFIG");
        assert!(*cfg == Configuration{theme: "From goodconfig.yaml".to_string(), show_notexe: true});

    }

    fn test_2() {

        let arg = Arguments{exe_filename: vec!("blah".to_string()), config: None, show_notexe: true}; 
        let cfg = Configuration::new(&arg).unwrap();

        println!("{:?}", cfg);
        assert!(*cfg == Configuration{theme: "No config file".to_string(), show_notexe: true});

    }

    fn test_3() {

        env::set_var("NCEXE_CONFIG", "tests/goodconfig.yaml");
        assert!(env::var("NCEXE_CONFIG") == Ok("tests/goodconfig.yaml".to_string()));

        let arg = Arguments{exe_filename: vec!("blah".to_string()), config: None, show_notexe: true}; 
        let cfg = Configuration::new(&arg).unwrap();

        println!("{:?}", cfg);
        env::remove_var("NCEXE_CONFIG");
        assert!(*cfg == Configuration{theme: "From goodconfig.yaml".to_string(), show_notexe: true});

    }

}