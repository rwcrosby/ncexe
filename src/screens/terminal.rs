//!
//! The terminal screen
//! 

use once_cell::sync::Lazy;

// ------------------------------------------------------------------------

pub struct TermWin {
    pub win: pancurses::Window,
}

impl TermWin {

    pub fn new() -> Self {

        let win = TermWin {
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

impl Default for TermWin {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for TermWin {} 
unsafe impl Send for TermWin {} 

impl Drop for TermWin {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

// ------------------------------------------------------------------------
// Global screen object

pub static TERMWIN: Lazy<TermWin> = Lazy::new(|| {
    TermWin::new()
});

// ------------------------------------------------------------------------

#[test]
pub fn screen_test_1() {

    use once_cell::sync::Lazy;
    use super::terminal::TERMWIN;

    Lazy::force(&TERMWIN);

    TERMWIN.win.printw("Curses test 1");
    TERMWIN.win.getch();
    TERMWIN.term();
}

