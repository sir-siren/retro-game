use std::io::stdout;
use std::time::{Duration, Instant};

use crate::engine::input::{Key, poll_key};
use crate::engine::renderer::Buffer;
use crate::engine::terminal::game_viewport;
use crate::types::error::GameError;
use crate::types::game::GameResult;
use crate::types::geometry::TerminalSize;

/// Game loop implementations must conform to this for standard driver execution.
pub trait GameLoop {
    /// Notifies the game that the viewport size changed.
    fn resize(&mut self, size: TerminalSize);

    /// Advance the simulation by one tick.
    fn tick(&mut self);

    /// Respond to user events.
    fn handle_input(&mut self, key: Key);

    /// Renders state to a fresh foreground buffer.
    fn render(&self, buffer: &mut Buffer);

    /// Returns a GameResult if the loop should terminate.
    fn status(&self) -> Option<GameResult>;
}

/// Drives a generalized tick-based loop.
///
/// # Errors
///
/// Returns `AppError` mapping to IO or terminal failures.
pub fn run_loop<G: GameLoop>(
    game: &mut G,
    tick_ms: u64,
    mut viewport: TerminalSize,
) -> Result<GameResult, GameError> {
    let mut out = stdout();
    let mut buffer = Buffer::new(viewport);
    game.resize(viewport);

    loop {
        let frame_start = Instant::now();
        let tick_duration = Duration::from_millis(tick_ms);

        // Resize detection loop and input drain
        if let Some(key) = poll_key(tick_duration)? {
            match key {
                Key::Quit => return Ok(GameResult::Quit),
                Key::None => {
                    // Possible resize, re-fetch bounds.
                    let new_vp = game_viewport()?;
                    if new_vp != viewport {
                        viewport = new_vp;
                        buffer = Buffer::new(viewport);
                        game.resize(viewport);
                    }
                }
                k => game.handle_input(k),
            }
        }

        game.tick();

        if let Some(res) = game.status() {
            return Ok(res);
        }

        buffer.clear();
        game.render(&mut buffer);

        // We fetch the raw size from the terminal direct to ensure correct offset flush
        let (raw_w, raw_h) = crossterm::terminal::size()?;
        buffer.flush(raw_w, raw_h, &mut out)?;

        let elapsed = frame_start.elapsed();
        if elapsed < tick_duration {
            std::thread::sleep(tick_duration - elapsed);
        }
    }
}
