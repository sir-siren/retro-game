use crate::engine::renderer::Buffer;
use crate::games::runner::state::{ObstacleType, RunnerState};

/// Pushes the domain state into the physical render plane.
pub fn render(state: &RunnerState, buffer: &mut Buffer) {
    // Fill HUD
    let hud = format!(
        "Level: {}   Score: {:04}   Speed: {}mph   Lives: {}",
        state.level.0, state.score.0, state.speed * 10, state.lives.0
    );
    buffer.print(2, 0, &hud);

    let ground = state.ground_row();
    let width = state.bounds.width;
    
    // Draw ground string
    let mut ground_layer = String::new();
    for _ in 0..width {
        ground_layer.push('-');
    }
    buffer.print(0, ground, &ground_layer);

    // Obstacles
    for obs in &state.obstacles {
        if obs.col >= width {
            continue;
        }
        match obs.kind {
            ObstacleType::Single => {
                buffer.place(obs.col, ground.saturating_sub(1), '|');
            }
            ObstacleType::Double => {
                buffer.place(obs.col, ground.saturating_sub(1), '|');
                if obs.col + 1 < width {
                    buffer.place(obs.col + 1, ground.saturating_sub(1), '|');
                }
            }
            ObstacleType::Ceiling => {
                let ceil_h = state.bounds.height.saturating_sub(state.bounds.height / 2).saturating_sub(1);
                for i in 1..=ceil_h {
                    buffer.place(obs.col, i, 'T');
                }
            }
        }
    }

    // Player
    if state.hurt_ticks > 0 && (state.hurt_ticks % 4) < 2 {
        buffer.print(4, state.player_row, "***");
    } else {
        buffer.print(4, state.player_row, "[O]");
    }

    if state.is_game_over {
        let msg = "GAME OVER - Press Any Key";
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
    }
}
