use ratatui::style::{Modifier, Style};

use crate::engine::renderer::Buffer;
use crate::games::dino::state::{DinoObstacleKind, DinoState};
use crate::ui::theme;

const DINO_STAND: [&str; 3] = [
    " \u{2584}\u{2588}\u{2588}",
    "\u{2588}\u{2588}\u{2588}\u{2588}",
    " \u{2588}\u{2588} ",
];
const DINO_DUCK: [&str; 2] = ["      ", "\u{2584}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}"];
const DINO_JUMP: [&str; 3] = [
    " \u{2584}\u{2588}\u{2588}",
    "\u{2588}\u{2588}\u{2588}\u{2588}",
    "\u{2580}  \u{2580}",
];
const BIRD_FRAME_A: [&str; 2] = ["/  ", " \u{2550}\u{2550}"];
const BIRD_FRAME_B: [&str; 2] = ["    ", "\u{2572}\u{2550}\u{2550}"];

/// Determine fg/bg based on day/night cycle.
/// Night starts at 700, day returns at 900, alternates every ~700 after.
const fn current_theme(score: u32) -> (Style, Style) {
    let is_night = if score < 700 {
        false
    } else {
        let cycle = score.saturating_sub(700) / 700;
        cycle % 2 == 0
    };

    if is_night {
        let body = Style::new()
            .fg(theme::DINO_NIGHT_FG)
            .bg(theme::DINO_NIGHT_BG);
        let ground = Style::new().fg(theme::DINO_GROUND).bg(theme::DINO_NIGHT_BG);
        (body, ground)
    } else {
        let body = Style::new().fg(theme::DINO_BODY);
        let ground = Style::new().fg(theme::DINO_GROUND);
        (body, ground)
    }
}

pub fn render(state: &DinoState, buffer: &mut Buffer) {
    let (body_style, ground_style) = current_theme(state.score.0);

    draw_hud(state, buffer);
    draw_ground(state, buffer, ground_style);
    draw_clouds(state, buffer);
    draw_obstacles(state, buffer, body_style);
    draw_dino(state, buffer, body_style);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &DinoState, buffer: &mut Buffer) {
    let hi_text = format!("HI {:05}", state.high_score);
    buffer.print(
        state.bounds.width.saturating_sub(22),
        0,
        &hi_text,
        theme::style_muted(),
    );

    // flash score at 100-point milestones
    let is_milestone_flash = state.score.0 > 0 && state.score.0 % 100 < 3 && state.tick % 6 < 3;

    let score_style = if is_milestone_flash {
        Style::new().fg(theme::BG_MAIN).bg(theme::WHITE)
    } else {
        theme::style_hud()
    };

    let score_text = format!("{:05}", state.score.0);
    buffer.print_right(0, &score_text, 2, score_style);
}

fn draw_ground(state: &DinoState, buffer: &mut Buffer, ground_style: Style) {
    let ground_y = DinoState::ground_line(state.bounds);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let scroll = state.ground_scroll.round().max(0.0) as u16 % 4;

    for x in 0..state.bounds.width {
        let pattern_x = (x + scroll) % 4;
        let ch = match pattern_x {
            2 => '\u{2582}',
            _ => '\u{2581}',
        };
        buffer.place(x, ground_y, ch, ground_style);
    }

    // pebble row below ground
    let pebble_y = ground_y.saturating_add(1);
    if pebble_y < state.bounds.height {
        let pebble_style = Style::new().fg(theme::MUTED);
        for x in 0..state.bounds.width {
            let pattern = (x.wrapping_add(scroll.wrapping_mul(2))) % 7;
            let ch = match pattern {
                1 => '.',
                4 => ',',
                _ => ' ',
            };
            buffer.place(x, pebble_y, ch, pebble_style);
        }
    }
}

fn draw_clouds(state: &DinoState, buffer: &mut Buffer) {
    let cloud_offset = u16::try_from(state.tick / 3).unwrap_or(u16::MAX);
    let cloud_positions: [(u16, u16); 3] = [
        (20_u16.wrapping_sub(cloud_offset % 60), 3),
        (50_u16.wrapping_sub(cloud_offset % 80), 2),
        (75_u16.wrapping_sub(cloud_offset % 90), 4),
    ];
    let cloud_style = Style::new().fg(theme::MUTED);

    for (cx, cy) in &cloud_positions {
        if *cx < state.bounds.width.saturating_sub(5) && *cx > 0 {
            buffer.print(*cx, *cy, "\u{2550}\u{2550}\u{2550}", cloud_style);
        }
    }
}

