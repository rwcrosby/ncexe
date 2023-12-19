//! 
//! The executable file header window
//! 

use anyhow::Result;
use pancurses::COLOR_PAIR;

use crate::{
    color::Colors,
    formatter::FormatBlock,
    window::{
        Coords,
        ExeWindow,
        Margins,
    },
    windows::screen::Screen,
};

// ------------------------------------------------------------------------

pub fn show(
    screen : &Screen,
    parent: Option<&ExeWindow>,
    colors: &Colors,
    title: &str,
    fmt_blk: &Box<FormatBlock>,
    mapped_data: &[u8],
) -> Result<()> {
    
    // Create the window

    let dim = Coords{line: fmt_blk.fields.len() as i32,
                     col: (fmt_blk.max_text_len + 3 + fmt_blk.max_value_len) as i32};

    let color_set = colors.set("header");

    let margins = Margins{top: 1, bottom: 1, left: 2, right: 2 };

    let w = ExeWindow::new(
        dim, 
        title, 
        color_set,
        parent,
        &margins,
        screen, 
    )?;

    // Display the fields
    
    let pw = &w.win;

    for (idx, fld) in fmt_blk.fields.iter().enumerate() {

        let df = &mapped_data
            [fld.offset as usize..fld.offset as usize + fld.y_field.size];

        pw.mv(idx as i32 + margins.top, 
              margins.left);

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
