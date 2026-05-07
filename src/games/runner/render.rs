use crate::engine::renderer::Buffer;
use crate::games::runner::state::RunnerState;

const PLAYER_CAR: [&str; 3] = ["╒═══╕", "│◈█◈│", "╘═══╛"];
const ENEMY_CAR: [&str; 3] = ["╔═══╗", "║◈▓◈║", "╚═══╝"];
const ENEMY_TRUCK: [&str; 3] = ["╔═════╗", "║◈▓▓▓◈║", "╚═════╝"];

pub fn render(state: &RunnerState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_road(state, buffer);
    draw_obstacles(state, buffer);
    draw_player(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &RunnerState, buffer: &mut Buffer) {
    let score_text = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text);

    let pct = f32::from(state.speed.saturating_sub(30)) / 170.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let filled = (pct * 10.0) as u16;
    let bar: String = (0..10u16)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    let speed_text = format!("{}mph [{}]", state.speed, bar);
    buffer.print_right(0, &speed_text, 2);

    buffer.horizontal_line(1, 0, state.bounds.width, '─');
}

fn draw_road(state: &RunnerState, buffer: &mut Buffer) {
    let top = RunnerState::hud_height();
    let bot = state.bounds.height.saturating_sub(1);
    let w = state.bounds.width;

    for y in top..bot {
        buffer.place(0, y, '▐');
        buffer.place(1, y, '░');
    }

    for y in top..bot {
        buffer.place(w.saturating_sub(2), y, '░');
        buffer.place(w.saturating_sub(1), y, '▌');
    }

    let scroll = state.road_scroll % 8;
    for i in 0..RunnerState::lane_count().saturating_sub(1) {
        let y = state.lane_divider_y(i);
        for x in 2..w.saturating_sub(2) {
            // Pattern: 3 dash, 5 gap, repeat every 8 cols
            if (x + scroll) % 8 < 3 {
                buffer.place(x, y, '─');
            }
        }
    }

    buffer.horizontal_line(bot, 0, w, '═');
}

fn draw_obstacles(state: &RunnerState, buffer: &mut Buffer) {
    for car in &state.obstacles {
        if state.speed >= 80 {
            let trail_ch = if state.speed >= 140 { '▒' } else { '░' };
            let trail_len = (state.speed / 50).min(4);
            let cy = state.lane_y(car.lane);
            for t in 1..=trail_len {
                let tx = car.col + car.width + t;
                if tx < state.bounds.width.saturating_sub(2) {
                    if cy > 0 {
                        buffer.place(tx, cy - 1, trail_ch);
                    }
                    buffer.place(tx, cy, trail_ch);
                    if cy + 1 < state.bounds.height {
                        buffer.place(tx, cy + 1, trail_ch);
                    }
                }
            }
        }

        let sprites: &[&str; 3] = if car.width > 5 {
            &ENEMY_TRUCK
        } else {
            &ENEMY_CAR
        };
        let cy = state.lane_y(car.lane);
        for (offset, line) in sprites.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let y = cy.saturating_sub(1) + offset as u16;
            if car.col < state.bounds.width {
                buffer.print(car.col, y, line);
            }
        }
    }
}

fn draw_player(state: &RunnerState, buffer: &mut Buffer) {
    let cy = state.lane_y(state.player_lane);
    let col = RunnerState::player_col();

    for (offset, line) in PLAYER_CAR.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let y = cy.saturating_sub(1) + offset as u16;
        buffer.print(col, y, line);
    }
}

fn draw_overlays(state: &RunnerState, buffer: &mut Buffer) {
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

        buffer.print(cx.saturating_sub(10), cy + 5, "Press any key to exit");
    }
}
