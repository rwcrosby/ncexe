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

    let mut msgs = vec![];
    for cause in error.chain() {
        msgs.push(cause.to_string());
    }

    for line in msgs {
        print!("{}", line);
    }

}

// ------------------------------------------------------------------------

pub fn window(
    title: &str,
    msg: &str,
    attr: (i16, i16),
) {

    let cp = 2;
    init_pair(cp, attr.0, attr.1);
    
    let width: i32 = 4 + msg.len() as i32;
    let ypos: i32 = SCREEN.win.get_max_y() / 2 - 5;
    let xpos: i32 = SCREEN.win.get_max_x() / 2 - width;

    let pw = newwin(5, width, ypos, xpos);

    let a = COLOR_PAIR(cp as u32);

    pw.attrset(a);
    pw.bkgd(a);

    // This will draw a single line border
    pw.border(0,0,0,0,0,0,0,0);

    let title_y: i32 = 0;
    let title_x: i32 = (width - title.len() as i32) / 2;
    pw.mvaddstr(title_y, title_x, title);
    pw.mvchgat(title_y, title_x, title.len() as i32, 0, cp);

    let msg_y: i32 = 2;
    let msg_x: i32 = 2;
    pw.mvaddstr(msg_y, msg_x, msg);

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
        "The Message", 
        (pancurses::COLOR_WHITE, pancurses::COLOR_RED)
    );

    SCREEN.win.touch();
    pancurses::doupdate();

    SCREEN.term();

    assert!(true);

}