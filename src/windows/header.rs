//!
//! Standard header window
//!

use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Text},
    widgets::Paragraph,
};

use crate::color::WindowColors;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// ------------------------------------------------------------------------

type LineFn<'a> = Box<dyn Fn(usize) -> (i32, String) + 'a>;

// ------------------------------------------------------------------------

pub struct Header<'a> {
    window_colors: &'a WindowColors,
    line2_fn: LineFn<'a>,
}

impl Header<'_> {

    /// Create a header using `window_colors`, building the second
    /// line using `line2_fn`

    pub fn new<'a>(
        window_colors: &'a WindowColors,
        line2_fn: LineFn<'a>,
    ) -> Header<'a> {
        Header { window_colors, line2_fn }
    }

    // --------------------------------------------------------------------

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let width = area.width as usize;

        let title = make_title(
            &format!("{} v{}", NAME, VERSION),
            "",
            "Use the arrow keys to navigate, q to go back",
            width,
        ).unwrap_or_default();

        let (x2, line2_str) = (self.line2_fn)(width);
        let x2 = x2.max(0) as usize;
        let line2_padded = format!("{}{}", " ".repeat(x2), line2_str);

        let lines = vec![
            Line::styled(title, self.window_colors.title),
            Line::styled(line2_padded, self.window_colors.title),
        ];

        let paragraph = Paragraph::new(Text::from(lines))
            .style(self.window_colors.bkgr);

        f.render_widget(paragraph, area);
    }

}

// ------------------------------------------------------------------------
/// Create the title string

fn make_title(left: &str, middle: &str, right: &str, cols: usize) -> Result<String> {

    let gutter_size = isize::try_from(cols)?
        - isize::try_from(left.len() + middle.len() + right.len())?;

    let title = if gutter_size < 2 {
        String::from(&(left.to_owned() + " " + middle + " " + right)[..cols])
    } else {
        let lgutter = gutter_size / 2;
        let rgutter = gutter_size / 2 + if gutter_size - lgutter * 2 > 0 { 1 } else { 0 };

        left.to_owned()
            + &" ".repeat(usize::try_from(lgutter)?)
            + middle
            + &" ".repeat(usize::try_from(rgutter)?)
            + right
    };

    Ok(title)
}

