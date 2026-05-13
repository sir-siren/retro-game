use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

use crate::engine::ArcadeTerminal;
use crate::engine::input::{Key, parse_key};
use crate::games::bricks::Bricks;
use crate::games::dino::Dino;
use crate::games::flappy::Flappy;
use crate::games::invaders::Invaders;
use crate::games::minesweeper::Minesweeper;
use crate::games::pong::Pong;
use crate::games::runner::Runner;
use crate::games::snake::Snake;
use crate::games::tetris::Tetris;
use crate::persistence::ScoreDb;
use crate::types::game::Game;
use crate::types::geometry::TerminalSize;
use crate::ui::theme;

/// Game entry for the menu list.
struct GameEntry {
    label: &'static str,
    key: &'static str,
}

const GAMES: [GameEntry; 9] = [
    GameEntry {
        label: "Runner",
        key: "runner",
    },
    GameEntry {
        label: "Bricks",
        key: "bricks",
    },
    GameEntry {
        label: "Snake",
        key: "snake",
    },
    GameEntry {
        label: "Dino",
        key: "dino",
    },
    GameEntry {
        label: "Tetris",
        key: "tetris",
    },
    GameEntry {
        label: "Pong",
        key: "pong",
    },
    GameEntry {
        label: "Space Invaders",
        key: "invaders",
    },
    GameEntry {
        label: "Minesweeper",
        key: "minesweeper",
    },
    GameEntry {
        label: "Flappy Bird",
        key: "flappy",
    },
];

/// Runs the interactive arcade menu.
///
/// # Errors
///
/// Returns an error when terminal input, rendering, or game execution fails.
pub fn run_menu(terminal: &mut ArcadeTerminal) -> anyhow::Result<()> {
    // open the score database -- if it fails, log and continue without scores
    let db = ScoreDb::open().ok();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        // draw menu
        terminal.draw(|frame| {
            let area = frame.area();
            draw_menu(frame, area, &list_state, db.as_ref());
        })?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key_event) = event::read()? {
            let key = parse_key(key_event);
            match key {
                Key::Dir(crate::types::geometry::Direction::Up) => {
                    let i = list_state.selected().unwrap_or(0);
                    let prev = if i == 0 { GAMES.len() - 1 } else { i - 1 };
                    list_state.select(Some(prev));
                }
                Key::Dir(crate::types::geometry::Direction::Down) => {
                    let i = list_state.selected().unwrap_or(0);
                    let next = if i >= GAMES.len() - 1 { 0 } else { i + 1 };
                    list_state.select(Some(next));
                }
                Key::Action => {
                    let selected = list_state.selected().unwrap_or(0);
                    launch_game(terminal, selected, db.as_ref())?;
                }
                Key::Number(n) if usize::from(n) <= GAMES.len() => {
                    let idx = usize::from(n - 1);
                    list_state.select(Some(idx));
                    launch_game(terminal, idx, db.as_ref())?;
                }
                Key::Quit => break,
                _ => {}
            }
        }
    }

    Ok(())
}

/// Runs one game by its menu key, skipping the interactive menu.
///
/// Returns `Ok(false)` when no game exists for `game_key`.
///
/// # Errors
///
/// Returns an error when terminal input, rendering, or game execution fails.
pub fn run_game_by_key(terminal: &mut ArcadeTerminal, game_key: &str) -> anyhow::Result<bool> {
    let Some(index) = game_index(game_key) else {
        return Ok(false);
    };
    let db = ScoreDb::open().ok();
    launch_game(terminal, index, db.as_ref())?;
    Ok(true)
}

