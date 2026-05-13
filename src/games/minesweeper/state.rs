use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    #[must_use]
    pub const fn dimensions(self) -> (u16, u16, u16) {
        match self {
            Self::Easy => (9, 9, 10),
            Self::Medium => (16, 16, 40),
            Self::Hard => (30, 16, 99),
        }
    }

    #[must_use]
    pub const fn level(self) -> Level {
        match self {
            Self::Easy => Level(1),
            Self::Medium => Level(3),
            Self::Hard => Level(5),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MineCell {
    pub has_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent: u8,
}

#[derive(Debug, Clone)]
pub struct MinesweeperState {
    pub cells: Vec<MineCell>,
    pub board_width: u16,
    pub board_height: u16,
    pub mine_count: u16,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub difficulty: Difficulty,
    pub is_generated: bool,
    pub elapsed_ticks: u64,
    pub score: Score,
    pub level: Level,
    pub bounds: TerminalSize,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl MinesweeperState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let difficulty = Difficulty::Easy;
        let (board_width, board_height, mine_count) = difficulty.dimensions();
        Self {
            cells: vec![MineCell::default(); usize::from(board_width * board_height)],
            board_width,
            board_height,
            mine_count,
            cursor_x: 0,
            cursor_y: 0,
            difficulty,
            is_generated: false,
            elapsed_ticks: 0,
            score: Score(0),
            level: difficulty.level(),
            bounds,
            is_game_over: false,
            is_complete: false,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        if self.is_generated {
            return;
        }
        let (board_width, board_height, mine_count) = difficulty.dimensions();
        self.difficulty = difficulty;
        self.board_width = board_width;
        self.board_height = board_height;
        self.mine_count = mine_count;
        self.cells = vec![MineCell::default(); usize::from(board_width * board_height)];
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.level = difficulty.level();
    }

    #[must_use]
    pub fn index(&self, x: u16, y: u16) -> usize {
        usize::from(y * self.board_width + x)
    }

    #[must_use]
    pub const fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.board_width as i32 && y < self.board_height as i32
    }

    #[must_use]
    pub fn flags_used(&self) -> u16 {
        self.cells.iter().filter(|cell| cell.is_flagged).count() as u16
    }
}
