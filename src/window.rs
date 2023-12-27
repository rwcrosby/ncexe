//! 
//! The application window type and associated elements
//! 

use anyhow::Result;

// ------------------------------------------------------------------------
/// Y/X coordinates
#[derive(Debug)]
pub struct Coords {
    pub line: i32,
    pub col: i32,
}

// ------------------------------------------------------------------------
/// Window margins
#[derive(Debug)]
pub struct Margins {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

// ------------------------------------------------------------------------
/// Application window
#[derive(Debug)]
pub struct ExeWindow<'a> {

    /// Reference to the underlying screen (main window)
    pub screen: &'a Screen,

    /// Parent window (if one exists)
    _parent: Option<&'a ExeWindow<'a>>,

    /// This window's curses window
    pub win : pancurses::Window,

    /// Window margins
    pub margins: &'a Margins,

    /// Window margins
    pub colors: &'a WindowColors,

    /// Reference to the main window
    // pub main_window : &'a MainWindow,

    /// User requested dimensions of the writable area
    pub desired: Coords, 

    /// Available dimensions of the writable area
    pub avail: Coords, 

}

// ------------------------------------------------------------------------
impl<'a> ExeWindow<'a> {

    pub fn new( desired: Coords,
                title : &str,
                colors : &'a WindowColors,
                parent: Option<&'a ExeWindow>,
                margins: &'a Margins,
                screen: &'a Screen ) -> Result<Box<ExeWindow<'a>>> {

        let sc = &screen.win;

        // Figure out the x dimensions to use for the window
        let screen_cols = sc.get_max_x();

        let des_win_cols = desired.col + margins.left + margins.right; 
        let mut avail_cols = screen_cols - margins.left - margins.right;

        let mut beg_col  = 0;
        let mut cols = screen_cols;

        if des_win_cols < screen_cols {
            beg_col = (screen_cols - des_win_cols) / 2;
            cols = des_win_cols; 
            avail_cols = desired.col;
        };

        // Figure out the y dimensions to use for the window
        
        let screen_lines = sc.get_max_y();

        let des_win_lines = desired.line + margins.top + margins.bottom; 
        let mut avail_lines = screen_lines - margins.top - margins.bottom;
        
        let mut beg_line = 0;
        let mut lines = screen_lines;
        
        if des_win_lines < screen_lines {
            beg_line = (screen_lines - des_win_lines) / 2;
            lines = des_win_lines; 
            avail_lines = desired.line;
        };
        
        // Create the window
        
        let win = Box::new(ExeWindow{   
            screen, 
            margins,
            _parent: parent,
            colors,
            desired,
            avail: Coords{ line: avail_lines, col: avail_cols}, 
            win : pancurses::newwin(
                lines, 
                cols, 
                beg_line, 
                beg_col
            )
        } );

        let w = &win.win;

        // Configure the new window

        w.keypad(true);
        
        // Show the title
        
        w.bkgd(colors.title);
        w.attrset(colors.title);
        w.draw_box(0, 0);
        
        let display_title = format!(" {} ", title);
        
        if display_title.len() as i32 <=  cols - margins.left - margins.right {
            let y = (cols - display_title.len() as i32) / 2;
            w.attrset(colors.title);
            w.mvprintw(0, y as i32, display_title);
        }

        Ok(win)

    }

}

// ------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    use crate::windows::screen::Screen;

    #[test]
    pub fn old_test_1() {

        let w = Screen::new();
        
        // w.win.printw("Curses test 1 <\u{21d1}>");
        // w.win.getch();

        pancurses::init_pair(128, 10, 240);
        pancurses::init_pair(129, 20, 230);
        pancurses::init_pair(130, 30, 220);
        pancurses::init_pair(131, 40, 210);

        let cs = WindowColors{bkgr: 0, title: 129, text: 130, value: 131};
        let margins = Margins{top: 1, bottom: 2, left: 3, right: 4 };

        let sw = ExeWindow::new(Coords{line: 10, col: 10}, 
                                "Blah", 
                                &cs, 
                                None, 
                                &margins,
                                &w ).unwrap();

        sw.win.mvaddstr(2, 1, "x");
        let x = sw.win.mvaddstr(20, 1, "Overflow");
        sw.win.mvaddstr(3, 1, format!("{}", x));

        let ch = sw.win.getch();
        
        sw.win.printw(format!("{:?}", ch));
        sw.win.refresh();
        
        sw.win.getch();

    }
}
