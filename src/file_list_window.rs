use pancurses::{Input, Window};

use crate::ExeFormat;

use core::fmt::Debug;

#[allow(unused)]
#[derive(Debug)]
pub struct FileListWindow<'a> {

    x_start: i32,
    x_len: i32,
    y_start: i32,
    y_len: i32,
    lines : Vec<&'a Box<dyn ExeFormat>>,
    win : &'a Window,

}

impl<'a> FileListWindow<'a> {

    pub fn new( x_start: i32, 
                x_len: i32, 
                y_start: i32, 
                y_len: i32, 
                win: &'a Window) -> FileListWindow<'a> {
        FileListWindow { x_start, x_len, y_start, y_len, lines: vec![], win}
    }

    pub fn add_line(&mut self, line : &'a Box<dyn ExeFormat> ) {
        self.lines.push(line);
    }

    pub fn show(&self) {

        let w = self.win;

        let highlight = |win_idx: i32, file_idx: usize| {
            w.mvchgat(self.x_start + win_idx, 
                      self.y_start, 
                      self.lines[file_idx].to_string().len() as i32, 
                      pancurses::A_REVERSE, 0);
        };

        let unhighlight = |win_idx: i32, file_idx: usize| {
            w.mvchgat(self.x_start + win_idx, 
                      self.y_start,
                      self.lines[file_idx].to_string().len() as i32, 
                      pancurses::A_NORMAL, 0);
        };

        let mut file_idx : usize = 0;
        let mut win_idx : i32 = 0;
        let mut top_idx : usize = 0;

        let max_line = self.lines.iter()
                                .max_by(|a, b| a.to_string().len().cmp(&b.to_string().len()))
                                .unwrap()
                                .to_string()
                                .len();

        // TODO Make atrtributes work in iterm
        let invis = |win_idx: i32| {
            // w.mvchgat(self.x_start + win_idx, start_y, max_line as i32, pancurses::A_INVIS, 0);
            w.mvprintw(self.x_start + win_idx, self.y_start, " ".repeat(max_line));
        };

        self.write_lines(file_idx, &invis);
        highlight(0, 0);

        loop {

            w.mvprintw(0, 0, format!("x_start: {} x_len: {} y_start: {} y_len: {}", self.x_start, self.x_len, self.y_start, self.y_len));
            w.mvprintw(1, 0, format!("file_inx:{} lines.len{} win_idx: {}", file_idx, self.lines.len(), win_idx));

            match w.getch() {
               Some(Input::Character(c)) => 
                    if c == 'q' || c == '\u{1b}' { 
                        break; 
                    } else { 
                        println!("{:?}", c)
                    }

                Some(Input::KeyDown) => 

                    if file_idx < self.lines.len() - 1 {

                        if win_idx < self.y_len - 1 {

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

        println!("Ending file_list_window::show");

    }

    fn write_lines<F>(&self, mut idx: usize, invis: F)
        where F : Fn(i32) {

        let w  = &self.win;

        for l_idx in 0..self.y_len {
            invis(l_idx);
            if idx < self.lines.len() {
                w.mvprintw((l_idx + self.x_start) as i32, self.y_start, self.lines[idx].to_string());
                idx += 1;
            };
        }

    }

}

#[cfg(test)]
mod tests {

    use pancurses::{initscr, endwin, noecho};
    use crate::ExeType;
    use super::*;

    impl ExeFormat for std::string::String {
        fn to_string(&self) -> String {
            ToString::to_string(self)
        }
        fn exe_type(&self) -> ExeType { ExeType::NOPE }
        fn filename(&self) -> &str {""} 
    }

    fn window_test(lines: &Vec<Box<dyn ExeFormat>>) {

        let w = initscr();
        noecho();
        w.keypad(true);
        
        let mut flw = FileListWindow::new(3, 3, 3, w.get_max_y(), &w);
        lines.iter().for_each(|l| flw.add_line(&l));
        flw.show();
        endwin();

        println!("{:?}", flw);

    }

    #[test]
    // #[ignore]
    fn test_1() {
        
        let w = initscr();
        w.printw("Scroll_test_1");
        w.getch();

        endwin();

    }

    #[test]
    // #[ignore]
    fn test_2() {
    
        let mut lines : Vec<Box<dyn ExeFormat>> = vec![];
        (0..10).for_each(|i| lines.push(Box::new(format!("Line {}", i))));

        window_test(&lines);

    }

    #[test]
    // #[ignore]
    fn test_3() {
    
        let lines : Vec<Box<dyn ExeFormat>> = vec![
            Box::new("Something".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something 12".to_string()),
            Box::new("Something 123".to_string()),
            Box::new("Something 12".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something".to_string()),
        ];
        
        window_test(&lines);
        
    }

    #[test]
    // #[ignore]
    fn test_4() {
    
        let lines : Vec<Box<dyn ExeFormat>> = vec![
            Box::new("Something".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something".to_string()),
        ];

        window_test(&lines);
    }

}

