use crate::engine::renderer::Buffer;
use crate::games::dino::state::{DinoObstacleKind, DinoState};

const DINO_STAND: [&str; 3] = [" ▄██", "████", " ██ "];

const DINO_DUCK: [&str; 2] = ["      ", "▄█████"];

const DINO_JUMP: [&str; 3] = [" ▄██", "████", "▀  ▀"];

const BIRD_FRAME_A: [&str; 2] = ["/  ", " ══"];
const BIRD_FRAME_B: [&str; 2] = ["    ", "╲══"];

pub fn render(state: &DinoState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_ground(state, buffer);
    draw_clouds(state, buffer);
    draw_obstacles(state, buffer);
    draw_dino(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &DinoState, buffer: &mut Buffer) {
    let hi_text = format!("HI {:05}", state.high_score);
    buffer.print(state.bounds.width.saturating_sub(22), 0, &hi_text);

    let score_text = format!("{:05}", state.score.0);
    buffer.print_right(0, &score_text, 2);
}

fn draw_ground(state: &DinoState, buffer: &mut Buffer) {
    let ground_y = DinoState::ground_line(state.bounds);
    let scroll = state.ground_scroll % 4;

    for x in 0..state.bounds.width {
        let pattern_x = (x + scroll) % 4;
        let ch = match pattern_x {
            0 => '▁',
            1 => '▁',
            2 => '▂',
            _ => '▁',
        };
        buffer.place(x, ground_y, ch);
    }
}

fn draw_clouds(state: &DinoState, buffer: &mut Buffer) {
    let cloud_offset = (state.tick / 3) as u16;
    let cloud_positions: [(u16, u16); 3] = [
        (20_u16.wrapping_sub(cloud_offset % 60), 3),
        (50_u16.wrapping_sub(cloud_offset % 80), 2),
        (75_u16.wrapping_sub(cloud_offset % 90), 4),
    ];

    for (cx, cy) in &cloud_positions {
        if *cx < state.bounds.width.saturating_sub(5) && *cx > 0 {
            buffer.print(*cx, *cy, "═══");
        }
    }
}

fn draw_obstacles(state: &DinoState, buffer: &mut Buffer) {
    let ground_y = DinoState::ground_line(state.bounds);
    let stand = ground_y.saturating_sub(2);
    let bird_frame = state.tick % 10 < 5;

    for obs in &state.obstacles {
        if obs.col >= state.bounds.width {
            continue;
        }

        match obs.kind {
            DinoObstacleKind::SmallCactus => {
                buffer.place(obs.col, stand, '│');
                buffer.print(obs.col, stand + 1, "╥");
            }
            DinoObstacleKind::LargeCactus => {
                buffer.place(obs.col, stand.saturating_sub(1), '│');
                buffer.place(obs.col, stand, '┤');
                buffer.print(obs.col, stand + 1, "╥");
            }
            DinoObstacleKind::DoubleCactus => {
                buffer.place(obs.col, stand, '│');
                buffer.print(obs.col, stand + 1, "╥");
                if obs.col + 3 < state.bounds.width {
                    buffer.place(obs.col + 3, stand, '│');
                    buffer.print(obs.col + 3, stand + 1, "╥");
                }
            }
            DinoObstacleKind::LowBird => {
                let frames = if bird_frame {
                    &BIRD_FRAME_A
                } else {
                    &BIRD_FRAME_B
                };
                for (i, line) in frames.iter().enumerate() {
                    buffer.print(obs.col, stand + i as u16, line);
                }
            }
            DinoObstacleKind::HighBird => {
                let row = stand.saturating_sub(4);
                let frames = if bird_frame {
                    &BIRD_FRAME_A
                } else {
                    &BIRD_FRAME_B
                };
                for (i, line) in frames.iter().enumerate() {
                    buffer.print(obs.col, row + i as u16, line);
                }
            }
        }
    }
}

fn draw_dino(state: &DinoState, buffer: &mut Buffer) {
    let col = 8u16;

    if state.is_game_over {
        let sprite = &DINO_STAND;
        for (i, line) in sprite.iter().enumerate() {
            buffer.print(col, state.dino_row + i as u16, line);
        }
        return;
    }

    if state.is_ducking {
        for (i, line) in DINO_DUCK.iter().enumerate() {
            buffer.print(col, state.dino_row + i as u16, line);
        }
    } else if state.is_jumping {
        for (i, line) in DINO_JUMP.iter().enumerate() {
            buffer.print(col, state.dino_row + i as u16, line);
        }
    } else {
        let step = (state.tick / 4) % 2;
        let sprite = &DINO_STAND;
        for (i, line) in sprite.iter().enumerate() {
            buffer.print(col, state.dino_row + i as u16, line);
        }
        if step == 0 {
            buffer.print(col + 1, state.dino_row + 2, "█ ");
        } else {
            buffer.print(col + 1, state.dino_row + 2, " █");
        }
    }
}

fn draw_overlays(state: &DinoState, buffer: &mut Buffer) {
    if state.is_game_over {
        let cx = state.bounds.width / 2;
        let cy = state.bounds.height / 2;

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(2), "╔════════════╗");
        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "║ GAME  OVER ║");
        buffer.print(cx.saturating_sub(6), cy, "╚════════════╝");

        let sub = format!("Score: {:05}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sx = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sx, cy + 2, &sub);

        buffer.print(cx.saturating_sub(10), cy + 4, "Press any key to exit");
    }
}
