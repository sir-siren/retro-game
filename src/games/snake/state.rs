use std::collections::VecDeque;
use std::time::Duration;
use crate::types::geometry::{Direction, Level, Lives, Score, TerminalSize, Vec2};

/// Encapsulates all data required to simulate the Snake domain.
#[derive(Debug, Clone)]
pub struct SnakeState {
    /// Ordered points of the player, index 0 is head.
    pub segments: VecDeque<Vec2>,
    /// Movement orientation.
    pub direction: Direction,
    /// Ordered action queue.
    pub input_queue: VecDeque<Direction>,
    /// Exact cell location of current objective.
    pub food: Vec2,
    /// Static collision geometry injected via progression.
    pub walls: Vec<Vec2>,
    /// Total points.
    pub score: Score,
    /// Game difficulty.
    pub level: Level,
    /// Attempts.
    pub lives: Lives,
    /// Delay between physical steps.
    pub tick_rate: Duration,
    /// Accumulated ticks since last move step.
    pub tick_accumulator: Duration,
    /// Screen constraints.
    pub bounds: TerminalSize,
    /// True on game death.
    pub is_game_over: bool,
    /// Frame delay for clear messages.
    pub transition_ticks: u16,
    /// Signaler when reaching score threshold.
    pub showing_clear: bool,
    /// Overall lifecycle exit request.
    pub is_complete: bool,
}

impl SnakeState {
    /// Builds layout and centers entity.
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let mut state = Self {
            segments: VecDeque::new(),
            direction: Direction::Up,
            input_queue: VecDeque::new(),
            food: Vec2::new(0, 0),
            walls: Vec::new(),
            score: Score(0),
            level: Level(1),
            lives: Lives(3),
            tick_rate: Duration::from_millis(150),
            tick_accumulator: Duration::ZERO,
            bounds,
            is_game_over: false,
            transition_ticks: 0,
            showing_clear: false,
            is_complete: false,
        };
        state.reset_snake();
        state
    }

    /// Triggers entity reconstruction on death or level up.
    pub fn reset_snake(&mut self) {
        #[allow(clippy::cast_possible_wrap)]
        let cx = (self.bounds.width / 2) as i32;
        #[allow(clippy::cast_possible_wrap)]
        let cy = (self.bounds.height / 2) as i32;

        self.segments.clear();
        self.segments.push_back(Vec2::new(cx, cy));
        self.segments.push_back(Vec2::new(cx, cy + 1));
        self.segments.push_back(Vec2::new(cx, cy + 2));
        
        self.direction = Direction::Up;
        self.input_queue.clear();
        self.tick_accumulator = Duration::ZERO;
        
        // Spawn food blindly. Ideally we'd loop until we find a free spot, but we inject a basic linear offset generator relying on random logic in outer modules.
        // For here, just put it at a fixed offset that will be corrected by logic tick if invalid.
        self.food = Vec2::new(cx + 5, cy);
    }
}
