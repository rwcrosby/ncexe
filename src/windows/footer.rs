//!
//! Footer window
//! 

use anyhow::Result;

use crate::{
    color::WindowColors,
    windows::Coords,
};

// ------------------------------------------------------------------------

pub type LineFn<'a> = Box<dyn Fn(usize) -> (i32, String) + 'a>;

pub struct Footer<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
    line_fn: LineFn<'a>
}

impl Footer<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        line_fn: LineFn<'a>,
    ) -> Footer<'a>
    {
        let pwin = pancurses::newwin(1, 1, 0, 0);
        Footer{ window_colors, pwin, line_fn }
    }

    pub fn show(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, true)
    }

    pub fn resize(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, false)
    }

    // --------------------------------------------------------------------

    fn paint(&mut self, screen_size: &Coords, init: bool) -> Result<()> {

        let size: Coords = Coords{y: 1, x: screen_size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        self.pwin.mvwin(i32::try_from(screen_size.y)? - 1, 0);

        if init {
            self.pwin.bkgd(self.window_colors.bkgr)
        } else {
            self.pwin.erase()
        };

        let (x, line) = (self.line_fn)(size.x);
        self.pwin.attrset(self.window_colors.title);
        self.pwin.mvprintw(0, x, line);

        self.pwin.noutrefresh();
        
        Ok(())

    }

}