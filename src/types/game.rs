use crate::types::geometry::{Level, Score, TerminalSize};

/// How a game session ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    /// The user explicitly forced an exit before finishing.
    Quit,
    /// The user depleted their lives.
    GameOver {
        /// Points gathered before losing.
        score: Score,
        /// Maximum level attained.
        level: Level,
    },
    /// The user successfully survived all levels.
    Complete {
        /// Final points sum.
        score: Score,
        /// Maximum level attained (should generally be max).
        level: Level,
    },
}

/// Core interface each arcade module must fulfill to be playable.
pub trait Game {
    /// The human-readable title shown on menus.
    #[must_use]
    fn name(&self) -> &str;

    /// Drive the game logic, blocking until GameResult determines an exit condition.
    ///
    /// # Errors
    ///
    /// Yields `anyhow::Error` when rendering bindings fail fatally.
    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult>;
}
