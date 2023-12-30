//!
//! Standard header window
//! 

use anyhow::Result;

use crate::{
    color::WindowColors,
    windows::Coords,
};

// ------------------------------------------------------------------------

type Line2Fn = dyn Fn(usize) -> String;

// ------------------------------------------------------------------------

pub struct Header<'a> {
    window_colors: &'a  WindowColors,
    pub pwin: pancurses::Window,
    line2_fn: &'a Line2Fn,
}

impl Header<'_> {

    /// Create a header window using `window_colors`, building the second
    /// line using `line2_fn`

    pub fn new<'a> (
        window_colors: &'a WindowColors, 
        line2_fn: &'a Line2Fn,
    ) -> Box<Header<'a>> 
    {
        let pwin = pancurses::newwin(2, 1, 0, 0);
        Box::new(Header{ window_colors, pwin, line2_fn })
    }

    // --------------------------------------------------------------------

    pub fn show(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, true)
    }
    
    pub fn resize(&mut self, size: &Coords) -> Result<()> {
        self.paint(size, false)
    }
    
    // --------------------------------------------------------------------

    fn paint(&mut self, size: &Coords, init: bool) -> Result<()> {
        
        // Size is 2 lines by full width

        let size: Coords = Coords{y:  2, x: size.x};

        self.pwin.resize(i32::try_from(size.y)?, i32::try_from(size.x)?);
        if init {
            self.pwin.bkgd(self.window_colors.bkgr)
        } else {
            self.pwin.erase()
        };

        let line1 = make_title(
            "ncexe v23.12.1", 
            "",
            "Use the arrow keys to navigate, q to go back",
            size.x
        )?;
        let line2 = (self.line2_fn)(size.x);
        
        self.pwin.attrset(self.window_colors.title);
        self.pwin.mvprintw(0, 0, line1);
        self.pwin.mvprintw(1, 0, line2);
        
        self.pwin.noutrefresh();

        Ok(())

    }

}

// ------------------------------------------------------------------------
/// Create the title string

fn make_title(left: &str, middle: &str, right: &str, cols: usize ) -> Result<String> {

    let gutter_size = isize::try_from(cols)? - isize::try_from(left.len() + middle.len() + right.len())?;


    let title = if gutter_size < 2 {            // Need to truncate

        String::from(&(left.to_owned() + " " + middle + " " + right)[..cols])

    } else {                                    // Everything fits

        let lgutter = gutter_size / 2;
        let rgutter = gutter_size / 2 + if gutter_size - lgutter * 2 > 0 {1} else {0};

        left.to_owned() + 
            &(" ".repeat(usize::try_from(lgutter)?)) + 
            middle + 
            &(" ".repeat(usize::try_from(rgutter)?)) + 
            right

    };


    Ok(title)
}
