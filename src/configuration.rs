use serde::Deserialize;
use std::{env, path::PathBuf};

use crate::Arguments;

// ------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub struct Configuration {
    theme: String,
    show_notexe: bool,
}

// ------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
struct YamlConfig {
    show_notexe: bool,
    theme: String,
}

// ------------------------------------------------------------------------

impl<'a> Configuration {
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
        
        match std::fs::symlink_metadata(&cfile) {
            Ok(_) => load_config_file(&cfile),
            Err(_) => {
                println!("Config file <{}> not found", cfile.display().to_string());
                Ok(Box::new(Configuration{theme: "No config file".to_string(),
                show_notexe : args.show_notexe}))
            }
        }
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