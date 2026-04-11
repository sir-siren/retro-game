use crate::engine::renderer::Buffer;
use crate::games::snake::state::SnakeState;

pub fn render(state: &SnakeState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_food(state, buffer);
    draw_snake(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &SnakeState, buffer: &mut Buffer) {
    let score_text: String = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text);

    buffer.horizontal_line(1, 0, state.bounds.width, '─');
}

fn draw_food(state: &SnakeState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if state.food.x >= 0 && state.food.y >= 0 {
        buffer.place(state.food.x as u16, state.food.y as u16, '□');
    }
}

fn draw_snake(state: &SnakeState, buffer: &mut Buffer) {
    for (i, seg) in state.segments.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        if seg.x >= 0 && seg.y >= 0 {
            let ch = if i == 0 { '▣' } else { '█' };
            buffer.place(seg.x as u16, seg.y as u16, ch);
        }
    }
}

fn draw_overlays(state: &SnakeState, buffer: &mut Buffer) {
    if state.is_game_over {
        let cx: u16 = state.bounds.width / 2;
        let cy: u16 = state.bounds.height / 2;

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "╔════════════╗");
        buffer.print(cx.saturating_sub(6), cy, "║ GAME  OVER ║");
        buffer.print(cx.saturating_sub(6), cy + 1, "╚════════════╝");

        let sub: String = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sx: u16 = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sx, cy + 3, &sub);
    }
}
