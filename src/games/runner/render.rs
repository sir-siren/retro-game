//! Renders Runner game state into the character buffer.

use crate::engine::renderer::Buffer;
use crate::games::runner::state::{ObstacleType, RunnerState};

/// Projects domain state onto the render plane.
pub fn render(state: &RunnerState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_ground(state, buffer);
    draw_obstacles(state, buffer);
    draw_player(state, buffer);
    draw_overlays(state, buffer);
}

/// Renders the top status row.
fn draw_hud(state: &RunnerState, buffer: &mut Buffer) {
    let hud = format!(
        "Level: {}   Score: {:04}   Speed: {}mph   Lives: {}",
        state.level.0,
        state.score.0,
        u32::from(state.speed) * 10,
        state.lives.0,
    );
    buffer.print(2, 0, &hud);
}

/// Draws the scrolling ground line.
fn draw_ground(state: &RunnerState, buffer: &mut Buffer) {
    let ground = state.ground_row();
    for x in 0..state.bounds.width {
        buffer.place(x, ground, '-');
    }
}

/// Renders each active obstacle.
fn draw_obstacles(state: &RunnerState, buffer: &mut Buffer) {
    let ground = state.ground_row();
    let width = state.bounds.width;

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
                let top = 1u16;
                let ceil_bottom = state.bounds.height / 3;
                for row in top..=ceil_bottom {
                    buffer.place(obs.col, row, 'T');
                }
            }
        }
    }
}

/// Renders the player with hurt flicker.
fn draw_player(state: &RunnerState, buffer: &mut Buffer) {
    let flash = state.hurt_ticks > 0 && (state.hurt_ticks % 4) < 2;
    if flash {
        buffer.print(4, state.player_row, "***");
    } else {
        buffer.print(4, state.player_row, "[O]");
    }
}

/// Renders game-over overlay.
fn draw_overlays(state: &RunnerState, buffer: &mut Buffer) {
    if state.is_game_over {
        let msg = "GAME OVER - Press Any Key";
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
        let sub = format!("Score: {:04}  Level: {}", state.score.0, state.level.0);
        let sx = state.bounds.width.saturating_sub(sub.len() as u16) / 2;
        buffer.print(sx, cy + 1, &sub);
    }
}
