use crossterm::terminal::size;
use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

/// Returns the usable game viewport, capped at 95% of terminal dimensions.
///
/// # Errors
///
/// Yields `GameError` if standard output size query fails.
pub fn game_viewport() -> Result<TerminalSize, GameError> {
    let (cols, rows) = size()?;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(TerminalSize {
        width: (f32::from(cols) * 0.95) as u16,
        height: (f32::from(rows) * 0.95) as u16,
    })
}
