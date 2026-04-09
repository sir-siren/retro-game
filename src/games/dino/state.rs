//! Pure domain state for the Dino game.

use crate::types::geometry::{Level, Lives, Score, TerminalSize};

/// Obstacle variant with its geometry encoded in the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinoObstacleKind {
    /// Two-row cactus — jump to clear.
    SmallCactus,
    /// Three-row cactus — jump to clear.
    LargeCactus,
    /// Two side-by-side small cacti — jump to clear.
    DoubleCactus,
    /// Low-flying bird at head height — duck to clear.
    LowBird,
    /// High-flying bird well above dino — no action needed, visual only.
    HighBird,
}

/// A single obstacle active on the track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DinoObstacle {
    /// Left-edge column.
    pub col: u16,
    /// What kind of obstacle this is.
    pub kind: DinoObstacleKind,
}

/// All state required to simulate one Dino game session.
#[derive(Debug, Clone)]
pub struct DinoState {
    /// Dino body row — the lower of the two rows it occupies when standing.
    pub dino_row: u16,
    /// True while the dino is airborne.
    pub is_jumping: bool,
    /// True while the player holds the duck key.
    pub is_ducking: bool,
    /// Upward velocity; positive = rising, negative = falling.
    pub jump_velocity: i16,
    /// Active obstacles ordered right-to-left.
    pub obstacles: Vec<DinoObstacle>,
    /// Accumulated run score.
    pub score: Score,
    /// Current difficulty level (1–5).
    pub level: Level,
    /// Lives remaining.
    pub lives: Lives,
    /// Columns scrolled per tick.
    pub speed: u16,
    /// Tick counter for timing spawns and score increments.
    pub tick: u64,
    /// Invincibility frames remaining after a hit.
    pub hurt_ticks: u16,
    /// True when all lives are spent.
    pub is_game_over: bool,
    /// True when the session should exit back to the menu.
    pub is_complete: bool,
    /// Viewport limits.
    pub bounds: TerminalSize,
}

impl DinoState {
    /// Constructs the initial game state for the given viewport.
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        Self {
            dino_row: Self::stand_row(bounds),
            is_jumping: false,
            is_ducking: false,
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

    /// Row index of the visual ground line.
    #[must_use]
    pub fn ground_line(bounds: TerminalSize) -> u16 {
        bounds.height.saturating_sub(2)
    }

    /// Normal standing body row — one above the ground line.
    #[must_use]
    pub fn stand_row(bounds: TerminalSize) -> u16 {
        Self::ground_line(bounds).saturating_sub(1)
    }
}
