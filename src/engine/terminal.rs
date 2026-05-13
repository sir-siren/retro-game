use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

/// Reads the current terminal dimensions.
///
/// # Errors
///
/// Returns an error when crossterm cannot query the active terminal.
pub fn terminal_size() -> Result<TerminalSize, GameError> {
    let (width, height) = crossterm::terminal::size()?;
    Ok(TerminalSize { width, height })
}
