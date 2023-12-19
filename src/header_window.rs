use anyhow::Result;
use pancurses::COLOR_PAIR;

use crate::color::Colors;
use crate::formatter::FormatBlock;
use crate::main_window::MainWindow;
use crate::window;

pub fn show(
    mw : &MainWindow,
    colors: &Colors,
    fmt_blk: &Box<FormatBlock>,
    title: &str,
    mapped_data: &[u8],
) -> Result<()> {
    
    // Load the format specification

    let lines = fmt_blk.fields.len();
    let cols = fmt_blk.max_text_len + 3 + fmt_blk.max_value_len;

    let color_set = colors.set("header");

    // Create the window
    
    let w = window::ExeWindow::new(
        lines, 
        cols, 
        title, 
        color_set,
        mw, 
    )?;

    let pw = &w.win;
    let _mpw = &mw.win;

    // Display the fields

    for (idx, fld) in fmt_blk.fields.iter().enumerate() {

        let df = &mapped_data
            [fld.offset as usize..fld.offset as usize + fld.y_field.size];

        pw.mv((idx + window::TMARGIN) as i32, 
              window::LMARGIN as i32);

        pw.attrset(COLOR_PAIR(color_set.text as u32));
        pw.addstr(format!("{fname:>nl$.nl$} : ", 
                          nl = fmt_blk.max_text_len,
                          fname=fld.y_field.name));

        pw.attrset(COLOR_PAIR(color_set.value as u32));
        pw.addstr((fld.fmt_fn)(df));

    };

    pw.getch();

    Ok(())

}
