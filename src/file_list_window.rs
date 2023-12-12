use pancurses::{Input, A_NORMAL, A_REVERSE};
use std::error;

use crate::main_window::MainWindow;
use crate::window;
use crate::{Formatter, ETYPELENGTH};

type ExeItem<'a> = Box<dyn Formatter + 'a>;
type ExeList<'a> = Vec<ExeItem<'a>>;

// ------------------------------------------------------------------------

pub fn show(executables: &ExeList, mw: &MainWindow) -> Result<(), Box<dyn error::Error>> {

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

    let w = window::ExeWindow::new(
        line_len, 
        executables.len(), 
        "Files Selected", 
        mw
    )?;

    let pw = &w.win;
    let mpw = &mw.win;

    // TODO Shorten filenames
    if w.avail_canvas_cols < w.desired_canvas_cols {
        return Err(format!("Window too narrow, need {} columns, only have {}", w.desired_canvas_cols, w.avail_canvas_cols).into());
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

    // Line closured

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

    // Create key handler closures

    let key_up_handler = key_up_generator(&w, &executables, highlight, fmt_line );
    let key_down_handler = key_down_generator(&w, &executables, highlight, fmt_line );
    let key_pgup_handler = key_pgup_generator(&w, &executables, highlight, fmt_line );
    let key_pgdown_handler = key_pgdown_generator(&w, &executables, highlight, fmt_line );
    let key_home_handler = key_home_generator(&w, &executables, highlight, fmt_line );
    let key_end_handler = key_end_generator(&w, &executables, highlight, fmt_line );

    // Do it!

    let mut win_idx: usize = 0;
    let mut top_idx: usize = 0;

    write_lines(&w, &executables[0..w.avail_canvas_lines as usize], &fmt_line);
    highlight(0, true);

    loop {
        #[cfg(debug_assertions)]
        pw.mvprintw(pw.get_max_y() - 1, 0,
            format!("e_l {:3}: c_l {:2}: t_i {:2}: w_i {:2}, {:?}",
                executables.len(),
                w.avail_canvas_lines,
                top_idx,
                win_idx,
                pw.get_max_yx()));

        indicate_more_up(&w, top_idx);
        indicate_more_down(&w, top_idx, executables.len());

        match pw.getch() {

            Some(Input::KeyUp) => key_up_handler(&mut win_idx, &mut top_idx),
            Some(Input::KeyDown) => key_down_handler(&mut win_idx, &mut top_idx),
            Some(Input::KeyPPage) => key_pgup_handler(&mut win_idx, &mut top_idx),
            Some(Input::KeyNPage) => key_pgdown_handler(&mut win_idx, &mut top_idx),
            Some(Input::KeyHome) => key_home_handler(&mut win_idx, &mut top_idx),
            Some(Input::KeyEnd) => key_end_handler(&mut win_idx, &mut top_idx),

            Some(Input::KeyResize) => {

                // Will the existing window fit on the screen, if so jut move it

                let (old_flw_y, old_flw_x) = pw.get_beg_yx();
                let (mut new_flw_y, mut new_flw_x) = (old_flw_y, old_flw_x);
                let (old_flw_l, old_flw_c) = pw.get_max_yx();

                let (new_mw_l, new_mw_c) = mpw.get_max_yx();

                if  new_mw_l >= old_flw_l {
                    new_flw_y = (new_mw_l - old_flw_l) / 2;
                }

                if  new_mw_c >= old_flw_c {
                    new_flw_x = (new_mw_c - old_flw_c) / 2;
                }

                pw.mvwin(new_flw_y as i32, new_flw_x as i32);

                #[cfg(debug_assertions)]
                pw.mvprintw(0, 0,
                    format!("new mw {}/{}, old flw {}/{} {}/{}, new flw {}/{}",
                            new_mw_l, new_mw_c,
                            old_flw_l, old_flw_c,
                            old_flw_y, old_flw_x,
                            new_flw_y, new_flw_x)
                );

                // Clean up the screen
                mpw.touch();
                mpw.refresh();

            }

            Some(Input::Character(c)) => match c {
                'q' | '\u{1b}' => break,
                '\n' => (),
                _ => ()
                }

            _ => (),

        }

    };

    Ok(())

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

fn key_up_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if *win_idx > 0 {
            highlight_fn(*win_idx, false);
            *win_idx -= 1;
        } else if *top_idx > 0 {
            *top_idx -= 1;
            write_lines(
                &w, 
                &exes[*top_idx..*top_idx + w.avail_canvas_lines], 
                &fmt_fn
            );
        }

        highlight_fn(*win_idx, true);

    }
}

// ------------------------------------------------------------------------

fn key_down_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if *win_idx < w.avail_canvas_lines - 1 {
            highlight_fn(*win_idx, false);
            *win_idx += 1;
        } else if *top_idx + (w.avail_canvas_lines) < exes.len()  {
            *top_idx += 1;
            write_lines(
                &w, 
                &exes[*top_idx..*top_idx + w.avail_canvas_lines], 
                &fmt_fn
            );
        }

        highlight_fn(*win_idx, true)

    }
}

// ------------------------------------------------------------------------

fn key_pgup_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if *top_idx > 0 {

            if *top_idx  > w.avail_canvas_lines {
                *top_idx -= w.avail_canvas_lines;
            } else {
                *top_idx = 0;
                
            }

            write_lines(&w, &exes[*top_idx..(*top_idx + w.avail_canvas_lines)], &fmt_fn);
            *win_idx = 0;

        } else {
            highlight_fn(*win_idx, false);
            *win_idx = 0;
        }

        highlight_fn(*win_idx, true);

    }
}

// ------------------------------------------------------------------------

fn key_pgdown_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if w.avail_canvas_lines + *top_idx < exes.len() {

            if *top_idx + w.avail_canvas_lines as usize * 2 > exes.len() {
                *top_idx = exes.len() - w.avail_canvas_lines;
            } else {
                *top_idx += w.avail_canvas_lines;
            }
            
            write_lines(&w, &exes[*top_idx..(*top_idx + w.avail_canvas_lines)], &fmt_fn);
            *win_idx = 0;

        } else {
            highlight_fn(*win_idx, false);
            *win_idx = w.avail_canvas_lines - 1;
        }

        highlight_fn(*win_idx, true);

    }
}

// ------------------------------------------------------------------------

fn key_home_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if *top_idx != 0 {
            *top_idx = 0;
            write_lines(&w, &exes[0..w.avail_canvas_lines], &fmt_fn);
        }
        if *win_idx != 0 {
            highlight_fn(*win_idx, false);
            *win_idx = 0;
            highlight_fn(*win_idx, true);
        }
    }

}

// ------------------------------------------------------------------------

fn key_end_generator<'a>
(
    w: &'a window::ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(usize, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut usize, &mut usize)  + 'a
{

    move | win_idx: &mut usize, top_idx: &mut usize | {

        if *top_idx + w.avail_canvas_lines != exes.len() {
            *top_idx = exes.len() - w.avail_canvas_lines;
            write_lines(&w, &exes[*top_idx..], &fmt_fn);
        }
        if *win_idx != w.avail_canvas_lines - 1 {
            highlight_fn(*win_idx, false);
            *win_idx = w.avail_canvas_lines - 1;
            highlight_fn(*win_idx, true);
        }

    }

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
        fn show(&self, _mw: &MainWindow) -> Result<(), Box<dyn error::Error>> { Ok(()) }
    }

    fn window_test(lines: &ExeList) {
        let w = MainWindow::new();
        show(lines, &w).unwrap();
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
