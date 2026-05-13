pub mod logic;
pub mod render;
pub mod state;

use crate::engine::ArcadeTerminal;
use crate::engine::input::Key;
use crate::engine::loop_::{GameLoop, run_loop};
use crate::engine::renderer::Buffer;
use crate::games::dino::state::{DinoState, DinoStatus};
use crate::types::game::{Game, GameResult};
use crate::types::geometry::{Direction, TerminalSize};

pub struct Dino {
    state: DinoState,
    is_down_held: bool,
    retry_requested: bool,
}

impl Dino {
    #[must_use]
    pub fn new(viewport: TerminalSize) -> Self {
        Self {
            state: DinoState::new(viewport),
            is_down_held: false,
            retry_requested: false,
        }
    }
}

impl GameLoop for Dino {
    fn resize(&mut self, size: TerminalSize) {
        self.state.bounds = size;
        let stand = DinoState::stand_y(size);
        if !self.state.status.is_jumping() {
            self.state.dino_y = stand;
        }
    }

    fn tick(&mut self) {
        if !self.is_down_held {
            logic::release_duck(&mut self.state);
        }
        logic::tick(&mut self.state);
    }

    fn handle_input(&mut self, key: Key) {
        if self.state.status.is_game_over() {
            match key {
                Key::Retry | Key::Action => self.retry_requested = true,
                Key::Quit => self.state.status = DinoStatus::Complete,
                _ => {}
            }
            return;
        }
        self.is_down_held = matches!(key, Key::Dir(Direction::Down));
        logic::handle_input(&mut self.state, key);
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
        } else if self.state.status.is_complete() {
            Some(GameResult::GameOver {
                score: self.state.score,
                level: self.state.level,
            })
        } else {
            None
        }
    }
}

impl Game for Dino {
    fn name(&self) -> &'static str {
        "Dino"
    }

    fn run(&mut self, terminal: &mut ArcadeTerminal) -> anyhow::Result<GameResult> {
        Ok(run_loop(self, 33, terminal)?)
    }
}
