use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PongBall {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone)]
pub struct PongState {
    pub player_y: f32,
    pub cpu_y: f32,
    pub ball: PongBall,
    pub player_score: u8,
    pub cpu_score: u8,
    pub score: Score,
    pub level: Level,
    pub tick: u64,
    pub bounds: TerminalSize,
    pub is_two_player: bool,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl PongState {
    pub const PADDLE_HEIGHT: u16 = 5;
    pub const WINNING_SCORE: u8 = 11;

    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let middle_y = f32::from(bounds.height) / 2.0;
        Self {
            player_y: middle_y,
            cpu_y: middle_y,
            ball: PongBall {
                x: f32::from(bounds.width) / 2.0,
                y: middle_y,
                dx: 0.8,
                dy: 0.35,
            },
            player_score: 0,
            cpu_score: 0,
            score: Score(0),
            level: Level(1),
            tick: 0,
            bounds,
            is_two_player: false,
            is_game_over: false,
            is_complete: false,
        }
    }

    pub fn reset_ball(&mut self, direction: f32) {
        self.ball.x = f32::from(self.bounds.width) / 2.0;
        self.ball.y = f32::from(self.bounds.height) / 2.0;
        let speed = f32::from(self.level.0).mul_add(0.08, 0.65);
        self.ball.dx = speed * direction.signum();
        self.ball.dy = if self.tick % 2 == 0 { 0.35 } else { -0.35 };
    }

    #[must_use]
    pub const fn left_paddle_x() -> u16 {
        3
    }

    #[must_use]
    pub const fn right_paddle_x(&self) -> u16 {
        self.bounds.width.saturating_sub(4)
    }
}
