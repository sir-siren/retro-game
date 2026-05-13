use ratatui::style::Style;

use crate::engine::renderer::Buffer;
use crate::games::snake::state::SnakeState;
use crate::ui::theme;

pub fn render(state: &SnakeState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_food(state, buffer);
    draw_snake(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &SnakeState, buffer: &mut Buffer) {
    let score_text: String = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text, theme::style_hud());

    buffer.horizontal_line(
        1,
        0,
        state.bounds.width,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_food(state: &SnakeState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if state.food.x >= 0 && state.food.y >= 0 {
        let food_style = Style::new().fg(theme::SNAKE_FOOD);
        buffer.place(
            state.food.x as u16,
            state.food.y as u16,
            '\u{25a1}',
            food_style,
        );
    }
}

fn draw_snake(state: &SnakeState, buffer: &mut Buffer) {
    let head_style = Style::new().fg(theme::SNAKE_HEAD);
    let body_style = Style::new().fg(theme::SNAKE_BODY);

    for (i, seg) in state.segments.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        if seg.x >= 0 && seg.y >= 0 {
            let (ch, style) = if i == 0 {
                ('\u{25a3}', head_style)
            } else {
                ('\u{2588}', body_style)
            };
            buffer.place(seg.x as u16, seg.y as u16, ch, style);
        }
    }
}

fn draw_overlays(state: &SnakeState, buffer: &mut Buffer) {
    if state.is_game_over {
        let cx: u16 = state.bounds.width / 2;
        let cy: u16 = state.bounds.height / 2;
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

        let sub: String = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sub_col: u16 = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sub_col, cy + 3, &sub, text_style);

        let retry_style = Style::new().fg(theme::SUCCESS);
        let quit_style = theme::style_muted();
        buffer.print(cx.saturating_sub(8), cy + 5, "[R] Retry", retry_style);
        buffer.print(cx.saturating_sub(8), cy + 6, "[Q] Quit to Menu", quit_style);
    }
}
