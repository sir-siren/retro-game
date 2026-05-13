pub mod logic;
pub mod render;
pub mod state;

use crate::engine::ArcadeTerminal;
use crate::engine::input::Key;
use crate::engine::loop_::{GameLoop, run_loop};
use crate::engine::renderer::Buffer;
use crate::games::pong::state::PongState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

pub struct Pong {
    state: PongState,
    retry_requested: bool,
}

impl Pong {
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: PongState::new(viewport),
            retry_requested: false,
        }
    }
}

impl GameLoop for Pong {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
    }

    fn tick(&mut self) {
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.is_game_over || self.state.is_complete {
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

impl Game for Pong {
    fn name(&self) -> &'static str {
        "Pong"
    }

    fn run(&mut self, terminal: &mut ArcadeTerminal) -> anyhow::Result<GameResult> {
        Ok(run_loop(self, 33, terminal)?)
    }
}
