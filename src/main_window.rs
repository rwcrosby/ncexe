use pancurses;

use gettextrs;

// use crate::file_list_window::FileListWindow;
// use crate::Formatter;

// ------------------------------------------------------------------------
// File list window

#[derive(Debug)]
pub struct MainWindow {
    pub win: pancurses::Window,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let win = MainWindow {
            win: pancurses::initscr(),
        };

        gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");

        pancurses::noecho();
        win.win.keypad(true);

        win
    }

}

impl Drop for MainWindow {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    pub fn test_1() {
        let w = MainWindow::new();
        w.win.printw("Curses test 1");
        w.win.getch();
    }
}

