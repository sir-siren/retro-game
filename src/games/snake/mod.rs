pub mod logic;
pub mod render;
pub mod state;

use crate::engine::input::Key;
use crate::engine::loop_::{run_loop, GameLoop};
use crate::engine::renderer::Buffer;
use crate::games::snake::state::SnakeState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

/// Exposes module mechanics through common trait container.
pub struct Snake {
    state: SnakeState,
}

impl Snake {
    /// Binds standard starting rules into new engine state.
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: SnakeState::new(viewport),
        }
    }
}

impl GameLoop for Snake {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
    }

    fn tick(&mut self) {
        if self.state.is_game_over && self.state.lives.0 == 0 {
            return;
        }
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if (self.state.is_game_over || self.state.is_complete) && key != Key::None {
            self.state.is_complete = true;
        } else {
            logic::handle_input(&mut self.state, key);
        }
    }

    fn render(&self, buffer: &mut Buffer) {
        render::render(&self.state, buffer);
    }

    fn status(&self) -> Option<GameResult> {
        if self.state.is_complete {
            if self.state.is_game_over {
                Some(GameResult::GameOver {
                    score: self.state.score,
                    level: self.state.level,
                })
            } else {
                Some(GameResult::Complete {
                    score: self.state.score,
                    level: self.state.level,
                })
            }
        } else {
            None
        }
    }
}

impl Game for Snake {
    fn name(&self) -> &str {
        "Snake"
    }

    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult> {
        // Runs at standard 33ms, logic ticking defers to accumulator in Snake state.
        let res = run_loop(self, 33, viewport)?;
        Ok(res)
    }
}
