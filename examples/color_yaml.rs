#![allow(dead_code)]
use anyhow::{bail, Result};
use pancurses::{chtype, COLOR_PAIR, A_BOLD, A_NORMAL};
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YamlWindowColors {
    bkgr: i16,
    title: (i16, String),
    text: (i16, String),
    value: (i16, String),
}

#[derive(Debug, Deserialize)]
struct YamlWindowSetColors {
    header: YamlWindowColors,
    scrollable_region: YamlWindowColors,
    footer: YamlWindowColors,
}

type YamlColorThemes = HashMap<String, HashMap<String, YamlWindowSetColors>>;

#[derive(Debug)]
pub struct WindowColors {
    bkgr: chtype,
    title: chtype,
    text: chtype,
    value: chtype,
}

#[derive(Debug)]
pub struct WindowSetColors {
    header: WindowColors,
    scrollable_region: WindowColors,
    footer: WindowColors,
}

pub type WindowSets = HashMap<String, WindowSetColors>;
pub type ColorThemes = HashMap<String, WindowSets>;

fn main() {


    let yml: YamlColorThemes = serde_yaml::from_str(YAML).unwrap();

    let themes = to_color_themes(&yml, 10);
    println!("{:?}", themes);

}

fn to_color_themes(yml: &YamlColorThemes,  mut pair_no: u32) -> Result<ColorThemes> {


    let mut ct: ColorThemes = HashMap::from([]);

    for (theme_name,  theme_value) in yml.iter() {

        let mut ws: WindowSets = HashMap::from([]);

        for (win_name, win_value) in theme_value.iter() {
 
            ws.insert(win_name.clone(), WindowSetColors{
                header: win_value.header.to_window_colors(&mut pair_no)?,
                scrollable_region: win_value.scrollable_region.to_window_colors(&mut pair_no)?,
                footer: win_value.footer.to_window_colors(&mut pair_no)?,
            });
            
        }

        ct.insert(theme_name.clone() , ws);

    } 

    println!("Generated ColorThemes {:?}", ct);

    Ok(ct)

}

impl YamlWindowColors {

    fn to_window_colors(&self, pair_no: &mut u32) -> Result<WindowColors> {

        pancurses::init_pair(*pair_no as i16, 0, self.bkgr);
        let bkgr: chtype = COLOR_PAIR(*pair_no); 

        let title = make_curses_attribute(pair_no, self.bkgr, &self.title)?;
        let text = make_curses_attribute(pair_no, self.bkgr, &self.text)?;
        let value = make_curses_attribute(pair_no, self.bkgr, &self.value)?;

        Ok(WindowColors{bkgr, title, text, value})

    }

}

fn make_curses_attribute(pair_no: &mut u32, bkgr: i16, fgr: &(i16, String)) -> Result<chtype> {

    pancurses::init_pair((*pair_no) as i16, fgr.0, bkgr);
    let ch: chtype = COLOR_PAIR(*pair_no) | match fgr.1.as_str() {
        "Bold" => A_BOLD,
        "Normal" => A_NORMAL,
        v => bail!("Invalid attribute {}", v),
    };

    *pair_no += 1;
    Ok(ch)

}

const YAML: &str = "
---

dark:
    file_list: 
        header:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        scrollable_region:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        footer:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
    file_header: 
        header:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        scrollable_region:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        footer:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
    
";