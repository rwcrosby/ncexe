//!
//! The terminal screen
//! 

use once_cell::sync::Lazy;

// ------------------------------------------------------------------------

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

    pub fn term(&self) {
        pancurses::endwin();
    }

}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for Screen {} 
unsafe impl Send for Screen {} 

impl Drop for Screen {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

// ------------------------------------------------------------------------
// Global screen object

pub static SCREEN: Lazy<Screen> = Lazy::new(|| {
    Screen::new()
});

// ------------------------------------------------------------------------

#[test]
pub fn screen_test_1() {

    use once_cell::sync::Lazy;
    use super::screen::SCREEN;

    Lazy::force(&SCREEN);

    SCREEN.win.printw("Curses test 1");
    SCREEN.win.getch();
    SCREEN.term();
}

