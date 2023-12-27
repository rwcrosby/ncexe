//!
//! Header window
//! 

use anyhow::Result;

use crate::color::WindowColors;
use crate::windows::Coords;

pub struct Header<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
    line2_fn: &'a dyn Fn(usize) -> &'a str,
}

impl Header<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        line2_fn: &'a dyn Fn(usize) -> &'a str,
    ) -> Box<Header<'a>> 
    {
        let pwin = pancurses::newwin(2, 1, 0, 0);
        Box::new(Header{ window_colors, pwin, line2_fn })
    }

    pub fn show(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, true)
    }
    
    pub fn resize(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, false)
    }
    
    fn paint(&mut self, size: &Coords, init: bool) -> Result<()> {
        
        // Size is 2 lines by full width

        let size: Coords = Coords{y:  2, x: size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        if init {
            self.pwin.bkgd(self.window_colors.bkgr)
        } else {
            self.pwin.erase()
        };

        let line2 = (self.line2_fn)(size.x);
        self.pwin.attrset(self.window_colors.title);
        self.pwin.mvprintw(1, 0, line2);

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();

        Ok(())

    }

}

fn show_corners(win: &pancurses::Window, size: &Coords, wc: &WindowColors) -> Result<()> {
    
    win.attrset(wc.title);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(0, i32::try_from(size.x).unwrap() - 2, "UR");

    Ok(())
}