//!
//! Modules for the various screens in the program
//!

pub mod details_list;
pub mod file_header;
pub mod file_list;
pub mod terminal;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};

use crate::{
    screens::terminal::TERMWIN,
    windows::{
        footer::Footer,
        header::Header,
        scrollable_region::ScrollableRegion,
    },
};

// ------------------------------------------------------------------------

pub fn show(
    hdr_win: &mut Header,
    scr_win: &mut ScrollableRegion,
    ftr_win: &mut Footer,
) -> Result<()> {

    loop {
        {
            let mut terminal = TERMWIN.terminal.lock().unwrap();
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Min(0),
                        Constraint::Length(1),
                    ])
                    .split(f.area());

                hdr_win.render(f, chunks[0]);
                scr_win.render(f, chunks[1]);
                ftr_win.render(f, chunks[2]);
            })?;
        }

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Enter => {
                    scr_win.key_enter_handler()?;
                    // Force full redraw after enter (may have opened/closed nested window)
                    TERMWIN.terminal.lock().unwrap().clear()?;
                }
                code @ (KeyCode::Down
                | KeyCode::Up
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::Home
                | KeyCode::End) => {
                    scr_win.handle_key(code)?;
                }
                _ => {}
            },
            Event::Resize(_, _) => {
                // ratatui automatically uses the new size on next draw()
            }
            _ => {}
        }
    }

    Ok(())
}
