//!
//! Scrollable region of the screen
//! 

use anyhow::Result;

use crate::windows::Coords;
use crate::color::WindowColors;

pub struct ScrollableRegion<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
}

impl ScrollableRegion<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        screen_size: &Coords,
    ) -> Box<ScrollableRegion<'a>> 
    {
        let pwin = pancurses::newwin(1, screen_size.x, 2, 0);
        Box::new(ScrollableRegion{ window_colors, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is lines - header and footer sizes by full width
        
        let size: Coords = Coords{y:  screen_size.y - 3, x: screen_size.x};
        
        self.pwin.resize(size.y, size.x);
        self.pwin.bkgd(self.window_colors.title as u32);

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.refresh();
        
        Ok(())

    }

    pub fn resize(&mut self, new_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  new_size.y - 3, x: new_size.x};

        self.pwin.resize(size.y, size.x);
        self.pwin.erase();

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, dim: &Coords, wc: &WindowColors) -> Result<()> {

    win.attrset(wc.text);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(dim.y - 1, 0, "LL");
    win.mvprintw(0, dim.x - 2, "UR");
    win.mvprintw(dim.y - 1, dim.x - 2, "LR");

    Ok(())
}