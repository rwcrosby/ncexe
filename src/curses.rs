use pancurses;
use pancurses::{Input};

use crate::ExeFormat;

// File list window
pub struct ExeWin {
    win: pancurses::Window,
}

impl ExeWin {

    pub fn new() -> ExeWin {
        let win = ExeWin{win: pancurses::initscr()};

        pancurses::noecho();
        win.win.keypad(true);

        win
    }

    pub fn show(&self, exe_vec:  &Vec<Box<dyn ExeFormat>>) {
        self.win.draw_box(0, 0);
        let title = " File List ";
        let y = (self.win.get_max_x() - title.len() as i32) / 2;
        self.win.mv(0, y);
        self.win.printw(title);
        
        let start_x = 2;
        let start_y = 2;

        let mut x = start_x;
        let y = start_y;

        for exe in exe_vec {
            self.win.mv(x, y);
            self.win.printw(exe.filename());
            x += 1;
        }

        // try to reverse the first line

        let highlight = |idx: u32| {
            self.win.mv(start_x + idx as i32, start_y);
            self.win.chgat(exe_vec[idx as usize].filename().len() as i32, pancurses::A_REVERSE, 0);
        };

        let unhighlight = |idx: u32| {
            self.win.mv(start_x + idx as i32, start_y);
            self.win.chgat(exe_vec[idx as usize].filename().len() as i32, pancurses::A_NORMAL, 0);
        };

        let mut file_idx : u32 = 0;

        highlight(0);

        loop {
            let ch = self.win.getch();
            if ch.is_none() { continue; }
            match ch.unwrap() {
                Input::Character(c) => 
                    if c == 'q' || c == '\u{1b}' { 
                        break; 
                    } else { 
                        println!("{:?}", c)
                    }
                Input::KeyDown => 
                    if file_idx < (exe_vec.len() - 1) as u32 {
                        unhighlight(file_idx);
                        file_idx += 1;
                        highlight(file_idx)
                    }
                Input::KeyUp => 
                    if file_idx > 0 {
                        unhighlight(file_idx);
                        file_idx -= 1;
                        highlight(file_idx)
                    }
                v => { self.win.printw(format!("haha: {:?}", v)); },
            };

        }

    }

}

impl Drop for ExeWin {
    fn drop (&mut self) {
        pancurses::endwin();
    }
}

#[test]
pub fn testwin() {

    let w = ExeWin::new();
    w.win.printw("Hello World");
    w.win.getch();
    
}
