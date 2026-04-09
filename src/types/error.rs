use std::io;
use thiserror::Error;

/// Application errors encompassing terminal and generic execution bounds.
#[derive(Debug, Error)]
pub enum AppError {
    /// Any I/O error during terminal rendering or configuration.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    /// A localized game error mapping up to App scope.
    #[error("Game execution failed: {0}")]
    Game(#[from] GameError),
}

/// Errors specific to game logic, state, and internal limits.
#[derive(Debug, Error)]
pub enum GameError {
    /// Terminal wrapper for game-time failures.
    #[error("I/O error during game: {0}")]
    Io(#[from] io::Error),
    /// Triggered when window size drops below game playability.
    #[error("Terminal is too small")]
    TerminalTooSmall,
}
