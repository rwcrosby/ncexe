//!
//! Scrollable region of the screen
//! 

use anyhow::Result;
use pancurses::{
    A_REVERSE,
    Input
};

use crate::color::{
    Colors,
    WindowColors, 
};

use super::{
    Coords,
    line::{
        Line,
        ToScreen,
    }, screen::Screen,
};

// ------------------------------------------------------------------------

pub struct ScrollableRegion<'a> {

    pub pwin: pancurses::Window,
    screen: &'a Screen,
    colors: &'a Colors,

    /// Dimensions of the window
    size: Coords,

    /// set of line objects to display
    lines : Vec<Box<dyn Line>>,

    /// Index into lines of the top line in the window
    top_idx: usize,

    /// Index into the window of the currently selected line
    win_idx: usize,

    /// Colors to use for this scrollable region
    window_colors: &'a WindowColors,

    /// Regions that have been expanded inline
    expanded_sets: Vec<ExpandedRegion>,

}

// --------------------------------------------------------------------

struct ExpandedRegion {
    line_id: usize,
    num_lines: usize,
}

impl<'a> ScrollableRegion<'a> {

    // --------------------------------------------------------------------

    pub fn new (
        window_colors: &'a WindowColors,
        lines: Vec<Box<dyn Line>>,
        screen: &'a Screen,
        colors: &'a Colors,
    ) -> Box<ScrollableRegion<'a>> {

        let pwin = pancurses::newwin(1, 1, 2, 0); 
        pwin.keypad(true);

