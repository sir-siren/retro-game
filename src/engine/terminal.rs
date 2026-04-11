use std::io::{Write, stdout};

use crossterm::{cursor, execute, terminal};

use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

pub fn game_viewport() -> Result<TerminalSize, GameError> {
    let (cols, rows) = terminal::size()?;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(TerminalSize {
        width: (f32::from(cols) * 0.95) as u16,
        height: (f32::from(rows) * 0.95) as u16,
    })
}

pub fn clear_screen() -> Result<(), GameError> {
    let mut out: std::io::Stdout = stdout();
    execute!(
        out,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    out.flush()?;
    Ok(())
}
