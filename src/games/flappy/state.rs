use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pipe {
    pub x: f32,
    pub gap_y: u16,
    pub is_scored: bool,
}

#[derive(Debug, Clone)]
pub struct FlappyState {
    pub bird_y: f32,
    pub velocity_y: f32,
    pub pipes: Vec<Pipe>,
    pub score: Score,
    pub level: Level,
    pub tick: u64,
    pub bounds: TerminalSize,
    pub ground_scroll: u16,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl FlappyState {
    pub const BIRD_X: u16 = 12;

    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            bird_y: f32::from(bounds.height) / 2.0,
            velocity_y: 0.0,
            pipes: Vec::with_capacity(6),
            score: Score(0),
            level: Level(1),
            tick: 0,
            bounds,
            ground_scroll: 0,
            is_game_over: false,
            is_complete: false,
        }
    }

    #[must_use]
    pub const fn ground_y(&self) -> u16 {
        self.bounds.height.saturating_sub(2)
    }

    #[must_use]
    pub fn gap_height(&self) -> u16 {
        9u16.saturating_sub(u16::from(self.level.0.min(6))).max(4)
    }

    #[must_use]
    pub fn pipe_speed(&self) -> f32 {
        f32::from(self.level.0).mul_add(0.08, 0.45)
    }
}
