use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::types::geometry::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Quit,
    Dir(Direction),
    Action,
    Retry,
    Pause,
    Hold,
    Flag,
    Number(u8),
    None,
}

#[must_use]
pub const fn parse_key(key: KeyEvent) -> Key {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Key::Quit,
        KeyCode::Char('q' | 'Q') => Key::Quit,
        KeyCode::Char('r' | 'R') => Key::Retry,
        KeyCode::Char('p' | 'P') => Key::Pause,
        KeyCode::Char('c' | 'C') => Key::Hold,
        KeyCode::Char('f' | 'F') => Key::Flag,
        KeyCode::Left | KeyCode::Char('a' | 'A') => Key::Dir(Direction::Left),
        KeyCode::Right | KeyCode::Char('d' | 'D') => Key::Dir(Direction::Right),
        KeyCode::Up | KeyCode::Char('w' | 'W') => Key::Dir(Direction::Up),
        KeyCode::Down | KeyCode::Char('s' | 'S') => Key::Dir(Direction::Down),
        KeyCode::Char(' ') | KeyCode::Enter => Key::Action,
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' =>
        {
            #[allow(clippy::cast_possible_truncation)]
            Key::Number(c as u8 - b'0')
        }
        _ => Key::None,
    }
}

/// Polls for one terminal key event.
///
/// # Errors
///
/// Returns an error when crossterm cannot poll or read from the terminal.
pub fn poll_key(timeout: Duration) -> std::io::Result<Option<Key>> {
    if event::poll(timeout)? {
        let ev: Event = event::read()?;
        return match ev {
            Event::Key(key_event) => Ok(Some(parse_key(key_event))),
            _ => Ok(Some(Key::None)),
        };
    }
    Ok(None)
}
