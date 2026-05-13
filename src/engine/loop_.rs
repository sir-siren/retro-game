use std::time::{Duration, Instant};

use crossterm::event::{self, Event};

use crate::engine::ArcadeTerminal;
use crate::engine::input::{Key, parse_key};
use crate::engine::renderer::Buffer;
use crate::engine::terminal::terminal_size;
use crate::types::error::GameError;
use crate::types::game::GameResult;
use crate::types::geometry::TerminalSize;
use crate::ui::components;

pub trait GameLoop {
    fn resize(&mut self, size: TerminalSize);
    fn tick(&mut self);
    fn handle_input(&mut self, key: Key);
    fn render(&self, buffer: &mut Buffer);
    fn status(&self) -> Option<GameResult>;
}

/// Runs a frame-timed terminal game loop.
///
/// # Errors
///
/// Returns an error when terminal input, resize handling, clearing, or drawing fails.
pub fn run_loop<G: GameLoop>(
    game: &mut G,
    tick_ms: u64,
    terminal: &mut ArcadeTerminal,
) -> Result<GameResult, GameError> {
    let mut viewport = terminal_size()?;
    let mut buffer = Buffer::new(viewport);
    game.resize(viewport);
    render_countdown_sequence(game, &mut buffer, terminal)?;

    loop {
        let frame_start = Instant::now();
        let tick_duration = Duration::from_millis(tick_ms);

        if event::poll(tick_duration)? {
            match event::read()? {
                Event::Key(key_event) => {
                    let key = parse_key(key_event);
                    match key {
                        Key::Quit => {
                            game.handle_input(key);
                            if let Some(result) = game.status() {
                                return Ok(result);
                            }
                            return Ok(GameResult::Quit);
                        }
                        Key::Pause => {
                            if let Some(result) =
                                wait_while_paused(game, &mut buffer, &mut viewport, terminal)?
                            {
                                return Ok(result);
                            }
                        }
                        Key::None => {}
                        k => {
                            game.handle_input(k);
                            if let Some(result) = game.status() {
                                return Ok(result);
                            }
                        }
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

fn render_countdown_sequence<G: GameLoop>(
    game: &G,
    buffer: &mut Buffer,
    terminal: &mut ArcadeTerminal,
) -> Result<(), GameError> {
    for count in (1..=3).rev() {
        draw_game(game, buffer, terminal, |frame, area| {
            components::render_countdown(frame, area, count);
        })?;
        std::thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

fn wait_while_paused<G: GameLoop>(
    game: &mut G,
    buffer: &mut Buffer,
    viewport: &mut TerminalSize,
    terminal: &mut ArcadeTerminal,
) -> Result<Option<GameResult>, GameError> {
    loop {
        draw_game(game, buffer, terminal, components::render_pause_overlay)?;

        match event::read()? {
            Event::Key(key_event) => {
                let key = parse_key(key_event);
                if key == Key::Quit {
                    game.handle_input(key);
                    return Ok(game.status().or(Some(GameResult::Quit)));
                }
                game.handle_input(key);
                return Ok(game.status());
            }
            Event::Resize(width, height) => {
                let resized = TerminalSize { width, height };
                if resized != *viewport {
                    *viewport = resized;
                    *buffer = Buffer::new(resized);
                    game.resize(resized);
                    terminal.clear()?;
                }
            }
            _ => {}
        }
    }
}

fn draw_game<G, F>(
    game: &G,
    buffer: &mut Buffer,
    terminal: &mut ArcadeTerminal,
    overlay: F,
) -> Result<(), GameError>
where
    G: GameLoop,
    F: FnOnce(&mut ratatui::Frame<'_>, ratatui::layout::Rect),
{
    buffer.clear();
    game.render(buffer);

    terminal.draw(|frame| {
        let area = frame.area();
        buffer.render_to(frame.buffer_mut(), area);
        overlay(frame, area);
    })?;

    Ok(())
}
