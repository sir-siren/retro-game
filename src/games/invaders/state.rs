use crate::types::geometry::{Level, Score, TerminalSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alien {
    pub x: i32,
    pub y: i32,
    pub row: u8,
    pub is_alive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bullet {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShieldCell {
    pub x: i32,
    pub y: i32,
    pub hp: u8,
}

#[derive(Debug, Clone)]
pub struct InvadersState {
    pub player_x: i32,
    pub aliens: Vec<Alien>,
    pub player_bullets: Vec<Bullet>,
    pub alien_bullets: Vec<Bullet>,
    pub shields: Vec<ShieldCell>,
    pub score: Score,
    pub level: Level,
    pub tick: u64,
    pub direction: i32,
    pub bounds: TerminalSize,
    pub is_game_over: bool,
    pub is_complete: bool,
}

impl InvadersState {
    #[must_use]
    pub fn new(bounds: TerminalSize) -> Self {
        let mut state = Self {
            player_x: i32::from(bounds.width) / 2,
            aliens: Vec::with_capacity(55),
            player_bullets: Vec::with_capacity(3),
            alien_bullets: Vec::with_capacity(12),
            shields: Vec::with_capacity(96),
            score: Score(0),
            level: Level(1),
            tick: 0,
            direction: 1,
            bounds,
            is_game_over: false,
            is_complete: false,
        };
        state.spawn_wave();
        state.spawn_shields();
        state
    }

    pub fn spawn_wave(&mut self) {
        self.aliens.clear();
        let start_x = 8;
        let start_y = 3 + i32::from(self.level.0.saturating_sub(1)).min(4);
        for row in 0..5u8 {
            for col in 0..11i32 {
                self.aliens.push(Alien {
                    x: start_x + col * 4,
                    y: start_y + i32::from(row) * 2,
                    row,
                    is_alive: true,
                });
            }
        }
    }

    pub fn spawn_shields(&mut self) {
        self.shields.clear();
        let spacing = i32::from(self.bounds.width) / 5;
        let y = i32::from(self.bounds.height.saturating_sub(7));
        for shield in 1..=4 {
            let cx = spacing * shield;
            for dy in 0_i32..3 {
                for dx in -3_i32..=3 {
                    if dy == 2 && dx.abs() <= 1 {
                        continue;
                    }
                    self.shields.push(ShieldCell {
                        x: cx + dx,
                        y: y + dy,
                        hp: 2,
                    });
                }
            }
        }
    }
}
