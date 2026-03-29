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
//!     a generated set of styles for the title, text, and value
//! - The theme in use will be set during instantiation of the Colors object in the main
//!     from data in the configuration

use anyhow::{anyhow, bail, Result};
use once_cell::sync::OnceCell;
use ratatui::style::{Color, Modifier, Style};
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc};

// ------------------------------------------------------------------------
/// Ratatui style definitions for a window's colors

#[derive(Debug, Clone, Copy)]
pub struct WindowColors {
    pub bkgr: Style,
    pub title: Style,
    pub text: Style,
    pub value: Style,
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

        let yml: YamlColorThemes = serde_yaml::from_str(YAML).unwrap();
        let themes = to_color_themes(&yml)?;

        let window_sets = match themes.get(theme) {
            Some(ws) => ws.clone(),
            None => bail!("Theme {} not found", theme)
        };

        Ok(Colors { _themes: themes, window_sets })

    }

    pub fn get_window_set_colors(&self, name: &str) -> Result<&WindowSetColors> {
        self.window_sets.get(name)
            .ok_or(anyhow!("Colorset {} not found", name))
    }

    pub fn bkgr(&self) -> Result<Style> {
        let ws = self.window_sets.get("file_list")
            .ok_or(anyhow!("No file list window set"))?;
        Ok(ws.header.bkgr)
    }

    pub fn global() -> &'static Colors {
        COLORS.get().expect("Colors not initialized")
    }

}

// So the once_cell stuff compiles
unsafe impl Send for Colors {}
unsafe impl Sync for Colors {}

// ------------------------------------------------------------------------
// Yaml description objects

#[derive(Debug, Deserialize)]
struct YamlWindowColors {
    bkgr: u8,
    title: (u8, String),
    text: (u8, String),
    value: (u8, String),
}

impl YamlWindowColors {

    fn to_window_colors(&self) -> Result<WindowColors> {
        let bkgr = Style::default().bg(Color::Indexed(self.bkgr));
        let title = make_style(self.bkgr, &self.title)?;
        let text = make_style(self.bkgr, &self.text)?;
        let value = make_style(self.bkgr, &self.value)?;
        Ok(WindowColors { bkgr, title, text, value })
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

fn to_color_themes(yml: &YamlColorThemes) -> Result<ColorThemes> {

    let mut ct: ColorThemes = HashMap::new();

    for (theme_name, theme_value) in yml.iter() {

        let mut ws: WindowSets = HashMap::new();

        for (win_name, win_value) in theme_value.iter() {
            ws.insert(win_name.clone(), WindowSetColors {
                header: win_value.header.to_window_colors()?,
                scrollable_region: win_value.scrollable_region.to_window_colors()?,
                footer: win_value.footer.to_window_colors()?,
            });
        }

        ct.insert(theme_name.clone(), Rc::new(ws));

    }

    Ok(ct)

}

// ------------------------------------------------------------------------

fn make_style(bkgr: u8, fgr: &(u8, String)) -> Result<Style> {
    let modifier = match fgr.1.as_str() {
        "Bold" => Modifier::BOLD,
        "Normal" => Modifier::empty(),
        v => bail!("Invalid attribute {}", v),
    };
    Ok(Style::default()
        .fg(Color::Indexed(fgr.0))
        .bg(Color::Indexed(bkgr))
        .add_modifier(modifier))
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
    use super::*;

    #[test]
    fn color_info() {
        let c = Colors::new("dark").unwrap();
        let wsc = c.get_window_set_colors("file_list").unwrap();

        // Verify styles are created correctly
        assert_eq!(wsc.header.bkgr, Style::default().bg(Color::Indexed(232)));
        assert_eq!(
            wsc.header.title,
            Style::default()
                .fg(Color::Indexed(160))
                .bg(Color::Indexed(232))
                .add_modifier(Modifier::BOLD)
        );
        assert_eq!(
            wsc.header.text,
            Style::default()
                .fg(Color::Indexed(127))
                .bg(Color::Indexed(232))
        );
    }

}
