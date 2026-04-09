use std::io::Write;
use crossterm::{cursor, queue, style::Print};
use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

/// A fixed-size character grid written atomically to stdout each frame.
#[derive(Debug)]
pub struct Buffer {
    width:  u16,
    height: u16,
    cells:  Vec<char>,
    prev:   Vec<char>,
}

impl Buffer {
    /// Creates a new, blank double-buffered render plane.
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

    /// Clears the foreground buffer to spaces.
    pub fn clear(&mut self) {
        self.cells.fill(' ');
    }

    /// Sets a specific character at the given coordinates if they fall within bounds.
    pub fn place(&mut self, x: u16, y: u16, c: char) {
        if x < self.width && y < self.height {
            let idx = usize::from(y) * usize::from(self.width) + usize::from(x);
            self.cells[idx] = c;
        }
    }

    /// Write a string beginning at the specified coordinates.
    pub fn print(&mut self, x: u16, y: u16, text: &str) {
        let mut curr_x = x;
        for c in text.chars() {
            self.place(curr_x, y, c);
            curr_x = curr_x.saturating_add(1);
        }
    }

    /// Emits the differences between the new frame and previous frame to standard output.
    ///
    /// # Errors
    ///
    /// Yields `GameError::Io` if stdout queuing or flushing fails.
    pub fn flush<W: Write>(&mut self, term_width: u16, term_height: u16, out: &mut W) -> Result<(), GameError> {
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
