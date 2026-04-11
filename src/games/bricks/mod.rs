pub mod logic;
pub mod render;
pub mod state;

use crate::engine::input::Key;
use crate::engine::loop_::{GameLoop, run_loop};
use crate::engine::renderer::Buffer;
use crate::games::bricks::state::BricksState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

pub struct Bricks {
    state: BricksState,
}

impl Bricks {
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: BricksState::new(viewport),
        }
    }
}

impl GameLoop for Bricks {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
        let p_max = size.width.saturating_sub(self.state.paddle_width);
        if self.state.paddle_col > p_max {
            self.state.paddle_col = p_max;
        }
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

impl Game for Bricks {
    fn name(&self) -> &str {
        "Bricks"
    }

    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult> {
        let res = run_loop(self, 33, viewport)?;
        Ok(res)
    }
}
