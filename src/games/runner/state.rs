use crate::types::geometry::{Level, Lives, Score, TerminalSize};

/// The variant of the incoming obstacle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType {
    /// A single ground block `|`.
    Single,
    /// A double ground block `||`.
    Double,
    /// An overhead stalactite hanging from the ceiling.
    Ceiling,
}

/// An obstacle on the track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obstacle {
    /// The column index, using sub-pixel floats or just u16. Since the spec says speed is u16 (pixels per tick), we track pure integers.
    pub col: u16,
    /// What kind of obstacle this is.
    pub kind: ObstacleType,
}

/// Pure domain state for the Runner game.
#[derive(Debug, Clone)]
pub struct RunnerState {
    /// Player's Y coordinate.
    pub player_row: u16,
    /// If the player is currently airborne.
    pub is_jumping: bool,
    /// Upwards trajectory force.
    pub jump_velocity: i16,
    /// List of active obstacles on screen.
    pub obstacles: Vec<Obstacle>,
    /// Accumulated survival score.
    pub score: Score,
    /// Current difficulty level, 1 through 5.
    pub level: Level,
    /// Lives remaining before failure.
    pub lives: Lives,
    /// Advance rate of obstacles per tick. We interpret this as sub-cell accumulation if speed < 1 cell/tick, or we can use floats. For simplicity, speed will represent sub-cell units, or ticks-per-move.
    pub speed: u16,
    /// Frame accumulation.
    pub tick: u64,
    /// Visual flash counter after a hit.
    pub hurt_ticks: u16,
    /// Flag for ending the game due to death.
    pub is_game_over: bool,
    /// Flag for completing all levels.
    pub is_complete: bool,
    /// Current constraints.
    pub bounds: TerminalSize,
}

impl RunnerState {
    /// Construct the initial starting condition.
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            player_row: bounds.height.saturating_sub(2), // Ground is height - 1.
            is_jumping: false,
            jump_velocity: 0,
            obstacles: Vec::new(),
            score: Score(0),
            level: Level(1),
            lives: Lives(3),
            speed: 1,
            tick: 0,
            hurt_ticks: 0,
            is_game_over: false,
            is_complete: false,
            bounds,
        }
    }

    /// Ground level for collision and rendering.
    #[must_use]
    pub fn ground_row(&self) -> u16 {
        self.bounds.height.saturating_sub(1) // bottom-most valid cell is height-1, ground line is height-1, player stands at height-2
    }
}
