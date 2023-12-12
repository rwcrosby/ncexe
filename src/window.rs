#![allow(dead_code)]
//! Functions to manage curses windows

use std::error;

use crate::main_window::MainWindow;

#[derive(Debug)]
pub struct ExeWindow<'a> {

    /// Reference to the main window
    main_window : &'a MainWindow,

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

    pub fn new( desired_canvas_cols : usize,
                desired_canvas_lines : usize,
                title : &str,
                main_window: &'a MainWindow ) -> Result<Box<ExeWindow<'a>>, Box<dyn error::Error>> {

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

        w.draw_box(0, 0);
        w.keypad(true);

        // Show the title

        let display_title = format!(" {} ", title);

        if display_title.len() <=  cols - LMARGIN - RMARGIN {
            let y = (cols - display_title.len()) / 2;
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
        w.win.keypad(true);
        
        let sw = ExeWindow::new(10, 10, "Blah", &w).unwrap();

        sw.win.mvaddstr(2, 1, "x");
        let x = sw.win.mvaddstr(20, 1, "x");
        sw.win.mvaddstr(0, 0, format!("{}", x));

        sw.win.getch();
        
    }
}
