// color palette for the entire arcade -- all ratatui Color::Rgb values
use ratatui::style::{Color, Modifier, Style};

// -- global palette --------------------------------------------------------

pub const BG_MAIN: Color = Color::Rgb(18, 18, 24);
pub const BG_PANEL: Color = Color::Rgb(28, 28, 40);
pub const BORDER: Color = Color::Rgb(80, 80, 120);
pub const TITLE_GOLD: Color = Color::Rgb(255, 200, 60);
pub const HIGHLIGHT: Color = Color::Rgb(0, 255, 180);
pub const DANGER: Color = Color::Rgb(255, 80, 80);
pub const SUCCESS: Color = Color::Rgb(80, 255, 120);
pub const MUTED: Color = Color::Rgb(120, 120, 150);
pub const WHITE: Color = Color::Rgb(220, 220, 220);

// -- runner ----------------------------------------------------------------

pub const RUNNER_PLAYER: Color = Color::Rgb(255, 140, 0);
pub const RUNNER_ENEMY: Color = Color::Rgb(200, 50, 50);

// -- bricks ----------------------------------------------------------------

// one color per row, wraps around for deeper layouts
pub const BRICK_COLORS: [Color; 6] = [
    Color::Rgb(255, 60, 60),
    Color::Rgb(255, 160, 40),
    Color::Rgb(255, 230, 50),
    Color::Rgb(50, 220, 80),
    Color::Rgb(60, 130, 255),
    Color::Rgb(180, 80, 255),
];
pub const BRICK_ARMORED: Color = Color::Rgb(220, 220, 220);

// -- snake -----------------------------------------------------------------

pub const SNAKE_BODY: Color = Color::Rgb(0, 220, 100);
pub const SNAKE_HEAD: Color = Color::Rgb(0, 255, 130);
pub const SNAKE_FOOD: Color = Color::Rgb(255, 255, 80);

// -- dino (chrome-accurate greys) ------------------------------------------

pub const DINO_BODY: Color = Color::Rgb(83, 83, 83);
pub const DINO_GROUND: Color = Color::Rgb(172, 172, 172);
pub const DINO_NIGHT_BG: Color = Color::Rgb(30, 30, 30);
pub const DINO_NIGHT_FG: Color = Color::Rgb(200, 200, 200);

// -- minesweeper number colors (classic scheme) ----------------------------

pub const MINE_COLORS: [Color; 8] = [
    Color::Blue,
    Color::Green,
    Color::Red,
    Color::Rgb(128, 0, 128),
    Color::Rgb(128, 0, 0),
    Color::Cyan,
    Color::Black,
    Color::Gray,
];

// -- reusable styles -------------------------------------------------------

#[must_use]
pub const fn style_title() -> Style {
    Style::new().fg(TITLE_GOLD).add_modifier(Modifier::BOLD)
}

#[must_use]
pub const fn style_highlight() -> Style {
    Style::new()
        .fg(BG_MAIN)
        .bg(HIGHLIGHT)
        .add_modifier(Modifier::BOLD)
}

#[must_use]
pub const fn style_muted() -> Style {
    Style::new().fg(MUTED)
}

#[must_use]
pub const fn style_danger() -> Style {
    Style::new().fg(DANGER).add_modifier(Modifier::BOLD)
}

#[must_use]
pub const fn style_hud() -> Style {
    Style::new().fg(WHITE)
}

#[must_use]
pub const fn style_border() -> Style {
    Style::new().fg(BORDER)
}
