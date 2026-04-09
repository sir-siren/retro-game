use crate::engine::renderer::Buffer;
use crate::games::snake::state::SnakeState;

/// Projects grid blocks onto physical text buffers.
pub fn render(state: &SnakeState, buffer: &mut Buffer) {
    let hud = format!(
        "Level: {}   Score: {:04}   Lives: {}",
        state.level.0, state.score.0, state.lives.0
    );
    buffer.print(2, 0, &hud);

    // Render food
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    buffer.place(state.food.x as u16, state.food.y as u16, '*');

    // Walls
    for wall in &state.walls {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        buffer.place(wall.x as u16, wall.y as u16, 'X');
    }

    // Body
    for (i, seg) in state.segments.iter().enumerate() {
        let ch = if i == 0 { '@' } else { '#' };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        buffer.place(seg.x as u16, seg.y as u16, ch);
    }

    if state.showing_clear {
        let msg = "LEVEL CLEAR";
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }

    if state.is_game_over {
        let msg = "GAME OVER - Press Any Key";
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }
}
