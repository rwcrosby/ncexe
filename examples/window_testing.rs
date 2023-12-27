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

    let hdr_fn = | sc: usize | 
        if sc > hdr.len() {
            &hdr
        }
        else {
            &hdr.as_str()[0..sc as usize]
        };

    let mut lines: Vec<&dyn Line> = Vec::from([]);

    let mut hdr_win = header::Header::new(&cs1.header, &hdr_fn);
    let mut scr_win = scrollable_region::ScrollableRegion::new(&cs2.scrollable_region, 
        &mut lines);
    let mut ftr_win = footer::Footer::new(&cs1.footer);

    let mut win_set = WindowSet::new(
        &screen, 
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win);

    win_set.show().unwrap();

}