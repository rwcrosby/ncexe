//!
//! Header window
//! 

use anyhow::Result;

use crate::color::WindowColors;
use crate::windows::Coords;

pub struct Header<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
}

impl Header<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        screen_size: &Coords,
    ) -> Box<Header<'a>> 
    {
        let pwin = pancurses::newwin(2, screen_size.x, 0, 0);
        Box::new(Header{ window_colors, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is 2 lines by full width
        
        let size: Coords = Coords{y:  2, x: screen_size.x};
        
        self.pwin.resize(size.y, size.x);
        self.pwin.bkgd(self.window_colors.bkgr);

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.refresh();
        
        Ok(())

    }

    pub fn resize(&mut self, new_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  2, x: new_size.x};

        self.pwin.resize(size.y, size.x);
        self.pwin.erase();

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, dim: &Coords, wc: &WindowColors) -> Result<()> {
    
    win.attrset(wc.title);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(dim.y - 1, 0, "LL");
    win.mvprintw(0, dim.x - 2, "UR");
    win.mvprintw(dim.y - 1, dim.x - 2, "LR");

    Ok(())
}