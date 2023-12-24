extern crate ncexe;

use std::collections::HashMap;

use anyhow::{bail, Result};

use ncexe::windows::screen::Screen;
use ncexe::color;
use pancurses::chtype;


fn main() -> Result<()> {

    let screen = Screen::new();

    let w = &screen.win;

    let _c = color::Colors::new();

    let colors = color::color_map().unwrap();

    show_colors("Normal", w, &colors, pancurses::A_NORMAL, 2)?;
    show_colors("Bold", w, &colors, pancurses::A_BOLD, 14)?;

    w.refresh();
    w.getch();

    Ok(())

}

fn show_colors(
    title: &str, 
    w: &pancurses::Window,
    colors: &Box<HashMap<&str, u8>>, 
    attr: pancurses::chtype,
    y_start: i32 )
    -> Result<()> {

    pancurses::init_pair(255, 0, 7);

    w.mv(y_start, 2);
    w.attrset(pancurses::COLOR_PAIR(255) + pancurses::A_NORMAL);
    w.addstr(title);

    for (y, fgr) in [
        "yellow", 
        "orange",
        "red",
        "magenta",
        "violet",
        "blue",
        "cyan",
        "green",
        ].iter().enumerate() {

        w.mv(y as i32 + y_start + 2, 2);
        
        for (x, bgr) in [
            "base03", 
            "base02",
            "base01",
            "base00",
            "base0",
            "base1",
            "base2",
            "base3",

            ].iter().enumerate() {

            let fc = colors[fgr] as i16;
            let bc = colors[bgr] as i16;

            let pno = (x + (y * 8 ) + 1) as i16;

            pancurses::init_pair(pno, fc, bc );
            if w.attrset(pancurses::COLOR_PAIR(pno as chtype) + attr) != 0
                { bail!("attrset") };

            if w.addstr(format!("{}/{:03.3},{}/{:03.3} ", 
                        &fgr[0..2], fc , &bgr[4..], bc)) != 0 
                { bail!("addstr") };
                
        }

    }

    Ok(())

}