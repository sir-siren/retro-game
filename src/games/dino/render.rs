//! Renders the Dino game state into the character buffer.

use crate::engine::renderer::Buffer;
use crate::games::dino::state::{DinoObstacleKind, DinoState};

/// Projects the dino domain state onto the render buffer.
pub fn render(state: &DinoState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_ground(state, buffer);
    draw_obstacles(state, buffer);
    draw_dino(state, buffer);
    draw_overlays(state, buffer);
}

/// Renders the top HUD row.
fn draw_hud(state: &DinoState, buffer: &mut Buffer) {
    let hud = format!(
        "Level: {}   Score: {:05}   Speed: {}mph   Lives: {}",
        state.level.0,
        state.score.0,
        state.speed * 8,
        state.lives.0,
    );
    buffer.print(2, 0, &hud);
}

/// Draws the ground line across the full viewport width.
fn draw_ground(state: &DinoState, buffer: &mut Buffer) {
    let row = DinoState::ground_line(state.bounds);
    for x in 0..state.bounds.width {
        buffer.place(x, row, '_');
    }
}

/// Renders each obstacle according to its kind.
fn draw_obstacles(state: &DinoState, buffer: &mut Buffer) {
    let ground = DinoState::ground_line(state.bounds);
    let stand = ground.saturating_sub(1);

    for obs in &state.obstacles {
        if obs.col >= state.bounds.width {
            continue;
        }

        match obs.kind {
            DinoObstacleKind::SmallCactus => {
                buffer.place(obs.col, stand.saturating_sub(1), '|');
                buffer.place(obs.col, stand, 'Y');
            }
            DinoObstacleKind::LargeCactus => {
                buffer.place(obs.col, stand.saturating_sub(2), '|');
                buffer.place(obs.col, stand.saturating_sub(1), '|');
                buffer.place(obs.col, stand, 'Y');
            }
            DinoObstacleKind::DoubleCactus => {
                buffer.place(obs.col, stand.saturating_sub(1), '|');
                buffer.place(obs.col, stand, 'Y');
                if obs.col + 2 < state.bounds.width {
                    buffer.place(obs.col + 2, stand.saturating_sub(1), '|');
                    buffer.place(obs.col + 2, stand, 'Y');
                }
            }
            DinoObstacleKind::LowBird => {
                // At dino head height — duck under this.
                let row = stand.saturating_sub(1);
                buffer.print(obs.col, row, ">o<");
            }
            DinoObstacleKind::HighBird => {
                // Well above dino — purely visual.
                let row = stand.saturating_sub(5);
                buffer.print(obs.col, row, ">o<");
            }
        }
    }
}

/// Draws the player dino character with hurt flicker.
fn draw_dino(state: &DinoState, buffer: &mut Buffer) {
    let col = 5u16;
    let flash = state.hurt_ticks > 0 && (state.hurt_ticks % 4) < 2;

    if flash {
        buffer.print(col, state.dino_row, "***");
        return;
    }

    if state.is_ducking {
        // Flat ducking pose — one row, wider.
        buffer.print(col, state.dino_row, "[o=]");
    } else {
        // Standing — head row above body row.
        buffer.print(col, state.dino_row.saturating_sub(1), " o ");
        buffer.print(col, state.dino_row, "[|]");
    }
}

/// Draws game-over or level-complete overlay messages.
fn draw_overlays(state: &DinoState, buffer: &mut Buffer) {
    if state.is_game_over {
        let msg = "GAME OVER - Press Any Key";
        let cx = state.bounds.width.saturating_sub(msg.len() as u16) / 2;
        let cy = state.bounds.height / 2;
        buffer.print(cx, cy, msg);
        let sub = format!("Score: {:05}  Level: {}", state.score.0, state.level.0);
        let sx = state.bounds.width.saturating_sub(sub.len() as u16) / 2;
        buffer.print(sx, cy + 1, &sub);
    }
}
