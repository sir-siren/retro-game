use crate::types::geometry::{Level, Lives, Score, TerminalSize};

/// Physical translation vector and floating positional state for sub-cell increments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallPhysics {
    /// Exact column sub-pixel.
    pub x: f32,
    /// Exact row sub-pixel.
    pub y: f32,
    /// Per-tick X translation.
    pub dx: f32,
    /// Per-tick Y translation.
    pub dy: f32,
}

/// Target block state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    /// Top-left column index.
    pub col: u16,
    /// Top-left row index.
    pub row: u16,
    /// Number of hits remaining before destruction.
    pub hp: u8,
    /// Whether it still participates in collisions and rendering.
    pub is_alive: bool,
}

/// Pure domain state for Bricks match.
#[derive(Debug, Clone)]
pub struct BricksState {
    /// Paddle anchor point.
    pub paddle_col: u16,
    /// Float-based moving projectile.
    pub ball: BallPhysics,
    /// Matrix of remaining targets.
    pub bricks: Vec<Brick>,
    /// Accumulated total hits.
    pub score: Score,
    /// Progression constraint stage.
    pub level: Level,
    /// Available retries.
    pub lives: Lives,
    /// Viewport limits locking ball and paddle movement.
    pub bounds: TerminalSize,
    /// True when player has no lives.
    pub is_game_over: bool,
    /// Wait delay before transitioning to the next active state.
    pub transition_ticks: u16,
    /// A temporary flag signaling a level was completed, allowing rendering of the clear message.
    pub showing_clear: bool,
    /// Exit requested flag.
    pub is_complete: bool,
}

impl BricksState {
    /// Builds initial layout centered.
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let mut state = Self {
            paddle_col: bounds.width.saturating_sub(5) / 2,
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

    /// Snaps ball to the starting vertical alignment just over paddle.
    pub fn reset_ball(&mut self) {
        let paddle_center = f32::from(self.paddle_col) + 2.0; // 5 wide
        self.ball.x = paddle_center;
        self.ball.y = f32::from(self.bounds.height.saturating_sub(3)); // Just above paddle
        
        let speed_mult = 1.0 + (f32::from(self.level.0) - 1.0) * 0.1;
        self.ball.dx = 0.5 * speed_mult;
        self.ball.dy = -0.5 * speed_mult;
    }

    /// Bootstraps grid matching target level width rules.
    pub fn spawn_level_bricks(&mut self) {
        self.bricks.clear();
        let rows = 4 + u16::from(self.level.0.saturating_sub(1));
        
        // Bricks are 4 wide [##]. Let's space them 5 wide.
        let brick_outer_width = 5;
        let cols = self.bounds.width / brick_outer_width;
        
        let offset_x = (self.bounds.width.saturating_sub(cols * brick_outer_width)) / 2;
        
        for r in 0..rows {
            for c in 0..cols {
                let top_y = 2 + r * 2;
                let left_x = offset_x + c * brick_outer_width;
                
                let mut hp = 1;
                // Add armored bricks starting level 3. Row check just makes it structured.
                if self.level.0 >= 3 && r % 2 == 0 {
                    hp = 2;
                }
                
                self.bricks.push(Brick {
                    col: left_x,
                    row: top_y,
                    hp,
                    is_alive: true,
                });
            }
        }
    }
}
