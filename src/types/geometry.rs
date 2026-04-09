/// Represents domain-level scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Score(pub u32);

/// Represents the level progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Level(pub u8);

/// Represents remaining lives of the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lives(pub u8);

/// A discrete 2D coordinate on the terminal grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    /// X coordinate (column).
    pub x: i32,
    /// Y coordinate (row).
    pub y: i32,
}

impl Vec2 {
    /// Constructs a new coordinate component.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Floating point 2D coordinate for fine physics interpolation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2F {
    /// X coordinate (column).
    pub x: f32,
    /// Y coordinate (row).
    pub y: f32,
}

impl Vec2F {
    /// Constructs a new float vector.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Screen bounds for rendering and game limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalSize {
    /// Number of wide text cells.
    pub width: u16,
    /// Number of tall text cells.
    pub height: u16,
}

/// Four-way directions for movement logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Points up the screen (decreasing Y).
    Up,
    /// Points down the screen (increasing Y).
    Down,
    /// Points left (decreasing X).
    Left,
    /// Points right (increasing X).
    Right,
}
