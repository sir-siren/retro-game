use crate::types::error::GameError;
use crate::types::geometry::TerminalSize;

pub fn terminal_size() -> Result<TerminalSize, GameError> {
    let (width, height) = crossterm::terminal::size()?;
    Ok(TerminalSize { width, height })
}
