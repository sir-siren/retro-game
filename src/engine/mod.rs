//! Core terminal engine modules.

/// Raw keypress input abstraction.
pub mod input;
/// Tick-based game loop driver.
pub mod loop_;
/// Double-buffered character renderer.
pub mod renderer;
/// Terminal size detection and screen utilities.
pub mod terminal;
