use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Game execution failed: {0}")]
    Game(#[from] GameError),
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("I/O error during game: {0}")]
    Io(#[from] io::Error),
    #[error("Terminal is too small")]
    TerminalTooSmall,
}