        Box::new(ScrollableRegion{ 
            pwin,
            colors,
            screen,
            lines,
            size: Coords{y: 0, x: 0},
            top_idx: 0,
            win_idx: 0,
            window_colors, 
            expanded_sets: Vec::from([])
        })

    }

    // --------------------------------------------------------------------

    pub fn show(&mut self, screen_size: &Coords) -> Result<()> {
        self.paint(screen_size, true)
    }

    pub fn resize(&mut self, screen_size: &Coords) -> Result<()> {
        self.paint(screen_size, false)
    }

    // --------------------------------------------------------------------

    pub fn handle_key(&mut self, ch: Input ) -> Result<()> {
        
        match ch {
            Input::KeyDown => self.key_down_handler()?,
            Input::KeyUp => self.key_up_handler()?,
            Input::KeyPPage => self.key_pgup_handler()?,
            Input::KeyNPage => self.key_pgdown_handler()?,
            Input::KeyHome => self.key_home_handler()?,
            Input::KeyEnd => self.key_end_handler()?,
            Input::KeyEnter => self.key_enter_handler()?,
            _ => ()
        }

        Ok(())
    }

    // --------------------------------------------------------------------
    /// Toggle highlight on a line


    fn highlight(&self,  highlight: bool) -> Result<()> {

        for p in 1..self.size.x - 1 {

            let ch = self.pwin.mvinch(
                i32::try_from(self.win_idx)?, 
                i32::try_from(p)?
            );

            let rev_ch = if highlight {
                ch | A_REVERSE
            } else {
                ch & !A_REVERSE
            };

            self.pwin.addch(rev_ch);

        }

        Ok(())

    }

    // --------------------------------------------------------------------
    
    fn key_down_handler(&mut self) -> Result<()> {

        if self.win_idx < self.size.y - 1 && 
           self.win_idx + self.top_idx < self.lines.len() - 1 {
            self.highlight(false)?;
            self.win_idx += 1;
        } else if self.top_idx + self.size.y < self.lines.len() {
            self.top_idx += 1;
            self.write_lines()?;
        }

        self.highlight(true)
    }

    // --------------------------------------------------------------------
    
    fn key_up_handler(&mut self) -> Result<()> {

        if self.win_idx > 0 {
            self.highlight(false)?;
            self.win_idx -= 1;
        } else if self.top_idx > 0 {
            self.top_idx -= 1;
            self.write_lines()?;
        }

        self.highlight(true)

    }

    // --------------------------------------------------------------------
    
    fn key_pgdown_handler(&mut self) -> Result<()> {

        if self.size.y + self.top_idx < self.lines.len() {

            if self.top_idx + self.size.y > self.lines.len() {
                self.top_idx = self.lines.len() - self.size.y 
            } else {
                self.top_idx += self.size.y
            }

            self.write_lines()?;

        } else {

            self.highlight(false)?;
            self.win_idx = std::cmp::min(self.lines.len() - self.top_idx - 1, self.size.y - 1);

        }

        if self.top_idx + self.size.y > self.lines.len() {
            let first_blank = self.lines.len() - self.top_idx;
            (first_blank..self.size.y)
                .for_each(|l_no| {
                    self.pwin.mv(i32::try_from(l_no).unwrap(), 0);
                    self.pwin.clrtoeol();
                })
        }
        
        self.highlight(true)

    }

    // --------------------------------------------------------------------
    
    fn key_pgup_handler(&mut self) -> Result<()> {

        if self.top_idx > 0 {

            if self.top_idx > self.size.y {
                self.top_idx -= self.size.y
            } else {
                self.top_idx = 0;
            }

            self.write_lines()?;


        } else {
            self.highlight(false)?;
        }
        
        self.win_idx = 0;
        self.highlight(true)

    }

    // --------------------------------------------------------------------
    
    fn key_home_handler(&mut self) -> Result<()> {

        if self.top_idx > 0 {

            self.top_idx = 0;
            self.write_lines()?;


        }

        if self.win_idx > 0 {
            self.highlight(false)?;
            self.win_idx = 0;
            self.highlight(true)?;
        }

        Ok(())

    }

    // --------------------------------------------------------------------
    
    fn key_end_handler(&mut self) -> Result<()> {

        if self.top_idx + self.size.y <= self.lines.len() {
            self.top_idx = self.lines.len() - self.size.y;
            self.write_lines()?;
        }

        if self.win_idx != self.size.y - 1 {
            self.highlight(false)?;
            self.win_idx = if self.lines.len() - self.top_idx < self.size.y {
                self.lines.len() - self.top_idx - 1
            } else {
                self.size.y - 1
            };
            self.highlight(true)?;

        }

        Ok(())

    }

    // --------------------------------------------------------------------
    
    fn key_enter_handler(&mut self) -> Result<()> {

        let idx = self.top_idx + self.win_idx;
        let line = &self.lines[idx];


        if let Some(er_idx) = self.expanded_sets
                .iter()
                .position(| er | 
                
                    match line.line_id() {
                        None => false,
                        Some(n) => n == er.line_id
                    }

                ) {

            // If this line is in the list of expanded lines, delete the lines
            // and remove from the list

            let er = &self.expanded_sets[er_idx];

            let line_slice = &mut self.lines[idx+1..];

            line_slice.rotate_left(er.num_lines);
            self.lines.truncate(self.lines.len() - er.num_lines);

            self.expanded_sets.swap_remove(er_idx);

        } else if let Some(mut new_lines) = self.lines[idx].on_enter(
                self.screen, 
                self.colors)? {
                
            // If the enter handler returned some lines to add...

            let num_lines = new_lines.len();
            let line_id = line.line_id().unwrap(); // Panic if logic error
            self.lines.append(&mut new_lines);

            let line_slice = &mut self.lines[idx+1..];
            line_slice.rotate_right(num_lines);

            self.expanded_sets.push(
                ExpandedRegion{
                    line_id,
                    num_lines,
            });

        }

        Ok(())

    }

    // --------------------------------------------------------------------
    /// Reset and paint the screen

    fn paint(&mut self, size: &Coords, init: bool) -> Result<()> {
        
        self.size = Coords{y:  size.y - 3, x: size.x};
        self.pwin.resize(size.y.try_into()?, size.x.try_into()?);
        
        if init {
            self.pwin.bkgd(self.window_colors.text)
        } else {
            self.pwin.erase()
        };

        if self.win_idx > self.size.y - 1 {
            self.win_idx = self.size.y -1;
        }

        self.write_lines()?;
        self.highlight(true)?;
        self.pwin.noutrefresh();

        Ok(())

    }

    // --------------------------------------------------------------------
    // Set the scrolling indicators

    fn set_indicators(&self) {

        let last_col = self.size.x as i32 - 1;

        self.pwin.mvprintw(
            0, last_col,
            if self.top_idx > 0 { "\u{21d1}" } else { " " }
        );

        self.pwin.mvprintw(
            i32::try_from(self.size.y).unwrap() - 1, last_col,
            if self.size.y + self.top_idx >= self.lines.len() { 
                " " 
            } else {
                "\u{21d3}" 
            } 
        );
    
    }

    // --------------------------------------------------------------------
    // Write whatever lines are available to the screen

    fn write_lines(&self) -> Result<()> {

        let lim = if (self.size.y) <= self.lines.len() - self.top_idx {
            // More lines remain than will fit on the screen
            self.top_idx + self.size.y
        } else {
            // All remaining lines will fit on the screen
            self.lines.len()
        };

        for (y, line) in self.lines[self.top_idx..lim]
            .iter()
            .enumerate()
        {

            if let Some(ind) = line.line_ind() {
                self.pwin.mvaddch(y as i32, 0, ind);
            }
            line.as_pairs(self.size.x)?.show(&self.pwin, Coords{y: y, x: 1});
        }

        self.set_indicators();
        Ok(())

    }

}