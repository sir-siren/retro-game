use crate::engine::renderer::Buffer;
use crate::games::bricks::state::BricksState;

/// Paints projection into string slices.
pub fn render(state: &BricksState, buffer: &mut Buffer) {
    let hud: String = format!(
        "Level: {}   Score: {:04}   Lives: {}",
        state.level.0, state.score.0, state.lives.0
    );
    buffer.print(2, 0, &hud);

    // Bricks
    for brick in &state.bricks {
        if !brick.is_alive {
            continue;
        }
        let txt: &str = if brick.hp >= 2 { "[--]" } else { "[##]" };
        buffer.print(brick.col, brick.row, txt);
    }

    // Paddle
    let paddle_row: u16 = state.bounds.height.saturating_sub(2);
    buffer.print(state.paddle_col, paddle_row, "=====");

    // Ball
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let bx: u16 = state.ball.x.round() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let by: u16 = state.ball.y.round() as u16;
    buffer.place(bx, by, 'o');

    if state.showing_clear {
        let msg: &str = "LEVEL CLEAR";
        let cx: u16 = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy: u16 = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }

    if state.is_game_over {
        let msg: &str = "GAME OVER - Press Any Key";
        let cx: u16 = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy: u16 = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }
}
