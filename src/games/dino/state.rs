use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinoObstacleKind {
    SmallCactus,
    LargeCactus,
    DoubleCactus,
    LowBird,
    HighBird,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DinoObstacle {
    pub col: u16,
    pub kind: DinoObstacleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinoStatus {
    Running,
    Jumping,
    Ducking,
    GameOver,
    Complete,
}

impl DinoStatus {
    #[must_use]
    pub const fn is_game_over(self) -> bool {
        matches!(self, Self::GameOver)
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    #[must_use]
    pub const fn is_jumping(self) -> bool {
        matches!(self, Self::Jumping)
    }

    #[must_use]
    pub const fn is_ducking(self) -> bool {
        matches!(self, Self::Ducking)
    }
}

#[derive(Debug, Clone)]
pub struct DinoState {
    pub dino_row: u16,
    pub status: DinoStatus,
    pub jump_velocity: i16,
    pub obstacles: Vec<DinoObstacle>,
    pub score: Score,
    pub high_score: u32,
    pub level: Level,
    pub speed: u16,
    pub tick: u64,
    pub hurt_ticks: u16,
    pub bounds: TerminalSize,
    pub ground_scroll: u16,
}

impl DinoState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            dino_row: Self::stand_row(bounds),
            status: DinoStatus::Running,
            jump_velocity: 0,
            obstacles: Vec::with_capacity(8),
            score: Score(0),
            high_score: 0,
            level: Level(1),
            speed: 1,
            tick: 0,
            hurt_ticks: 0,
            bounds,
            ground_scroll: 0,
        }
    }

    #[must_use]
    pub const fn ground_line(bounds: TerminalSize) -> u16 {
        bounds.height.saturating_sub(3)
    }

    #[must_use]
    pub const fn stand_row(bounds: TerminalSize) -> u16 {
        Self::ground_line(bounds).saturating_sub(2)
    }
}
