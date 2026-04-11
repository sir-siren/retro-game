use std::io::Write;

use crossterm::{cursor, queue, style::Print};

use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

#[derive(Debug)]
pub struct Buffer {
    width: u16,
    height: u16,
    cells: Vec<char>,
    prev: Vec<char>,
}

impl Buffer {
    #[must_use]
    pub fn new(size: TerminalSize) -> Self {
        let area = usize::from(size.width) * usize::from(size.height);
        Self {
            width: size.width,
            height: size.height,
            cells: vec![' '; area],
            prev: vec![' '; area],
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
            self.place(curr_x, y, c);
            curr_x = curr_x.saturating_add(1);
        }
    }

    pub fn print_right(&mut self, y: u16, text: &str, margin: u16) {
        let len = text.len() as u16;
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

    pub fn flush<W: Write>(
        &mut self,
        term_width: u16,
        term_height: u16,
        out: &mut W,
    ) -> Result<(), GameError> {
        let offset_x = term_width.saturating_sub(self.width) / 2;
        let offset_y = term_height.saturating_sub(self.height) / 2;

        let mut current_offset: Option<(u16, u16)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
                let current = self.cells[idx];

                if current != self.prev[idx] {
                    let target_x = offset_x + x;
                    let target_y = offset_y + y;

                    if current_offset != Some((target_x, target_y)) {
                        queue!(out, cursor::MoveTo(target_x, target_y))?;
                    }
                    queue!(out, Print(current))?;
                    current_offset = Some((target_x + 1, target_y));

                    self.prev[idx] = current;
                }
            }
        }

        out.flush()?;
        Ok(())
    }
}
