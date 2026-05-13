use crate::engine::ArcadeTerminal;
use crate::types::geometry::{Level, Score};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    Quit,
    Retry { score: Score, level: Level },
    GameOver { score: Score, level: Level },
    Complete { score: Score, level: Level },
}

impl GameResult {
    #[must_use]
    pub const fn score(&self) -> Option<Score> {
        match self {
            Self::Retry { score, .. }
            | Self::GameOver { score, .. }
            | Self::Complete { score, .. } => Some(*score),
            Self::Quit => None,
        }
    }

    #[must_use]
    pub const fn level(&self) -> Option<Level> {
        match self {
            Self::Retry { level, .. }
            | Self::GameOver { level, .. }
            | Self::Complete { level, .. } => Some(*level),
            Self::Quit => None,
        }
    }

    #[must_use]
    pub const fn should_retry(&self) -> bool {
        matches!(self, Self::Retry { .. })
    }
}

pub trait Game {
    #[must_use]
    fn name(&self) -> &str;

    /// Runs the game until the player quits, retries, loses, or completes it.
    ///
    /// # Errors
    ///
    /// Returns an error when terminal I/O fails during rendering or input.
    fn run(&mut self, terminal: &mut ArcadeTerminal) -> anyhow::Result<GameResult>;
}