fn draw_obstacles(state: &DinoState, buffer: &mut Buffer, body_style: Style) {
    let ground_y = DinoState::ground_line(state.bounds);
    let stand = ground_y.saturating_sub(2);
    let bird_frame = state.tick % 10 < 5;
    let obstacle_style = body_style.add_modifier(Modifier::BOLD);

    for obs in &state.obstacles {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let col = obs.col.round().max(0.0) as u16;

        if col >= state.bounds.width {
            continue;
        }

        match obs.kind {
            DinoObstacleKind::SmallCactus => {
                buffer.place(col, stand, '\u{2502}', obstacle_style);
                buffer.print(col, stand + 1, "\u{2565}", obstacle_style);
            }
            DinoObstacleKind::LargeCactus => {
                buffer.place(col, stand.saturating_sub(1), '\u{2502}', obstacle_style);
                buffer.print(col, stand, "\u{2524}\u{251c}", obstacle_style);
                buffer.print(col, stand + 1, "\u{2565}\u{2565}", obstacle_style);
            }
            DinoObstacleKind::CactusCluster => {
                buffer.place(col, stand, '\u{2502}', obstacle_style);
                buffer.print(col, stand + 1, "\u{2565}", obstacle_style);
                if col + 2 < state.bounds.width {
                    buffer.place(col + 2, stand.saturating_sub(1), '\u{2502}', obstacle_style);
                    buffer.print(col + 2, stand, "\u{2524}", obstacle_style);
                    buffer.print(col + 2, stand + 1, "\u{2565}", obstacle_style);
                }
                if col + 5 < state.bounds.width {
                    buffer.place(col + 5, stand, '\u{2502}', obstacle_style);
                    buffer.print(col + 5, stand + 1, "\u{2565}", obstacle_style);
                }
            }
            DinoObstacleKind::LowBird => {
                let frames = if bird_frame {
                    &BIRD_FRAME_A
                } else {
                    &BIRD_FRAME_B
                };
                for (i, line) in frames.iter().enumerate() {
                    let row = stand + u16::try_from(i).unwrap_or(0);
                    buffer.print(col, row, line, obstacle_style);
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
                    buffer.print(
                        col,
                        row + u16::try_from(i).unwrap_or(0),
                        line,
                        obstacle_style,
                    );
                }
            }
        }
    }
}

fn draw_dino(state: &DinoState, buffer: &mut Buffer, body_style: Style) {
    let col = 8u16;
    let dino_row = state.dino_row();

    if state.status.is_game_over() {
        for (i, line) in DINO_STAND.iter().enumerate() {
            buffer.print(
                col,
                dino_row + u16::try_from(i).unwrap_or(0),
                line,
                body_style,
            );
        }
        return;
    }

    if state.status.is_ducking() {
        for (i, line) in DINO_DUCK.iter().enumerate() {
            buffer.print(
                col,
                dino_row + u16::try_from(i).unwrap_or(0),
                line,
                body_style,
            );
        }
    } else if state.status.is_jumping() {
        for (i, line) in DINO_JUMP.iter().enumerate() {
            buffer.print(
                col,
                dino_row + u16::try_from(i).unwrap_or(0),
                line,
                body_style,
            );
        }
    } else {
        let step = (state.tick / 4) % 2;
        for (i, line) in DINO_STAND.iter().enumerate() {
            buffer.print(
                col,
                dino_row + u16::try_from(i).unwrap_or(0),
                line,
                body_style,
            );
        }
        if step == 0 {
            buffer.print(col + 1, dino_row + 2, "\u{2588} ", body_style);
        } else {
            buffer.print(col + 1, dino_row + 2, " \u{2588}", body_style);
        }
    }
}

fn draw_overlays(state: &DinoState, buffer: &mut Buffer) {
    if state.status.is_game_over() {
        let cx = state.bounds.width / 2;
        let cy = state.bounds.height / 2;
        let border_style = Style::new().fg(theme::DANGER);
        let text_style = theme::style_title();

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(2), "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}", border_style);
        buffer.print(
            cx.saturating_sub(6),
            cy.saturating_sub(1),
            "\u{2551} GAME  OVER \u{2551}",
            border_style,
        );
        buffer.print(cx.saturating_sub(6), cy, "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}", border_style);

        let sub = format!("Score: {:05}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sub_col = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sub_col, cy + 2, &sub, text_style);

        let retry_style = Style::new().fg(theme::SUCCESS);
        let quit_style = theme::style_muted();
        buffer.print(cx.saturating_sub(8), cy + 4, "[R] Retry", retry_style);
        buffer.print(cx.saturating_sub(8), cy + 5, "[Q] Quit to Menu", quit_style);
    }
}
