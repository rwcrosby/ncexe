//!
//!
//! Footer window
//! 

use anyhow::Result;

use crate::color::WindowColors;
use crate::windows::Coords;

pub struct Footer<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
}

impl Footer<'_> {

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
    ) -> Box<Footer<'a>> 
    {
        let pwin = pancurses::newwin(1, 1, 0, 0);
        Box::new(Footer{ window_colors, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is 1 lines by full width
        
        let size: Coords = Coords{y: 1, x: screen_size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        self.pwin.mvwin(i32::try_from(screen_size.y)? - 1, 0);

        self.pwin.bkgd(self.window_colors.bkgr);

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();
        
        Ok(())

    }

    pub fn resize(&mut self, screen_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  1, x: screen_size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        self.pwin.mvwin(i32::try_from(screen_size.y)? - 1, 0);

        self.pwin.erase();

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, size: &Coords, wc: &WindowColors) -> Result<()> {
    
    win.attrset(wc.value);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(0, i32::try_from(size.x)? - 2, "UR");

    Ok(())
}