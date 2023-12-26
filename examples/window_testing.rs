extern crate ncexe;

use ncexe::windows::Coords;
use ncexe::windows::WindowSet;
use ncexe::windows::screen::Screen;
use ncexe::windows::header;
use ncexe::windows::scrollable_region;
use ncexe::windows::footer;
use ncexe::color;


fn main() {

    let screen = Screen::new();

    let screen_size: Coords = screen.win.get_max_yx().into();

    let colors = color::Colors::new("dark").unwrap();
    let cs1 = colors.get_window_set_colors("file_list").unwrap();
    let cs2 = colors.get_window_set_colors("file_header").unwrap();

    screen.win.bkgd(cs1.header.text);
    screen.win.refresh();

    let mut hdr_win = header::Header::new(&cs1.header, &screen_size );
    let mut scr_win = scrollable_region::ScrollableRegion::new(&cs2.scrollable_region, &screen_size);
    let mut ftr_win = footer::Footer::new(&cs1.footer, &screen_size);

    let mut win_set = WindowSet::new(
        &screen, 
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win);

    win_set.show().unwrap();

}