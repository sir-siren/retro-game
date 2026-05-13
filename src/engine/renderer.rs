use ratatui::buffer::Buffer as RatBuffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;

use crate::types::geometry::TerminalSize;

#[derive(Clone, Copy)]
struct StyledCell {
    ch: char,
    style: Style,
}

impl Default for StyledCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: Style::default(),
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    // stored as a flat vec for cache locality -- row-major order
    cells: Vec<StyledCell>,
}

// Debug requires StyledCell to impl Debug, but we only need it on Buffer
// for the GameLoop trait bound. the manual impl sidesteps that.
impl std::fmt::Debug for StyledCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.ch)
    }
}

impl Buffer {
    #[must_use]
    pub fn new(size: TerminalSize) -> Self {
        let area = usize::from(size.width) * usize::from(size.height);
        Self {
            width: size.width,
            height: size.height,
            cells: vec![StyledCell::default(); area],
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.ch = ' ';
            cell.style = Style::default();
        }
    }

    /// Place a single styled character at (x, y).
    pub fn place(&mut self, x: u16, y: u16, ch: char, style: Style) {
        if x < self.width && y < self.height {
            let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
            self.cells[idx] = StyledCell { ch, style };
        }
    }

    /// Place a character with default (no color) styling -- backward compat.
    pub fn place_plain(&mut self, x: u16, y: u16, ch: char) {
        self.place(x, y, ch, Style::default());
    }

    /// Print a string starting at (x, y) with the given style.
    pub fn print(&mut self, x: u16, y: u16, text: &str, style: Style) {
        let mut curr_x = x;
        for ch in text.chars() {
            if curr_x >= self.width {
                break;
            }
            self.place(curr_x, y, ch, style);
            curr_x = curr_x.saturating_add(1);
        }
    }

    /// Print with default styling -- backward compat helper.
    pub fn print_plain(&mut self, x: u16, y: u16, text: &str) {
        self.print(x, y, text, Style::default());
    }

    pub fn print_styled(&mut self, x: u16, y: u16, spans: &[Span<'_>]) {
        let mut curr_x = x;
        for span in spans {
            for ch in span.content.chars() {
                if curr_x >= self.width {
                    return;
                }
                self.place(curr_x, y, ch, span.style);
                curr_x = curr_x.saturating_add(1);
            }
        }
    }

    /// Right-aligned print with margin.
    pub fn print_right(&mut self, y: u16, text: &str, margin: u16, style: Style) {
        #[allow(clippy::cast_possible_truncation)]
        let len = text.chars().count() as u16;
        let x = self.width.saturating_sub(len).saturating_sub(margin);
        self.print(x, y, text, style);
    }

    /// Right-aligned print with default style.
    pub fn print_right_plain(&mut self, y: u16, text: &str, margin: u16) {
        self.print_right(y, text, margin, Style::default());
    }

    /// Solid horizontal line.
    pub fn horizontal_line(&mut self, y: u16, x_start: u16, x_end: u16, ch: char, style: Style) {
        for x in x_start..x_end.min(self.width) {
            self.place(x, y, ch, style);
        }
    }

    /// Horizontal line with default style.
    pub fn horizontal_line_plain(&mut self, y: u16, x_start: u16, x_end: u16, ch: char) {
        self.horizontal_line(y, x_start, x_end, ch, Style::default());
    }

    /// Dashed line pattern -- 2 on, 2 off.
    pub fn dashed_line(&mut self, y: u16, x_start: u16, x_end: u16, style: Style) {
        for x in x_start..x_end.min(self.width) {
            if x % 4 < 2 {
                self.place(x, y, '\u{2500}', style);
            }
        }
    }

    /// Blit our buffer into the ratatui frame buffer, centered.
    pub fn render_to(&self, rat_buf: &mut RatBuffer, area: Rect) {
        let offset_x = area.x + area.width.saturating_sub(self.width) / 2;
        let offset_y = area.y + area.height.saturating_sub(self.height) / 2;

        for y in 0..self.height.min(area.height) {
            for x in 0..self.width.min(area.width) {
                let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
                let cell = &self.cells[idx];
                let tx = offset_x + x;
                let ty = offset_y + y;
                if tx < area.x + area.width && ty < area.y + area.height {
                    let rat_cell = &mut rat_buf[(tx, ty)];
                    rat_cell.set_char(cell.ch);
                    rat_cell.set_style(cell.style);
                }
            }
        }
    }
}
