//! 
//! The underlying curses screen
//! 

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Screen {
    pub win: pancurses::Window,
}

impl Screen {
    pub fn _new() -> Screen {
        let win = Screen {
            win: pancurses::initscr(),
        };

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
        let w = Screen::_new();
        w.win.printw("Curses test 1");
        w.win.getch();
    }
}

