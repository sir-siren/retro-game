pub mod logic;
pub mod render;
pub mod state;

use crate::engine::ArcadeTerminal;
use crate::engine::input::Key;
use crate::engine::loop_::{GameLoop, run_loop};
use crate::engine::renderer::Buffer;
use crate::games::runner::state::RunnerState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

pub struct Runner {
    state: RunnerState,
    retry_requested: bool,
}

impl Runner {
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: RunnerState::new(viewport),
            retry_requested: false,
        }
    }
}

impl GameLoop for Runner {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
    }

    fn tick(&mut self) {
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.is_game_over {
            match key {
                Key::Retry | Key::Action => self.retry_requested = true,
                Key::Quit => self.state.is_complete = true,
                _ => {}
            }
        } else {
            logic::handle_input(&mut self.state, key);
        }
    }

    fn render(&self, buffer: &mut Buffer) {
        render::render(&self.state, buffer);
    }

    fn status(&self) -> Option<GameResult> {
        if self.retry_requested {
            Some(GameResult::Retry {
                score: self.state.score,
                level: self.state.level,
            })
        } else if self.state.is_complete {
            Some(GameResult::GameOver {
                score: self.state.score,
                level: self.state.level,
            })
        } else {
            None
        }
    }
}

impl Game for Runner {
    fn name(&self) -> &'static str {
        "Runner"
    }

    fn run(&mut self, terminal: &mut ArcadeTerminal) -> anyhow::Result<GameResult> {
        Ok(run_loop(self, 33, terminal)?)
    }
}
