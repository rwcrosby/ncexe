#![allow(dead_code)]
//! Functions to manage curses windows

use anyhow::Result;

use crate::{main_window::MainWindow, color::ColorSet};

#[derive(Debug)]
pub struct ExeWindow<'a> {

    /// Reference to the main window
    pub main_window : &'a MainWindow,

    /// This window's curses window
    pub win : pancurses::Window,

    /// User requested dimensions of the writable area
    pub desired_canvas_cols : usize, 
    pub desired_canvas_lines : usize,

    /// Available dimensions of the writable area
    pub avail_canvas_cols : usize, 
    pub avail_canvas_lines : usize,

}

pub const LMARGIN : usize = 2;
pub const RMARGIN : usize = 2;
pub const TMARGIN : usize = 2;
pub const BMARGIN : usize = 1;

impl<'a> ExeWindow<'a> {

    pub fn new( desired_canvas_lines : usize,
                desired_canvas_cols : usize,
                title : &str,
                colors : &ColorSet,
                main_window: &'a MainWindow ) -> Result<Box<ExeWindow<'a>>> {

        let mw = &main_window.win;

        // Figure out the x dimensions to use for the window
        let mw_cols = mw.get_max_x() as usize;

        let des_win_cols = desired_canvas_cols + LMARGIN + RMARGIN; 
        let mut avail_canvas_cols = mw_cols - LMARGIN - RMARGIN;

        let mut beg_col  = 0;
        let mut cols = mw_cols;

        if des_win_cols < mw_cols {
            beg_col = (mw_cols - des_win_cols) / 2;
            cols = des_win_cols; 
            avail_canvas_cols = desired_canvas_cols;
        };

        // Figure out the y dimensions to use for the window
        
        let mw_lines = mw.get_max_y() as usize;

        let des_win_lines = desired_canvas_lines + TMARGIN + BMARGIN; 
        let mut avail_canvas_lines = mw_lines - TMARGIN - BMARGIN;
        
        let mut beg_line = 0;
        let mut lines = mw_lines;
        
        if des_win_lines < mw_lines {
            beg_line = (mw_lines - des_win_lines) / 2;
            lines = des_win_lines; 
            avail_canvas_lines = desired_canvas_lines;
        };
        
        // Create the window
        
        let win = Box::new(ExeWindow{   main_window, 
                                        desired_canvas_cols, desired_canvas_lines,
                                        avail_canvas_cols, avail_canvas_lines,
                                        win : pancurses::newwin(
                                            lines as i32, 
                                            cols as i32, 
                                            beg_line as i32, 
                                            beg_col as i32
                                        )} );

        let w = &win.win;

        // Configure the new window

        w.keypad(true);
        
        // Show the title
        
        w.bkgd(pancurses::COLOR_PAIR(colors.frame as u32));
        w.attrset(pancurses::COLOR_PAIR(colors.frame as u32));
        w.draw_box(0, 0);
        
        let display_title = format!(" {} ", title);
        
        if display_title.len() <=  cols - LMARGIN - RMARGIN {
            let y = (cols - display_title.len()) / 2;
            w.attrset(pancurses::COLOR_PAIR(colors.title as u32));
            w.mvprintw(0, y as i32, display_title);
        }

        Ok(win)

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::main_window::MainWindow;

    #[test]
    pub fn window_test_1() {

        let w = MainWindow::new();
        
        w.win.printw("Curses test 1 <\u{21d1}>");
        w.win.getch();

        pancurses::init_pair(128, 10, 240);
        pancurses::init_pair(129, 20, 230);
        pancurses::init_pair(130, 30, 220);
        pancurses::init_pair(131, 40, 210);

        let cs = ColorSet{frame: 128, title: 129, text: 130, value: 131};

        let sw = ExeWindow::new(10, 10, "Blah", &cs, &w ).unwrap();

        sw.win.mvaddstr(2, 1, "x");
        let x = sw.win.mvaddstr(20, 1, "Overflow");
        sw.win.mvaddstr(3, 1, format!("{}", x));

        sw.win.getch();
        
    }
}
