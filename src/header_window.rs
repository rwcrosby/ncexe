use pancurses::{Input, Window};

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct HeaderWindow<'a> {

    x_start: i32,
    x_len: i32,
    y_start: i32,
    y_len: i32,

    win : &'a Window,


}
