#![allow(dead_code)]
use anyhow::{bail, Result};
use ratatui::style::{Color, Modifier, Style};
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YamlWindowColors {
    bkgr: u8,
    title: (u8, String),
    text: (u8, String),
    value: (u8, String),
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
    bkgr: Style,
    title: Style,
    text: Style,
    value: Style,
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

    let themes = to_color_themes(&yml);
    println!("{:?}", themes);

}

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

        ct.insert(theme_name.clone(), ws);

    }

    println!("Generated ColorThemes {:?}", ct);

    Ok(ct)

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