fn launch_game(
    terminal: &mut ArcadeTerminal,
    index: usize,
    db: Option<&ScoreDb>,
) -> anyhow::Result<()> {
    let viewport = current_size()?;

    // retry loop -- if the game returns Retry, start it again
    loop {
        terminal.clear()?;

        let result = match index {
            0 => Runner::new(viewport).run(terminal)?,
            1 => Bricks::new(viewport).run(terminal)?,
            2 => Snake::new(viewport).run(terminal)?,
            3 => Dino::new(viewport).run(terminal)?,
            4 => Tetris::new(viewport).run(terminal)?,
            5 => Pong::new(viewport).run(terminal)?,
            6 => Invaders::new(viewport).run(terminal)?,
            7 => Minesweeper::new(viewport).run(terminal)?,
            8 => Flappy::new(viewport).run(terminal)?,
            _ => return Ok(()),
        };

        if let Some(score_db) = db {
            let game_key = GAMES.get(index).map_or("unknown", |g| g.key);
            if let (Some(score), Some(level)) = (result.score(), result.level()) {
                let _ignored = score_db.save_score(game_key, score.0, level.0);
            }
        }

        if !result.should_retry() {
            break;
        }
    }

    terminal.clear()?;
    Ok(())
}

fn game_index(game_key: &str) -> Option<usize> {
    GAMES
        .iter()
        .position(|game| game.key.eq_ignore_ascii_case(game_key))
}

fn current_size() -> anyhow::Result<TerminalSize> {
    let (w, h) = crossterm::terminal::size()?;
    Ok(TerminalSize {
        width: w,
        height: h,
    })
}

fn draw_menu(frame: &mut ratatui::Frame, area: Rect, list_state: &ListState, db: Option<&ScoreDb>) {
    // fill background
    let bg_block = Block::default().style(Style::new().bg(theme::BG_MAIN));
    frame.render_widget(bg_block, area);

    // vertical layout: title | game list | footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    draw_title(frame, chunks[0]);
    draw_game_list(frame, chunks[1], list_state, db);
    draw_footer(frame, chunks[2]);
}

fn draw_title(frame: &mut ratatui::Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme::BORDER))
        .style(Style::new().bg(theme::BG_PANEL));

    let title = Paragraph::new(Line::from(vec![Span::styled(
        "TERMINAL ARCADE",
        Style::new()
            .fg(theme::TITLE_GOLD)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center)
    .block(title_block);

    frame.render_widget(title, area);
}

fn draw_game_list(
    frame: &mut ratatui::Frame,
    area: Rect,
    list_state: &ListState,
    db: Option<&ScoreDb>,
) {
    let items: Vec<ListItem> = GAMES
        .iter()
        .map(|game| {
            let hi_score = db.and_then(|d| d.high_score(game.key).ok()).unwrap_or(0);

            let score_str = if hi_score > 0 {
                format!("HI: {hi_score}")
            } else {
                "HI: ---".into()
            };

            let line = Line::from(vec![
                Span::styled(game.label, Style::new().fg(theme::WHITE)),
                Span::raw("  "),
                Span::styled(score_str, Style::new().fg(theme::MUTED)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme::BORDER))
        .style(Style::new().bg(theme::BG_PANEL));

    let list = List::new(items)
        .block(list_block)
        .highlight_style(theme::style_highlight())
        .highlight_symbol("> ");

    // clone state for rendering -- ratatui needs &mut ListState
    let mut render_state = list_state.clone();
    frame.render_stateful_widget(list, area, &mut render_state);
}

fn draw_footer(frame: &mut ratatui::Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[Up/Down]", Style::new().fg(theme::HIGHLIGHT)),
        Span::raw(" Navigate  "),
        Span::styled("[Enter]", Style::new().fg(theme::HIGHLIGHT)),
        Span::raw(" Play  "),
        Span::styled("[1-9]", Style::new().fg(theme::HIGHLIGHT)),
        Span::raw(" Quick  "),
        Span::styled("[Q]", Style::new().fg(theme::DANGER)),
        Span::raw(" Quit"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::new().fg(theme::MUTED).bg(theme::BG_MAIN));

    frame.render_widget(footer, area);
}
