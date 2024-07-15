extern crate ncexe;

use ncexe::exe_types::ETYPE_LENGTH;
use ncexe::windows;
use ncexe::windows::line::Line;
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

    // let enter_fn = Box::new(| _idx: usize, _line: &dyn Line | Ok(()) );

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

    let lines: Vec<Box<dyn Line>> = Vec::from([]);

    let mut hdr_win = header::Header::new(&cs1.header, Box::new(hdr_fn));
    let mut scr_win = scrollable_region::ScrollableRegion::new(
        &cs2.scrollable_region, 
        lines,
        &colors,
    );
    
    let mut ftr_win = footer::Footer::new(&cs1.footer, Box::new(footer_fn));

    windows::show(
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win).unwrap();

}