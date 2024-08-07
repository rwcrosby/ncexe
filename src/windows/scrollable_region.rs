//!
//! Scrollable region of the screen
//! 

use anyhow::Result;
use pancurses::{
    A_REVERSE,
    Input
};

use crate::color::WindowColors;

use super::{
    Coords,
    line::{
        Line,
        ToScreen, LineVec,
    }, 
};

// --------------------------------------------------------------------

type ScrollableRegionLines<'srl> = Vec<ScrollableRegionLine<'srl>>;

#[derive(Debug)]
enum EnterType {
    NewWindow,
    /// Tuple is number of expanded lines, amount to indent
    Expandable((usize, usize)),
    None,
}

impl EnterType {
    fn set_num_lines(&mut self, new_num_lines: usize) -> usize {
        if let EnterType::Expandable(mut enter_type) = self {
            enter_type.0 = new_num_lines;
            *self = EnterType::Expandable(enter_type);
            enter_type.0
        } else {        
            panic!("set_num called on invalid type {:?}", self)
        }
    }
}

struct ScrollableRegionLine<'srl> {

    /// Ownership of the Line is passed here
    line: Box<dyn Line<'srl> + 'srl>,

    /// What to do on enter on the line
    enter: EnterType,

    /// Indent for THIS line
    indent: usize,

}
 
fn make_scrollable_lines (
    lines: LineVec,
    indent: usize
) -> ScrollableRegionLines {

    lines
        .into_iter()
        .map(| line | {

            let enter = if line.new_window() {
                EnterType::NewWindow
            } else if let Some(indent) = line.expand() {
                EnterType::Expandable((0, indent))
            } else {
                EnterType::None
            };

            ScrollableRegionLine{line, enter, indent } 

        })
        .collect()

}

// ------------------------------------------------------------------------

pub struct ScrollableRegion<'sr> {

    pub pwin: pancurses::Window,

    /// Dimensions of the window
    size: Coords,

    /// Set of lines to display
    lines: ScrollableRegionLines<'sr>,

    /// Index into lines of the top line in the window
    top_idx: usize,

    /// Index into the window of the currently selected line
    win_idx: usize,

    /// Colors to use for this scrollable region
    window_colors: &'sr WindowColors,

}

impl<'sr> ScrollableRegion<'sr> {

    pub fn new (
        window_colors: &'sr WindowColors,
        lines: LineVec<'sr>,
    ) -> ScrollableRegion<'sr> {

        let pwin = pancurses::newwin(1, 1, 2, 0); 
        pwin.keypad(true);

        ScrollableRegion{ 
            pwin,
            lines: make_scrollable_lines(lines, 0),
            size: Coords{y: 0, x: 0},
            top_idx: 0,
            win_idx: 0,
            window_colors, 
        }

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
        let line = &mut self.lines[idx];

        match line.enter {

            EnterType::NewWindow =>
                
                self.lines[idx].line.new_window_fn()?,

            EnterType::Expandable((num_lines, indent)) => 
                
                if num_lines > 0 {
                    
                    line.enter.set_num_lines(0);

                    let line_slice = &mut self.lines[idx+1..];
        
                    line_slice.rotate_left(num_lines);
                    self.lines.truncate(self.lines.len() - num_lines);
        

                } else if let Some(new_lines) = line.line.expand_fn()? {

                    let num_lines = new_lines.len();
                    line.enter.set_num_lines(num_lines);
        
                    let mut to_append = make_scrollable_lines(new_lines, indent);
        
                    self.lines.append(&mut to_append);
        
                    let line_slice = &mut self.lines[idx+1..];
                    line_slice.rotate_right(num_lines);
                
                },

            EnterType::None => (),
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
            0, 
            last_col,
            if self.top_idx > 0 { 
                "\u{21d1}" 
            } else { 
                " " 
            }
        );

        self.pwin.mvprintw(
            self.size.y as i32 - 1, 
            last_col,
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

            match line.enter {
                EnterType::Expandable((num_lines, _)) => 
                    self.pwin.mvaddch(y as i32, 0, 
                        if num_lines > 0 {
                            '-'
                        } else {
                            '+'
                        }),
                EnterType::NewWindow => 
                    self.pwin.mvaddch(y as i32, 0, '='),
                EnterType::None => 
                    self.pwin.mvaddch(y as i32, 0, ' '),
            };

            line.line
                .as_pairs(self.size.x - line.indent - 1)?
                .show(&self.pwin, Coords{y, x: line.indent + 1});
        }

        self.set_indicators();
        Ok(())

    }

}