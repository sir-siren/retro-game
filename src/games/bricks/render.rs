use crate::engine::renderer::Buffer;
use crate::games::bricks::state::BricksState;

pub fn render(state: &BricksState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_bricks(state, buffer);
    draw_paddle(state, buffer);
    draw_ball(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &BricksState, buffer: &mut Buffer) {
    let score_text = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text);

    let hearts: String = (0..state.lives.0).map(|_| '♥').collect();
    buffer.print_right(0, &hearts, 2);

    buffer.horizontal_line(1, 0, state.bounds.width, '─');
}

fn draw_bricks(state: &BricksState, buffer: &mut Buffer) {
    for brick in &state.bricks {
        if !brick.is_alive {
            continue;
        }
        let txt: &str = if brick.hp >= 2 {
            "████"
        } else {
            "▓▓▓▓"
        };
        buffer.print(brick.col, brick.row, txt);
    }
}

fn draw_paddle(state: &BricksState, buffer: &mut Buffer) {
    let paddle_row = state.paddle_row();
    let paddle_str: String = (0..state.paddle_width).map(|_| '▀').collect();
    buffer.print(state.paddle_col, paddle_row, &paddle_str);
}

fn draw_ball(state: &BricksState, buffer: &mut Buffer) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let bx = state.ball.x.round() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let by = state.ball.y.round() as u16;
    buffer.place(bx, by, '■');
}

fn draw_overlays(state: &BricksState, buffer: &mut Buffer) {
    if state.showing_clear {
        let msg = "═══ LEVEL CLEAR ═══";
        #[allow(clippy::cast_possible_truncation)]
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }

    if state.is_game_over {
        let cx = state.bounds.width / 2;
        let cy = state.bounds.height / 2;

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "╔════════════╗");
        buffer.print(cx.saturating_sub(6), cy, "║ GAME  OVER ║");
        buffer.print(cx.saturating_sub(6), cy + 1, "╚════════════╝");

        let sub = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sx = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sx, cy + 3, &sub);
    }
}
