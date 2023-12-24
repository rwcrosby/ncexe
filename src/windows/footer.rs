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
        screen_size: &Coords,
    ) -> Box<Footer<'a>> 
    {
        let pwin = pancurses::newwin(
            1, 
            screen_size.x, 
            screen_size.y - 1,
             0
        );
        Box::new(Footer{ window_colors, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is 1 lines by full width
        
        let size: Coords = Coords{y: 1, x: screen_size.x};
        self.pwin.resize(size.y, size.x);

        self.pwin.bkgd(self.window_colors.bkgr);

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.refresh();
        
        Ok(())

    }

    pub fn resize(&mut self, new_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  1, x: new_size.x};
        self.pwin.resize(size.y, size.x);

        self.pwin.mvwin(new_size.y - 1, 0);

        self.pwin.erase();

        show_corners(&self.pwin, &size, self.window_colors)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, dim: &Coords, wc: &WindowColors) -> Result<()> {
    
    win.attrset(wc.value);
    win.mvprintw(0, 0, "UL");
    win.mvprintw(0, dim.x - 2, "UR");

    Ok(())
}