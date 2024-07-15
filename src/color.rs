//! 
//! # Setup colors
//! 
//! ## Overall internal structure:
//! 
//! - theme (initially light/dark)
//!    - window (file_list, header, etc.)
//!       - header (these are color codes and attributes from the 256 space)
//!          - bkgr
//!          - title
//!          - text
//!          - value
//!       - scrollable_region (same as for header)
//!       - footer            (same as for header)
//! 
//! ## API
//!  
//! - A window set owner will request a WindowColorSet by the window's name 
//!     (e.g. file_list). 
//! - Each portion of the window set (header, etc.) will be passed
//!     a generated set of color_pairs and attributes for the title, text, and value
//! - The color set numbers will be generated by the Colors object which will own the 
//!     internal structure. 
//! - The theme is use will be set during instantiation of the Colors object in the main
//!     from data in the configuration

use anyhow::{
    anyhow, 
    bail, 
    Result
};
use once_cell::sync::OnceCell;
use pancurses::{
    chtype, 
    COLOR_PAIR, 
    A_BOLD, 
    A_NORMAL
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    rc::Rc
};


// ------------------------------------------------------------------------
/// Curses attribute definitions for a window's colors

#[derive(Debug, Clone, Copy)]
pub struct WindowColors {
    pub bkgr: chtype,
    pub title: chtype,
    pub text: chtype,
    pub value: chtype,
}

/// Set of color information for the set of windows on the screen

#[derive(Debug)]
pub struct WindowSetColors {
    pub header: WindowColors,
    pub scrollable_region: WindowColors,
    pub footer: WindowColors,
}

/// Map of colors for the various windows, window name is the key
pub type WindowSets = HashMap<String, WindowSetColors>;

/// Map keyed by theme name
pub type ColorThemes = HashMap<String, Rc<WindowSets>>;

// ------------------------------------------------------------------------
/// Overall container for color management
/// Map key is window name, value is the set of colors to use

#[derive(Debug)]
pub struct Colors {

    _themes: ColorThemes,
    window_sets: Rc<WindowSets>,

}

impl Colors {

    pub fn new(theme: &str) -> Result<Colors> {

        if !pancurses::has_colors() {
            bail!("Colors not supported");
        }

        pancurses::start_color();

        let yml: YamlColorThemes = serde_yaml::from_str(YAML).unwrap();
        let themes = to_color_themes(&yml, 10)?;
        
        let window_sets = match themes.get(theme) {
            Some(ws) => ws.clone(),
            None => bail!("Theme {} not found", theme)
        };

        Ok(Colors{_themes: themes, window_sets})

    }

    pub fn get_window_set_colors(
        &self, 
        name: &str
    ) -> Result<&WindowSetColors> {

        self.window_sets.get(name)
            .ok_or(anyhow!("Colorset {} not found", name))

    }

    pub fn bkgr(
        &self
    ) -> Result<chtype> {

        let ws = self.window_sets.get("file_list")
            .ok_or(anyhow!("No file list window set"))?;

        Ok(ws.header.bkgr)

    }

    // Class method to return the color objexct
    pub fn global() -> &'static Colors {
        COLORS.get().expect("Colors not initialized")
    }

}

// So the once_cell stuff compiles
unsafe impl Send for Colors {}
unsafe impl Sync for Colors {}


// ------------------------------------------------------------------------
// Yaml desription objects

#[derive(Debug, Deserialize)]
struct YamlWindowColors {
    bkgr: i16,
    title: (i16, String),
    text: (i16, String),
    value: (i16, String),
}

impl YamlWindowColors {

    fn to_window_colors(&self, pair_no: &mut u32) -> Result<WindowColors> {

        pancurses::init_pair(*pair_no as i16, 0, self.bkgr);
        let bkgr: chtype = COLOR_PAIR(*pair_no); 
        *pair_no += 1;

        let title = make_curses_attribute(pair_no, self.bkgr, &self.title)?;
        let text = make_curses_attribute(pair_no, self.bkgr, &self.text)?;
        let value = make_curses_attribute(pair_no, self.bkgr, &self.value)?;

        Ok(WindowColors{bkgr, title, text, value})

    }

}

