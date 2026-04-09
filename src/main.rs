#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]
#![warn(missing_docs)]

//! Terminal Arcade Suite in Rust.
//! A collection of lightweight terminal-based games avoiding external UI abstractions.

pub mod engine;
pub mod games;
pub mod menu;
pub mod types;

use std::io::stdout;
use std::panic;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};

/// System entrance bootstraps raw state and passes execution safely.
///
/// # Errors
/// 
/// Escalates unexpected application setup and IO faults.
fn main() -> anyhow::Result<()> {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = execute!(stdout(), cursor::Show);
        let _ = disable_raw_mode();
        default_hook(info);
    }));

    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let res = menu::run_menu();

    let _ = execute!(stdout(), cursor::Show);
    let _ = disable_raw_mode();
    
    res
}
