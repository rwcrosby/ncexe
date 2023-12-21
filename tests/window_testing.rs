extern crate ncexe;

use ncexe::windows::screen::Screen;
use ncexe::windows::header;
use ncexe::windows::scrollable_region;
use ncexe::windows::footer;
use ncexe::windows::window;
use ncexe::color;

#[test]
fn windows_test1() {

    let screen = Screen::new();

    let cs = color::ColorSet{frame: 0, title: 0, text: 0, value: 0};

    let hdr_win = header::Header::new(&cs);
    let scr_win = scrollable_region::ScrollableRegion::new(&cs);
    let ftr_win = footer::Footer::new(&cs);

    let win_set = window::WindowSet::new(&screen, &hdr_win, &scr_win, &ftr_win);

    win_set.show().unwrap();

}