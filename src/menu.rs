use std::io::stdout;
use std::time::Duration;

use crossterm::terminal::size;

use crate::engine::input::{Key, poll_key};
use crate::engine::renderer::Buffer;
use crate::engine::terminal::{clear_screen, game_viewport};
use crate::games::bricks::Bricks;
use crate::games::dino::Dino;
use crate::games::runner::Runner;
use crate::games::snake::Snake;
use crate::types::error::GameError;
use crate::types::game::Game;
use crate::types::geometry::TerminalSize;

pub fn run_menu() -> anyhow::Result<()> {
    let mut out: std::io::Stdout = stdout();

    clear_screen()?;

    let mut viewport: TerminalSize = game_viewport()?;
    let mut buffer: Buffer = Buffer::new(viewport);
    let mut needs_redraw: bool = true;

    loop {
        let new_vp: TerminalSize = game_viewport()?;
        if new_vp != viewport {
            viewport = new_vp;
            buffer = Buffer::new(viewport);
            needs_redraw = true;
        }

        if needs_redraw {
            buffer.clear();
            draw_menu(&mut buffer, viewport);
            let (raw_w, raw_h) = size()?;
            buffer.flush(raw_w, raw_h, &mut out)?;
            needs_redraw = false;
        }

        let Some(key) = poll_key(Duration::from_millis(50))? else {
            continue;
        };

        match key {
            Key::Number(1) => {
                clear_screen()?;
                Runner::new(viewport).run(viewport)?;
                redraw_after_game(&mut buffer, &mut out, viewport)?;
                needs_redraw = true;
            }
            Key::Number(2) => {
                clear_screen()?;
                Bricks::new(viewport).run(viewport)?;
                redraw_after_game(&mut buffer, &mut out, viewport)?;
                needs_redraw = true;
            }
            Key::Number(3) => {
                clear_screen()?;
                Snake::new(viewport).run(viewport)?;
                redraw_after_game(&mut buffer, &mut out, viewport)?;
                needs_redraw = true;
            }
            Key::Number(4) => {
                clear_screen()?;
                Dino::new(viewport).run(viewport)?;
                redraw_after_game(&mut buffer, &mut out, viewport)?;
                needs_redraw = true;
            }
            Key::Number(5) | Key::Quit => break,
            Key::None => needs_redraw = true,
            _ => {}
        }
    }

    Ok(())
}

fn redraw_after_game(
    buffer: &mut Buffer,
    out: &mut impl std::io::Write,
    viewport: TerminalSize,
) -> Result<(), GameError> {
    clear_screen()?;
    *buffer = Buffer::new(viewport);
    buffer.clear();
    draw_menu(buffer, viewport);
    let (raw_w, raw_h) = size()?;
    buffer.flush(raw_w, raw_h, out)?;
    Ok(())
}

fn draw_menu(buffer: &mut Buffer, vp: TerminalSize) {
    let cx = vp.width / 2;
    let mut y = vp.height.saturating_sub(18) / 2;

    buffer.print(cx.saturating_sub(12), y, "╔═══════════════════════╗");
    y += 1;
    buffer.print(cx.saturating_sub(12), y, "║   TERMINAL  ARCADE    ║");
    y += 1;
    buffer.print(cx.saturating_sub(12), y, "╚═══════════════════════╝");
    y += 2;

    buffer.print(cx.saturating_sub(10), y, "  ┌═┐");
    y += 1;
    buffer.print(cx.saturating_sub(10), y, "1 │█│  Runner");
    y += 1;
    buffer.print(cx.saturating_sub(10), y, "  └═┘");
    y += 1;

    buffer.print(cx.saturating_sub(10), y, "  ▓▓▓");
    y += 1;
    buffer.print(cx.saturating_sub(10), y, "2 ▓▓▓  Bricks");
    y += 1;

    buffer.print(cx.saturating_sub(10), y, "  ███");
    y += 1;
    buffer.print(cx.saturating_sub(10), y, "3 █□   Snake");
    y += 1;

    buffer.print(cx.saturating_sub(10), y, "  ▄██");
    y += 1;
    buffer.print(cx.saturating_sub(10), y, "4 ██   Dino");
    y += 2;

    buffer.print(cx.saturating_sub(10), y, "5      Quit");
    y += 2;

    buffer.horizontal_line(y, cx.saturating_sub(12), cx + 12, '─');
    y += 1;
    buffer.print(cx.saturating_sub(6), y, "Select: 1-5");
}
