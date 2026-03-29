//!
//! The terminal screen
//!

use std::io::{self, Stdout};
use std::sync::Mutex;

use crossterm::{
    cursor,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use once_cell::sync::Lazy;
use ratatui::{backend::CrosstermBackend, Terminal};

// ------------------------------------------------------------------------

pub struct TermWin {
    pub terminal: Mutex<Terminal<CrosstermBackend<Stdout>>>,
}

impl TermWin {

    pub fn new() -> Self {
        enable_raw_mode().expect("enable raw mode");
        execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)
            .expect("enter alternate screen");
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).expect("create terminal");
        TermWin {
            terminal: Mutex::new(terminal),
        }
    }

    pub fn term(&self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
    }

}

impl Default for TermWin {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TermWin {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
    }
}

unsafe impl Sync for TermWin {}
unsafe impl Send for TermWin {}

// ------------------------------------------------------------------------
// Global screen object

pub static TERMWIN: Lazy<TermWin> = Lazy::new(TermWin::new);

// ------------------------------------------------------------------------

#[test]
#[ignore]
pub fn screen_test_1() {
    use once_cell::sync::Lazy;
    use super::terminal::TERMWIN;

    Lazy::force(&TERMWIN);

    let mut terminal = TERMWIN.terminal.lock().unwrap();
    terminal.draw(|f| {
        use ratatui::{text::Line, widgets::Paragraph};
        let area = f.area();
        f.render_widget(Paragraph::new(Line::raw("Ratatui test 1")), area);
    }).unwrap();

    drop(terminal);

    crossterm::event::read().unwrap();

    TERMWIN.term();
}
