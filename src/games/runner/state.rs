use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrafficCar {
    pub lane: u8,
    pub col: u16,
    pub width: u16,
}

#[derive(Debug, Clone)]
pub struct RunnerState {
    pub player_lane: u8,
    pub speed: u16,
    pub obstacles: Vec<TrafficCar>,
    pub score: Score,
    pub level: Level,
    pub tick: u64,
    pub is_game_over: bool,
    pub is_complete: bool,
    pub bounds: TerminalSize,
}

impl RunnerState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            player_lane: 1,
            speed: 60,
            obstacles: Vec::with_capacity(16),
            score: Score(0),
            level: Level(1),
            tick: 0,
            is_game_over: false,
            is_complete: false,
            bounds,
        }
    }

    #[must_use]
    pub fn lane_count() -> u8 {
        4
    }

    #[must_use]
    pub fn hud_height() -> u16 {
        2
    }

    #[must_use]
    pub fn lane_y(&self, lane: u8) -> u16 {
        let playfield_height: u16 = self.bounds.height.saturating_sub(Self::hud_height() + 1);
        let lane_height: u16 = playfield_height / u16::from(Self::lane_count());
        Self::hud_height() + u16::from(lane) * lane_height + lane_height / 2
    }

    #[must_use]
    pub fn lane_divider_y(&self, divider_index: u8) -> u16 {
        let playfield_height: u16 = self.bounds.height.saturating_sub(Self::hud_height() + 1);
        let lane_height: u16 = playfield_height / u16::from(Self::lane_count());
        Self::hud_height() + u16::from(divider_index + 1) * lane_height
    }

    #[must_use]
    pub fn player_col(&self) -> u16 {
        6
    }
}
