pub mod logic;
pub mod render;
pub mod state;

use crate::engine::input::Key;
use crate::engine::loop_::{GameLoop, run_loop};
use crate::engine::renderer::Buffer;
use crate::games::snake::state::SnakeState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

pub struct Snake {
    state: SnakeState,
}

impl Snake {
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
        if self.state.is_game_over {
            return;
        }
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.is_game_over && key != Key::None {
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
            Some(GameResult::GameOver {
                score: self.state.score,
                level: self.state.level,
            })
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
        let res: GameResult = run_loop(self, 33, viewport)?;
        Ok(res)
    }
}
