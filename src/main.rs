#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

pub mod engine;
pub mod games;
pub mod menu;
pub mod types;

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
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
        let _ = disable_raw_mode();
        default_hook(info);
    }));

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = menu::run_menu(&mut terminal);

    execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;
    result
}
