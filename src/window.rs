#![allow(dead_code)]
//! Functions to manage curses windows

use crate::main_window::MainWindow;

#[derive(Debug)]
pub struct ExeWindow<'a> {

    /// Reference to the main window
    main_window : &'a MainWindow,

    /// Reference to the this window
    pub win : pancurses::Window,

    /// User requested dimensions of the writable area
    pub desired_canvas_cols : i32, 
    pub desired_canvas_lines : i32,

    /// Available dimensions of the writable area
    pub avail_canvas_cols : i32, 
    pub avail_canvas_lines : i32,

}

pub const LMARGIN : i32 = 2;
pub const RMARGIN : i32 = 2;
pub const TMARGIN : i32 = 2;
pub const BMARGIN : i32 = 1;

impl<'a> ExeWindow<'a> {

    pub fn new( desired_canvas_cols : i32,
                desired_canvas_lines : i32,
                title : &str,
                main_window: &'a MainWindow ) -> Result<Box<ExeWindow<'a>>, String> {

        let mw = &main_window.win;

        // Figure out the x dimensions to use for the window
        let mw_cols = mw.get_max_x();

        let des_win_cols = desired_canvas_cols + LMARGIN + RMARGIN; 
        let mut avail_canvas_cols = mw_cols - LMARGIN - RMARGIN;

        let mut beg_col : i32 = 0;
        let mut cols = mw_cols;

        if des_win_cols < mw_cols {
            beg_col = (mw_cols - des_win_cols) / 2;
            cols = des_win_cols; 
            avail_canvas_cols = desired_canvas_cols;
        };

        // Figure out the y dimensions to use for the window
        
        let mw_lines = mw.get_max_y();

        let des_win_lines = desired_canvas_lines + TMARGIN + BMARGIN; 
        let mut avail_canvas_lines = mw_lines - TMARGIN - BMARGIN;
        
        let mut beg_line : i32 = 0;
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
                                        win : mw.subwin(lines, cols, beg_line, beg_col)
                                            .map_err(|e| format!("Create subwin error {}", e))? } );

        let w = &win.win;

        // Configure the new window

        w.draw_box(0, 0);
        w.keypad(true);


        // Show the title

        let display_title = format!(" {} ", title);

        if display_title.len() as i32 <=  cols - LMARGIN - RMARGIN {
            let y = (cols - display_title.len() as i32) / 2;
            w.mvprintw(0, y, display_title);
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
        w.win.printw("Curses test 1");
        w.win.getch();
        
        let sw = ExeWindow::new(10, 10, "Blahx", &w).unwrap();

        sw.win.mvaddstr(2, 1, "x");
        let x = sw.win.mvaddstr(20, 1, "x");
        sw.win.mvaddstr(0, 0, format!("{}", x));

        sw.win.getch();
        
    }
}
