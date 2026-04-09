//! Terminal size detection, viewport math, and screen clearing.

use std::io::{Write, stdout};

use crossterm::{cursor, execute, terminal};

use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

/// Returns the usable game viewport capped at 95% of terminal dimensions.
///
/// # Errors
///
/// Yields `GameError` if the terminal size query fails.
pub fn game_viewport() -> Result<TerminalSize, GameError> {
    let (cols, rows) = terminal::size()?;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(TerminalSize {
        width: (f32::from(cols) * 0.95) as u16,
        height: (f32::from(rows) * 0.95) as u16,
    })
}

/// Clears the entire terminal and moves cursor to the origin.
///
/// Must be called when transitioning between menu and games to prevent
/// stale game content bleeding through the diff-based buffer renderer.
///
/// # Errors
///
/// Yields `GameError` if the stdout write fails.
pub fn clear_screen() -> Result<(), GameError> {
    let mut out = stdout();
    execute!(
        out,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    out.flush()?;
    Ok(())
}
