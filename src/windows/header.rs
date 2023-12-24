//!
//! Header window
//! 

use anyhow::Result;

use crate::color::ColorSet;
use crate::windows::Coords;

pub struct Header<'a> {
    cs: &'a  ColorSet,
    pub pwin: pancurses::Window,
}

impl Header<'_> {

    pub fn new<'a> (
        cs: &'a ColorSet, 
        screen_size: &Coords,
    ) -> Box<Header<'a>> 
    {
        let pwin = pancurses::newwin(2, screen_size.x, 0, 0);
        Box::new(Header{ cs, pwin })
    }

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {

        // Size is 2 lines by full width
        
        let size: Coords = Coords{y:  2, x: screen_size.x};
        
        self.pwin.resize(size.y, size.x);
        self.pwin.bkgd(pancurses::COLOR_PAIR(self.cs.frame as u32));

        show_corners(&self.pwin, &size, self.cs)?;

        self.pwin.refresh();
        
        Ok(())

    }

    pub fn resize(&mut self, new_size: &Coords) -> Result<()> {

        let size: Coords = Coords{y:  2, x: new_size.x};

        self.pwin.resize(size.y, size.x);
        self.pwin.erase();

        show_corners(&self.pwin, &size, self.cs)?;

        self.pwin.noutrefresh();

        Ok(())
    }

}

fn show_corners(win: &pancurses::Window, dim: &Coords, cs: &ColorSet) -> Result<()> {
    
    win.attrset(pancurses::COLOR_PAIR(cs.title as u32));
    win.mvprintw(0, 0, "UL");
    win.mvprintw(dim.y - 1, 0, "LL");
    win.mvprintw(0, dim.x - 2, "UR");
    win.mvprintw(dim.y - 1, dim.x - 2, "LR");

    Ok(())
}