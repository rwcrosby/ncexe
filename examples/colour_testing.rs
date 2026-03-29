/// Color palette demo using crossterm directly.
/// Shows a grid of foreground/background color combinations.

use std::collections::HashMap;
use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal::{
        disable_raw_mode, enable_raw_mode, size,
        Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    event::{self, Event},
};

fn main() -> anyhow::Result<()> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    let colors = color_map();

    show_colors("Normal", &mut stdout, &colors, false, 2)?;
    show_colors("Bold", &mut stdout, &colors, true, 14)?;

    stdout.flush()?;

    // Wait for keypress
    loop {
        if let Ok(Event::Key(_)) = event::read() {
            break;
        }
    }

    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())

}

fn show_colors(
    title: &str,
    stdout: &mut impl Write,
    colors: &HashMap<&str, u8>,
    bold: bool,
    y_start: u16,
) -> anyhow::Result<()> {

    queue!(stdout, MoveTo(2, y_start), SetColors(Colors::new(Color::White, Color::Black)), Print(title))?;

    let fgr_names = ["yellow", "orange", "red", "magenta", "violet", "blue", "cyan", "green"];
    let bgr_names = ["base03", "base02", "base01", "base00", "base0", "base1", "base2", "base3"];

    for (y, fgr) in fgr_names.iter().enumerate() {
        queue!(stdout, MoveTo(2, y as u16 + y_start + 2))?;

        for bgr in bgr_names.iter() {
            let fc = Color::AnsiValue(*colors.get(fgr).unwrap());
            let bc = Color::AnsiValue(*colors.get(bgr).unwrap());

            let label = format!("{}/{},{}/{}  ",
                &fgr[0..2], colors[fgr],
                &bgr[4..], colors[bgr]);

            if bold {
                queue!(stdout,
                    SetColors(Colors::new(fc, bc)),
                    crossterm::style::SetAttribute(crossterm::style::Attribute::Bold),
                    Print(&label),
                    crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
                )?;
            } else {
                queue!(stdout, SetColors(Colors::new(fc, bc)), Print(&label))?;
            }
        }
    }

    Ok(())

}

// ------------------------------------------------------------------------

pub fn color_map<'a>() -> HashMap<&'a str, u8> {
    HashMap::from([
        ("base03", 0xe8u8),
        ("base02", 0xec),
        ("base01", 0xef),
        ("base00", 0xf2),
        ("base0", 0xf5),
        ("base1", 0xf8),
        ("base2", 0xfb),
        ("base3", 0xff),
        ("yellow", 226),
        ("orange", 166),
        ("red", 160),
        ("magenta", 127),
        ("violet", 62),
        ("blue", 32),
        ("cyan", 43),
        ("green", 34),
    ])
}
