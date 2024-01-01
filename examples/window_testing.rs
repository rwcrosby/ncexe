extern crate ncexe;

use ncexe::exe_types::ETYPE_LENGTH;
use ncexe::windows::line::Line;
use ncexe::windows::WindowSet;
use ncexe::windows::screen::Screen;
use ncexe::windows::header;
use ncexe::windows::scrollable_region;
use ncexe::windows::footer;
use ncexe::color;


fn main() {

    let screen = Screen::new();

    let colors = color::Colors::new("dark").unwrap();
    let cs1 = colors.get_window_set_colors("file_list").unwrap();
    let cs2 = colors.get_window_set_colors("file_header").unwrap();

    screen.win.bkgd(cs1.header.text);
    screen.win.refresh();

    let tl = ETYPE_LENGTH as i32;
    let ml = 10i32;
        
    let hdr = format!(" {etype:<tl$.tl$} {size:>ml$.ml$} {filename}", 
        tl=tl as usize, etype="Type",
        ml=ml as usize, size="Size",
        filename="Name",
    );

    let hdr_fn = move | _sc: usize | (0, hdr.clone());

    let enter_fn = Box::new(| _idx: usize, _line: &dyn Line | Ok(()) );

    let footer_fn = move | sc: usize| {

        let txt = format!("{} Files {} Bytes",
            0,
            0 );

        let excess = i32::try_from(sc).unwrap() - i32::try_from(txt.len()).unwrap();

        let start_pos: i32 = if excess <= 0 {
            0
        } else {
            excess / 2
        };

        (start_pos, txt)

    };

    let mut lines: Vec<&dyn Line> = Vec::from([]);

    let hdr_win = header::Header::new(&cs1.header, Box::new(hdr_fn));
    let scr_win = scrollable_region::ScrollableRegion::new(
        &cs2.scrollable_region, 
        &mut lines,
        Box::new(enter_fn),
    );
    let ftr_win = footer::Footer::new(&cs1.footer, Box::new(footer_fn));

    let mut win_set = WindowSet::new(
        &screen, 
        hdr_win, 
        scr_win, 
        ftr_win);

    win_set.show().unwrap();

}