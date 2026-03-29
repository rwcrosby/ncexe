fn base_windows() {
    use crossterm::{
        cursor,
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, text::Line, widgets::Paragraph, Terminal};
    use std::io;

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide).unwrap();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let area = f.area();
        let lines: Vec<Line> = (0..10)
            .map(|i| Line::raw(format!("Line {}", i)))
            .collect();
        f.render_widget(Paragraph::new(lines), area);
    }).unwrap();

    // Wait for keypress
    loop {
        if let Ok(Event::Key(_)) = event::read() {
            break;
        }
    }

    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn windows_test1() {
        base_windows()
    }

}
