//!
//! Popup windows
//!

use anyhow::Error;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event},
    execute, queue,
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal,
};
use std::io::{self, Write};

// ------------------------------------------------------------------------

pub fn error_window(error: &Error) {
    let mut lines = vec![];
    for cause in error.chain() {
        match lines.len() {
            0 => lines.push(format!("Error: {}", cause)),
            1 => {
                lines.push("Cause:".into());
                lines.push(format!("    {}", cause))
            }
            _ => lines.push(format!("    {}", cause)),
        }
    }
    window("Error", lines, (Color::White, Color::Red));
}

// ------------------------------------------------------------------------
/// Draw a popup window using crossterm directly.
///
/// NOTE: Must NOT be called from within a ratatui `terminal.draw()` closure,
/// as the direct crossterm writes would be overwritten when draw() flushes.
/// Call from event loop handlers only; the next draw() cycle will redraw cleanly.

pub fn window(title: &str, lines: Vec<String>, colors: (Color, Color)) {

    let (term_width, term_height) = terminal::size().unwrap_or((80, 24));

    let max_line_len = lines.iter().fold(0, |ml, line| ml.max(line.len()));
    let inner_width = max_line_len.max(title.len());
    let width = (4 + inner_width) as u16;
    let height = (4 + lines.len()) as u16;
    let ypos = term_height.saturating_sub(height) / 2;
    let xpos = term_width.saturating_sub(width) / 2;

    let mut stdout = io::stdout();

    let _ = execute!(
        stdout,
        SetColors(Colors::new(colors.0, colors.1)),
    );

    // Draw box
    for row in 0..height {
        for col in 0..width {
            let top_bottom = row == 0 || row == height - 1;
            let left_right = col == 0 || col == width - 1;
            let ch = if top_bottom && left_right {
                '+'
            } else if top_bottom {
                '-'
            } else if left_right {
                '|'
            } else {
                ' '
            };
            let _ = queue!(stdout, MoveTo(xpos + col, ypos + row), Print(ch));
        }
    }

    // Title centered on top border
    let title_x = xpos + (width.saturating_sub(title.len() as u16)) / 2;
    let _ = queue!(stdout, MoveTo(title_x, ypos), Print(title));

    // Message lines
    for (i, line) in lines.iter().enumerate() {
        let _ = queue!(stdout, MoveTo(xpos + 2, ypos + 2 + i as u16), Print(line));
    }

    let _ = execute!(stdout, ResetColor);
    let _ = stdout.flush();

    // Wait for any key
    loop {
        if let Ok(Event::Key(_)) = event::read() {
            break;
        }
    }
}
