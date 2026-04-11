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

#[derive(Debug, Clone)]
pub struct DinoState {
    pub dino_row: u16,
    pub is_jumping: bool,
    pub is_ducking: bool,
    pub jump_velocity: i16,
    pub obstacles: Vec<DinoObstacle>,
    pub score: Score,
    pub high_score: u32,
    pub level: Level,
    pub speed: u16,
    pub tick: u64,
    pub hurt_ticks: u16,
    pub is_game_over: bool,
    pub is_complete: bool,
    pub bounds: TerminalSize,
    pub ground_scroll: u16,
}

impl DinoState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            dino_row: Self::stand_row(bounds),
            is_jumping: false,
            is_ducking: false,
            jump_velocity: 0,
            obstacles: Vec::with_capacity(8),
            score: Score(0),
            high_score: 0,
            level: Level(1),
            speed: 1,
            tick: 0,
            hurt_ticks: 0,
            is_game_over: false,
            is_complete: false,
            bounds,
            ground_scroll: 0,
        }
    }

    #[must_use]
    pub fn ground_line(bounds: TerminalSize) -> u16 {
        bounds.height.saturating_sub(3)
    }

    #[must_use]
    pub fn stand_row(bounds: TerminalSize) -> u16 {
        Self::ground_line(bounds).saturating_sub(2)
    }
}
