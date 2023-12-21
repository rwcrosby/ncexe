//! 
//! Show the file list window
//!

use anyhow::{bail, Result};
use pancurses::{Input, A_NORMAL, A_REVERSE, COLOR_PAIR};

use crate::exe_types::{
    ETYPE_LENGTH,
    ExeType,
    ExeFormat,
};

use crate::{
    Formatter, 
    color::Colors,
    window::{
        Coords,
        Margins,
        ExeWindow,
    }, 
    old_windows::screen::Screen,
};

// ------------------------------------------------------------------------

type ExeItem<'a> = Box<dyn ExeFormat + 'a>;
type ExeList<'a> = Vec<ExeItem<'a>>;

// ------------------------------------------------------------------------

pub fn show(
    executables: &ExeList, 
    mw: &Screen,
    fmt: &Formatter,
    colors: &Colors) -> Result<()> {

    // Setup the line info and header

    let max_name_len = executables
        .iter()
        .max_by_key(|exe| exe.filename().len())
        .unwrap()
        .filename()
        .len();

    let hdr_line = format!("{etype:<l0$.l0$} {fname:<l1$.l1$} {fsize:>10.10}",
        l0 = ETYPE_LENGTH,
        etype = "File Type",
        l1 = max_name_len,
        fname = "Name",
        fsize = "Length"
    );

    let line_len = hdr_line.len();

    let color_set = colors.set("file_list");

    let margins = Margins{top: 2, bottom: 1, left: 2, right: 2 };

    // Create the window
                
    let w = ExeWindow::new(
        Coords{line: executables.len() as i32, col: line_len as i32}, 
        "Files Selected", 
        color_set,
        None,
        &margins,
        mw, 
    )?;

    let pw = &w.win;
    // let mpw = &mw.win;

    // TODO Shorten filenames
    if w.avail.col < w.desired.col {
        bail!("Window too narrow, need {} columns, only have {}", 
              w.desired.col, 
              w.avail.col);
    }

    pw.mvaddstr(1, 2, hdr_line);

    #[cfg(scum)]
    {
        pw.mvaddstr(0, 0, 
            format!("ll {:3}: acl {:3}: acc {:3}: maxl {:3}: maxc {:3}",
                line_len,
                w.avail.line, w.avail.col,
                pw.get_max_y(), pw.get_max_x()
            ),
        );
    }

    // Line handling closures

    let highlight = |win_idx: i32, highlight| {
        pw.mvchgat(
            margins.top + win_idx,
            margins.left,
            w.avail.col,
            if highlight { A_REVERSE } else { A_NORMAL },
            color_set.text as i16,
        );
    };

    let fmt_line = |exe: &ExeItem| -> String {
        format!(
            "{etype:l0$.l0$} {fname:l1$.l1$} {fsize:10}",
            l0 = ETYPE_LENGTH,
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
    let key_enter_handler = key_enter_generator(&w, &executables, fmt, colors );
    let key_resize_handler = key_resize_generator(&w, &executables, fmt, colors );

    // Do it!

    let mut win_idx: i32 = 0;
    let mut top_idx: usize = 0;

    pw.attrset(COLOR_PAIR(color_set.text as u32));
    write_lines(&w.win, &executables[0..w.avail.line as usize], &fmt_line, &margins);
    highlight(0, true);

    loop {
        #[cfg(scum)]
        pw.mvprintw(pw.get_max_y() - 1, 0,
            format!("e_l {:3}: c_l {:2}: t_i {:2}: w_i {:2}, {:?}",
                executables.len(),
                w.avail.line,
                top_idx,
                win_idx,
                pw.get_max_yx()));

        indicate_more_up(&w, top_idx);
        indicate_more_down(&w, top_idx, executables.len());

        match pw.getch() {

            Some(Input::KeyUp) => key_up_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyDown) => key_down_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyPPage) => key_pgup_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyNPage) => key_pgdown_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyHome) => key_home_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyEnd) => key_end_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::KeyResize) => key_resize_handler(&mut win_idx, &mut top_idx)?,
            Some(Input::Character(c)) => match c {
                'q' | '\u{1b}' => break,
                '\n' => key_enter_handler(&mut win_idx, &mut top_idx)?,
                _ => ()
                }
            _ => (),

        }

    };

    Ok(())

}

// ------------------------------------------------------------------------

fn indicate_more_up(
    w : &ExeWindow,
    top_idx : usize
) {

    let pos = w.win.get_cur_yx();

    w.win.mvprintw(
        w.margins.top as i32, 
        (w.margins.left - 1) as i32, 
        if top_idx > 0 { "\u{21d1}" } else { " " }
    );

    w.win.mv(pos.0, pos.1);
    
}

// ------------------------------------------------------------------------

fn indicate_more_down(
    w : &ExeWindow,
    top_idx : usize,
    num_exe : usize
) {

    let pos = w.win.get_cur_yx();
    
    w.win.mvprintw(
        w.win.get_max_y() - w.margins.bottom as i32  - 1, 
        (w.margins.left - 1) as i32, 
        if w.avail.line as usize + top_idx  == num_exe { " " } else {"\u{21d3}" } 
    );
        
    w.win.mv(pos.0, pos.1);
    
}

// ------------------------------------------------------------------------

fn write_lines(
    pw: &pancurses::Window,
    exe_list: &[ExeItem],
    fmt_fn: impl Fn(&ExeItem) -> String,
    margins: &Margins,
)
{
    for (idx, exe) in exe_list.iter().enumerate()  {
        pw.mvprintw(
            margins.top + idx as i32,
            margins.left,
            fmt_fn(exe),
        );
        pw.refresh();
    };
}

