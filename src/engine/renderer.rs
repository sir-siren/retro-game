use ratatui::{buffer::Buffer as RatBuffer, layout::Rect};

use crate::types::geometry::TerminalSize;

#[derive(Debug)]
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    cells: Vec<char>,
}

impl Buffer {
    #[must_use]
    pub fn new(size: TerminalSize) -> Self {
        let area = usize::from(size.width) * usize::from(size.height);
        Self {
            width: size.width,
            height: size.height,
            cells: vec![' '; area],
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(' ');
    }

    pub fn place(&mut self, x: u16, y: u16, c: char) {
        if x < self.width && y < self.height {
            let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
            self.cells[idx] = c;
        }
    }

    pub fn print(&mut self, x: u16, y: u16, text: &str) {
        let mut curr_x = x;
        for c in text.chars() {
            if curr_x >= self.width {
                break;
            }
            self.place(curr_x, y, c);
            curr_x = curr_x.saturating_add(1);
        }
    }

    pub fn print_right(&mut self, y: u16, text: &str, margin: u16) {
        #[allow(clippy::cast_possible_truncation)]
        let len = text.chars().count() as u16;
        let x = self.width.saturating_sub(len).saturating_sub(margin);
        self.print(x, y, text);
    }

    pub fn horizontal_line(&mut self, y: u16, x_start: u16, x_end: u16, ch: char) {
        for x in x_start..x_end.min(self.width) {
            self.place(x, y, ch);
        }
    }

    pub fn dashed_line(&mut self, y: u16, x_start: u16, x_end: u16) {
        for x in x_start..x_end.min(self.width) {
            if x % 4 < 2 {
                self.place(x, y, '─');
            }
        }
    }

    pub fn render_to(&self, rat_buf: &mut RatBuffer, area: Rect) {
        let offset_x = area.x + area.width.saturating_sub(self.width) / 2;
        let offset_y = area.y + area.height.saturating_sub(self.height) / 2;

        for y in 0..self.height.min(area.height) {
            for x in 0..self.width.min(area.width) {
                let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
                let c = self.cells[idx];
                let tx = offset_x + x;
                let ty = offset_y + y;
                if tx < area.x + area.width && ty < area.y + area.height {
                    rat_buf[(tx, ty)].set_char(c);
                }
            }
        }
    }
}
