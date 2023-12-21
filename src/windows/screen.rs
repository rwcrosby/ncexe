//!
//! The terminal screen
//! 

pub struct Screen {
    pub win: pancurses::Window,
}

impl Screen {

    pub fn new() -> Box<Screen> {

        let win = Box::new(Screen {
            win: pancurses::initscr(),
        });

        pancurses::noecho();
        win.win.keypad(true);
        win.win.refresh();

        win

    }
}

impl Drop for Screen {
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
        let w = Screen::new();
        w.win.printw("Curses test 1");
        w.win.getch();
    }
}

