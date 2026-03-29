//!
//! Scrollable region of the screen
//!

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::color::WindowColors;

use super::{line, Coords};

// ------------------------------------------------------------------------

pub struct ScrollableRegion<'sr> {

    /// Set of lines to display
    lines: line::LineVec<'sr>,

    /// Index into lines of the top line in the window
    top_idx: usize,

    /// Index into the visible window of the currently selected line
    win_idx: usize,

    /// Cached dimensions from last render
    size: Coords,

    /// Colors to use for this scrollable region
    window_colors: &'sr WindowColors,

}

impl<'sr> ScrollableRegion<'sr> {

    pub fn new(
        window_colors: &'sr WindowColors,
        lines: line::LineVec<'sr>,
    ) -> ScrollableRegion<'sr> {
        ScrollableRegion {
            lines,
            size: Coords { y: 0, x: 0 },
            top_idx: 0,
            win_idx: 0,
            window_colors,
        }
    }

    // --------------------------------------------------------------------

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let height = area.height as usize;
        let width = area.width as usize;
        self.size = Coords { y: height, x: width };

        if height == 0 || width == 0 {
            return;
        }

        // Clamp selection to visible area
        if !self.lines.is_empty() && self.win_idx >= height {
            self.win_idx = height - 1;
        }

        let total = self.lines.len();
        let lim = (self.top_idx + height).min(total);
        let visible_count = lim.saturating_sub(self.top_idx);

        let can_scroll_up = self.top_idx > 0;
        let can_scroll_down = self.top_idx + height < total;

        let mut ratatui_lines: Vec<Line<'static>> = Vec::new();

        for (win_pos, line_idx) in (self.top_idx..lim).enumerate() {
            let line = &self.lines[line_idx];

            // Reserve 2 chars: 1 for action indicator, 1 for scroll indicator
            let content_width = width.saturating_sub(2);
            let pairs = line.as_pairs(content_width).unwrap_or_default();

            // Action indicator character
            let indicator = match line.action_type() {
                None => ' ',
                Some(line::ActionType::NewWindow(_)) => '=',
                Some(line::ActionType::Expandable(_, n, _)) => {
                    if *n > 0 { '-' } else { '+' }
                }
            };

            // Build spans
            let mut spans: Vec<Span<'static>> = Vec::new();
            spans.push(Span::styled(
                indicator.to_string(),
                self.window_colors.text,
            ));

            let mut content_len = 0usize;
            for (style_opt, text) in &pairs {
                let style = style_opt.unwrap_or(self.window_colors.text);
                content_len += text.len();
                spans.push(Span::styled(text.clone(), style));
            }

            // Padding to fill the row before the scroll indicator column
            let pad_len = content_width.saturating_sub(content_len);
            if pad_len > 0 {
                spans.push(Span::styled(
                    " ".repeat(pad_len),
                    self.window_colors.bkgr,
                ));
            }

            // Scroll indicator at last column
            let scroll_char = if win_pos == 0 && can_scroll_up {
                "\u{21d1}" // ⇑
            } else if win_pos == visible_count.saturating_sub(1) && can_scroll_down {
                "\u{21d3}" // ⇓
            } else {
                " "
            };
            spans.push(Span::styled(
                scroll_char.to_string(),
                self.window_colors.text,
            ));

            // Apply REVERSED modifier to all spans of the selected line
            let line_widget = if win_pos == self.win_idx {
                let reversed: Vec<Span<'static>> = spans
                    .into_iter()
                    .map(|s| {
                        Span::styled(
                            s.content.into_owned(),
                            s.style.add_modifier(Modifier::REVERSED),
                        )
                    })
                    .collect();
                Line::from(reversed)
            } else {
                Line::from(spans)
            };

            ratatui_lines.push(line_widget);
        }

        // Pad remaining rows with background-colored blank lines
        while ratatui_lines.len() < height {
            ratatui_lines.push(Line::styled(
                " ".repeat(width),
                self.window_colors.bkgr,
            ));
        }

        let paragraph = Paragraph::new(ratatui_lines).style(self.window_colors.bkgr);
        f.render_widget(paragraph, area);
    }

    // --------------------------------------------------------------------

    pub fn handle_key(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Down => self.key_down_handler()?,
            KeyCode::Up => self.key_up_handler()?,
            KeyCode::PageDown => self.key_pgdown_handler()?,
            KeyCode::PageUp => self.key_pgup_handler()?,
            KeyCode::Home => self.key_home_handler()?,
            KeyCode::End => self.key_end_handler()?,
            _ => {}
        }
        Ok(())
    }

    // --------------------------------------------------------------------

    pub fn key_enter_handler(&mut self) -> Result<()> {
        let idx = self.top_idx + self.win_idx;
        if idx >= self.lines.len() {
            return Ok(());
        }
        let line = &mut self.lines[idx];

        if let Some(at) = line.action_type_mut() {
            match at {
                line::ActionType::NewWindow(nwf) => nwf()?,
                line::ActionType::Expandable(_, ref mut num_lines, _) => {
                    *num_lines = 0;
                }
            }
        }

        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_down_handler(&mut self) -> Result<()> {
        let total = self.lines.len();
        if total == 0 { return Ok(()); }

        let abs_idx = self.top_idx + self.win_idx;
        if abs_idx >= total - 1 { return Ok(()); }

        if self.win_idx < self.size.y - 1 {
            self.win_idx += 1;
        } else {
            self.top_idx += 1;
        }
        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_up_handler(&mut self) -> Result<()> {
        if self.win_idx > 0 {
            self.win_idx -= 1;
        } else if self.top_idx > 0 {
            self.top_idx -= 1;
        }
        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_pgdown_handler(&mut self) -> Result<()> {
        let total = self.lines.len();
        let height = self.size.y;
        if height == 0 || total == 0 { return Ok(()); }

        if self.top_idx + height < total {
            // Advance viewport by one page
            self.top_idx = (self.top_idx + height).min(total.saturating_sub(height));
        } else {
            // Already on last page: move selection to last visible item
            self.win_idx = (total - self.top_idx - 1).min(height - 1);
        }
        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_pgup_handler(&mut self) -> Result<()> {
        let height = self.size.y;
        if height == 0 { return Ok(()); }

        self.top_idx = self.top_idx.saturating_sub(height);
        self.win_idx = 0;
        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_home_handler(&mut self) -> Result<()> {
        self.top_idx = 0;
        self.win_idx = 0;
        Ok(())
    }

    // --------------------------------------------------------------------

    fn key_end_handler(&mut self) -> Result<()> {
        let total = self.lines.len();
        let height = self.size.y;
        if height == 0 || total == 0 { return Ok(()); }

        if total > height {
            self.top_idx = total - height;
            self.win_idx = height - 1;
        } else {
            self.top_idx = 0;
            self.win_idx = total - 1;
        }
        Ok(())
    }

}
