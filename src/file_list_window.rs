use pancurses::Input;

use crate::main_window::MainWindow;
use crate::window;
use crate::{Formatter, ETYPELENGTH};

type ExeList<'a> = Vec<Box<dyn Formatter + 'a>>;

// ------------------------------------------------------------------------

pub fn show(executables: &ExeList, mw: &MainWindow) {
    // Figure out the desired line length

    let max_name_len = executables
        .iter()
        .max_by_key(|exe| exe.filename().len())
        .unwrap()
        .filename()
        .len();

    let hdr_line = format!(
        "{etype:<l0$.l0$} {fname:<l1$.l1$} {fsize:>10.10}",
        l0 = ETYPELENGTH,
        etype = "File Type",
        l1 = max_name_len,
        fname = "Name",
        fsize = "Length"
    );

    let line_len = hdr_line.len() as i32;

    // This might panic on a curses error
    let w = window::ExeWindow::new(line_len, executables.len() as i32, "Files Selected", mw)
        .unwrap();
    let pw = &w.win;

    // TODO Shorten filenames
    if w.avail_canvas_cols < w.desired_canvas_cols {
        if max_name_len < 20 {
            panic!("Window too narrow");
        }
    }

    #[cfg(debug_assertions)]
    {
        pw.mvaddstr(0, 0,
            format!("{} {} {} {} {} {} {} {}",
                executables.len(),
                line_len,
                w.desired_canvas_lines, w.desired_canvas_cols,
                w.avail_canvas_lines, w.avail_canvas_cols,
                pw.get_max_y(), pw.get_max_x()
            ),
        );
    }

    pw.mvaddstr(1, 2, hdr_line);

    // Closures to reset a lines attributes

    let highlight = |win_idx: i32| {
        pw.mvchgat(
            window::TMARGIN + win_idx,
            window::LMARGIN,
            w.avail_canvas_cols,
            pancurses::A_REVERSE,
            0,
        );
    };

    let normal = |win_idx: i32| {
        pw.mvchgat(
            window::TMARGIN + win_idx,
            window::LMARGIN,
            w.avail_canvas_cols,
            pancurses::A_NORMAL,
            0,
        );
    };

    // TODO Make invis atrtribute work in iterm
    let invis = |win_idx: i32| {
        // w.mvchgat(self.x_start + win_idx, start_y, max_line as i32, pancurses::A_INVIS, 0);
        pw.mvprintw(
            window::TMARGIN + win_idx,
            window::LMARGIN,
            " ".repeat(w.avail_canvas_cols as usize),
        );
    };

    let fmt_line = |exe: &Box<dyn Formatter + '_>| -> String {
        format!(
            "{etype:l0$.l0$} {fname:l1$.l1$} {fsize:10}",
            l0 = ETYPELENGTH,
            etype = exe.exe_type().to_string(),
            l1 = max_name_len,
            fname = exe.filename(),
            fsize = exe.len()
        )
    };

    let mut win_idx: i32 = 0;
    let mut top_idx: usize = 0;

    write_lines(&w, &executables[0..w.avail_canvas_lines as usize], &invis, &fmt_line);
    highlight(0);

    loop {
        #[cfg(debug_assertions)]
        {
            pw.mvprintw(pw.get_max_y(), 0,
                format!(
                    "exe.len: {} top_idx {} win_idx: {}",
                    executables.len(),
                    top_idx,
                    win_idx
                ),
            );
        }

        match pw.getch() {

            Some(Input::KeyDown) => {

                if win_idx < w.avail_canvas_lines - 1 {
                    normal(win_idx);
                    win_idx += 1;
                } else if top_idx + (w.avail_canvas_lines as usize) < executables.len() - 1 {
                    invis(win_idx);
                    top_idx += 1;
                    write_lines(
                        &w, 
                        &executables[top_idx..top_idx + w.avail_canvas_lines as usize], 
                        &invis, 
                        &fmt_line
                    );
                }

                highlight(win_idx)

            }

            Some(Input::KeyUp) => {

                if win_idx > 0 {
                    normal(win_idx);
                    win_idx -= 1;
                } else if top_idx > 0 {
                    invis(win_idx);
                    top_idx -= 1;
                    write_lines(
                        &w, 
                        &executables[top_idx..top_idx + w.avail_canvas_lines as usize], 
                        &invis, 
                        &fmt_line
                    );
                }

                highlight(win_idx)

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

fn write_lines<I, F>(
    win: &Box<window::ExeWindow>,
    exe_list: &[Box<dyn Formatter + '_>],
    invis: I,
    fmt_fn: F,
) where
    I: Fn(i32),
    F: Fn(&Box<dyn Formatter + '_>) -> String,
{
    for (idx, exe) in exe_list.iter().enumerate()  {
        win.win.mvprintw(
            (idx as i32 + window::TMARGIN) as i32,
            window::LMARGIN,
            fmt_fn(exe),
        );
    };

    for idx in exe_list.len()..win.avail_canvas_lines as usize {
        invis(idx as i32);
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
