pub mod input;
pub mod loop_;
pub mod renderer;
pub mod terminal;

pub type ArcadeTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;
