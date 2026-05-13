use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinoObstacleKind {
    SmallCactus,
    LargeCactus,
    CactusCluster,
    LowBird,
    HighBird,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DinoObstacle {
    pub col: f32,
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
    pub dino_y: f32,
    pub status: DinoStatus,
    pub velocity_y: f32,
    pub obstacles: Vec<DinoObstacle>,
    pub score: Score,
    pub high_score: u32,
    pub level: Level,
    pub speed: f32,
    pub tick: u64,
    pub bounds: TerminalSize,
    pub ground_scroll: f32,
    pub score_progress: f32,
    pub spawn_cooldown: u16,
}

impl DinoState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            dino_y: Self::stand_y(bounds),
            status: DinoStatus::Running,
            velocity_y: 0.0,
            obstacles: Vec::with_capacity(8),
            score: Score(0),
            high_score: 0,
            level: Level(1),
            speed: 6.0,
            tick: 0,
            bounds,
            ground_scroll: 0.0,
            score_progress: 0.0,
            spawn_cooldown: 40,
        }
    }

    #[must_use]
    pub const fn ground_line(bounds: TerminalSize) -> u16 {
        bounds.height.saturating_sub(3)
    }

    #[must_use]
    pub fn stand_y(bounds: TerminalSize) -> f32 {
        f32::from(Self::ground_line(bounds).saturating_sub(2))
    }

    #[must_use]
    pub fn dino_row(&self) -> u16 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let row = self.dino_y.round().max(0.0) as u16;
        row
    }
}
