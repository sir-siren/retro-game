use ratatui::style::Style;

use crate::engine::renderer::Buffer;
use crate::games::flappy::state::FlappyState;
use crate::ui::theme;

pub fn render(state: &FlappyState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_pipes(state, buffer);
    draw_bird(state, buffer);
    draw_ground(state, buffer);
    draw_overlay(state, buffer);
}

fn draw_hud(state: &FlappyState, buffer: &mut Buffer) {
    buffer.print(
        2,
        0,
        &format!("Score: {}", state.score.0),
        theme::style_hud(),
    );
    buffer.print_right(
        0,
        &format!("Level {}", state.level.0),
        2,
        theme::style_muted(),
    );
    buffer.horizontal_line(
        1,
        0,
        state.bounds.width,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_pipes(state: &FlappyState, buffer: &mut Buffer) {
    let pipe_style = Style::new().fg(theme::SUCCESS);
    for pipe in &state.pipes {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let x = pipe.x.round().max(0.0) as u16;
        for dx in 0..4 {
            let col = x.saturating_add(dx);
            for y in 2..state.ground_y() {
                let is_gap = y >= pipe.gap_y && y <= pipe.gap_y.saturating_add(state.gap_height());
                if !is_gap {
                    buffer.place(col, y, '\u{2588}', pipe_style);
                }
            }
        }
    }
}

fn draw_bird(state: &FlappyState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let y = state.bird_y.round().max(0.0) as u16;
    let sprite = if state.tick % 8 < 4 { ">O" } else { ">o" };
    buffer.print(
        FlappyState::BIRD_X,
        y,
        sprite,
        Style::new().fg(theme::TITLE_GOLD),
    );
}

fn draw_ground(state: &FlappyState, buffer: &mut Buffer) {
    let y = state.ground_y();
    let style = Style::new().fg(theme::DINO_GROUND);
    for x in 0..state.bounds.width {
        let ch = if (x + state.ground_scroll) % 5 == 0 {
            '\u{2593}'
        } else {
            '\u{2581}'
        };
        buffer.place(x, y, ch, style);
    }
}

fn draw_overlay(state: &FlappyState, buffer: &mut Buffer) {
    if !state.is_game_over {
        return;
    }

    let y = state.bounds.height / 2;
    buffer.print(
        state.bounds.width.saturating_sub(9) / 2,
        y,
        "GAME OVER",
        theme::style_title(),
    );
    buffer.print(
        state.bounds.width.saturating_sub(9) / 2,
        y + 2,
        "[R] Retry",
        Style::new().fg(theme::SUCCESS),
    );
    buffer.print(
        state.bounds.width.saturating_sub(16) / 2,
        y + 3,
        "[Q] Quit to Menu",
        theme::style_muted(),
    );
}
