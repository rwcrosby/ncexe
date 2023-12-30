//!
//!
//! Footer window
//! 

use anyhow::Result;

use crate::{
    color::WindowColors,
    windows::Coords,
};

// ------------------------------------------------------------------------

type LineFn = dyn Fn(usize, usize, usize) -> (i32, String);

pub struct Footer<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
    line_fn: &'a LineFn
}

impl Footer<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        line_fn: &'a LineFn,
    ) -> Box<Footer<'a>> 
    {
        let pwin = pancurses::newwin(1, 1, 0, 0);
        Box::new(Footer{ window_colors, pwin, line_fn })
    }

    pub fn show(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, false)
    }

    pub fn resize(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, false)
    }

    // --------------------------------------------------------------------

    fn paint(&mut self, size: &Coords, init: bool) -> Result<()> {

        // Size is 1 line by full width

        let size: Coords = Coords{y:  1, x: size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        if init {
            self.pwin.bkgd(self.window_colors.bkgr)
        } else {
            self.pwin.erase()
        };

        let (x, line1) = (self.line_fn)(size.x, 0, 0);

        self.pwin.attrset(self.window_colors.title);
        self.pwin.mvprintw(0, x, line1);
        
        self.pwin.noutrefresh();

        Ok(())

    }
        

}

fn _show_corners(win: &pancurses::Window, size: &Coords, wc: &WindowColors) -> Result<()> {
    
    win.attrset(wc.value);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(0, i32::try_from(size.x)? - 2, "UR");

    Ok(())
}