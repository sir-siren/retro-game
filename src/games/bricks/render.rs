use ratatui::style::{Modifier, Style};

use crate::engine::renderer::Buffer;
use crate::games::bricks::state::BricksState;
use crate::ui::theme;

pub fn render(state: &BricksState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_bricks(state, buffer);
    draw_paddle(state, buffer);
    draw_ball(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &BricksState, buffer: &mut Buffer) {
    let score_text = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text, theme::style_hud());

    let hearts: String = (0..state.lives.0).map(|_| '\u{2665}').collect();
    buffer.print_right(0, &hearts, 2, Style::new().fg(theme::DANGER));

    buffer.horizontal_line(
        1,
        0,
        state.bounds.width,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_bricks(state: &BricksState, buffer: &mut Buffer) {
    for brick in &state.bricks {
        if !brick.is_alive {
            continue;
        }

        let style = if brick.hp >= 2 {
            // armored -- white and bold
            Style::new()
                .fg(theme::BRICK_ARMORED)
                .add_modifier(Modifier::BOLD)
        } else {
            // color by row -- wrap around the palette
            let color_idx = (brick.row as usize) % theme::BRICK_COLORS.len();
            Style::new().fg(theme::BRICK_COLORS[color_idx])
        };

        let txt: &str = if brick.hp >= 2 {
            "\u{2588}\u{2588}\u{2588}\u{2588}"
        } else {
            "\u{2593}\u{2593}\u{2593}\u{2593}"
        };
        buffer.print(brick.col, brick.row, txt, style);
    }
}

fn draw_paddle(state: &BricksState, buffer: &mut Buffer) {
    let paddle_row = state.paddle_row();
    let paddle_str: String = (0..state.paddle_width).map(|_| '\u{2580}').collect();
    buffer.print(
        state.paddle_col,
        paddle_row,
        &paddle_str,
        Style::new().fg(theme::HIGHLIGHT),
    );
}

fn draw_ball(state: &BricksState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ball_col = state.ball.x.round() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ball_row = state.ball.y.round() as u16;
    buffer.place(
        ball_col,
        ball_row,
        '\u{25a0}',
        Style::new().fg(theme::WHITE),
    );
}

fn draw_overlays(state: &BricksState, buffer: &mut Buffer) {
    if state.showing_clear {
        let msg = "\u{2550}\u{2550}\u{2550} LEVEL CLEAR \u{2550}\u{2550}\u{2550}";
        #[allow(clippy::cast_possible_truncation)]
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(
            cx,
            cy,
            msg,
            Style::new().fg(theme::SUCCESS).add_modifier(Modifier::BOLD),
        );
    }

    if state.is_game_over {
        let cx = state.bounds.width / 2;
        let cy = state.bounds.height / 2;
        let border_style = Style::new().fg(theme::DANGER);
        let text_style = theme::style_title();

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}", border_style);
        buffer.print(
            cx.saturating_sub(6),
            cy,
            "\u{2551} GAME  OVER \u{2551}",
            border_style,
        );
        buffer.print(cx.saturating_sub(6), cy + 1, "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}", border_style);

        let sub = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sub_col = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sub_col, cy + 3, &sub, text_style);

        let retry_style = Style::new().fg(theme::SUCCESS);
        let quit_style = theme::style_muted();
        buffer.print(cx.saturating_sub(8), cy + 5, "[R] Retry", retry_style);
        buffer.print(cx.saturating_sub(8), cy + 6, "[Q] Quit to Menu", quit_style);
    }
}
