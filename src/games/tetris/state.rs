use std::time::Duration;

use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetromino {
    #[must_use]
    pub const fn color_index(self) -> u8 {
        match self {
            Self::I => 0,
            Self::O => 1,
            Self::T => 2,
            Self::S => 3,
            Self::Z => 4,
            Self::J => 5,
            Self::L => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    pub kind: Tetromino,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct TetrisState {
    pub board: [[Option<u8>; 10]; 20],
    pub current: ActivePiece,
    pub next: Tetromino,
    pub hold: Option<Tetromino>,
    pub can_hold: bool,
    pub score: Score,
    pub level: Level,
    pub lines_cleared: u16,
    pub tick_rate: Duration,
    pub tick_accumulator: Duration,
    pub tick: u64,
    pub seed: u64,
    pub bounds: TerminalSize,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl TetrisState {
    pub const BOARD_WIDTH: u16 = 10;
    pub const BOARD_HEIGHT: u16 = 20;

    #[must_use]
    pub const fn new(bounds: TerminalSize) -> Self {
        let first = Tetromino::T;
        let next = Tetromino::I;
        Self {
            board: [[None; 10]; 20],
            current: ActivePiece {
                kind: first,
                rotation: 0,
                x: 3,
                y: 0,
            },
            next,
            hold: None,
            can_hold: true,
            score: Score(0),
            level: Level(1),
            lines_cleared: 0,
            tick_rate: Duration::from_millis(700),
            tick_accumulator: Duration::ZERO,
            tick: 0,
            seed: 0x1234_5678,
            bounds,
            is_game_over: false,
            is_complete: false,
        }
    }
}
