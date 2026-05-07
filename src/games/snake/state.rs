use std::collections::VecDeque;
use std::time::Duration;

use crate::types::geometry::{Direction, Level, Score, TerminalSize, Vec2};

#[derive(Debug, Clone)]
pub struct SnakeState {
    pub segments: VecDeque<Vec2>,
    pub direction: Direction,
    pub input_queue: VecDeque<Direction>,
    pub food: Vec2,
    pub score: Score,
    pub level: Level,
    pub tick_rate: Duration,
    pub tick_accumulator: Duration,
    pub bounds: TerminalSize,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl SnakeState {
    pub const HUD_HEIGHT: u16 = 2;

    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let mut state = Self {
            segments: VecDeque::new(),
            direction: Direction::Right,
            input_queue: VecDeque::new(),
            food: Vec2::new(0, 0),
            score: Score(0),
            level: Level(1),
            tick_rate: Duration::from_millis(120),
            tick_accumulator: Duration::ZERO,
            bounds,
            is_game_over: false,
            is_complete: false,
        };
        state.reset_snake();
        state
    }

    pub fn reset_snake(&mut self) {
        let cx = i32::from(self.bounds.width) / 2;
        let cy = i32::from(self.bounds.height) / 2;

        self.segments.clear();
        self.segments.push_back(Vec2::new(cx, cy));
        self.segments.push_back(Vec2::new(cx - 1, cy));
        self.segments.push_back(Vec2::new(cx - 2, cy));

        self.direction = Direction::Right;
        self.input_queue.clear();
        self.tick_accumulator = Duration::ZERO;

        self.food = Vec2::new(cx + 8, cy);
    }

    #[must_use]
    pub const fn play_area_top() -> i32 {
        Self::HUD_HEIGHT as i32
    }

    #[must_use]
    pub const fn play_area_bottom(&self) -> i32 {
        self.bounds.height as i32 - 1
    }

    #[must_use]
    pub const fn play_area_right(&self) -> i32 {
        self.bounds.width as i32 - 1
    }
}
