//!
//! Footer window
//!

use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::Paragraph,
};

use crate::color::WindowColors;

// ------------------------------------------------------------------------

pub type LineFn<'a> = Box<dyn Fn(usize) -> (i32, String) + 'a>;

pub struct Footer<'a> {
    window_colors: &'a WindowColors,
    line_fn: LineFn<'a>,
}

impl Footer<'_> {

    pub fn new<'a>(
        window_colors: &'a WindowColors,
        line_fn: LineFn<'a>,
    ) -> Footer<'a> {
        Footer { window_colors, line_fn }
    }

    // --------------------------------------------------------------------

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let width = area.width as usize;
        let (x, line_str) = (self.line_fn)(width);
        let x = x.max(0) as usize;
        let line_padded = format!("{}{}", " ".repeat(x), line_str);

        let paragraph = Paragraph::new(Line::styled(line_padded, self.window_colors.title))
            .style(self.window_colors.bkgr);

        f.render_widget(paragraph, area);
    }

}
