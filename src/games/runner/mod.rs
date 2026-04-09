pub mod logic;
pub mod render;
pub mod state;

use crate::engine::input::Key;
use crate::engine::loop_::{run_loop, GameLoop};
use crate::engine::renderer::Buffer;
use crate::games::runner::state::RunnerState;
use crate::types::game::{Game, GameResult};
use crate::types::geometry::TerminalSize;

/// The Runner game encapsulation wrapper.
pub struct Runner {
    state: RunnerState,
}

impl Runner {
    /// Constructs a fresh Game instance.
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: RunnerState::new(viewport),
        }
    }
}

impl GameLoop for Runner {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
        // Fix player grounded alignment if resized.
        let gr = self.state.ground_row().saturating_sub(1);
        if !self.state.is_jumping {
            self.state.player_row = gr;
        }
    }

    fn tick(&mut self) {
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.is_game_over && key != Key::None {
            self.state.is_complete = true; // Signal exit back to menu
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

impl Game for Runner {
    fn name(&self) -> &str {
        "Runner"
    }

    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult> {
        // Runner plays at 33ms tick intervals.
        let res = run_loop(self, 33, viewport)?;
        Ok(res)
    }
}