// ------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct YamlWindowSetColors {
    header: YamlWindowColors,
    scrollable_region: YamlWindowColors,
    footer: YamlWindowColors,
}

type YamlColorThemes = HashMap<String, HashMap<String, YamlWindowSetColors>>;

// ------------------------------------------------------------------------
// Global initialization of the color object

pub static COLORS: OnceCell<Colors> = OnceCell::new();

pub fn init(theme: &str) {

    let colors = Colors::new(theme)
        .expect("Unable to initialize colors");

    COLORS.set(colors).expect("Unable to set colors static");

}

// ------------------------------------------------------------------------

fn to_color_themes(
    yml: &YamlColorThemes,  
    mut pair_no: u32
) -> Result<ColorThemes> 
{

    let mut ct: ColorThemes = HashMap::new();

    for (theme_name,  theme_value) in yml.iter() {

        let mut ws: WindowSets = HashMap::from([]);

        for (win_name, win_value) in theme_value.iter() {
 
            ws.insert(win_name.clone(), WindowSetColors{
                header: win_value.header.to_window_colors(&mut pair_no)?,
                scrollable_region: win_value.scrollable_region.to_window_colors(&mut pair_no)?,
                footer: win_value.footer.to_window_colors(&mut pair_no)?,
            });
            
        }

        ct.insert(theme_name.clone() , Rc::new(ws));

    } 

    Ok(ct)

}

// ------------------------------------------------------------------------

fn make_curses_attribute(
    pair_no: &mut u32, 
    bkgr: i16, 
    fgr: &(i16, String)
) -> Result<chtype> {

    pancurses::init_pair((*pair_no) as i16, fgr.0, bkgr);
    let ch: chtype = COLOR_PAIR(*pair_no) | match fgr.1.as_str() {
        "Bold" => A_BOLD,
        "Normal" => A_NORMAL,
        v => bail!("Invalid attribute {}", v),
    };

    *pair_no += 1;
    Ok(ch)

}

// ------------------------------------------------------------------------

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
            bkgr: 236
            title: [160, Bold]
            text: [43, Bold]
            value: [127, Normal]
        footer:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Bold]
    file_header: 
        header:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        scrollable_region:
            bkgr: 242
            title: [160, Bold]
            text: [226, Normal]
            value: [226, Normal]
        footer:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Bold]
    list: 
        header:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Normal]
        scrollable_region:
            bkgr: 251
            title: [160, Bold]
            text: [127, Bold]
            value: [127, Bold]
        footer:
            bkgr: 232
            title: [160, Bold]
            text: [127, Normal]
            value: [166, Bold]
    
";

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use pancurses::newwin;
    use std::env;

    use super::*;

    #[test]
    fn color_info() {

        env::set_var("TERM", "screen-256color");

        let _mw = pancurses::initscr();
        let c = Colors::new("dark").unwrap();
        
        let w = newwin(10, 30, 5, 5);
        let wsc = c.get_window_set_colors("file_list").unwrap(); 

        w.bkgd(wsc.header.bkgr);
        w.attrset(wsc.header.value);
        w.draw_box(0, 0);

        w.attrset(wsc.header.title);
        w.mvaddstr(0, 0, "A Title");

        w.attrset(wsc.header.text);
        w.mvaddstr(1, 2, "Some Text");

        w.attrset(wsc.header.text | pancurses::A_REVERSE);
        w.mvaddstr(2, 2, "Some Text Reversed");

        let pch = | ch | 
            println!("{:08x}x {:x}x {:x}x {:x}x", 
                ch,  
                ch & pancurses::A_CHARTEXT,
                ch & pancurses::A_ATTRIBUTES,
                ch & pancurses::A_COLOR);

        let ch1 = w.mvinch(0,0); 
        pch(ch1);
        let ch2 = w.mvinch(1,2); 
        pch(ch2);
        let ch3 = w.mvinch(2,2); 
        pch(ch3);
        
        pancurses::endwin();

        // Seems that the color pair choosen may vary??

        assert!(ch1 == 0x00201741 || ch1 == 0x00200b41);
        assert!(ch2 == 0x00001853 || ch2 == 0x00000c53);
        assert!(ch3 == 0x00041853 || ch3 == 0x00040c53);
            
    }

}
