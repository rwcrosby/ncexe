//!
//! Scrollable region of the screen
//! 

use anyhow::Result;
use pancurses::{
    A_REVERSE,
    Input
};

use crate::{
    color::{
        Colors,
        WindowColors,
    },
    formatter::Formatter,
};

use super::{
    Coords,
    line::Line,
    header_window,
    screen::Screen,
};

// ------------------------------------------------------------------------

pub struct ScrollableRegion<'a> {

    window_colors: &'a  WindowColors,

    pub pwin: pancurses::Window,

    /// Dimensions of the window
    size: Coords,

    /// set of line objects to display
    lines : &'a mut Vec<&'a dyn Line>,

    /// Index into lines of the top line in the window
    top_idx: usize,

    /// Index into the window of the currently selected line
    win_idx: usize,

    screen: &'a Screen,
    fmt: &'a Formatter,
    colors: &'a Colors,

}

impl<'a> ScrollableRegion<'a> {

    // --------------------------------------------------------------------

    pub fn new (
        window_colors: &'a WindowColors,
        lines: &'a mut Vec<&'a dyn Line >,
        screen: &'a Screen,
        fmt: &'a Formatter,
        colors: &'a Colors,
    ) -> Box<ScrollableRegion<'a>> 
    {

        let pwin = pancurses::newwin(1, 1, 2, 0); 
        pwin.keypad(true);

        Box::new(ScrollableRegion{ 
            window_colors, 
            pwin,
            size: Coords{y: 0, x: 0},
            lines,
            top_idx: 0,
            win_idx: 0,
            screen,
            fmt,
            colors,
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

        for p in 1..self.size.x {

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
            self.write_lines();
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
            self.write_lines();
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

            self.write_lines();

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

            self.write_lines();


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
            self.write_lines();


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
            self.write_lines();
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

        let line = self.lines[self.top_idx + self.win_idx];

        header_window::show(
            line.to_executable(), 
            self.screen, 
            self.fmt, 
            self.colors
        )

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

        self.write_lines();
        self.highlight(true)?;
        self.pwin.noutrefresh();

        Ok(())

    }

    // --------------------------------------------------------------------
    // Set the scrolling indicators

    fn set_indicators(&self) {

        self.pwin.mvprintw(
            0, 0,
            if self.top_idx > 0 { "\u{21d1}" } else { " " }
        );

        self.pwin.mvprintw(
            i32::try_from(self.size.y).unwrap() - 1, 0,
            if self.size.y + self.top_idx >= self.lines.len() { 
                " " 
            } else {
                "\u{21d3}" 
            } 
        );
    
    }

    // --------------------------------------------------------------------
    // Write whatever lines are available to the screen

    fn write_lines(&self) {

        let lim = if (self.size.y) <= self.lines.len() - self.top_idx {
            // More lines remain than will fit on the screen
            self.top_idx + self.size.y
        } else {
            // All remaining lines will fit on the screen
            self.lines.len()
        };

        self.lines[self.top_idx..lim]
            .iter()
            .enumerate()
            .for_each(|(y, l)| {
                self.pwin.mv(i32::try_from(y).unwrap(), 0);
                l.as_line(self.size.x)
                    .iter()
                    .for_each(| li | { 
                        if let Some(attr) = li.0 {
                           self.pwin.attrset(attr);
                        };
                        self.pwin.printw(&li.1); 
                    })
            } );

        self.set_indicators();

    }

}