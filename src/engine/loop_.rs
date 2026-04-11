use std::io::stdout;
use std::time::{Duration, Instant};

use crate::engine::input::{Key, poll_key};
use crate::engine::renderer::Buffer;
use crate::engine::terminal::game_viewport;
use crate::types::error::GameError;
use crate::types::game::GameResult;
use crate::types::geometry::TerminalSize;

pub trait GameLoop {
    fn resize(&mut self, size: TerminalSize);
    fn tick(&mut self);
    fn handle_input(&mut self, key: Key);
    fn render(&self, buffer: &mut Buffer);
    fn status(&self) -> Option<GameResult>;
}

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

        if let Some(key) = poll_key(tick_duration)? {
            match key {
                Key::Quit => return Ok(GameResult::Quit),
                Key::None => {
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

        let (raw_w, raw_h) = crossterm::terminal::size()?;
        buffer.flush(raw_w, raw_h, &mut out)?;

        let elapsed = frame_start.elapsed();
        if elapsed < tick_duration {
            std::thread::sleep(tick_duration - elapsed);
        }
    }
}
