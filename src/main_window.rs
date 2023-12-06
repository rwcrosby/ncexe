use pancurses;

use crate::file_list_window::FileListWindow;
use crate::Formatter;

// ------------------------------------------------------------------------
// File list window

#[derive(Debug)]
pub struct MainWindow {
    win: pancurses::Window,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let win = MainWindow {
            win: pancurses::initscr(),
        };

        pancurses::noecho();
        win.win.keypad(true);

        win
    }

    pub fn show(&self, exe_vec: &Vec<Box<dyn Formatter + '_>>) {
        let w = &self.win;

        w.draw_box(0, 0);
        let title = " File List ";
        let y = (w.get_max_x() - title.len() as i32) / 2;
        w.mvprintw(0, y, title);

        let mut flw = FileListWindow::new(2, w.get_max_x() - 4, 2, w.get_max_y() - 4, &self.win);
        exe_vec.iter().for_each(|exe| flw.add_line(exe));

        flw.show();
    }
}

impl Drop for MainWindow {
    fn drop(&mut self) {
        pancurses::endwin();
        println!("Ending ncurses");
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
