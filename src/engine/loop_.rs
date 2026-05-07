use std::time::{Duration, Instant};

use crossterm::event::{self, Event};

use crate::engine::ArcadeTerminal;
use crate::engine::input::{Key, parse_key};
use crate::engine::renderer::Buffer;
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
    terminal: &mut ArcadeTerminal,
) -> Result<GameResult, GameError> {
    let initial = crossterm::terminal::size()?;
    let mut viewport = TerminalSize {
        width: initial.0,
        height: initial.1,
    };
    let mut buffer = Buffer::new(viewport);
    game.resize(viewport);

    loop {
        let frame_start = Instant::now();
        let tick_duration = Duration::from_millis(tick_ms);

        if event::poll(tick_duration)? {
            match event::read()? {
                Event::Key(key_event) => {
                    let key = parse_key(key_event);
                    match key {
                        Key::Quit => return Ok(GameResult::Quit),
                        Key::None => {}
                        k => game.handle_input(k),
                    }
                }
                Event::Resize(w, h) => {
                    let new_vp = TerminalSize {
                        width: w,
                        height: h,
                    };
                    if new_vp != viewport {
                        viewport = new_vp;
                        buffer = Buffer::new(viewport);
                        game.resize(viewport);
                        terminal.clear()?;
                    }
                }
                _ => {}
            }
        }

        game.tick();

        if let Some(res) = game.status() {
            return Ok(res);
        }

        buffer.clear();
        game.render(&mut buffer);

        terminal.draw(|frame| {
            let area = frame.area();
            buffer.render_to(frame.buffer_mut(), area);
        })?;

        let elapsed = frame_start.elapsed();
        if let Some(remaining) = tick_duration.checked_sub(elapsed) {
            std::thread::sleep(remaining);
        }
    }
}
