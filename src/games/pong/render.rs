use ratatui::style::Style;

use crate::engine::renderer::Buffer;
use crate::games::pong::state::PongState;
use crate::ui::theme;

pub fn render(state: &PongState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_divider(state, buffer);
    draw_paddle(
        buffer,
        PongState::left_paddle_x(),
        state.player_y,
        theme::HIGHLIGHT,
    );
    draw_paddle(buffer, state.right_paddle_x(), state.cpu_y, theme::DANGER);
    draw_ball(state, buffer);
    draw_overlay(state, buffer);
}

fn draw_hud(state: &PongState, buffer: &mut Buffer) {
    let score = format!("P1 {}   CPU {}", state.player_score, state.cpu_score);
    buffer.print(2, 0, &score, theme::style_hud());
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

fn draw_divider(state: &PongState, buffer: &mut Buffer) {
    let mid = state.bounds.width / 2;
    for y in (2..state.bounds.height.saturating_sub(1)).step_by(2) {
        buffer.place(mid, y, '\u{250a}', Style::new().fg(theme::MUTED));
    }
}

fn draw_paddle(buffer: &mut Buffer, x: u16, center_y: f32, color: ratatui::style::Color) {
    let half = PongState::PADDLE_HEIGHT / 2;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let center = center_y.round().max(0.0) as u16;
    for y in center.saturating_sub(half)..=center.saturating_add(half) {
        buffer.place(x, y, '\u{2588}', Style::new().fg(color));
    }
}

fn draw_ball(state: &PongState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let x = state.ball.x.round().max(0.0) as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let y = state.ball.y.round().max(0.0) as u16;
    buffer.place(x, y, 'O', Style::new().fg(theme::WHITE));
}

fn draw_overlay(state: &PongState, buffer: &mut Buffer) {
    if !state.is_game_over && !state.is_complete {
        return;
    }

    let msg = if state.is_complete {
        "YOU WIN"
    } else {
        "GAME OVER"
    };
    let x = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
    let y = state.bounds.height / 2;
    buffer.print(x, y, msg, theme::style_title());
    buffer.print(
        x.saturating_sub(4),
        y + 2,
        "[R] Retry",
        Style::new().fg(theme::SUCCESS),
    );
    buffer.print(
        x.saturating_sub(4),
        y + 3,
        "[Q] Quit to Menu",
        theme::style_muted(),
    );
}
