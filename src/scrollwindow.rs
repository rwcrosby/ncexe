#![allow(dead_code)]
#![allow(unused)]

use pancurses::{Input, Window};

use crate::{
    curses::{self, ExeWin}, 
    ExeFormat,
};

use core::{fmt,fmt::Debug,fmt::Formatter};

#[derive(Debug)]
pub struct ScrollWindow<'a> {

    x_start: i32,
    x_size: i32,
    win : &'a Window,
    lines : Vec<&'a Box<dyn ExeFormat>>,

}

impl<'a> ScrollWindow<'a> {

    pub fn new(x_start: i32, x_size: i32, win: &'a Window) -> ScrollWindow<'a> {
        ScrollWindow { lines: vec![], x_size, x_start, win}
    }

    pub fn add_line(&mut self, line : &'a Box<dyn ExeFormat> ) {
        self.lines.push(line);
    }

    pub fn show(&self) {

        let start_y = 3;

        let w = self.win;

        let highlight = |win_idx: i32, file_idx: usize| {
            w.mv(self.x_start + win_idx, start_y);
            w.chgat(self.lines[file_idx].to_string().len() as i32, pancurses::A_REVERSE, 0);
        };

        let unhighlight = |win_idx: i32, file_idx: usize| {
            w.mv(self.x_start + win_idx, start_y);
            w.chgat(self.lines[file_idx].to_string().len() as i32, pancurses::A_NORMAL, 0);
        };

        let mut file_idx : usize = 0;
        let mut win_idx : i32 = 0;
        let mut top_idx : usize = 0;

        let max_line = self.lines.iter()
                                .max_by(|a, b| a.to_string().len().cmp(&b.to_string().len()))
                                .unwrap()
                                .to_string()
                                .len();

        println!("max_line: {}", max_line);

        /// TODO Make atrtributes work in iterm
        let invis = |win_idx: i32| {
            // w.mvchgat(self.x_start + win_idx, start_y, max_line as i32, pancurses::A_INVIS, 0);
            w.mvprintw(self.x_start + win_idx, start_y, " ".repeat(max_line));
        };

        self.write_lines(file_idx, &invis);
        highlight(0, 0);

        loop {
            match w.getch() {
               Some(Input::Character(c)) => 
                    if c == 'q' || c == '\u{1b}' { 
                        break; 
                    } else { 
                        println!("{:?}", c)
                    }

                Some(Input::KeyDown) => 

                    if file_idx < self.lines.len() - 1 {

                        if win_idx < self.x_size - 1 {

                            unhighlight(win_idx, file_idx);
                            win_idx += 1;
                            

                        } else {

                            invis(win_idx);
                            top_idx += 1;
                            self.write_lines(top_idx, &invis);

                        }

                        file_idx += 1;
                        highlight(win_idx, file_idx)

                    }

                Some(Input::KeyUp) => 

                    if file_idx > 0 {

                        if win_idx > 0 {

                            unhighlight(win_idx, file_idx);
                            win_idx -= 1;

                            
                        } else {

                            invis(win_idx);
                            top_idx -= 1;
                            self.write_lines(top_idx, &invis);
                            
                    }

                    file_idx -= 1;
                    highlight(win_idx, file_idx)

                }
                    
                Some(_) => (),
                None => ()
            };

        }


    }

    fn write_lines<F>(&self, mut idx: usize, invis: F)
        where F : Fn(i32) {

        let w  = &self.win;

        for l_idx in 0..self.x_size {
            invis(l_idx);
            if idx < self.lines.len() {
                w.mvprintw((l_idx + self.x_start) as i32, 3, self.lines[idx].to_string());
                idx += 1;
            };
        }

    }

}

#[cfg(test)]
mod tests {

    use pancurses::{Input, Window};

    use crate::{curses::{self, ExeWin}, ExeType};
    use super::*;

    #[test]
    // #[ignore]
    fn scroll_test_1() {
        
        let w = pancurses::initscr();
        w.printw("Scroll_test_1");
        w.getch();

        pancurses::endwin();

    }

    impl ExeFormat for std::string::String {
        fn to_string(&self) -> String {
            ToString::to_string(self)
        }
        fn format(&self) {}
        fn exe_type(&self) -> ExeType { ExeType::NOPE }
        fn filename(&self) -> &str {""} 
    }

    #[test]
    // #[ignore]
    fn scroll_test_2() {
    
        let w = pancurses::initscr();
        pancurses::noecho();
        w.keypad(true);

        let mut sw = ScrollWindow::new(3, 3, &w);

        let s1 : String = "Something".to_string();

        let mut lines : Vec<Box<dyn ExeFormat>> = vec![];

        for i in 0..10 {
            lines.push(Box::new(format!("Line {}", i)));
        };

        lines.iter().for_each(|l| sw.add_line(&l));

        sw.show();

        println!("{:?}", sw);

        pancurses::endwin();
        
    }

    #[test]
    // #[ignore]
    fn scroll_test_3() {
    
        let w = pancurses::initscr();
        pancurses::noecho();
        w.keypad(true);

        let mut sw = ScrollWindow::new(3, 3, &w);

        let mut lines : Vec<Box<dyn ExeFormat>> = vec![];

        lines.push(Box::new("Something".to_string()));
        lines.push(Box::new("Something 1".to_string()));
        lines.push(Box::new("Something 12".to_string()));
        lines.push(Box::new("Something 123".to_string()));
        lines.push(Box::new("Something 12".to_string()));
        lines.push(Box::new("Something 1".to_string()));
        lines.push(Box::new("Something".to_string()));

        lines.iter().for_each(|l| sw.add_line(&l));

        sw.show();

        println!("{:?}", sw);
    
        pancurses::endwin();

    }
    #[test]
    // #[ignore]
    fn scroll_test_4() {
    
        let w = pancurses::initscr();
        pancurses::noecho();
        w.keypad(true);

        let mut sw = ScrollWindow::new(3, 7, &w);

        let mut lines : Vec<Box<dyn ExeFormat>> = vec![];

        lines.push(Box::new("Something".to_string()));
        lines.push(Box::new("Something 1".to_string()));
        lines.push(Box::new("Something 1".to_string()));
        lines.push(Box::new("Something".to_string()));

        lines.iter().for_each(|l| sw.add_line(&l));

        sw.show();

        println!("{:?}", sw);
    
        pancurses::endwin();

    }

}

