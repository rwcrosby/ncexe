use pancurses::{Input, A_NORMAL, A_REVERSE};

use crate::main_window::MainWindow;
use crate::window;
use crate::{Formatter, ETYPELENGTH};

type ExeItem<'a> = Box<dyn Formatter + 'a>;
type ExeList<'a> = Vec<ExeItem<'a>>;

// ------------------------------------------------------------------------

pub fn show(executables: &ExeList, mw: &MainWindow) {

    // Setup the line info and header

    let max_name_len = executables
        .iter()
        .max_by_key(|exe| exe.filename().len())
        .unwrap()
        .filename()
        .len();

    let hdr_line = format!("{etype:<l0$.l0$} {fname:<l1$.l1$} {fsize:>10.10}",
        l0 = ETYPELENGTH,
        etype = "File Type",
        l1 = max_name_len,
        fname = "Name",
        fsize = "Length"
    );

    let line_len = hdr_line.len();

    // This might panic on a curses error
    let w = window::ExeWindow::new(
        line_len, 
        executables.len(), 
        "Files Selected", 
        mw
    )
        .unwrap();

    let pw = &w.win;

    // TODO Shorten filenames
    if w.avail_canvas_cols < w.desired_canvas_cols {
        if max_name_len < 20 {
            panic!("Window too narrow");
        }
    }

    pw.mvaddstr(1, 2, hdr_line);

    #[cfg(debug_assertions)]
    {
        pw.mvaddstr(0, 0,
            format!("ll {:3}: acl {:3}: acc {:3}: maxl {:3}: maxc {:3}",
                line_len,
                w.avail_canvas_lines, w.avail_canvas_cols,
                pw.get_max_y(), pw.get_max_x()
            ),
        );
    }

    // Closures to reset a line's attributes

    let highlight = |win_idx, highlight| {
        pw.mvchgat(
            (window::TMARGIN + win_idx) as i32,
            window::LMARGIN as i32,
            w.avail_canvas_cols as i32,
            if highlight { A_REVERSE } else { A_NORMAL },
            0,
        );
    };

    let fmt_line = |exe: &ExeItem| -> String {
        format!(
            "{etype:l0$.l0$} {fname:l1$.l1$} {fsize:10}",
            l0 = ETYPELENGTH,
            etype = exe.exe_type().to_string(),
            l1 = max_name_len,
            fname = exe.filename(),
            fsize = exe.len()
        )
    };

    let mut win_idx: usize = 0;
    let mut top_idx: usize = 0;

    write_lines(&w, &executables[0..w.avail_canvas_lines as usize], &fmt_line);
    highlight(0, true);

    loop {
        #[cfg(debug_assertions)]
        {
            pw.mvprintw(pw.get_max_y() - 1, 0,
                format!(
                    "e_l {:3}: c_l {:2}: t_i {:2}: w_i {:2}",
                    executables.len(),
                    w.avail_canvas_lines,
                    top_idx,
                    win_idx
                ),
            );
        }

        indicate_more_up(&w, top_idx);
        indicate_more_down(&w, top_idx, executables.len());

        match pw.getch() {
            
            Some(Input::KeyUp) => {

                if win_idx > 0 {
                    highlight(win_idx, false);
                    win_idx -= 1;
                } else if top_idx > 0 {
                    top_idx -= 1;
                    write_lines(
                        &w, 
                        &executables[top_idx..top_idx + w.avail_canvas_lines], 
                        &fmt_line
                    );
                }

                highlight(win_idx, true)

            }

            Some(Input::KeyDown) => {

                if win_idx < w.avail_canvas_lines - 1 {
                    highlight(win_idx, false);
                    win_idx += 1;
                } else if top_idx + (w.avail_canvas_lines) < executables.len()  {
                    top_idx += 1;
                    write_lines(
                        &w, 
                        &executables[top_idx..top_idx + w.avail_canvas_lines], 
                        &fmt_line
                    );
                }

                highlight(win_idx, true)

            }

            Some(Input::KeyPPage) => {

                if top_idx > 0 {

                    if top_idx  > w.avail_canvas_lines {
                        top_idx -= w.avail_canvas_lines;
                    } else {
                        top_idx = 0;
                        
                    }

                    write_lines(&w, &executables[top_idx..(top_idx + w.avail_canvas_lines as usize)], &fmt_line);
                    win_idx = 0;

                } else {
                    highlight(win_idx, false);
                    win_idx = 0;
                }

                highlight(win_idx, true);

            }

            Some(Input::KeyNPage) => {

                if w.avail_canvas_lines as usize + top_idx < executables.len() {

                    if top_idx + w.avail_canvas_lines as usize * 2 > executables.len() {
                        top_idx = executables.len() - w.avail_canvas_lines as usize;
                    } else {
                        top_idx += w.avail_canvas_lines as usize;
                    }
                    
                    write_lines(&w, &executables[top_idx..(top_idx + w.avail_canvas_lines as usize)], &fmt_line);
                    win_idx = 0;

                } else {
                    highlight(win_idx, false);
                    win_idx = w.avail_canvas_lines - 1;
                }

                highlight(win_idx, true);

            }

            Some(Input::KeyHome) => {

                if top_idx != 0 {
                    top_idx = 0;
                    write_lines(&w, &executables[0..w.avail_canvas_lines as usize], &fmt_line);
                }
                if win_idx != 0 {
                    highlight(win_idx, false);
                    win_idx = 0;
                    highlight(win_idx, true);
                }
            }

            Some(Input::KeyEnd) => {

                if top_idx + w.avail_canvas_lines as usize != executables.len() {
                    top_idx = executables.len() - w.avail_canvas_lines as usize;
                    write_lines(&w, &executables[top_idx..], &fmt_line);
                }
                if win_idx != w.avail_canvas_lines - 1 {
                    highlight(win_idx, false);
                    win_idx = w.avail_canvas_lines - 1;
                    highlight(win_idx, true);
                }
            }


            Some(Input::Character(c)) => match c {
                'q' | '\u{1b}' => break,
                '\n' => (),
                _ => ()
                }

            _ => (),

        }

    }

    // pw.getch();

}

