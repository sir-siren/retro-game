//! Dino game — Chrome-style infinite obstacle runner.
//!
//! Jump over cacti with Space/Up. Duck under low birds with Down/S.
//! Speed increases every level. 3 lives, 5 levels.

pub mod logic;
pub mod render;
pub mod state;

use crate::engine::input::Key;
use crate::engine::loop_::{run_loop, GameLoop};
use crate::engine::renderer::Buffer;
use crate::games::dino::state::DinoState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::{Direction, TerminalSize};

/// Dino game session container.
pub struct Dino {
    state: DinoState,
    /// True while the down key is considered held.
    is_down_held: bool,
}

impl Dino {
    /// Creates a fresh Dino session for the given viewport.
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: DinoState::new(viewport),
            is_down_held: false,
        }
    }
}

impl GameLoop for Dino {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
        let stand = DinoState::stand_row(size);
        if !self.state.is_jumping {
            self.state.dino_row = stand;
        }
    }

    fn tick(&mut self) {
        if !self.is_down_held {
            logic::release_duck(&mut self.state);
        }
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.is_game_over && key != Key::None {
            self.state.is_complete = true;
            return;
        }

        self.is_down_held = matches!(key, Key::Dir(Direction::Down));
        logic::handle_input(&mut self.state, key);
    }

    fn render(&self, buffer: &mut Buffer) {
        render::render(&self.state, buffer);
    }

    fn status(&self) -> Option<GameResult> {
        if self.state.is_complete {
            Some(if self.state.is_game_over {
                GameResult::GameOver {
                    score: self.state.score,
                    level: self.state.level,
                }
            } else {
                GameResult::Complete {
                    score: self.state.score,
                    level: self.state.level,
                }
            })
        } else {
            None
        }
    }
}

impl Game for Dino {
    fn name(&self) -> &str {
        "Dino"
    }

    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult> {
        let res = run_loop(self, 33, viewport)?;
        Ok(res)
    }
}
