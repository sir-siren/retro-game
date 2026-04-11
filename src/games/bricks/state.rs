use crate::types::geometry::{Level, Lives, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallPhysics {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    pub col: u16,
    pub row: u16,
    pub hp: u8,
    pub is_alive: bool,
}

#[derive(Debug, Clone)]
pub struct BricksState {
    pub paddle_col: u16,
    pub paddle_width: u16,
    pub ball: BallPhysics,
    pub bricks: Vec<Brick>,
    pub score: Score,
    pub level: Level,
    pub lives: Lives,
    pub bounds: TerminalSize,
    pub is_game_over: bool,
    pub transition_ticks: u16,
    pub showing_clear: bool,
    pub is_complete: bool,
}

impl BricksState {
    pub const BRICK_WIDTH: u16 = 4;
    pub const BRICK_SPACING: u16 = 5;
    pub const HUD_HEIGHT: u16 = 2;
    pub const PADDLE_DEFAULT_WIDTH: u16 = 7;

    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let mut state = Self {
            paddle_col: bounds.width.saturating_sub(Self::PADDLE_DEFAULT_WIDTH) / 2,
            paddle_width: Self::PADDLE_DEFAULT_WIDTH,
            ball: BallPhysics {
                x: 0.0,
                y: 0.0,
                dx: 0.0,
                dy: 0.0,
            },
            bricks: Vec::new(),
            score: Score(0),
            level: Level(1),
            lives: Lives(3),
            bounds,
            is_game_over: false,
            transition_ticks: 0,
            showing_clear: false,
            is_complete: false,
        };
        state.reset_ball();
        state.spawn_level_bricks();
        state
    }

    pub fn reset_ball(&mut self) {
        let paddle_center = f32::from(self.paddle_col) + f32::from(self.paddle_width) / 2.0;
        self.ball.x = paddle_center;
        self.ball.y = f32::from(self.bounds.height.saturating_sub(4));

        let speed_mult = 1.0 + (f32::from(self.level.0) - 1.0) * 0.15;
        self.ball.dx = 0.6 * speed_mult;
        self.ball.dy = -0.6 * speed_mult;
    }

    pub fn spawn_level_bricks(&mut self) {
        self.bricks.clear();
        let rows = 3 + u16::from(self.level.0);
        let cols = self.bounds.width / Self::BRICK_SPACING;
        let offset_x = (self.bounds.width.saturating_sub(cols * Self::BRICK_SPACING)) / 2;

        for r in 0..rows {
            for c in 0..cols {
                let top_y = Self::HUD_HEIGHT + 1 + r;
                let left_x = offset_x + c * Self::BRICK_SPACING;

                let hp = if self.level.0 >= 3 && r == 0 { 2 } else { 1 };

                self.bricks.push(Brick {
                    col: left_x,
                    row: top_y,
                    hp,
                    is_alive: true,
                });
            }
        }
    }

    #[must_use]
    pub fn paddle_row(&self) -> u16 {
        self.bounds.height.saturating_sub(3)
    }
}
