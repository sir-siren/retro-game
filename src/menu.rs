use std::time::Duration;

use crossterm::event::{self, Event};

use crate::engine::ArcadeTerminal;
use crate::engine::input::{Key, parse_key};
use crate::engine::renderer::Buffer;
use crate::games::bricks::Bricks;
use crate::games::dino::Dino;
use crate::games::runner::Runner;
use crate::games::snake::Snake;
use crate::types::game::Game;
use crate::types::geometry::TerminalSize;

pub fn run_menu(terminal: &mut ArcadeTerminal) -> anyhow::Result<()> {
    let mut viewport = current_size()?;
    let mut buffer = Buffer::new(viewport);
    let mut needs_redraw = true;

    loop {
        let new_vp = current_size()?;
        if new_vp != viewport {
            viewport = new_vp;
            buffer = Buffer::new(viewport);
            terminal.clear()?;
            needs_redraw = true;
        }

        if needs_redraw {
            buffer.clear();
            draw_menu(&mut buffer, viewport);
            terminal.draw(|frame| {
                let area = frame.area();
                buffer.render_to(frame.buffer_mut(), area);
            })?;
            needs_redraw = false;
        }

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        match event::read()? {
            Event::Key(key_event) => {
                match parse_key(key_event) {
                    Key::Number(1) => {
                        terminal.clear()?;
                        Runner::new(viewport).run(terminal)?;
                        terminal.clear()?;
                        needs_redraw = true;
                    }
                    Key::Number(2) => {
                        terminal.clear()?;
                        Bricks::new(viewport).run(terminal)?;
                        terminal.clear()?;
                        needs_redraw = true;
                    }
                    Key::Number(3) => {
                        terminal.clear()?;
                        Snake::new(viewport).run(terminal)?;
                        terminal.clear()?;
                        needs_redraw = true;
                    }
                    Key::Number(4) => {
                        terminal.clear()?;
                        Dino::new(viewport).run(terminal)?;
                        terminal.clear()?;
                        needs_redraw = true;
                    }
                    Key::Number(5) | Key::Quit => break,
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                viewport = TerminalSize {
                    width: w,
                    height: h,
                };
                buffer = Buffer::new(viewport);
                terminal.clear()?;
                needs_redraw = true;
            }
            _ => {}
        }
    }

    Ok(())
}

fn current_size() -> anyhow::Result<TerminalSize> {
    let (w, h) = crossterm::terminal::size()?;
    Ok(TerminalSize {
        width: w,
        height: h,
    })
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