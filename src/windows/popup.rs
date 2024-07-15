//!
//! Popup windows
//! 

use anyhow::Error;

use pancurses::{
    init_pair,
    newwin, 
    COLOR_PAIR,
};

use super::screen::SCREEN;

// ------------------------------------------------------------------------

pub fn error_window(
    error: &Error
) {

    let mut lines = vec![];
    for cause in error.chain() {

        match lines.len() {
            0 => lines.push(format!("Error: {}", cause.to_string())),
            1 => {
                lines.push("Cause:".into());
                lines.push(format!("    {}", cause.to_string()))
            },
            _ => lines.push(format!("    {}", cause.to_string()))
        }

    }

    window(
        "Error", 
        lines, 
        (pancurses::COLOR_WHITE, pancurses::COLOR_RED),
    );

}

// ------------------------------------------------------------------------

pub fn window(
    title: &str,
    lines: Vec<String>,
    attr: (i16, i16),
) {

    let cp = 2;
    init_pair(cp, attr.0, attr.1);
    
    let max_line_len = lines
        .iter()
        .fold(0, | ml, line | std::cmp::max(ml, line.len()) );

    let width: i32 = 4 + max_line_len as i32;
    let height: i32 = 4 + lines.len() as i32;
    let ypos: i32 = (SCREEN.win.get_max_y() - height) / 2;
    let xpos: i32 = (SCREEN.win.get_max_x() - width) / 2;

    let pw = newwin(height, width, ypos, xpos);

    let a = COLOR_PAIR(cp as u32);

    pw.attrset(a);
    pw.bkgd(a);

    // This will draw a single line border
    pw.border(0,0,0,0,0,0,0,0);

    let title_y: i32 = 0;
    let title_x: i32 = (width - title.len() as i32) / 2;
    pw.mvaddstr(title_y, title_x, title);
    pw.mvchgat(title_y, title_x, title.len() as i32, 0, cp);

    let mut msg_y: i32 = 2;
    let msg_x: i32 = 2;
    for ref line in lines {
        pw.mvaddstr(msg_y, msg_x, line);
        msg_y += 1;
    }

    pw.getch();    

}

#[test]
pub fn popup_test_1() {

    use once_cell::sync::Lazy;
    use super::screen::SCREEN;

    Lazy::force(&SCREEN);

    pancurses::start_color();

    let pair_no: u32 = 10;

    pancurses::init_pair(
        pair_no as i16, 
        pancurses::COLOR_BLACK, 
        pancurses::COLOR_RED
    );

    window(
        "The Title", 
        vec!["The".to_string(), "Message".to_string()],
        (pancurses::COLOR_WHITE, pancurses::COLOR_RED)
    );

    SCREEN.win.touch();
    pancurses::doupdate();

    SCREEN.term();

    assert!(true);

}