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

    let colors = color::Colors::new().unwrap();
    let cs1 = colors.set("file_list").unwrap();
    let cs2 = colors.set("header").unwrap();

    screen.win.bkgd(pancurses::COLOR_PAIR(cs1.text as u32));
    screen.win.refresh();

    let mut hdr_win = header::Header::new(&cs1, &screen_size );
    let mut scr_win = scrollable_region::ScrollableRegion::new(&cs2, &screen_size);
    let mut ftr_win = footer::Footer::new(&cs1, &screen_size);

    let mut win_set = WindowSet::new(
        &screen, 
        &mut hdr_win, 
        &mut scr_win, 
        &mut ftr_win);

    win_set.show().unwrap();

}