// ------------------------------------------------------------------------

fn key_up_generator<'a> 
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()>  + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if *win_idx > 0 {
            highlight_fn(*win_idx, false);
            *win_idx -= 1;
        } else if *top_idx > 0 {
            *top_idx -= 1;
            write_lines(
                &w.win, 
                &exes[*top_idx..*top_idx + w.avail.line as usize], 
                &fmt_fn,
                &w.margins,
            );
        }

        highlight_fn(*win_idx, true);
        Ok(())

    }
}

// ------------------------------------------------------------------------

fn key_down_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if *win_idx < w.avail.line - 1 {
            highlight_fn(*win_idx, false);
            *win_idx += 1;
        } else if *top_idx + (w.avail.line as usize) < exes.len()  {
            *top_idx += 1;
            write_lines(
                &w.win, 
                &exes[*top_idx..*top_idx + w.avail.line as usize], 
                &fmt_fn,
                &w.margins,
            );
        }

        highlight_fn(*win_idx, true);
        Ok(())

    }
}

// ------------------------------------------------------------------------

fn key_pgup_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if *top_idx > 0 {

            if *top_idx  > w.avail.line as usize {
                *top_idx -= w.avail.line as usize;
            } else {
                *top_idx = 0;
                
            }

            write_lines(
                &w.win, 
                &exes[*top_idx..(*top_idx + w.avail.line as usize)], 
                &fmt_fn,
                &w.margins,
            );

            *win_idx = 0;

        } else {
            highlight_fn(*win_idx, false);
            *win_idx = 0;
        }

        highlight_fn(*win_idx, true);
        Ok(())

    }

}

// ------------------------------------------------------------------------

fn key_pgdown_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if w.avail.line as usize + *top_idx < exes.len() {

            if *top_idx + w.avail.line as usize as usize * 2 > exes.len() {
                *top_idx = exes.len() - w.avail.line as usize;
            } else {
                *top_idx += w.avail.line as usize;
            }
            
            write_lines(
                &w.win, 
                &exes[*top_idx..(*top_idx + w.avail.line as usize)], 
                &fmt_fn,
                &w.margins,
            );

            *win_idx = 0;

        } else {
            highlight_fn(*win_idx, false);
            *win_idx = w.avail.line - 1;
        }

        highlight_fn(*win_idx, true);
        Ok(())

    }

}

// ------------------------------------------------------------------------

fn key_home_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if *top_idx != 0 {
            *top_idx = 0;
            write_lines(
                &w.win, 
                &exes[0..w.avail.line as usize], 
                &fmt_fn, 
                &w.margins,
            );
        }
        if *win_idx != 0 {
            highlight_fn(*win_idx, false);
            *win_idx = 0;
            highlight_fn(*win_idx, true);
        }
        Ok(())

    }

}

// ------------------------------------------------------------------------

fn key_end_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    highlight_fn : impl Fn(i32, bool) + 'a,
    fmt_fn : impl Fn(&ExeItem) -> String + 'a,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | {

        if *top_idx + w.avail.line as usize != exes.len() {
            *top_idx = exes.len() - w.avail.line as usize;
            write_lines(
                &w.win, 
                &exes[*top_idx..], 
                &fmt_fn,
                &w.margins,
            );
        }

        if *win_idx != w.avail.line - 1 {
            highlight_fn(*win_idx, false);
            *win_idx = w.avail.line - 1;
            highlight_fn(*win_idx, true);
        }
        Ok(())

    }

}

// ------------------------------------------------------------------------

fn key_enter_generator<'a>
(
    w: &'a ExeWindow,
    exes: &'a[ExeItem],
    fmt : &'a Formatter,
    colors : &'a Colors,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | win_idx: &mut i32, top_idx: &mut usize | -> Result<()> {

        let exe = &exes[*top_idx + *win_idx as usize];

        if exe.exe_type() != ExeType::NOPE {
            exe.show(&w.screen, Some(&w), fmt, colors)?;

            w.win.touch();
            w.screen.win.touch();
            w.screen.win.refresh();

        }

        Ok(())

    }

}
// ------------------------------------------------------------------------

fn key_resize_generator<'a>
(
    w: &'a ExeWindow,
    _exes: &'a[ExeItem],
    _fmt : &'a Formatter,
    _colors : &'a Colors,
) -> impl Fn(&mut i32, &mut usize) -> Result<()> + 'a
{

    move | _win_idx: &mut i32, _top_idx: &mut usize | -> Result<()> {

        // Will the existing window fit on the screen, if so just move it

        let pw = &w.win;
        let mpw = &w.screen.win;

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

        #[cfg(scum)]
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

        Ok(())

    }

}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Screen;
    use crate::exe_types::{ExeType, NotExecutable};
    use pancurses::{endwin, initscr};

    impl ExeFormat for std::string::String {
        fn exe_type(&self) -> ExeType {
            ExeType::NOPE
        }
        fn show(
            &self, 
            _screen: &Screen,
            _parent: Option<&ExeWindow>,
            _fmt: &Formatter,
            _colors: &Colors,
        ) -> Result<()> 
        { 
            Ok(()) 
        }
    }

    fn window_test(_lines: &ExeList) {
        let _w = Screen::new();
        // show(lines, &w ).unwrap();
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
        let mut lines: Vec<Box<dyn ExeFormat>> = vec![];
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
        let lines: Vec<Box<dyn ExeFormat>> = vec![
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
        let lines: Vec<Box<dyn ExeFormat>> = vec![
            Box::new("Something".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something 1".to_string()),
            Box::new("Something".to_string()),
        ];

        window_test(&lines);
    }
}
