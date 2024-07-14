//!
//! The terminal screen
//! 

pub struct Screen {
    pub win: pancurses::Window,
}

impl Screen {

    pub fn new() -> Self {

        let win = Screen {
            win: pancurses::initscr(),
        };

        pancurses::noecho();
        win.win.keypad(true);
        win.win.refresh();
        pancurses::curs_set(0);

        win

    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

// ------------------------------------------------------------------------

#[test]
pub fn screen_test_1() {
    let w = Screen::new();
    w.win.printw("Curses test 1");
    w.win.getch();
}

