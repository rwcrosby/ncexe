//!
//!
//! Footer window
//! 

use anyhow::Result;

use crate::color::ColorSet;
use crate::windows::Coords;

pub struct Footer<'a> {
    cs: &'a  ColorSet,
    pub pwin: pancurses::Window,
}

impl Footer<'_> {

    pub fn new<'a> (
        cs: &'a ColorSet, 
        screen_size: &Coords,
    ) -> Box<Footer<'a>> 
    {
        let pwin = pancurses::newwin(
            1, 
            screen_size.x, 
            screen_size.y - 1,
             0
        );
        Box::new(Footer{ cs, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is 1 lines by full width
        
        let size: Coords = Coords{y: 1, x: screen_size.x};
        self.pwin.resize(size.y, size.x);

        self.pwin.bkgd(pancurses::COLOR_PAIR(self.cs.title as u32));

        show_corners(&self.pwin, &size)?;

        self.pwin.refresh();
        
        Ok(())

    }

    pub fn resize(&mut self, new_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  1, x: new_size.x};
        self.pwin.resize(size.y, size.x);

        self.pwin.mvwin(new_size.y - 1, 0);

        self.pwin.erase();

        show_corners(&self.pwin, &size)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, dim: &Coords) -> Result<()> {
    
    win.mvprintw(0, 0, "UL");
    win.mvprintw(0, dim.x - 2, "UR");

    Ok(())
}