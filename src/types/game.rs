use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    Quit,
    GameOver { score: Score, level: Level },
    Complete { score: Score, level: Level },
}

pub trait Game {
    #[must_use]
    fn name(&self) -> &str;

    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult>;
}
