use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::types::geometry::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Quit,
    Dir(Direction),
    Action,
    Number(u8),
    None,
}

#[must_use]
pub fn parse_key(key: KeyEvent) -> Key {
    match key.code {
        KeyCode::Char('q') => Key::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Key::Quit,
        KeyCode::Left | KeyCode::Char('a') => Key::Dir(Direction::Left),
        KeyCode::Right | KeyCode::Char('d') => Key::Dir(Direction::Right),
        KeyCode::Up | KeyCode::Char('w') => Key::Dir(Direction::Up),
        KeyCode::Down | KeyCode::Char('s') => Key::Dir(Direction::Down),
        KeyCode::Char(' ') | KeyCode::Enter => Key::Action,
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' =>
        {
            #[allow(clippy::cast_possible_truncation)]
            Key::Number(c as u8 - b'0')
        }
        _ => Key::None,
    }
}

pub fn poll_key(timeout: Duration) -> std::io::Result<Option<Key>> {
    if event::poll(timeout)? {
        let ev: Event = event::read()?;
        return match ev {
            Event::Key(key_event) => Ok(Some(parse_key(key_event))),
            Event::Resize(_, _) => Ok(Some(Key::None)),
            _ => Ok(Some(Key::None)),
        };
    }
    Ok(None)
}
