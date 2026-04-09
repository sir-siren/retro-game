use std::io::stdout;
use std::time::Duration;

use crate::engine::input::{Key, poll_key};
use crate::engine::renderer::Buffer;
use crate::engine::terminal::game_viewport;
use crate::games::bricks::Bricks;
use crate::games::runner::Runner;
use crate::games::snake::Snake;
use crate::types::game::Game;

/// Presents arcade core menu options and manages loop routing.
///
/// # Errors
///
/// Returns `anyhow::Error` when rendering or terminal input yields IO failures.
pub fn run_menu() -> anyhow::Result<()> {
    let mut out = stdout();

    loop {
        let viewport = game_viewport()?;
        let mut buffer = Buffer::new(viewport);

        render_menu(&mut buffer, viewport);

        let (raw_w, raw_h) = crossterm::terminal::size()?;
        buffer.flush(raw_w, raw_h, &mut out)?;

        if let Some(key) = poll_key(Duration::from_millis(50))? {
            match key {
                Key::Number(1) => {
                    let mut game = Runner::new(viewport);
                    let _ = game.run(viewport)?;
                }
                Key::Number(2) => {
                    let mut game = Bricks::new(viewport);
                    let _ = game.run(viewport)?;
                }
                Key::Number(3) => {
                    let mut game = Snake::new(viewport);
                    let _ = game.run(viewport)?;
                }
                Key::Number(4) | Key::Quit => {
                    break;
                }
                Key::None | Key::Number(_) | Key::Dir(_) | Key::Action => {}
            }
        }
    }

    Ok(())
}

fn render_menu(buffer: &mut Buffer, size: crate::types::geometry::TerminalSize) {
    let mut y = size.height / 2;
    if y > 5 {
        y -= 4;
    }

    let cx = size.width / 2;
    buffer.print(cx.saturating_sub(3), y, "ARCADE");
    y += 2;
    buffer.print(cx.saturating_sub(6), y, "1. Runner");
    y += 1;
    buffer.print(cx.saturating_sub(6), y, "2. Bricks");
    y += 1;
    buffer.print(cx.saturating_sub(6), y, "3. Snake");
    y += 1;
    buffer.print(cx.saturating_sub(6), y, "4. Quit");
    y += 2;
    buffer.print(cx.saturating_sub(6), y, "Select: _");
}
