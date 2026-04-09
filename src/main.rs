//! Terminal Arcade Suite — a collection of retro games running in raw TTY mode.
//!
//! Run with: `cargo run --release`
//! Navigate the menu with 1–5. Press `q` inside any game to return to the menu.

#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]
#![warn(missing_docs)]

/// Core terminal engine — input, loop driver, renderer, and terminal utilities.
pub mod engine;
/// All playable game modules.
pub mod games;
/// Main menu renderer and game router.
pub mod menu;
/// Shared domain types — geometry, errors, and game traits.
pub mod types;

use std::io::stdout;
use std::panic;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};

/// Bootstraps raw terminal mode, installs a panic hook to restore the terminal,
/// runs the menu, then restores the terminal on exit.
///
/// # Errors
///
/// Returns `anyhow::Error` on terminal setup or I/O failures.
fn main() -> anyhow::Result<()> {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = execute!(stdout(), cursor::Show);
        let _ = disable_raw_mode();
        default_hook(info);
    }));

    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let result = menu::run_menu();

    let _ = execute!(stdout(), cursor::Show);
    let _ = disable_raw_mode();

    result
}