// ------------------------------------------------------------------------

fn indicate_more_up(
    w : &window::ExeWindow,
    top_idx : usize
) {

    let pos = w.win.get_cur_yx();

    w.win.mvprintw(
        window::TMARGIN as i32, 
        (window::LMARGIN - 1) as i32, 
        if top_idx > 0 { "\u{21d1}" } else { " " }
    );

    w.win.mv(pos.0, pos.1);
    
}

// ------------------------------------------------------------------------

fn indicate_more_down(
    w : &window::ExeWindow,
    top_idx : usize,
    num_exe : usize
) {

    let pos = w.win.get_cur_yx();
    
    w.win.mvprintw(
        w.win.get_max_y() - window::BMARGIN as i32  - 1, 
        (window::LMARGIN - 1) as i32, 
        if w.avail_canvas_lines as usize + top_idx  == num_exe { " " } else {"\u{21d3}" } 
    );
        
    w.win.mv(pos.0, pos.1);
    
}

// ------------------------------------------------------------------------

fn write_lines<F>(
    win: &window::ExeWindow,
    exe_list: &[ExeItem],
    fmt_fn: F,
) where
    F: Fn(&ExeItem) -> String,
{
    for (idx, exe) in exe_list.iter().enumerate()  {
        win.win.mvprintw(
            (idx + window::TMARGIN) as i32,
            window::LMARGIN as i32,
            fmt_fn(exe),
        );
    };

}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MainWindow;
    use crate::{ExeType, NotExecutable};
    use pancurses::{endwin, initscr};

    impl Formatter for std::string::String {
        fn exe_type(&self) -> ExeType {
            ExeType::NOPE
        }
        fn show(&self, _mw: &MainWindow) {}
    }

    fn window_test(lines: &ExeList) {
        let w = MainWindow::new();
        show(lines, &w);
    }

    #[test]
    #[ignore]
    fn test_1() {
        let w = initscr();
        w.printw("Scroll_test_1");
        w.getch();

        endwin();
    }

    #[test]
    #[ignore]
    fn test_2() {
        let mut lines: Vec<Box<dyn Formatter>> = vec![];
        (0..10).for_each(|i| {
            lines.push(Box::new(NotExecutable {
                filename: "Blah",
                msg: format!("LIne {}", i),
            }))
        });

        window_test(&lines);
    }

    #[test]
    #[ignore]
    fn test_3() {
        let lines: Vec<Box<dyn Formatter>> = vec![
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
    #[ignore]
    fn test_4() {
        let lines: Vec<Box<dyn Formatter>> = vec![
            Box::new("Something".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something".to_string()),
        ];

        window_test(&lines);
    }
}
