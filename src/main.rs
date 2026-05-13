// lint config is in Cargo.toml [lints]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

pub mod engine;
pub mod games;
pub mod menu;
pub mod persistence;
pub mod types;
pub mod ui;

use std::io::stdout;
use std::panic;

use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{cursor, execute};
use ratatui::{Terminal, backend::CrosstermBackend};

fn main() -> anyhow::Result<()> {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        // already panicking -- cannot propagate errors, so report them to stderr instead
        if let Err(e) = execute!(stdout(), LeaveAlternateScreen, cursor::Show) {
            eprintln!("terminal restore failed: {e}");
        }
        if let Err(e) = disable_raw_mode() {
            eprintln!("raw mode disable failed: {e}");
        }
        default_hook(info);
    }));

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = match selected_game_arg() {
        Some(game_key) => match menu::run_game_by_key(&mut terminal, &game_key) {
            Ok(true) => Ok(()),
            Ok(false) => Err(anyhow::anyhow!("unknown game: {game_key}")),
            Err(error) => Err(error),
        },
        None => menu::run_menu(&mut terminal),
    };

    execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;
    result
}

fn selected_game_arg() -> Option<String> {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--game" {
            return args.next();
        }

        if let Some(game_key) = arg.strip_prefix("--game=") {
            return Some(game_key.to_owned());
        }
    }

    None
}
